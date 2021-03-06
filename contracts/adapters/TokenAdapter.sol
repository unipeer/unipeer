// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";

import "./AssetAdapterWithFees.sol";

contract TokenAdapter is AssetAdapterWithFees {
    using SafeERC20 for IERC20;

    address public tokenAddress;

    /* 0.49% or 100 gwei */
    constructor(
        address _token,
        uint16 _feeThousandthsPercent,
        uint256 _minFeeAmount
    ) public AssetAdapterWithFees(_feeThousandthsPercent, _minFeeAmount) {
        tokenAddress = _token;
    }

    function getBalance() public view override returns (uint256 amount) {
        return IERC20(tokenAddress).balanceOf(address(this));
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
        IERC20(tokenAddress).safeTransfer(_to, _amount);
    }
}
