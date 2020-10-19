import {ethers, run, waffle} from "@nomiclabs/buidler";
const {deployMockContract} = waffle;
import web3 from "web3";
import {Signer} from "ethers";
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
let admin: Signer;
let owner: Signer;
let buyer: Signer;
let oracle: Signer;

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
      await oracle.getAddress(),
      jobId,
    );

    const Escrow = await new EscrowFactory(admin);
    const Proxy = await new StaticProxyFactory(owner);

    const escrowNaked = await Escrow.deploy();

    const data = getInitializerData(
      Escrow,
      [comptroller.address, "seller@upi"],
      "initialize(address,string)",
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
        await escrow.address,
        await buyer.getAddress(),
        ethers.utils.parseEther("1"),
        "test@upi",
      ),
    )
      .to.emit(escrow, "AmountLocked")
      .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));
  });

  it("should fail requestFiatPayment when escrow doesn't have enough funds", async function () {
    await expect(
      comptroller.requestFiatPayment(
        await escrow.address,
        await buyer.getAddress(),
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
          await escrow.address,
          await buyer.getAddress(),
          ethers.utils.parseEther("1"),
          "test@upi",
        ),
      )
        .to.emit(escrow, "AmountLocked")
        .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));

      expect(await comptroller.getBalance(), "fees [initial]").to.equal(0);

      let hash = web3.utils.soliditySha3(
        {t: "address", v: comptroller.address},
        {t: "uint256", v: 1},
      ) as string;

      // complete a successful payment
      await expect(
        escrow.connect(oracle).fulfillFiatPayment(hash, true),
        "fulfillFiatPayment",
      ).to.be.not.reverted;

      expect(await comptroller.getBalance(), "fees [final]").to.equal(
        ethers.utils.parseEther("0.0049"),
      );
    });
  });
});
