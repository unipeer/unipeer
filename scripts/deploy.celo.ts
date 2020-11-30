import { run, ethers } from "hardhat";
import web3 from "web3";
import { constants } from "ethers";

import { CeloProvider, CeloWallet } from "@celo-tools/celo-ethers-wrapper";

import {
  Comptroller__factory,
  Comptroller as ComptrollerContract,
  EthEscrow__factory,
  EthEscrow as EthEscrowContract,
  EscrowFactory__factory,
  EscrowFactory as EscrowFactoryContract,
} from "../types";

import { getInitializerData } from "../utils";

const ORACLE_ADDRESS = "0x98cbfb4f664e6b35a32930c90e43f03b5eab50da";
const JOBID = "3dd25a102fe74157b1eae12b430336f4";

async function main() {
  await run("typechain");

  const provider = new CeloProvider("https://alfajores-forno.celo-testnet.org");
  await provider.ready;

  const account = new CeloWallet(
    process.env.CELO_PRIVATE_KEY as string,
    provider,
  );

  const Comptroller = await new Comptroller__factory(account);
  const EthEscrow = await new EthEscrow__factory(account);
  const EscrowFactory = await new EscrowFactory__factory(account);

  console.log("Deploying Comptroller...");
  let comptroller = await Comptroller.deploy(
    constants.AddressZero,
    ORACLE_ADDRESS,
    web3.utils.toHex(JOBID),
    { gasLimit: 8000000 },
  );
  console.log("Comptroller deployed to:", comptroller.address);

  console.log("Deploying Escrow...");
  const escrow = await EthEscrow.deploy();
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
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
