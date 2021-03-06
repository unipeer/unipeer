import { ethers } from "hardhat";
import { expect } from "chai";

import { Box__factory, Box as BoxContract } from "../types";

let box: BoxContract;

// Start test block
describe("Box", function () {
  beforeEach(async function () {
    const signer = await ethers.getSigners();
    const Box = await new Box__factory(signer[0]);

    box = await Box.deploy();
    await box.deployed();
  });

  // Test case
  it("retrieve returns a value previously stored", async function () {
    // Store a value
    await box.store(42);

    // Test if the returned value is the same one
    // Note that we need to use strings to compare the 256 bit integers
    expect((await box.retrieve()).toString()).to.equal("42");
  });
});
