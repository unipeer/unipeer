// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;

import "@chainlink/contracts/src/v0.6/Oracle.sol";

contract CeloOracle is Oracle {
    /**
     * @notice Deploy with the address of the LINK token
     * @dev Sets the LinkToken address for the imported LinkTokenInterface
     * @param _link The address of the LINK token
     */
    constructor(address _link) public Oracle(_link) {}
}
