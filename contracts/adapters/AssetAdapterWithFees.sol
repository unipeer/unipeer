// SPDX-License-Identifier: AGPL-3.0-or-later

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/math/SafeMath.sol";

import "./AssetAdapterWithLocking.sol";

abstract contract AssetAdapterWithFees is AssetAdapterWithLocking {
    uint16 public feeThousandthsPercent;
    uint256 public minFeeAmount;

    /**
     * @param _feeThousandthsPercent The fee percentage with three decimal places.
     * @param _minFeeAmount The minimuim fee to charge.
     */
    constructor(uint16 _feeThousandthsPercent, uint256 _minFeeAmount) public {
        require(_feeThousandthsPercent < (1 << 16), "fee % too high");
        require(_minFeeAmount <= (1 << 255), "minFeeAmount too high");
        feeThousandthsPercent = _feeThousandthsPercent;
        minFeeAmount = _minFeeAmount;
    }

    function getFee(uint256 _amount) internal view returns (uint256) {
        uint256 fee = (_amount * feeThousandthsPercent) / 100000;
        return fee < minFeeAmount ? minFeeAmount : fee;
    }

    function getAmountWithFee(uint256 _amount) public view returns (uint256) {
        uint256 baseAmount = _amount;
        return baseAmount + getFee(baseAmount);
    }

    function lockAssetWithFee(uint256 _amount) internal {
        uint256 totalAmount = getAmountWithFee(_amount);
        rawLockAsset(totalAmount);
    }

    function unlockAssetWithFee(uint256 _amount) internal {
        uint256 totalAmount = getAmountWithFee(_amount);
        rawUnlockAsset(totalAmount);
    }

    function sendAssetWithFee(
        address payable _to,
        uint256 _amount,
        address payable _feeCollector
    ) internal {
        unlockAssetWithFee(_amount);
        rawSendAsset(_amount, _to);
        rawSendAsset(getFee(_amount), _feeCollector);
    }
}
