import { task, usePlugin } from "@nomiclabs/buidler/config";
import { BuidlerConfig } from "@nomiclabs/buidler/config";

usePlugin("@nomiclabs/buidler-truffle5");
usePlugin("@nomiclabs/buidler-solhint");
usePlugin("buidler-gas-reporter");

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
  networks: {
    localhost: {
      url: "http://127.0.0.1:8545"
    },
    buidlerevm: {
      // See its defaults
    }
  },
  solc: {
    version: "0.6.10",
    optimizer: {
      enabled: false,
      runs: 200
    }
  },
  gasReporter: {
    currency: 'USD',
    gasPrice: 21,
    enabled: (process.env.REPORT_GAS) ? true : false
  }
};

export default config;
