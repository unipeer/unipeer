// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/utils/Address.sol";

import "./Escrow.sol";

contract EscrowFactory {
    event Created(
        address escrow,
        address owner,
        uint256 createdAt,
        string paymentid,
        uint256 amount
    );

    address payable comptroller;

    mapping(address => address[]) private escrows;

    constructor(address payable _comptroller) public {
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
        Escrow instance = new Escrow(msg.sender, comptroller, paymentid);

        escrow = address(instance);
        escrows[msg.sender].push(escrow);
        Address.sendValue(escrow, msg.value);

        // solhint-disable-next-line not-rely-on-time
        emit Created(escrow, msg.sender, block.timestamp, paymentid, msg.value);
    }
}
