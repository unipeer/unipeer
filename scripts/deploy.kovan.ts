import { run, ethers } from "hardhat";
import web3 from "web3";
import { constants } from "ethers";

import { Comptroller__factory, Escrow__factory } from "../types";

const ORACLE_ADDRESS = "0x98cbfb4f664e6b35a32930c90e43f03b5eab50da";
const JOBID = "3dd25a102fe74157b1eae12b430336f4";

async function main() {
  await run("typechain");

  const accounts = await ethers.getSigners();
  const account = accounts[0];

  const Comptroller = new Comptroller__factory(account);
  const Escrow = new Escrow__factory(account);

  console.log("Deploying Comptroller...");
  let comptroller = await Comptroller.deploy(
    constants.AddressZero,
    ORACLE_ADDRESS,
    web3.utils.toHex(JOBID),
  );
  console.log("Comptroller deployed to:", comptroller.address);

  console.log("Deploying Escrow...");
  const escrow = await Escrow.deploy(
    account.address,
    comptroller.address,
    "seller@upi",
  );
  console.log("Escrow deployed to:", escrow.address);

  // Verify the contracts on etherscan
  // The network will be the same as the one specified
  // when running this deploy script.
  // TODO: be DRY with arguments and addresses?
  await run("verify", {
    address: comptroller.address,
    constructorArguments: [
      constants.AddressZero,
      ORACLE_ADDRESS,
      web3.utils.toHex(JOBID),
    ],
  });

  await run("verify", {
    address: escrow.address,
    constructorArguments: [account.address, comptroller.address, "seller@upi"],
  });
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
