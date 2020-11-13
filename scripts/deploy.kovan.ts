import {run, ethers} from "hardhat";
import web3 from "web3";
import {constants} from "ethers";

import {
  Comptroller__factory,
  Comptroller as ComptrollerContract,
  Escrow__factory,
  Escrow as EscrowContract,
  EscrowFactory__factory,
  EscrowFactory as EscrowFactoryContract,
} from "../types";

import {getInitializerData} from "../utils";

const ORACLE_ADDRESS = "0x98cbfb4f664e6b35a32930c90e43f03b5eab50da";
const JOBID = "3dd25a102fe74157b1eae12b430336f4";

async function main() {
  await run("typechain");

  const accounts = await ethers.getSigners();
  const account = accounts[0];

  const Comptroller = await new Comptroller__factory(account);
  const Escrow = await new Escrow__factory(account);
  const EscrowFactory = await new EscrowFactory__factory(account);

  console.log("Deploying Comptroller...");
  let comptroller = await Comptroller.deploy(
    constants.AddressZero,
    ORACLE_ADDRESS,
    web3.utils.toHex(JOBID),
  );
  console.log("Comptroller deployed to:", comptroller.address);

  console.log("Deploying Escrow...");
  const escrow = await Escrow.deploy();
  console.log("Escrow deployed to:", escrow.address);

  console.log("Deploying EscrowFactory...");
  const escrowFactory = await EscrowFactory.deploy(
    escrow.address,
    comptroller.address,
  );
  console.log("EscrowFactory deployed to:", escrowFactory.address);

  console.log("Creating a new escrow...");
  await escrowFactory.newEscrow("seller@upi", {
    value: ethers.utils.parseEther("0.1"),
  });

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
  });

  await run("verify", {
    address: escrowFactory.address,
    constructorArguments: [escrow.address, comptroller.address],
  });
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
