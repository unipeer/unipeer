import { ethers, run } from "@nomiclabs/buidler";
import { Signer } from "ethers";

import { expect } from 'chai';

import { EscrowFactory } from "../types"
import { Escrow as EscrowContract } from "../types/Escrow"

let escrow: EscrowContract;
let admin: Signer;
let owner: Signer;

describe('Escrow', function () {
  before(async function () {
    [admin, owner] = await ethers.getSigners();
    const Escrow = await new EscrowFactory(admin);

    escrow = await Escrow.deploy(ethers.constants.AddressZero);
    escrow = escrow.connect(owner); // All transactions will be sent from the owner account.
  });

  it('can deposit additional funds to the contract', async function () {
    expect((await escrow.getUnlockedBalance()).toString()).to.equal('0');

    await escrow.initialize("test@upi");

    await owner.sendTransaction({
      to: escrow.address,
      value: ethers.utils.parseEther("1.0")
    });

    expect((await escrow.getUnlockedBalance()).toString()).to.equal('1000000000000000000');
  });

  it('can withdraw funds from the contract', async function () {
    expect((await escrow.getUnlockedBalance()).toString()).to.equal('1000000000000000000');

    await escrow.withdraw(ethers.utils.parseEther("1.0"), await owner.getAddress());

    expect((await escrow.getUnlockedBalance()).toString()).to.equal('0');
  });
});
