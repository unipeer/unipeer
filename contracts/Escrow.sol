// SPDX-License-Identifier: MIT

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@nomiclabs/buidler/console.sol";
import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";

import "./StaticProxy.sol";
import "./adapters/EthAdapter.sol";

contract Escrow is StaticStorage, ChainlinkClient, EthAdapter, Ownable {
  bool public fiatPaymentSuccessful;

  constructor() public {
  }

  function expectResponseFor(address _oracle, bytes32 _requestId) public onlyOwner {
    addChainlinkExternalRequest(_oracle, _requestId);
  }

  function fulfillFiatPayment(bytes32 _requestId, bool successful) public {
    validateChainlinkCallback(_requestId);
    fiatPaymentSuccessful = successful;
  }
}
