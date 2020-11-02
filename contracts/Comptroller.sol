// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;

import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";
import "@chainlink/contracts/src/v0.6/LinkTokenReceiver.sol";
import "@chainlink/contracts/src/v0.6/interfaces/LinkTokenInterface.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Address.sol";

import "hardhat/console.sol";

import "./Escrow.sol";
import "./WithStatus.sol";

contract Comptroller is ChainlinkClient, WithStatus, LinkTokenReceiver {
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

  function withdrawFees(address payable _to, uint256 _amount)
    public
    onlyOwner()
    statusAtLeast(Status.RETURN_ONLY)
  {
    Address.sendValue(_to, _amount);
  }

  /**
   * @notice Returns the address of the LINK token
   * @dev This is the public implementation for chainlinkTokenAddress, which is
   * an internal method of the ChainlinkClient contract
   */
  function getChainlinkToken() public view override returns (address) {
    return chainlinkTokenAddress();
  }

  function createFiatPaymentWithLinkRequest(
    address _seller,
    address payable _buyer,
    uint256 _amount,
    string calldata _senderpaymentid
  ) public statusAtLeast(Status.ACTIVE) {
    bytes memory payload = abi.encodeWithSignature(
      "requestFiatPaymentWithLink(address,address,uint256,string)",
      _seller,
      _buyer,
      _amount,
      _senderpaymentid
    );

    require(
      LinkTokenInterface(chainlinkTokenAddress()).transferAndCall(
        address(this),
        fee,
        payload
      ),
      "comptroller: unable to transferAndCall"
    );
  }

  function requestFiatPaymentWithLink(
    address _seller,
    address payable _buyer,
    uint256 _amount,
    string calldata _senderpaymentid
  ) public onlyLINK() {
    _requestFiatPayment(_seller, _buyer, _amount, _senderpaymentid);
  }

  function requestFiatPayment(
    address _seller,
    address payable _buyer,
    uint256 _amount,
    string calldata _senderpaymentid /* onlyOwner() */
  ) public {
    _requestFiatPayment(_seller, _buyer, _amount, _senderpaymentid);
  }

  function _requestFiatPayment(
    address _seller,
    address payable _buyer,
    uint256 _amount,
    string calldata _senderpaymentid
  ) internal statusAtLeast(Status.ACTIVE) {
    require(
      Address.isContract(_seller),
      "Comptroller: seller should an escrow contract"
    );
    Escrow escrow = Escrow(payable(_seller));
    require(
      escrow.getUnlockedBalance() >= _amount,
      "Comptroller: not enough funds in escrow"
    );

    Chainlink.Request memory req = buildChainlinkRequest(
      jobId, // Chainlink JobId
      _seller, // contract address with the callback function
      escrow.fulfillFiatPayment.selector // callback function selector
    );
    req.add("method", "collectrequest");
    req.add("receiver", escrow.paymentid());
    req.add("sender", _senderpaymentid);
    req.addUint("amount", _amount);

    bytes32 reqId = sendChainlinkRequest(req, fee);
    escrow.expectResponseFor(chainlinkOracleAddress(), reqId, _buyer, _amount);
  }

  /**
   * @dev We have the payable receive function to accept ether payment only
   * and not the fallback function to avoid delegating calls further.
   */
  receive() external payable {}
}
