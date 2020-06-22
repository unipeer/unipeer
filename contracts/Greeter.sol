// SPDX-License-Identifier: MIT

pragma solidity >=0.4.21 <0.7.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@nomiclabs/buidler/console.sol";
import "./StaticProxy.sol";

contract Greeter is StaticStorage {
  string private greeting;

  constructor(string memory _greeting) public {
    console.log("Deploying a Greeter with greeting:", _greeting);
    greeting = _greeting;
  }

  function greet() public view returns (string memory) {
    return greeting;
  }

  function setGreeting(string memory _greeting) public {
    console.log("Changing greeting from '%s' to '%s'", greeting, _greeting);
    greeting = _greeting;
  }
}
