// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;

import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Address.sol";

import "@nomiclabs/buidler/console.sol";

import "./Escrow.sol";
import "./adapters/EthAdapter.sol";

contract Comptroller is ChainlinkClient, Ownable, EthAdapter {
  bytes32 private jobId;
  uint256 private fee;

  /**
   *
   * @param _oracle The chainlink node oracle address to send requests
   * @param _jobId The JobId for the Request
   */
  constructor(
    address _link,
    address _oracle,
    bytes32 _jobId
  ) public {
    if (_link == address(0)) {
      setPublicChainlinkToken();
    } else {
      setChainlinkToken(_link);
    }
    setChainlinkOracle(_oracle);
    jobId = _jobId;
    fee = 0.01 * 10**18; // 0.01 LINK
  }

  function withdrawFees(address payable _to, uint256 _amount) public onlyOwner() {
    rawSendAsset(_amount, _to);
  }

  function requestFiatPayment(
    address _seller,
    address payable _buyer,
    uint256 amount,
    string calldata senderpaymentid
  ) public {
    require(
      Address.isContract(_seller),
      "Comptroller: seller should an escrow contract"
    );
    Escrow escrow = Escrow(payable(_seller));
    require(
      escrow.getUnlockedBalance() >= amount,
      "Comptroller: not enough funds in escrow"
    );

    Chainlink.Request memory req = buildChainlinkRequest(
      jobId, // Chainlink JobId
      _seller, // contract address with the callback function
      escrow.fulfillFiatPayment.selector // callback function selector
    );
    req.add("method", "collectrequest");
    req.add("receiver", escrow.paymentid());
    req.add("sender", senderpaymentid);
    req.addUint("amount", amount);

    bytes32 reqId = sendChainlinkRequest(req, fee);
    escrow.expectResponseFor(chainlinkOracleAddress(), reqId, _buyer, amount);
  }
}
