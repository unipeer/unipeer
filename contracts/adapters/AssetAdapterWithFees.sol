// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/math/SafeMath.sol";

import "./AssetAdapter.sol";

abstract contract AssetAdapterWithFees is AssetAdapter {
  uint16 public feeThousandthsPercent;
  uint256 public minFeeAmount;
  uint256 public collectedFees;

  constructor(uint16 _feeThousandthsPercent, uint256 _minFeeAmount) public {
    require(_feeThousandthsPercent < (1 << 16), "fee % too high");
    require(_minFeeAmount <= (1 << 255), "minFeeAmount too high");
    feeThousandthsPercent = _feeThousandthsPercent;
    minFeeAmount = _minFeeAmount;
  }

  function rawAccumulateFee(uint256 _amount) internal {
    collectedFees = SafeMath.add(collectedFees, _amount);
  }

  function accumulateFee(uint256 _amount) internal {
    rawAccumulateFee(getFee(_amount));
  }

  function withdrawFees(uint256 _amount, address payable _to) external virtual;

  function getFee(uint256 _amount) internal view returns (uint256) {
    uint256 fee = (_amount * feeThousandthsPercent) / 100000;
    return fee < minFeeAmount ? minFeeAmount : fee;
  }

  function getAmountWithFee(uint256 _amount) internal view returns (uint256) {
    uint256 baseAmount = _amount;
    return baseAmount + getFee(baseAmount);
  }

  function lockAssetWithFee(uint256 _amount) internal returns (uint256) {
    uint256 totalAmount = getAmountWithFee(_amount);
    rawLockAsset(totalAmount);
    return totalAmount;
  }

  function sendAssetWithFee(address payable _to, uint256 _amount) internal {
    rawSendAsset(getAmountWithFee(_amount), _to);
  }

  function sendAssetKeepingFee(uint256 _amount, address payable _to) internal {
    rawSendAsset(_amount, _to);
    accumulateFee(_amount);
  }
}
