/**
 * @authors: [@shalzz ]
 * @reviewers: []
 * @auditors: []
 * @bounties: []
 * @deployments: []
 * SPDX-License-Identifier: MIT
 */
pragma solidity ^0.8.0;

/**
 * @title IOracle
 * Oracle Interface for adapting to various
 * Subjective and Deterministic Oracles
 */
interface IOracle {

    /**
     * @dev Receive an answer from the Oracle.
     * @param _orderID ID of the order for which we are expecting an answer.
     * @param _answer The answer returned by the oracle that we trust.
     */
    function receiveAnswer(uint256 _orderID, uint256 _answer) external;
}
