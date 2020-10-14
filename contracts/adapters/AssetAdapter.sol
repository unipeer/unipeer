// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/math/SafeMath.sol";

abstract contract AssetAdapter {
  /**
   * @dev Get the current balance of the Asset held by the implementing contract.
   */
  function getBalance() internal virtual view returns (uint256 amount);

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
}
