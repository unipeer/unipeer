// SPDX-License-Identifier: MIT

pragma solidity >=0.6.0 <0.7.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@nomiclabs/buidler/console.sol";
import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";
import "./StaticProxy.sol";

contract Escrow is StaticStorage, ChainlinkClient {
    address private oracle;
    bytes32 private jobId;
    uint256 private fee;

    bool public fiatPaymentSuccessful;

    /**
     * Network: Ropsten
     * Oracle:
     *      Name:           Omniscience Ropsten
     *      Listing URL:    https://market.link/nodes/57587577-8ded-4d56-bb89-14da301e71cb
     *      Address:        0x83dA1beEb89Ffaf56d0B7C50aFB0A66Fb4DF8cB1
     * Job:
     *      Name:           ETH-USD CoinGecko
     *      Listing URL:    https://market.link/jobs/d630df4f-1ed1-449b-8c0b-c27ab7a581a2
     *      ID:             93547cb3c6784ec08a366be6211caa24
     *      Fee:            0.1 LINK
     */
    constructor() public {
        setPublicChainlinkToken();
        oracle = 0x83dA1beEb89Ffaf56d0B7C50aFB0A66Fb4DF8cB1; // oracle address
        jobId = "93547cb3c6784ec08a366be6211caa24"; //job id
        fee = 0.1 * 10 ** 18; // 0.1 LINK
    }

    /**
     * Make initial request
     */
    function requestFiatPayment() public {
        Chainlink.Request memory req = buildChainlinkRequest(jobId, address(this), this.fulfillFiatPayment.selector);
        sendChainlinkRequestTo(oracle, req, fee);
    }

    /**
     * Callback function
     */
    function fulfillFiatPayment(bytes32 _requestId, bool successful) public recordChainlinkFulfillment(_requestId) {
        fiatPaymentSuccessful = successful;
    }
}
