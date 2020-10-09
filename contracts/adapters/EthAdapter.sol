// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/utils/Address.sol";

import "./AssetAdapterWithFees.sol";

abstract contract EthAdapter is AssetAdapterWithFees {
  event Deposit(address, uint256);
  event AmountLocked(address indexed seller, uint256 amount);
  event AmountUnlocked(address indexed seller, uint256 amount);

  uint16 internal constant ETH_TYPE_ID = 1;
  uint256 private _amountLocked;
  uint256 private collectedFees;

  function initialize() public initializer {
    AssetAdapterWithFees.initialize(490, 100 * 10**9); /* 0.49% or 100 gwei */
  }

  function getBalance() internal override view returns (uint256 amount) {
    return address(this).balance;
  }

  /**
   * @dev Use openzeppelins Address#sendValue to circumvent gas price increase after
   * the istanbul fork. See Address#sendValue for more details or
   * https://diligence.consensys.net/blog/2019/09/stop-using-soliditys-transfer-now/.
   */
  function rawSendAsset(uint256 _amount, address payable _to)
    internal
    override
  {
    Address.sendValue(_to, _amount);
  }

  function getUnlockedBalance() public override view returns (uint256) {
    return SafeMath.sub(getBalance(), _amountLocked);
  }

  function rawLockAsset(uint256 _amount) internal override {
    require(
      getUnlockedBalance() >= _amount,
      "EthAdapter: insufficient funds to lock"
    );
    _amountLocked = SafeMath.add(_amountLocked, _amount);
    emit AmountLocked(address(this), _amount);
  }

  function rawUnlockAsset(uint256 _amount) internal override {
    _amountLocked = SafeMath.sub(_amountLocked, _amount);
    emit AmountUnlocked(address(this), _amount);
  }

  function rawAccumulateFee(uint256 _amount) internal override {
    collectedFees = SafeMath.add(collectedFees, _amount);
  }

  function getAccumulatedFees() public override view returns (uint256 amount) {
    return collectedFees;
  }

  /**
   * @dev We have the payable receive function to accept ether payment only
   * and not the fallback function to avoid delegating calls further.
   *
   * TODO: Switch to using a specific deposit function?
   */
  receive() external payable {
    emit Deposit(msg.sender, msg.value);
  }
}
