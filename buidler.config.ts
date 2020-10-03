import {task, usePlugin} from "@nomiclabs/buidler/config";
import {BuidlerConfig} from "@nomiclabs/buidler/config";

usePlugin("@nomiclabs/buidler-ethers");
usePlugin("@nomiclabs/buidler-solhint");

usePlugin("@openzeppelin/buidler-upgrades");

usePlugin("@blockchangers/buidler-typechain");
usePlugin("buidler-gas-reporter");
usePlugin("buidler-spdx-license-identifier");
usePlugin('buidler-local-networks-config-plugin');

// This is a sample Buidler task. To learn how to create your own go to
// https://buidler.dev/guides/create-task.html
task("accounts", "Prints the list of accounts", async (taskArgs, bre) => {
  const accounts = await bre.web3.eth.getAccounts();

  for (const account of accounts) {
    console.log(account);
  }
});

task("test", "Runs mocha tests").setAction(async (args, {run}, runSuper) => {
  if (!args.noCompile) {
    await run("typechain");
  }
  await runSuper(args);
});

const config: BuidlerConfig = {
  defaultNetwork: "buidlerevm",
  localNetworksConfig: '~/.buidler/networks.json',
  networks: {
    localhost: {
      url: "http://127.0.0.1:8545",
    },
  },
  solc: {
    version: "0.6.12",
    optimizer: {
      enabled: false,
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
  },
  paths: {
    cache: "./build/cache",
    artifacts: "./build/artifacts"
  }
};

export default config;
