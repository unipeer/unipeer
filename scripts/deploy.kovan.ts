import {run, ethers, upgrades} from "@nomiclabs/buidler";
import web3 from "web3";
import {ContractFactory} from "ethers";

import {ComptrollerFactory, EscrowFactory, StaticProxyFactory} from "../types";
import {Comptroller as ComptrollerContract} from "../types/Comptroller";
import {Escrow as EscrowContract} from "../types/Escrow";
import {StaticProxy as StaticProxyContract} from "../types/StaticProxy";

async function main() {
  await run("typechain");

  const accounts = await ethers.getSigners();
  const account = accounts[0];

  const Comptroller = await new ComptrollerFactory(account);
  const Escrow = await new EscrowFactory(account);
  const Proxy = await new StaticProxyFactory(account);

  console.log("Deploying Comptroller...");
  let comptroller = await Comptroller.deploy(
    "0x98cbfb4f664e6b35a32930c90e43f03b5eab50da",
    web3.utils.toHex("10cb58b1b1cc43268d0928f62cec31bb")
  );

  console.log("Comptroller deployed to:", comptroller.address);

  const escrow = await Escrow.deploy(comptroller.address);

  const data = getInitializerData(Escrow, ["test@upi"], "initialize");
  const proxy = await Proxy.deploy(escrow.address, data);

  console.log("Escrow deployed to:", proxy.address);

  // Verify the contracts on etherscan
  // The network will be the same as the one specified
  // when running this deploy script.
  // TODO: be DRY with arguments and addresses?
  await run("verify", {
    address: comptroller.address,
    constructorArguments: [
      "0x98cbfb4f664e6b35a32930c90e43f03b5eab50da",
      web3.utils.toHex("10cb58b1b1cc43268d0928f62cec31bb"),
    ],
  });

  await run("verify", {
    address: escrow.address,
    constructorArguments: [
      comptroller.address
    ],
  });

  await run("verify", {
    address: proxy.address,
    constructorArguments: [
      escrow.address,
      data
    ],
  });
}

function getInitializerData(
  ImplFactory: ContractFactory,
  args: unknown[],
  initializer?: string
): string {
  const allowNoInitialization = initializer === undefined && args.length === 0;
  initializer = initializer ?? "initialize";

  try {
    const fragment = ImplFactory.interface.getFunction(initializer);
    return ImplFactory.interface.encodeFunctionData(fragment, args);
  } catch (e: unknown) {
    if (e instanceof Error) {
      if (allowNoInitialization && e.message.includes("no matching function")) {
        return "0x";
      }
    }
    throw e;
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
