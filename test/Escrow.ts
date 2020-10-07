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
  before(async function () {
    [admin, owner] = await ethers.getSigners();
    const Escrow = await new EscrowFactory(admin);
    const Proxy = await new StaticProxyFactory(owner);

    const escrowNaked = await Escrow.deploy();

    const data = getInitializerData(Escrow, [ethers.constants.AddressZero, "test@upi"], "initialize");
    const proxy = await Proxy.deploy(escrowNaked.address, data);

    escrow = Escrow.attach(proxy.address).connect(owner);
  });

  it("can deposit additional funds to the contract", async function () {
    expect((await escrow.getUnlockedBalance()).toString()).to.equal("0");

    await owner.sendTransaction({
      to: escrow.address,
      value: ethers.utils.parseEther("1.0"),
    });

    expect((await escrow.getUnlockedBalance()).toString()).to.equal(
      "1000000000000000000"
    );
  });

  it("can withdraw funds from the contract", async function () {
    expect((await escrow.getUnlockedBalance()).toString()).to.equal(
      "1000000000000000000"
    );

    await escrow.withdraw(
      ethers.utils.parseEther("1.0"),
      await owner.getAddress()
    );

    expect((await escrow.getUnlockedBalance()).toString()).to.equal("0");
  });
});
