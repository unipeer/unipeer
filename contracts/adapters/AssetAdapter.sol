// SPDX-License-Identifier: MIT

pragma solidity ^0.6.0;

abstract contract AssetAdapter {
  uint16 public ASSET_TYPE;

  constructor(uint16 assetType) internal {
    ASSET_TYPE = assetType;
  }

  /**
   * Get the current balance of the Asset held by the implementing contract.
   */
  function getAmount() internal virtual view returns (uint256);

  /**
   * Ensure the described asset is sent to the given address.
   * Should revert if the transfer failed, but callers must also handle `false` being returned,
   * much like ERC-20's `transfer`.
   *
   * @param _amount Amount to transfer in the lowest unit (wei for ether)
   * @param _to Address to send the funds from the contract
   */
  function rawSendAsset(uint256 _amount, address payable _to)
    internal
    virtual
    returns (bool success);
}
