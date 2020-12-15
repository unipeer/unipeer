import { run, ethers } from "hardhat";

import { SimpleEscrow__factory } from "../types";

const COMP_ADDRESS = "0xcE589FC23EE7AB377e07c513e6b32e93ab57CF1B";
const NODE_ADDRESS = "0xDb9D0B46628753523b6cc375Af63edaFb3Ac5668";

async function main() {
  await run("typechain");

  const accounts = await ethers.getSigners();
  const account = accounts[0];

  const Escrow = new SimpleEscrow__factory(account);

  console.log("Deploying Escrow...");
  const escrow = await Escrow.deploy(COMP_ADDRESS, NODE_ADDRESS);
  await escrow.deployTransaction.wait(2);
  console.log("Escrow deployed to:", escrow.address);

  await run("verify", {
    address: escrow.address,
    constructorArguments: [COMP_ADDRESS, NODE_ADDRESS],
  });
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
