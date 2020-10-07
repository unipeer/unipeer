import {ethers, run} from "@nomiclabs/buidler";
import web3 from "web3";
import {Signer, utils} from "ethers";

import {expect} from "chai";

import {getInitializerData} from "../utils";

import {
  ComptrollerFactory,
  EscrowFactory,
  StaticProxyFactory,
  LinkTokenFactory,
  OracleFactory,
} from "../types";
import {Comptroller as ComptrollerContract} from "../types/Comptroller";
import {LinkToken as LinkTokenContract} from "../types/LinkToken";
import {Oracle as OracleContract} from "../types/Oracle";
import {Escrow as EscrowContract} from "../types/Escrow";
import {StaticProxy as StaticProxyContract} from "../types/StaticProxy";

let comptroller: ComptrollerContract;
let escrow: EscrowContract;
let admin: Signer;
let owner: Signer;
let buyer: Signer;

describe("Comptroller", function () {
  before(async function () {
    [admin, owner, buyer] = await ethers.getSigners();
    const LinkToken = await new LinkTokenFactory(admin);
    const Oracle = await new OracleFactory(admin);
    const Comptroller = await new ComptrollerFactory(admin);

    const token = await LinkToken.deploy();
    const oracle = await Oracle.deploy(token.address);
    const jobId = web3.utils.toHex("10cb58b1b1cc43268d0928f62cec31bb");
    comptroller = await Comptroller.deploy(
      oracle.address,
      jobId,
      token.address
    );

    const Escrow = await new EscrowFactory(admin);
    const Proxy = await new StaticProxyFactory(owner);

    const escrowNaked = await Escrow.deploy();

    const data = getInitializerData(Escrow, [comptroller.address, "seller@upi"], "initialize");
    const proxy = await Proxy.deploy(escrowNaked.address, data);

    escrow = Escrow.attach(proxy.address).connect(owner);

    // Transfer LinkToken to Comptroller
    token.transfer(comptroller.address, ethers.utils.parseEther("100.0"))
  });

  it("should correctly create a fiat payment request", async function () {
    // Deposit funds in the escrow
    await owner.sendTransaction({
      to: escrow.address,
      value: ethers.utils.parseEther("10.0"),
    });

    await comptroller.requestFiatPayment(
      await escrow.address,
      await buyer.getAddress(),
      utils.parseEther("1.0"),
      "test@upi"
    );
  });
});
