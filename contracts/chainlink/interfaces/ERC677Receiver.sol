// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.6.0;


interface ERC677Receiver {
  function onTokenTransfer(address _sender, uint _value, bytes memory _data) external;
}
