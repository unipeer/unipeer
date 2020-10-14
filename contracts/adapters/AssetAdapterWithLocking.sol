// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.6.0;

import "./AssetAdapter.sol";

abstract contract AssetAdapterWithLocking is AssetAdapter {
  event AmountLocked(address indexed seller, uint256 amount);
  event AmountUnlocked(address indexed seller, uint256 amount);

  uint256 private _amountLocked;

  function getUnlockedBalance() public view returns (uint256) {
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
}
