// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/math/SafeMath.sol";

abstract contract AssetAdapter {
  event AmountLocked(address indexed seller, uint256 amount);
  event AmountUnlocked(address indexed seller, uint256 amount);

  uint256 private _amountLocked;

  /**
   * @dev Get the current balance of the Asset held by the implementing contract.
   */
  function getBalance() internal virtual view returns (uint256 amount);

  function getUnlockedBalance() public view returns (uint256) {
    return SafeMath.sub(getBalance(), _amountLocked);
  }

  /**
   * @dev Ensure the described asset is sent to the given address.
   * Reverts on failure.
   *
   * @param _amount Amount to transfer in the lowest unit (wei for ether)
   * @param _recipient Address to send the funds from the contract
   */
  function rawSendAsset(uint256 _amount, address payable _recipient)
    internal
    virtual;

  /**
   * @dev Ensure the described asset is locked and not available for spend.
   * Reverts on failure.
   */
  function rawLockAsset(uint256 _amount) internal {
    require(
      getUnlockedBalance() >= _amount,
      "AssetAdapter: insufficient funds to lock"
    );
    _amountLocked = SafeMath.add(_amountLocked, _amount);
    emit AmountLocked(address(this), _amount);
  }

  function rawUnlockAsset(uint256 _amount) internal {
    _amountLocked = SafeMath.sub(_amountLocked, _amount);
    emit AmountUnlocked(address(this), _amount);
  }
}
