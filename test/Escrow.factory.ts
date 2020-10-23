import {ethers, run} from "hardhat";
import {Signer} from "ethers";

import {expect} from "chai";

import {
  EscrowFactory,
  EscrowContractFactoryFactory,
  Escrow as EscrowContract,
  EscrowContractFactory as EscrowContractFactoryContract,
} from "../types";

let Escrow: EscrowFactory;
let escrowFactory: EscrowContractFactoryContract;
let admin: Signer;
let owner: Signer;

describe("Escrow (Factory)", function () {
  beforeEach(async function () {
    [admin, owner] = await ethers.getSigners();
    Escrow = await new EscrowFactory(admin);
    const EscrowContractFactory = await new EscrowContractFactoryFactory(admin);

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

    const escrows = await escrowFactory.getEscrows(await owner.getAddress());
    const escrow = await Escrow.attach(escrows[0]);

    expect(await escrow.getUnlockedBalance(), "balance").to.equal(amount);
  });
});
