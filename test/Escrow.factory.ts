import {ethers, run} from "hardhat";
import {SignerWithAddress} from "@nomiclabs/hardhat-ethers/dist/src/signer-with-address";

import {expect} from "chai";

import {
  Escrow__factory,
  EscrowContractFactory__factory,
  Escrow as EscrowContract,
  EscrowContractFactory as EscrowContractFactoryContract,
} from "../types";

let Escrow: Escrow__factory;
let escrowFactory: EscrowContractFactoryContract;
let admin: SignerWithAddress;
let owner: SignerWithAddress;

describe("Escrow (Factory)", function () {
  beforeEach(async function () {
    [admin, owner] = await ethers.getSigners();
    Escrow = await new Escrow__factory(admin);
    const EscrowContractFactory = await new EscrowContractFactory__factory(
      admin,
    );

    const escrow = await Escrow.deploy();
    escrowFactory = await EscrowContractFactory.deploy(
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
    const escrow = await Escrow.attach(escrows[0]);

    expect(await escrow.getUnlockedBalance(), "balance").to.equal(amount);
  });
});
