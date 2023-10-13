// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.15;

import "forge-std/Script.sol";
import "kleros/IArbitrator.sol";

import "../src/Unipeer.sol";

contract Deploy is Script {
    uint256 constant SHARED_MULTI = 10_000;
    uint256 constant WIN_MULTI = 5000;
    uint256 constant LOSE_MULTI = 20_000;

    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        IArbitrator arbitrator = IArbitrator(0x60cE8c27757399735969d736Ba3987586501e514);
        Unipeer unipeer = new Unipeer({
            _admin: 0x3b434e0D2a6C7F53d5C556D7BAeE8942c351Cf1a,
            _relay: 0x98CbFB4F664e6b35a32930c90e43F03b5Eab50DA,
            _arbitrator: arbitrator,
            _arbitratorExtraData: bytes(""),
            _buyerTimeout: 30 minutes,
            _sellerTimeout: 30 minutes,
            _sharedStakeMultiplier: SHARED_MULTI,
            _winnerStakeMultiplier: WIN_MULTI,
            _loserStakeMultiplier: LOSE_MULTI,
            _tradeFeeRate: 5
        });

        vm.stopBroadcast();
    }
}
