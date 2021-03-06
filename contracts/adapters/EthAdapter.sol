// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/utils/Address.sol";

import "./AssetAdapterWithFees.sol";

/* 0.49% or 100 gwei */
contract EthAdapter is AssetAdapterWithFees(490, 100 * 10**9) {
    event Deposit(address indexed from, uint256 amount);

    function getBalance() public view override returns (uint256 amount) {
        return address(this).balance;
    }

    /**
     * @dev Use openzeppelins Address#sendValue to circumvent gas price increase after
     * the istanbul fork. See Address#sendValue for more details or
     * https://diligence.consensys.net/blog/2019/09/stop-using-soliditys-transfer-now/.
     */
    function rawSendAsset(uint256 _amount, address payable _to)
        internal
        override
    {
        Address.sendValue(_to, _amount);
    }

    /**
     * @dev We have the payable receive function to accept ether payment only
     * and not the fallback function to avoid delegating calls further.
     *
     * TODO: Switch to using a specific deposit function?
     */
    receive() external payable {
        emit Deposit(msg.sender, msg.value);
    }
}
