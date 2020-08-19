// SPDX-License-Identifier: MIT

pragma solidity >=0.4.21 <0.7.0;

import "./Proxy.sol";

/**
 * @title StaticStorage
 * @dev Implemented as a separate contract that is inheretited
 * so that the storage hierarchy is structured between both
 * the proxy and the logic contract.
 * When upgrading the Logic contract it is up to us to make
 * sure storage conflicts do not occur between different versions
 * of the logic contract.
 * It is up to the developer to have new versions of a logic contract
 * extend previous versions or manually guarantee that the storage hierarchy is
 * always appended to but not modified.
 *
 * For eg:
 * |Implementation_v0   |Implementation_v1        |
 * |--------------------|-------------------------|
 * |address _owner      |address _owner           |
 * |mapping _balances   |mapping _balances        |
 * |uint256 _supply     |uint256 _supply          |
 * |...                 |address _lastContributor | <=== Storage extension.
 * |                    |...                      |
 *
 */
contract StaticStorage {
  address internal implementation;
}

contract StaticProxy is Proxy, StaticStorage {
  constructor(address _implementation) public {
    implementation = _implementation;
  }

  function _implementation() public override view returns (address) {
    return implementation;
  }
}
