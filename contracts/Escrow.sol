// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.6.0;

import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/proxy/Initializable.sol";

import "@nomiclabs/buidler/console.sol";

import "./adapters/EthAdapter.sol";

contract Escrow is Initializable, Ownable, EthAdapter, ChainlinkClient {

  event AmountLocked(address indexed seller, uint256 amount);
  event AmountUnlocked(address indexed seller, uint256 amount);

  struct Job {
    address buyer;
    uint256 amount;
  }

  address public comptroller;
  uint256 lockedAmount;
  mapping(bytes32 => Job) jobs;

  // TODO: change this to be static with solpp?
  function initialize(address _comptroller) public initializer {
    comptroller = _comptroller;
  }

  modifier onlyComptroller() {
    require(comptroller == msg.sender, "Escrow: caller is not the comptroller");
    _;
  }

  function getUnlockedBalance() public view returns (uint256 amount) {
    return getBalance().sub(lockedAmount);
  }

  function withdraw(uint256 _amount, address _to) public onlyOwner() returns (bool success) {
    require(
      getUnlockedBalance() > _amount,
      "Escrow: cannot withdraw more than unlocked balance"
    );
    return rawSendAsset(_amount, payable(_to));
  }

  function lockAmount(uint256 _amount) internal {
    require(
      getUnlockedBalance() > _amount,
      "Escrow: insufficient funds to lock"
    );
    lockedAmount.add(_amount);
    emit AmountLocked(address(this), _amount);
  }

  function unlockAmount(uint256 _amount) internal {
    lockedAmount.sub(_amount);
    emit AmountUnlocked(address(this), _amount);
  }

  function expectResponseFor(
    address _oracle,
    bytes32 _requestId,
    address _buyer,
    uint256 _amount
  ) public onlyComptroller {
    lockAmount(_amount);
    jobs[_requestId] = Job({buyer: _buyer, amount: _amount});
    addChainlinkExternalRequest(_oracle, _requestId);
  }

  function fulfillFiatPayment(bytes32 _requestId, bool successful) public {
    validateChainlinkCallback(_requestId);

    Job memory job = jobs[_requestId];
    delete jobs[_requestId]; // cleanup storage

    if (successful) {
      rawSendAsset(job.amount, payable(job.buyer));
    } else {
      unlockAmount(job.amount);
    }
  }
}
