import { run, ethers, upgrades } from "hardhat";

async function main() {
  // You can run Buidler tasks from a script.
  // For example, we make sure everything is compiled by running "compile"
  await run("compile");

  const Box = await ethers.getContractFactory("Box");
  console.log("Deploying Box...");

  const box = await upgrades.deployProxy(Box, [42], { initializer: "store" });
  console.log("Box deployed to:", box.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
