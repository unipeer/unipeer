import { ethers, upgrades } from "hardhat";
import { expect } from "chai";

import { Box__factory, Box as BoxContract } from "../types";

let box: BoxContract;

// Start test block
describe("Box (proxy)", function () {
  beforeEach(async function () {
    const signer = await ethers.getSigners();
    const Box = new Box__factory(signer[0]);

    box = <BoxContract>(
      await upgrades.deployProxy(Box, [42], { initializer: "store" })
    );
    await box.deployed();

    console.log("Box deployed to:", box.address);
  });

  // Test case
  it("retrieve returns a value previously initialized", async function () {
    // Test if the returned value is the same one
    // Note that we need to use strings to compare the 256 bit integers
    expect((await box.retrieve()).toString()).to.equal("42");
  });
});
