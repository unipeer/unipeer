// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/utils/Address.sol";

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

  function sendValue(address payable _recipient, uint256 _amount) internal override {
    Address.sendValue(_recipient, _amount);
  }

  function rawSendAsset(address payable _to, uint256 _amount)
    internal
    override
    returns (bool success)
  {
    return _to.send(_amount);
  }
}
