import {ethers, run, waffle, web3} from "hardhat";
import {SignerWithAddress} from "@nomiclabs/hardhat-ethers/dist/src/signer-with-address";

const {deployMockContract} = waffle;
import {expect} from "chai";

import {
  ComptrollerFactory,
  Comptroller as ComptrollerContract,
  EscrowFactory,
  Escrow as EscrowContract,
  EscrowContractFactoryFactory,
  EscrowContractFactory as EscrowContractFactoryContract,
} from "../types";

import LinkTokenABI from "./abi/LinkToken.json";
import OracleABI from "./abi/Oracle.json";

let comptroller: ComptrollerContract;
let escrow: EscrowContract;
let escrowFactory: EscrowContractFactoryContract;

let admin: SignerWithAddress;
let owner: SignerWithAddress;
let buyer: SignerWithAddress;
let oracle: SignerWithAddress;

describe("Escrow", function () {
  beforeEach(async function () {
    [admin, owner, buyer, oracle] = await ethers.getSigners();

    const mockLink = await deployMockContract(admin, LinkTokenABI);
    await mockLink.mock.balanceOf.returns(ethers.utils.parseEther("999999"));
    await mockLink.mock.transferAndCall.returns(true);
    //const mockOracle = await deployMockContract(admin, OracleABI);

    const Comptroller = await new ComptrollerFactory(admin);

    comptroller = await Comptroller.deploy(
      mockLink.address,
      oracle.address,
      web3.utils.toHex("0d69f6d174a4446c9a7ffa21cd0f687c"),
    );

    const Escrow = await new EscrowFactory(admin);
    const EscrowContractFactory = await new EscrowContractFactoryFactory(admin);

    const escrowNaked = await Escrow.deploy();
    escrowFactory = await EscrowContractFactory.deploy(
      escrowNaked.address,
      comptroller.address,
    );

    await escrowFactory.connect(owner).newEscrow("seller@upi");
    const escrows = await escrowFactory.getEscrows(owner.address);
    escrow = await Escrow.attach(escrows[0]).connect(owner);
  });

  it("can deposit additional funds to the contract", async function () {
    expect(await escrow.getUnlockedBalance()).to.equal(0);

    const amount = ethers.utils.parseEther("1.0");
    await owner.sendTransaction({
      to: escrow.address,
      value: amount,
    });

    expect(await escrow.getUnlockedBalance()).to.equal(amount);
  });

  it("can withdraw funds from the contract", async function () {
    const amount = ethers.utils.parseEther("1");
    await owner.sendTransaction({
      to: escrow.address,
      value: amount,
    });

    expect(await escrow.getUnlockedBalance()).to.equal(amount);

    await escrow.withdraw(amount, owner.address);
    expect(await escrow.getUnlockedBalance()).to.equal(0);
  });

  describe("fulfillFiatPayment", function () {
    let hash: string;

    beforeEach(async function () {
      const amount = ethers.utils.parseEther("10");
      await owner.sendTransaction({
        to: escrow.address,
        value: amount,
      });
      expect(await escrow.getUnlockedBalance(), "balance [initial]").to.equal(
        amount,
      );

      // create a payment request
      await expect(
        comptroller.requestFiatPayment(
          escrow.address,
          buyer.address,
          ethers.utils.parseEther("1"),
          "test@upi",
        ),
      )
        .to.emit(escrow, "AmountLocked")
        .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));

      hash = web3.utils.soliditySha3(
        {t: "address", v: comptroller.address},
        {t: "uint256", v: 1},
      ) as string;
    });

    it("should correctly report unlocked balance", async function () {
      expect(
        await escrow.getUnlockedBalance(),
        "Balance after locked",
      ).to.equal(ethers.utils.parseEther("8.9951"));
    });

    it("should not withdraw more that unlocked balance", async function () {
      expect(
        await escrow.getUnlockedBalance(),
        "Balance after locked",
      ).to.equal(ethers.utils.parseEther("8.9951"));

      await expect(
        escrow.withdraw(ethers.utils.parseEther("10"), owner.address),
      ).to.be.reverted;
    });

    it("should correctly unlock funds after a failed fiat payment", async function () {
      // complete a failed payment
      await expect(
        escrow.connect(oracle).fulfillFiatPayment(hash, false),
        "fulfillFiatPayment",
      )
        .to.emit(escrow, "AmountUnlocked")
        .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));

      expect(await escrow.getUnlockedBalance(), "balance [unlocked]").to.equal(
        ethers.utils.parseEther("10"),
      );
    });

    it("should reset internal state after succcessful payment", async function () {
      // complete a successful payment
      await expect(
        escrow.connect(oracle).fulfillFiatPayment(hash, true),
        "fulfillFiatPayment",
      ).to.not.be.reverted;

      expect(await escrow.getUnlockedBalance(), "balance [final]").to.equal(
        ethers.utils.parseEther("8.9951"),
      );
    });
  });
});
