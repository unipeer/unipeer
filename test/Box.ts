import { ethers } from "@nomiclabs/buidler";
import { expect } from 'chai';

import { BoxFactory } from "../types"
import { Box as BoxContract } from "../types/Box"

let box: BoxContract;

// Start test block
describe('Box', function () {
  beforeEach(async function () {
    const signer = await ethers.getSigners();
    const Box= await new BoxFactory(signer[0]);

    box = await Box.deploy();
  });

  // Test case
  it('retrieve returns a value previously stored', async function () {
    // Store a value
    await box.store(42);

    // Test if the returned value is the same one
    // Note that we need to use strings to compare the 256 bit integers
    expect((await box.retrieve()).toString()).to.equal('42');
  });
});
