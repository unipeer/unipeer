// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.6.0;

import "./AssetAdapter.sol";

contract EthAdapter is AssetAdapter {
  uint16 internal constant ETH_TYPE_ID = 1;

  constructor() internal AssetAdapter(ETH_TYPE_ID) {}

  /**
   * @dev Get the current balance of the Asset held by the implementing contract.
   */
  function getBalance() internal override view returns (uint256 amount) {
    return address(this).balance;
  }

  function rawSendAsset(uint256 _amount, address payable _to)
    internal
    override
    returns (bool success)
  {
    return _to.send(_amount);
  }
}
