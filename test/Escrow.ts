import {ethers, run} from "@nomiclabs/buidler";
import {Signer} from "ethers";

import {expect} from "chai";

import {
  EscrowFactory,
  StaticProxyFactory,
  Escrow as EscrowContract,
  StaticProxy as StaticProxyContract,
} from "../types";

import {getInitializerData} from "../utils";

let escrow: EscrowContract;
let admin: Signer;
let owner: Signer;
let comptroller: Signer;
let oracle: Signer;
let buyer: Signer;

describe("Escrow", function () {
  beforeEach(async function () {
    [admin, owner, comptroller, oracle, buyer] = await ethers.getSigners();
    const Escrow = await new EscrowFactory(admin);
    const Proxy = await new StaticProxyFactory(owner);

    const escrowNaked = await Escrow.deploy();

    const data = getInitializerData(
      Escrow,
      [await comptroller.getAddress(), "test@upi"],
      "initialize(address,string)",
    );
    const proxy = await Proxy.deploy(escrowNaked.address, data);

    escrow = Escrow.attach(proxy.address).connect(owner);
  });

  it("can deposit additional funds to the contract", async function () {
    expect(await escrow.getUnlockedBalance()).to.equal(0);

    const amount = ethers.utils.parseEther("1.0");
    await owner.sendTransaction({
      to: escrow.address,
      value: amount,
    });

    expect(await escrow.getUnlockedBalance()).to.equal(amount);
  });

  it("can withdraw funds from the contract", async function () {
    const amount = ethers.utils.parseEther("1");
    await owner.sendTransaction({
      to: escrow.address,
      value: amount,
    });

    expect(await escrow.getUnlockedBalance()).to.equal(amount);

    await escrow.withdraw(amount, await owner.getAddress());
    expect(await escrow.getUnlockedBalance()).to.equal(0);
  });

  it("should correctly report unlocked balance", async function () {
    const amount = ethers.utils.parseEther("10");
    await owner.sendTransaction({
      to: escrow.address,
      value: amount,
    });
    expect(await escrow.getUnlockedBalance(), "balance [initial]").to.equal(
      amount,
    );

    await expect(
      escrow
        .connect(comptroller)
        .expectResponseFor(
          ethers.constants.AddressZero,
          ethers.utils.formatBytes32String("1"),
          await buyer.getAddress(),
          ethers.utils.parseEther("1"),
        ),
      "expectResponseFor",
    )
      .to.emit(escrow, "AmountLocked")
      .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));

    expect(await escrow.getUnlockedBalance(), "Balance after locked").to.equal(
      ethers.utils.parseEther("8.9951"),
    );
  });

  it("should not withdraw more that unlocked balance", async function () {
    const amount = ethers.utils.parseEther("10");
    await owner.sendTransaction({
      to: escrow.address,
      value: amount,
    });
    expect(await escrow.getUnlockedBalance()).to.equal(amount);

    await expect(
      escrow
        .connect(comptroller)
        .expectResponseFor(
          ethers.constants.AddressZero,
          ethers.utils.formatBytes32String("1"),
          await buyer.getAddress(),
          ethers.utils.parseEther("1"),
        ),
      "expectResponseFor",
    )
      .to.emit(escrow, "AmountLocked")
      .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));

    expect(await escrow.getUnlockedBalance(), "Balance after locked").to.equal(
      ethers.utils.parseEther("8.9951"),
    );

    await expect(
      escrow.withdraw(ethers.utils.parseEther("10"), await owner.getAddress()),
    ).to.be.reverted;
  });

  describe("fulfillFiatPayment", function () {
    it("should correctly unlock funds after a failed fiat payment", async function () {
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
        escrow
          .connect(comptroller)
          .expectResponseFor(
            await oracle.getAddress(),
            ethers.utils.formatBytes32String("1"),
            await buyer.getAddress(),
            ethers.utils.parseEther("1"),
          ),
        "expectResponseFor",
      )
        .to.emit(escrow, "AmountLocked")
        .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));

      // complete a failed payment
      await expect(
        escrow
          .connect(oracle)
          .fulfillFiatPayment(ethers.utils.formatBytes32String("1"), false),
        "fulfillFiatPayment",
      )
        .to.emit(escrow, "AmountUnlocked")
        .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));

      expect(await escrow.getUnlockedBalance(), "balance [unlocked]").to.equal(
        amount,
      );
    });

    it("should reset internal state after succcessful payment", async function () {
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
        escrow
          .connect(comptroller)
          .expectResponseFor(
            await oracle.getAddress(),
            ethers.utils.formatBytes32String("1"),
            await buyer.getAddress(),
            ethers.utils.parseEther("1"),
          ),
        "expectResponseFor",
      )
        .to.emit(escrow, "AmountLocked")
        .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));

      // complete a successful payment
      await expect(
        escrow
          .connect(oracle)
          .fulfillFiatPayment(ethers.utils.formatBytes32String("1"), true),
        "fulfillFiatPayment",
      ).to.not.be.reverted;

      expect(await escrow.getUnlockedBalance(), "balance [final]").to.equal(
        ethers.utils.parseEther("8.9951"),
      );
    });
  });
});
