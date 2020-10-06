// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.6.0;

interface ERC677 {
  function transferAndCall(address to, uint value, bytes memory data) external returns (bool success);

  event Transfer(address indexed from, address indexed to, uint value, bytes data);
}
