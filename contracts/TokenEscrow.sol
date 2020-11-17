// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;

import "./Escrow.sol";
import "./adapters/TokenAdapter.sol";

contract TokenEscrow is TokenAdapter, Escrow {
  function initialize(
    address _owner,
    address payable _comptroller,
    string calldata _paymentid,
    address _token
  ) public initializer {
    Escrow.initializeEscrow(_owner, _comptroller, _paymentid);
    tokenAddress = _token;
  }
}
