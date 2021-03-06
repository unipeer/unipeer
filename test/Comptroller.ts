import { ethers, waffle, web3 } from "hardhat";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/dist/src/signer-with-address";

const { deployMockContract } = waffle;
import { expect } from "chai";

import {
  Comptroller__factory,
  Comptroller as ComptrollerContract,
  Escrow__factory,
  Escrow as EscrowContract,
} from "../types";

import LinkTokenABI from "@chainlink/contracts/abi/v0.4/LinkTokenInterface.json";

let comptroller: ComptrollerContract;
let escrow: EscrowContract;

let admin: SignerWithAddress;
let owner: SignerWithAddress;
let buyer: SignerWithAddress;
let oracle: SignerWithAddress;

describe("Comptroller", function () {
  beforeEach(async function () {
    [admin, owner, buyer, oracle] = await ethers.getSigners();

    const mockLink = await deployMockContract(
      admin,
      LinkTokenABI.compilerOutput.abi,
    );
    await mockLink.mock.balanceOf.returns(ethers.utils.parseEther("999999"));
    await mockLink.mock.transferAndCall.returns(true);
    //const mockOracle = await deployMockContract(admin, OracleABI);

    const Comptroller = new Comptroller__factory(admin);

    comptroller = await Comptroller.deploy(
      mockLink.address,
      oracle.address,
      web3.utils.toHex("0d69f6d174a4446c9a7ffa21cd0f687c"),
    );

    const Escrow = new Escrow__factory(admin);

    escrow = await Escrow.deploy(
      owner.address,
      comptroller.address,
      "seller@upi",
    );
    escrow = escrow.connect(owner);
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

  it("should fail requestFiatPayment with Status.FINALIZE_ONLY", async function () {
    // Deposit funds in the escrow
    await owner.sendTransaction({
      to: escrow.address,
      value: ethers.utils.parseEther("10"),
    });

    // Change status
    await expect(comptroller.setStatus(2), "setStatus")
      .to.emit(comptroller, "StatusChanged")
      .withArgs(3, 2);

    await expect(
      comptroller.requestFiatPayment(
        escrow.address,
        buyer.address,
        ethers.utils.parseEther("1"),
        "test@upi",
      ),
    ).to.be.revertedWith("invalid contract status");
  });

  describe("withdrawFees", function () {
    let hash: string;

    beforeEach(async function () {
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

      hash = web3.utils.soliditySha3(
        { t: "address", v: comptroller.address },
        { t: "uint256", v: 1 },
      ) as string;
    });

    it("should be able to accept fees from escrow", async function () {
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

    it("can withdraw fees successfully", async function () {
      // complete a successful payment
      await expect(
        escrow.connect(oracle).fulfillFiatPayment(hash, true),
        "fulfillFiatPayment",
      ).to.be.not.reverted;

      expect(
        await web3.eth.getBalance(comptroller.address),
        "fees [final]",
      ).to.equal(ethers.utils.parseEther("0.0049"));

      // withdraw fees
      await expect(
        comptroller.withdrawFees(
          admin.address,
          ethers.utils.parseEther("0.0049"),
        ),
        "withdrawFees",
      ).to.be.not.reverted;
    });

    it("should not allow withdrawing fees with Status.STOPPED", async function () {
      // complete a successful payment
      await expect(
        escrow.connect(oracle).fulfillFiatPayment(hash, true),
        "fulfillFiatPayment",
      ).to.be.not.reverted;

      expect(
        await web3.eth.getBalance(comptroller.address),
        "fees [final]",
      ).to.equal(ethers.utils.parseEther("0.0049"));

      await expect(comptroller.setStatus(0), "setStatus")
        .to.emit(comptroller, "StatusChanged")
        .withArgs(3, 0);

      // withdraw fees
      await expect(
        comptroller.withdrawFees(
          admin.address,
          ethers.utils.parseEther("0.0049"),
        ),
        "withdrawFees",
      ).to.be.revertedWith("invalid contract status");
    });
  });
});
