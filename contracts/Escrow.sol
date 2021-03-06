// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;
pragma experimental ABIEncoderV2;

import "@chainlink/contracts/src/v0.6/ChainlinkClient.sol";

import "hardhat/console.sol";

import "./adapters/EthAdapter.sol";
import "./utils/WithStatus.sol";

contract Escrow is ChainlinkClient, EthAdapter {
    struct Job {
        address payable buyer;
        uint256 amount;
    }
    mapping(bytes32 => Job) private jobs;

    address public owner;
    address payable public comptroller;
    string public paymentid;

    constructor(
        address _owner,
        address payable _comptroller,
        string memory _paymentid
    ) public {
        owner = _owner;
        comptroller = _comptroller; // TODO: change this to be static with solpp?
        paymentid = _paymentid;
    }

    modifier onlyOwner() {
        require(owner == msg.sender, "Ownable: caller is not the owner");
        _;
    }

    modifier onlyComptroller() {
        require(
            comptroller == msg.sender,
            "Escrow: caller is not the comptroller"
        );
        _;
    }

    modifier statusAtLeast(Status _status) {
        require(
            WithStatus(comptroller).status() >= _status,
            "invalid contract status"
        );
        _;
    }

    function withdraw(uint256 _amount, address payable _to)
        public
        onlyOwner()
        statusAtLeast(Status.RETURN_ONLY)
    {
        require(
            getUnlockedBalance() >= _amount,
            "Escrow: insufficient unlocked funds to withdraw"
        );
        rawSendAsset(_amount, _to);
    }

    function expectResponseFor(
        address _oracle,
        bytes32 _requestId,
        address payable _buyer,
        uint256 _amount
    ) public onlyComptroller statusAtLeast(Status.FINALIZE_ONLY) {
        lockAssetWithFee(_amount); // check
        jobs[_requestId] = Job({ buyer: _buyer, amount: _amount }); // effects
        addChainlinkExternalRequest(_oracle, _requestId); // interaction
    }

    function fulfillFiatPayment(bytes32 _requestId, bool successful) public {
        validateChainlinkCallback(_requestId);

        Job memory job = jobs[_requestId];
        delete jobs[_requestId]; // cleanup storage

        if (successful) {
            sendAssetWithFee(job.buyer, job.amount, comptroller);
        } else {
            unlockAssetWithFee(job.amount);
        }
    }
}
