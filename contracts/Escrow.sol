// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;

import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";

import "@nomiclabs/buidler/console.sol";

import "./adapters/EthAdapter.sol";

contract Escrow is EthAdapter, ChainlinkClient {
  struct Job {
    address payable buyer;
    uint256 amount;
  }
  mapping(bytes32 => Job) private jobs;

  address public owner;
  address payable public comptroller;
  string public paymentid;

  function initialize(address payable _comptroller, string calldata _paymentid)
    public
    initializer
  {
    EthAdapter.initialize();
    owner = msg.sender;
    comptroller = _comptroller; // TODO: change this to be static with solpp?
    paymentid = _paymentid;
  }

  modifier onlyOwner() {
    require(owner == msg.sender, "Ownable: caller is not the owner");
    _;
  }

  modifier onlyComptroller() {
    require(comptroller == msg.sender, "Escrow: caller is not the comptroller");
    _;
  }

  function withdraw(uint256 _amount, address payable _to) public onlyOwner() {
    require(
      getUnlockedBalance() >= _amount,
      "Escrow: insufficient unlocked funds to withdraw"
    );
    rawSendAsset(_amount, _to);
  }

  function expectResponseFor(
    address _oracle,
    bytes32 _requestId,
    address payable _buyer,
    uint256 _amount
  ) public onlyComptroller {
    lockAssetWithFee(_amount);                        // check
    jobs[_requestId] = Job({                          // effects
      buyer: _buyer,
      amount: _amount
    });
    addChainlinkExternalRequest(_oracle, _requestId); // interaction
  }

  function fulfillFiatPayment(bytes32 _requestId, bool successful) public {
    validateChainlinkCallback(_requestId);

    Job memory job = jobs[_requestId];
    delete jobs[_requestId]; // cleanup storage

    if (successful) {
      sendAssetWithFee(job.amount, job.buyer, comptroller);
    } else {
      unlockAssetWithFee(job.amount);
    }
  }
}
