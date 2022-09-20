pragma solidity ^0.8.9;

import "kleros/examples/CentralizedArbitratorWithAppeal.sol";

/**
 * Fixes the appealCost function of the CentralizedArbitratorWithAppeal
 */
contract Arbitrator is CentralizedArbitratorWithAppeal {


    function appealCost(uint256 _disputeID, bytes memory _extraData) public view override returns (uint256) {
        return arbitrationFee * 2;
    }
}
