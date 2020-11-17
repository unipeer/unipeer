// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;

import "./Escrow.sol";
import "./adapters/EthAdapter.sol";

contract EthEscrow is EthAdapter, Escrow {
  function initialize(
    address _owner,
    address payable _comptroller,
    string calldata _paymentid
  ) public initializer {
    Escrow.initializeEscrow(_owner, _comptroller, _paymentid);
  }
}
