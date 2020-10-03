// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.6.0;

import "@openzeppelin/contracts/utils/Address.sol";

import "./AssetAdapter.sol";

contract EthAdapter is AssetAdapter {
  event Deposit(address, uint);

  uint16 internal constant ETH_TYPE_ID = 1;

  function getBalance() internal override view returns (uint256 amount) {
    return address(this).balance;
  }

  /**
   * Use openzeppelins Address#sendValue to circumvent gas price increase after
   * the istanbul fork.
   *
   * See Address#sendValue for more details or
   * https://diligence.consensys.net/blog/2019/09/stop-using-soliditys-transfer-now/.
   */
  function sendValue(address payable _recipient, uint256 _amount) internal override {
    Address.sendValue(_recipient, _amount);
  }

  function rawSendAsset(address payable _to, uint256 _amount)
    internal
    override
    returns (bool success)
  {
    return _to.send(_amount);
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
