// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract SimpleEscrow is Ownable {
    /* 0.49% or 100 gwei */
    uint16 public constant feeThousandthsPercent = 490;
    uint256 public constant minFeeAmount = 100 * 10**9;

    address payable public comptroller;
    address public node;

    constructor(address payable _comptroller, address _node) public {
        comptroller = _comptroller;
        node = _node;
    }

    modifier onlyChainlinkNode() {
        require(node == msg.sender, "SimpleEscrow: caller is not the node");
        _;
    }

    function withdraw(uint256 _amount, address payable _to) public onlyOwner() {
        require(
            address(this).balance >= _amount,
            "Escrow: insufficient funds to withdraw"
        );
        Address.sendValue(_to, _amount);
    }

    function getFee(uint256 _amount) internal pure returns (uint256) {
        uint256 fee = (_amount * feeThousandthsPercent) / 100000;
        return fee < minFeeAmount ? minFeeAmount : fee;
    }

    function getAmountWithFee(uint256 _amount) public pure returns (uint256) {
        uint256 baseAmount = _amount;
        return baseAmount + getFee(baseAmount);
    }

    function chainlinkTransfer(address payable _to, uint256 _amount)
        public
        onlyChainlinkNode
    {
        require(
            address(this).balance >= _amount,
            "Escrow: not enough funds to send"
        );
        Address.sendValue(_to, _amount);
        Address.sendValue(comptroller, getFee(_amount));
    }

    /**
     * @dev We have the payable receive function to accept ether payment only
     * and not the fallback function to avoid delegating calls further.
     */
    receive() external payable {}
}
