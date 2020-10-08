// SPDX-License-Identifier: GPL-3.0-only

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/utils/Address.sol";

import "./AssetAdapterWithFees.sol";

abstract contract EthAdapter is AssetAdapterWithFees {
  event Deposit(address, uint256);

  uint16 internal constant ETH_TYPE_ID = 1;

  constructor(uint16 _feeThousandthsPercent, uint256 _minFeeAmount)
    public
    AssetAdapterWithFees(_feeThousandthsPercent, _minFeeAmount)
  {}

  function getBalance() internal override view returns (uint256 amount) {
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
