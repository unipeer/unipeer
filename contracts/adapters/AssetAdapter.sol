// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.6.0;

abstract contract AssetAdapter {

  /**
   * @dev Get the current balance of the Asset held by the implementing contract.
   */
  function getBalance() internal virtual view returns (uint256 amount);

  /**
   * @dev Safe alternative to transfer/send for ether.
   * Reverts on failure.
   *
   * @param _recipient Address to send the funds from the contract
   * @param _amount Amount to transfer in the lowest unit (wei for ether)
   */
  function sendValue(address payable _recipient, uint256 _amount) internal virtual;


  /**
   * @dev Ensure the described asset is sent to the given address.
   * Should revert if the transfer failed, but callers must also handle `false` being returned,
   * much like ERC-20's `transfer`.
   *
   * @param _to Address to send the funds from the contract
   * @param _amount Amount to transfer in the lowest unit (wei for ether)
   */
  function rawSendAsset(address payable _to, uint256 _amount)
    internal
    virtual
    returns (bool success);
}
