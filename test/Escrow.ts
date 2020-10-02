import { ethers, run } from "@nomiclabs/buidler";
import * as etherstype from "ethers";

import { expect } from 'chai';

import { EscrowFactory } from "../types"
import { Escrow as EscrowContract } from "../types/Escrow"

let escrow: EscrowContract;
let accounts: etherstype.Signer[];

describe('Escrow', function () {
  beforeEach(async function () {
    accounts = await ethers.getSigners();
    const Escrow = await new EscrowFactory(accounts[0]);

    escrow = await Escrow.deploy();
  });

  it('can get balance when no amount is locked', async function () {
    expect((await escrow.getUnlockedBalance()).toString()).to.equal('0');

    await accounts[0].sendTransaction({
      to: escrow.address,
      value: ethers.utils.parseEther("1.0")
    });

    expect((await escrow.getUnlockedBalance()).toString()).to.equal('1000000000000000000');
  });
});
