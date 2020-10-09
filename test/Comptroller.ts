import {ethers, run, waffle} from "@nomiclabs/buidler";
const { deployMockContract } = waffle;
import web3 from "web3";
import {Signer} from "ethers";
import {expect} from "chai";

import {getInitializerData} from "../utils";

import {
  ComptrollerFactory,
  EscrowFactory,
  StaticProxyFactory,
} from "../types";
import {Comptroller as ComptrollerContract} from "../types/Comptroller";
import {Escrow as EscrowContract} from "../types/Escrow";
import {StaticProxy as StaticProxyContract} from "../types/StaticProxy";

import LinkTokenABI from "./abi/LinkToken.json";
import OracleABI from "./abi/Oracle.json";

let comptroller: ComptrollerContract;
let escrow: EscrowContract;
let admin: Signer;
let owner: Signer;
let buyer: Signer;


describe("Comptroller", function () {
  beforeEach(async function () {
    [admin, owner, buyer] = await ethers.getSigners();

    const mockLink = await deployMockContract(admin, LinkTokenABI);
    await mockLink.mock.balanceOf.returns(ethers.utils.parseEther('999999'));
    await mockLink.mock.transferAndCall.returns(true);
    const mockOracle = await deployMockContract(admin, OracleABI);

    const Comptroller = await new ComptrollerFactory(admin);

    const jobId = web3.utils.toHex("10cb58b1b1cc43268d0928f62cec31bb");
    comptroller = await Comptroller.deploy(
      mockLink.address,
      mockOracle.address,
      jobId,
    );

    const Escrow = await new EscrowFactory(admin);
    const Proxy = await new StaticProxyFactory(owner);

    const escrowNaked = await Escrow.deploy();

    const data = getInitializerData(
      Escrow,
      [comptroller.address, "seller@upi"],
      "initialize(address,string)"
    );
    const proxy = await Proxy.deploy(escrowNaked.address, data);

    escrow = Escrow.attach(proxy.address).connect(owner);
  });

  it("should correctly create a fiat payment request", async function () {
    // Deposit funds in the escrow
    await owner.sendTransaction({
      to: escrow.address,
      value: ethers.utils.parseEther("10"),
    });

    await expect(
      comptroller.requestFiatPayment(
        await escrow.address,
        await buyer.getAddress(),
        ethers.utils.parseEther("1"),
        "test@upi"
      )
    )
      .to.emit(escrow, "AmountLocked")
      .withArgs(escrow.address, ethers.utils.parseEther("1.0049"));
  });

  it("should fail requestFiatPayment when escrow doesn't have enough funds", async function () {
    await expect(
      comptroller.requestFiatPayment(
        await escrow.address,
        await buyer.getAddress(),
        ethers.utils.parseEther("1"),
        "test@upi"
      )
    ).to.be.revertedWith("Comptroller: not enough funds in escrow");
  });

  describe("Escrow", function () {
  });
});
