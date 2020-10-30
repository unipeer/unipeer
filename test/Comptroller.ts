import {ethers, run, waffle, web3} from "hardhat";
import {SignerWithAddress} from "@nomiclabs/hardhat-ethers/dist/src/signer-with-address";

const {deployMockContract} = waffle;
import {expect} from "chai";

import {getInitializerData} from "../utils";

import {
  ComptrollerFactory,
  EscrowFactory,
  StaticProxyFactory,
  Comptroller as ComptrollerContract,
  Escrow as EscrowContract,
  StaticProxy as StaticProxyContract,
} from "../types";

import LinkTokenABI from "./abi/LinkToken.json";
import OracleABI from "./abi/Oracle.json";

let comptroller: ComptrollerContract;
let escrow: EscrowContract;
let admin: SignerWithAddress;
let owner: SignerWithAddress;
let buyer: SignerWithAddress;
let oracle: SignerWithAddress;

describe("Comptroller", function () {
  beforeEach(async function () {
    [admin, owner, buyer, oracle] = await ethers.getSigners();

    const mockLink = await deployMockContract(admin, LinkTokenABI);
    await mockLink.mock.balanceOf.returns(ethers.utils.parseEther("999999"));
    await mockLink.mock.transferAndCall.returns(true);
    //const mockOracle = await deployMockContract(admin, OracleABI);

    const Comptroller = await new ComptrollerFactory(admin);

    const jobId = web3.utils.toHex("0d69f6d174a4446c9a7ffa21cd0f687c");
    comptroller = await Comptroller.deploy(
      mockLink.address,
      oracle.address,
      jobId,
    );

    const Escrow = await new EscrowFactory(admin);
    const Proxy = await new StaticProxyFactory(owner);

    const escrowNaked = await Escrow.deploy();

    const data = getInitializerData(
      Escrow,
      [owner.address, comptroller.address, "seller@upi"],
      "initialize(address,address,string)",
    );
    const proxy = await Proxy.deploy(escrowNaked.address, data);

    escrow = Escrow.attach(proxy.address).connect(owner);
  });

  it("should correctly create a fiat payment request", async function () {
    // Deposit funds in the escrow
    await owner.sendTransaction({
      to: escrow.address,
      value: ethers.utils.parseEther("10"),
    });

    await expect(
      comptroller.requestFiatPayment(
        escrow.address,
        buyer.address,
        ethers.utils.parseEther("1"),
        "test@upi",
      ),
    )
      .to.emit(escrow, "AmountLocked")
      .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));
  });

  it("should not revert when calling createFiatPaymentWithLinkRequest", async function () {
    await expect(
      comptroller.createFiatPaymentWithLinkRequest(
        escrow.address,
        buyer.address,
        ethers.utils.parseEther("1"),
        "test@upi",
      ),
    ).to.be.not.reverted;
  });

  it("should not allow just anyone to call requestFiatPaymentWithLink", async function () {
    await expect(
      comptroller.requestFiatPaymentWithLink(
        escrow.address,
        buyer.address,
        ethers.utils.parseEther("1"),
        "test@upi",
      ),
    ).to.be.reverted;
  });

  it("should fail requestFiatPayment when escrow doesn't have enough funds", async function () {
    await expect(
      comptroller.requestFiatPayment(
        escrow.address,
        buyer.address,
        ethers.utils.parseEther("1"),
        "test@upi",
      ),
    ).to.be.revertedWith("Comptroller: not enough funds in escrow");
  });

  // TOOD: add tests for withdrawFees function
  describe("withdrawFees", function () {
    it("should be able to accept fees from escrow", async function () {
      const amount = ethers.utils.parseEther("10");
      await owner.sendTransaction({
        to: escrow.address,
        value: amount,
      });
      expect(await escrow.getUnlockedBalance(), "balance [initial]").to.equal(
        amount,
      );

      // create a payment request
      await expect(
        comptroller.requestFiatPayment(
          escrow.address,
          buyer.address,
          ethers.utils.parseEther("1"),
          "test@upi",
        ),
      )
        .to.emit(escrow, "AmountLocked")
        .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));

      expect(
        await web3.eth.getBalance(comptroller.address),
        "fees [initial]",
      ).to.equal("0");

      let hash = web3.utils.soliditySha3(
        {t: "address", v: comptroller.address},
        {t: "uint256", v: 1},
      ) as string;

      // complete a successful payment
      await expect(
        escrow.connect(oracle).fulfillFiatPayment(hash, true),
        "fulfillFiatPayment",
      ).to.be.not.reverted;

      expect(
        await web3.eth.getBalance(comptroller.address),
        "fees [final]",
      ).to.equal(ethers.utils.parseEther("0.0049"));
    });
  });
});
