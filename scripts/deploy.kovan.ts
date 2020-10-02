import {run, ethers, upgrades } from "@nomiclabs/buidler";
import web3 from "web3";

import {ComptrollerFactory, EscrowFactory} from "../types";
import {Comptroller as ComptrollerContract} from "../types/Comptroller";
import {Escrow as EscrowContract} from "../types/Escrow";

async function main() {
  await run("compile");

  const accounts = await ethers.getSigners();
  const Comptroller = await new ComptrollerFactory(accounts[0]);
  console.log("Deploying Comptroller...");

  let comptroller = await Comptroller.deploy(
    "0x98cbfb4f664e6b35a32930c90e43f03b5eab50da",
    web3.utils.toHex("10cb58b1b1cc43268d0928f62cec31bb")
  );

  console.log("Comptroller deployed to:", comptroller.address);

  const Escrow = await new EscrowFactory(accounts[0]);
  const escrow = await upgrades.deployProxy(Escrow, [comptroller.address, ""], {
    initializer: "initialize",
  });

  console.log("Escrow deployed to:", escrow.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
