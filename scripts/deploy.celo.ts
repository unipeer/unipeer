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

import {
  CeloOracle__factory,
  CeloOracle as CeloOracleContract,
  CeloLinkToken__factory,
  CeloLinkToken as CeloLinkTokenContract,
} from "../types";

const JOBID = "3dd25a102fe74157b1eae12b430336f4";

async function resolveAddress(tx: any) {
  let receipt = await tx.deployTransaction.wait();
  return receipt.contractAddress;
}

async function main() {
  await run("typechain");

  const provider = new CeloProvider("https://alfajores-forno.celo-testnet.org");
  await provider.ready;

  const account = new CeloWallet(
    process.env.CELO_PRIVATE_KEY as string,
    provider,
  );

  const CeloLink = new CeloLinkToken__factory(account);
  const CeloOracle = new CeloOracle__factory(account);
  const Comptroller = new Comptroller__factory(account);
  const EthEscrow = new EthEscrow__factory(account);
  const EscrowFactory = new EscrowFactory__factory(account);

  let link: CeloLinkTokenContract = await CeloLink.deploy();
  let linkAddress = await resolveAddress(link);
  console.log("Link deployed to:", linkAddress);

  let oracle: CeloOracleContract = await CeloOracle.deploy(linkAddress, {
    gasLimit: 8000000,
  });
  let oracleAddress = await resolveAddress(oracle);
  console.log("Oracle deployed to:", oracleAddress);

  console.log("Deploying Comptroller...");
  let comptroller = await Comptroller.deploy(
    linkAddress,
    oracleAddress,
    web3.utils.toHex(JOBID),
  );
  let compAddress = await resolveAddress(comptroller);
  console.log("Comptroller deployed to:", compAddress);

  let linkDeployed = CeloLink.attach(linkAddress);
  await linkDeployed.transfer(compAddress, ethers.utils.parseEther("100"));

  console.log("Deploying Escrow...");
  const escrow = await EthEscrow.deploy();
  let escrowAddress = await resolveAddress(escrow);
  console.log("Escrow deployed to:", escrowAddress);

  console.log("Deploying EscrowFactory...");
  const escrowFactory = await EscrowFactory.deploy(escrowAddress, compAddress);
  let escrowFactoryAddress = await resolveAddress(escrowFactory);
  console.log("EscrowFactory deployed to:", escrowFactoryAddress);

  let factoryDeployed = EscrowFactory.attach(escrowFactoryAddress);
  console.log("Creating a new escrow...");
  let tx = await factoryDeployed.newEscrow("seller@upi", {
    value: ethers.utils.parseEther("0.1"),
  });
  console.log(tx);
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
