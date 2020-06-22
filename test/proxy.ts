import { web3, artifacts, assert, contract } from "@nomiclabs/buidler";
import { GreeterContract, StaticProxyContract } from "../types/truffle-contracts"

const Greeter: GreeterContract = artifacts.require("Greeter");
const Proxy: StaticProxyContract = artifacts.require("StaticProxy");

contract("Static Proxy", accounts => {
  it("Should correctly set the implementation contract address", async function() {
    const greeter = await Greeter.new("Hello, world!");
    const proxy = await Proxy.new(greeter.address);

    assert.equal(await proxy.implementation(), greeter.address)
  });

  it("Should delegate calls to the implementation contract", async function() {
    const greeter = await Greeter.new("Hello, world!");
    const proxy = await Proxy.new(greeter.address);

    const greeterProxy = await Greeter.at(proxy.address)

    // The proxy state is not affected by the constructor
    // of the Logic Contract
    assert.equal(await greeterProxy.greet(), "");

    await greeterProxy.setGreeting("Hola, mundo!");

    assert.equal(await greeterProxy.greet(), "Hola, mundo!");
  });
});
