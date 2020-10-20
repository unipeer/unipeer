import {task, usePlugin} from "@nomiclabs/buidler/config";
import {BuidlerConfig} from "@nomiclabs/buidler/config";

// Loads environment variables
// Used only in development
require("dotenv").config({silent: true});

usePlugin("@nomiclabs/buidler-waffle");
usePlugin("@nomiclabs/buidler-solhint");
usePlugin("@nomiclabs/buidler-etherscan");

usePlugin("@openzeppelin/buidler-upgrades");

usePlugin("@unipeer/buidler-typechain");
usePlugin("buidler-gas-reporter");
usePlugin("buidler-spdx-license-identifier");
usePlugin("buidler-local-networks-config-plugin");
usePlugin("buidler-abi-exporter");

// This is a sample Buidler task. To learn how to create your own go to
// https://buidler.dev/guides/create-task.html
task("accounts", "Prints the list of accounts", async (taskArgs, bre) => {
  const accounts = await bre.web3.eth.getAccounts();

  for (const account of accounts) {
    console.log(account);
  }
});

const config: BuidlerConfig = {
  defaultNetwork: "buidlerevm",
  localNetworksConfig: "~/.buidler/networks.json",
  networks: {
    localhost: {
      url: "http://127.0.0.1:8545",
    },
  },
  solc: {
    version: "0.6.12",
    optimizer: {
      enabled: process.env.PRODUCTION ? true : false,
      runs: 200,
    },
  },
  gasReporter: {
    currency: "USD",
    gasPrice: 21,
    enabled: process.env.REPORT_GAS ? true : false,
  },
  spdxLicenseIdentifier: {
    overwrite: true,
    runOnCompile: true,
  },
  typechain: {
    outDir: "types",
    target: "ethers-v5",
    onTest: true,
    onCompile: false,
  },
  abiExporter: {
    path: "./abi",
    only: ["Comptroller", "Escrow", "EscrowContractFactory"],
    clear: true,
  },
  etherscan: {
    // Your API key for Etherscan
    // Obtain one at https://etherscan.io/
    apiKey: process.env.ETHERSCAN_API,
  },
  paths: {
    cache: "./build/cache",
    artifacts: "./build/artifacts",
  },
};

export default config;
