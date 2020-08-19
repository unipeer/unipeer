// SPDX-License-Identifier: MIT

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@nomiclabs/buidler/console.sol";
import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";
import "./StaticProxy.sol";

contract Escrow is StaticStorage, ChainlinkClient {
  bytes32 private jobId;
  uint256 private fee;

  bool public fiatPaymentSuccessful;

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

  function requestFiatPayment() public {
    Chainlink.Request memory req = buildChainlinkRequest(
      jobId,
      address(this),
      this.fulfillFiatPayment.selector
    );
    sendChainlinkRequest(req, fee);
  }

  function fulfillFiatPayment(bytes32 _requestId, bool successful)
    public
    recordChainlinkFulfillment(_requestId)
  {
    fiatPaymentSuccessful = successful;
  }
}
