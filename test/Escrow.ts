import {ethers, run} from "@nomiclabs/buidler";
import {Signer} from "ethers";

import {expect} from "chai";

import {EscrowFactory, StaticProxyFactory} from "../types";
import {Escrow as EscrowContract} from "../types/Escrow";
import {StaticProxy as StaticProxyContract} from "../types/StaticProxy";

import {getInitializerData} from "../utils";

let escrow: EscrowContract;
let admin: Signer;
let owner: Signer;

describe("Escrow", function () {
  beforeEach(async function () {
    [admin, owner] = await ethers.getSigners();
    const Escrow = await new EscrowFactory(admin);
    const Proxy = await new StaticProxyFactory(owner);

    const escrowNaked = await Escrow.deploy();

    const data = getInitializerData(
      Escrow,
      [ethers.constants.AddressZero, "test@upi"],
      "initialize(address,string)"
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
    const amount = ethers.utils.parseEther("1.0");
    await owner.sendTransaction({
      to: escrow.address,
      value: amount,
    });

    expect(await escrow.getUnlockedBalance()).to.equal(amount);

    await escrow.withdraw(amount, await owner.getAddress());
    expect(await escrow.getUnlockedBalance()).to.equal(0);
  });

  it("should not withdraw more than available funds", async function () {
    const amount = ethers.utils.parseEther("1.0");
    await owner.sendTransaction({
      to: escrow.address,
      value: amount,
    });

    expect(await escrow.getUnlockedBalance()).to.equal(amount);

    await expect(
      escrow.withdraw(ethers.utils.parseEther("2.0"), await owner.getAddress())
    ).to.be.reverted;
  });

  it("should reset internal state after succcessful payment", async function () {
  });
});
