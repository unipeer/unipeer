// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;

import "./StaticProxy.sol";

contract EscrowContractFactory {
  event Created(
    address escrow,
    address owner,
    uint256 createdAt,
    string paymentid,
    uint256 amount
  );

  address escrowImpl;
  address comptroller;

  mapping(address => address[]) escrows;

  constructor(address _escrow, address _comptroller) public {
    escrowImpl = _escrow;
    comptroller = _comptroller;
  }

  function getEscrows(address _user) public view returns (address[] memory) {
    return escrows[_user];
  }

  function newEscrow(string calldata paymentid)
    public
    payable
    returns (address payable escrow)
  {
    bytes memory payload = abi.encodeWithSignature(
      "initialize(address,string)",
      comptroller,
      paymentid
    );
    StaticProxy proxy = new StaticProxy(escrowImpl, payload);
    escrow = address(proxy);
    escrows[msg.sender].push(escrow);
    escrow.transfer(msg.value);
    emit Created(escrow, msg.sender, block.timestamp, paymentid, msg.value);
  }
}
