// SPDX-License-Identifier: UNLICENSED

pragma solidity >=0.4.21 <0.7.0;

/**
 * @title Proxy
 * @dev Implements delegation of calls to other contracts, with proper
 * forwarding of return values and bubbling of failures.
 * It defines a fallback function that delegates all calls to the address
 * returned by the abstract implementation() public function.
 */
abstract contract Proxy {
  /**
   * @return The Address of the implementation.
   */
  function _implementation() public virtual view returns (address);

  /**
   * @dev Fallback function.
   * Implemented entirely in `_fallback`.
   */
  fallback() external payable {
    _fallback();
  }

  /**
   * @dev Receive function called when no calldata is provided
   * Implemented entirely in `_fallback`.
   */
  receive() external payable {
    _fallback();
  }

  /**
   * @dev fallback implementation.
   * Extracted to enable manual triggering.
   */
  function _fallback() internal {
    address _impl = _implementation();
    require(_impl != address(0), "impl is empty");

    _delegate(_impl);
  }

  /**
   * @dev Delegates execution to an implementation contract.
   * This is a low level function that doesn't return to its internal call site.
   * It will return to the external caller whatever the implementation returns.
   * @param _impl Address to delegate.
   */
  function _delegate(address _impl) internal {
    assembly {
      let ptr := mload(0x40)

      // (1) copy incoming call data
      calldatacopy(ptr, 0, calldatasize())

      // (2) forward call to logic contract
      // out and outsize are 0 because we don't know the size yet.
      let result := delegatecall(gas(), _impl, ptr, calldatasize(), 0, 0)
      let size := returndatasize()

      // (3) retrieve return data
      returndatacopy(ptr, 0, size)

      // (4) forward return data back to caller
      switch result
        // delegatecall returns 0 on error.
        case 0 {
          revert(ptr, size)
        }
        default {
          return(ptr, size)
        }
    }
  }
}
