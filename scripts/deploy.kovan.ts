import {run, ethers, upgrades} from "hardhat";
import web3 from "web3";
import {constants} from "ethers";

import {
  ComptrollerFactory,
  EscrowFactory,
  EscrowContractFactoryFactory,
  Comptroller as ComptrollerContract,
  Escrow as EscrowContract,
  EscrowContractFactory as EscrowContractFactoryContract,
} from "../types";

import {getInitializerData} from "../utils";

async function main() {
  await run("typechain");

  const accounts = await ethers.getSigners();
  const account = accounts[0];

  const Comptroller = await new ComptrollerFactory(account);
  const Escrow = await new EscrowFactory(account);
  const EscrowContractFactory = await new EscrowContractFactoryFactory(account);

  console.log("Deploying Comptroller...");
  let comptroller = await Comptroller.deploy(
    constants.AddressZero,
    "0x98cbfb4f664e6b35a32930c90e43f03b5eab50da",
    web3.utils.toHex("0d69f6d174a4446c9a7ffa21cd0f687c"),
  );
  console.log("Comptroller deployed to:", comptroller.address);

  console.log("Deploying Escrow...");
  const escrow = await Escrow.deploy();
  console.log("Escrow deployed to:", escrow.address);

  console.log("Deploying EscrowFactory...");
  const escrowFactory = await EscrowContractFactory.deploy(
    escrow.address,
    comptroller.address,
  );
  console.log("EscrowFactory deployed to:", escrow.address);

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
      "0x98cbfb4f664e6b35a32930c90e43f03b5eab50da",
      web3.utils.toHex("0d69f6d174a4446c9a7ffa21cd0f687c"),
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
