import { ethers, run } from "hardhat";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/dist/src/signer-with-address";

import { expect } from "chai";

import {
  EthEscrow__factory,
  EthEscrow as EthEscrowContract,
  EscrowFactory__factory,
  EscrowFactory as EscrowFactoryContract,
} from "../types";

let EthEscrow: EthEscrow__factory;
let escrowFactory: EscrowFactoryContract;
let admin: SignerWithAddress;
let owner: SignerWithAddress;

describe("Escrow (Factory)", function () {
  beforeEach(async function () {
    [admin, owner] = await ethers.getSigners();
    EthEscrow = await new EthEscrow__factory(admin);
    const EscrowFactory = await new EscrowFactory__factory(admin);

    const escrow = await EthEscrow.deploy();
    escrowFactory = await EscrowFactory.deploy(
      escrow.address,
      ethers.constants.AddressZero,
    );
  });

  it("should successfully deploy a new proxied escrow", async function () {
    await expect(escrowFactory.newEscrow("seller@upi")).to.emit(
      escrowFactory,
      "Created",
    );
  });

  it("should create and transfer funds to escrow", async function () {
    const amount = ethers.utils.parseEther("1.1");

    await expect(
      escrowFactory.connect(owner).newEscrow("seller@upi", {
        value: amount,
      }),
    ).to.not.be.reverted;

    const escrows = await escrowFactory.getEscrows(owner.address);
    const escrow = await EthEscrow.attach(escrows[0]);

    expect(await escrow.getUnlockedBalance(), "balance").to.equal(amount);
  });
});
