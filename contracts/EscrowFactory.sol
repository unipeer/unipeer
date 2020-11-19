// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/utils/Address.sol";

import "./StaticProxy.sol";

contract EscrowFactory {
    event Created(
        address escrow,
        address owner,
        uint256 createdAt,
        string paymentid,
        uint256 amount
    );

    address comptroller;
    address escrowImpl;
    mapping(address => address) tokenEscrowImpls;

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
            "initialize(address,address,string)",
            msg.sender,
            comptroller,
            paymentid
        );

        StaticProxy proxy = new StaticProxy(escrowImpl, payload);
        escrow = address(proxy);
        escrows[msg.sender].push(escrow);
        Address.sendValue(escrow, msg.value);
        emit Created(escrow, msg.sender, block.timestamp, paymentid, msg.value);
    }

    function newTokenEscrow(address token, string calldata paymentid)
        public
        payable
        returns (address payable escrow)
    {
        bytes memory payload = abi.encodeWithSignature(
            "initialize(address,address,string,address)",
            msg.sender,
            comptroller,
            paymentid,
            token
        );

        address tokenEscrowImpl = tokenEscrowImpls[token];
        StaticProxy proxy = new StaticProxy(tokenEscrowImpl, payload);
        escrow = address(proxy);
        escrows[msg.sender].push(escrow);
        emit Created(escrow, msg.sender, block.timestamp, paymentid, msg.value);

        Address.sendValue(escrow, msg.value);
    }
}
