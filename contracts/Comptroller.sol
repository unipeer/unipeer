// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@nomiclabs/buidler/console.sol";
import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";

import "./Escrow.sol";

contract Comptroller is ChainlinkClient {
  bytes32 private jobId;
  uint256 private fee;

  struct PaymentDetails {
    string sender;
    uint256 amount;
  }

  /**
   *
   * @param _oracle The chainlink node oracle address to send requests
   * @param _jobId The JobId for the Request
   */
  constructor(address _oracle, bytes32 _jobId) public {
    setPublicChainlinkToken();
    setChainlinkOracle(_oracle);
    jobId = _jobId;
    fee = 0.01 * 10**18; // 0.01 LINK
  }

  function requestFiatPayment(
    address _seller,
    address _buyer,
    PaymentDetails calldata payment
  ) public {
    Escrow escrow = Escrow(payable(_seller));
    require(
      escrow.getUnlockedBalance() > payment.amount,
      "Comptroller: not enough funds in escrow"
    );

    Chainlink.Request memory req = buildChainlinkRequest(
      jobId, // Chainlink JobId
      _seller, // contract address with the callback function
      escrow.fulfillFiatPayment.selector // callback function selector
    );
    req.add("method", "collectrequest");
    req.add("receiver", escrow.paymentid());
    req.add("sender", payment.sender);
    req.addUint("amount", payment.amount);

    bytes32 reqId = sendChainlinkRequest(req, fee);
    escrow.expectResponseFor(
      chainlinkOracleAddress(),
      reqId,
      _buyer,
      payment.amount
    );
  }
}
