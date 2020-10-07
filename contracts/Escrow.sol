// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;

import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";
import "@openzeppelin/contracts/proxy/Initializable.sol";

import "@nomiclabs/buidler/console.sol";

import "./adapters/EthAdapter.sol";

contract Escrow is Initializable, EthAdapter, ChainlinkClient() {
  event AmountLocked(address indexed seller, uint256 amount);
  event AmountUnlocked(address indexed seller, uint256 amount);

  struct Job {
    address buyer;
    uint256 amount;
  }

  address public owner;
  address public comptroller;

  uint256 private amountLocked;
  mapping(bytes32 => Job) private jobs;

  string public paymentid;

  function initialize(address _comptroller, string calldata _paymentid) public initializer {
    owner = msg.sender;
    comptroller = _comptroller;  // TODO: change this to be static with solpp?
    paymentid = _paymentid;
  }

  /**
   * @dev Throws if called by any account other than the owner.
   */
  modifier onlyOwner() {
    require(owner == msg.sender, "Ownable: caller is not the owner");
    _;
  }

  /**
   * @dev Throws if called by any account other than the owner.
   */
  modifier onlyComptroller() {
    require(comptroller == msg.sender, "Escrow: caller is not the comptroller");
    _;
  }

  function _lockAmount(uint256 _amount) internal {
    require(
      getUnlockedBalance() >= _amount,
      "Escrow: insufficient funds to lock"
    );
    amountLocked = amountLocked.add(_amount);
    emit AmountLocked(address(this), _amount);
  }

  function _unlockAmount(uint256 _amount) internal {
    amountLocked = amountLocked.sub(_amount);
    emit AmountUnlocked(address(this), _amount);
  }

  function getUnlockedBalance() public view returns (uint256) {
    return getBalance().sub(amountLocked);
  }

  function withdraw(uint256 _amount, address _to) public onlyOwner() {
    sendValue(payable(_to), _amount);
  }

  function expectResponseFor(
    address _oracle,
    bytes32 _requestId,
    address _buyer,
    uint256 _amount
  ) public onlyComptroller {
    _lockAmount(_amount);
    jobs[_requestId] = Job({buyer: _buyer, amount: _amount});
    addChainlinkExternalRequest(_oracle, _requestId);
  }

  function fulfillFiatPayment(bytes32 _requestId, bool successful) public {
    validateChainlinkCallback(_requestId);

    Job memory job = jobs[_requestId];
    delete jobs[_requestId]; // cleanup storage

    if (successful) {
      sendValue(payable(job.buyer), job.amount);
    } else {
      _unlockAmount(job.amount);
    }
  }
}
