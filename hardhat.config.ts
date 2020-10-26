import {HardhatUserConfig, task} from "hardhat/config";

import "@nomiclabs/hardhat-web3";
import "@nomiclabs/hardhat-waffle";
import "@nomiclabs/hardhat-solhint";
import "@nomiclabs/hardhat-etherscan";

// Loads environment variables
// Used only in development
require("dotenv").config({silent: true});

//usePlugin("@openzeppelin/buidler-upgrades");
import "@unipeer/hardhat-typechain";

// usePlugin("buidler-gas-reporter");
import "hardhat-spdx-license-identifier";
// usePlugin("buidler-local-networks-config-plugin");

// This is a sample hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
task("accounts", "Prints the list of accounts", async (taskArgs, hre) => {
  const accounts = await hre.ethers.getSigners();

  for (const account of accounts) {
    console.log(account);
  }
});

const config: HardhatUserConfig = {
  defaultNetwork: "hardhat",
  // localNetworksConfig: "~/.buidler/networks.json",
  networks: {
    localhost: {
      url: "http://127.0.0.1:8545",
    },
  },
  solidity: {
    compilers: [
      {
        version: "0.6.12",
        settings: {
          optimizer: {
            enabled: true,
          },
        },
      },
      {
        version: "0.7.4",
        settings: {
          optimizer: {
            enabled: true,
          },
        },
      },
    ],
  },
  /*
  gasReporter: {
    currency: "USD",
    gasPrice: 21,
    enabled: process.env.REPORT_GAS ? true : false,
  },
  */
  spdxLicenseIdentifier: {
    overwrite: true,
    runOnCompile: true,
  },
  typechain: {
    outDir: "types",
    target: "ethers-v5",
    runOnCompile: true,
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
