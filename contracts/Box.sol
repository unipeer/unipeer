// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.6.0;

import "@nomiclabs/buidler/console.sol";

contract Box {
    uint256 private value;

    // Emitted when the stored value changes
    event ValueChanged(uint256 newValue);

    // Stores a new value in the contract
    function store(uint256 newValue) public {
        console.log("Changing value from", value, "to", newValue);
        value = newValue;
        emit ValueChanged(newValue);
    }

    // Reads the last stored value
    function retrieve() public view returns (uint256) {
        return value;
    }
}
