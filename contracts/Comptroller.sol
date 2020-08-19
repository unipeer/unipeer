// SPDX-License-Identifier: MIT

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@nomiclabs/buidler/console.sol";
import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";

import "./Escrow.sol";

contract Comptroller is ChainlinkClient {
  bytes32 private jobId;
  uint256 private fee;

  /**
   *
   * @param _oracle The chainlink node oracle address to send requests
   * @param _jobId The JobId for the Request
   */
  constructor(address _oracle, bytes32 _jobId) public {
    setPublicChainlinkToken();
    setChainlinkOracle(_oracle);
    jobId = _jobId;
    fee = 0.1 * 10**18; // 0.1 LINK
  }

  function requestFiatPayment(address _seller, details) public {
    Escrow escrow = Escrow(_seller);
    Chainlink.Request memory req = buildChainlinkRequest(
      jobId,
      _seller,
      escrow.fulfillFiatPayment.selector
    );
    bytes32 reqId = sendChainlinkRequest(req, fee);
    escrow.expectResponseFor(chainlinkOracleAddress(), reqId);
  }
}
