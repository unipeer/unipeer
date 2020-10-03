import { ethers, run } from "@nomiclabs/buidler";
import * as etherstype from "ethers";

import { expect } from 'chai';

import { EscrowFactory } from "../types"
import { Escrow as EscrowContract } from "../types/Escrow"

let escrow: EscrowContract;
let accounts: etherstype.Signer[];

describe('Escrow', function () {
  before(async function () {
    accounts = await ethers.getSigners();
    const Escrow = await new EscrowFactory(accounts[0]);

    escrow = await Escrow.deploy(ethers.constants.AddressZero);
    await escrow.initialize("test@upi");
  });

  it('can deposit additional funds to the contract', async function () {
    expect((await escrow.getUnlockedBalance()).toString()).to.equal('0');

    await accounts[0].sendTransaction({
      to: escrow.address,
      value: ethers.utils.parseEther("1.0")
    });

    expect((await escrow.getUnlockedBalance()).toString()).to.equal('1000000000000000000');
  });

  it('can withdraw funds from the contract', async function () {
    expect((await escrow.getUnlockedBalance()).toString()).to.equal('1000000000000000000');

    await escrow.withdraw(ethers.utils.parseEther("1.0"), await accounts[1].getAddress());

    expect((await escrow.getUnlockedBalance()).toString()).to.equal('0');
  });
});
