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

        // Kleros Liquid
        IArbitrator arbitrator = IArbitrator(0x1128eD55ab2d796fa92D2F8E1f336d745354a77A);
        // Centralized arbitrator
        // IArbitrator arbitrator = IArbitrator(0xADfaB872278c09E2C826b6aF2b974A7129Ee1666);
        Unipeer unipeer = new Unipeer({
            _admin: 0x3b434e0D2a6C7F53d5C556D7BAeE8942c351Cf1a,
            _relay: 0x03d68902B6d2300427effD6354C644D613332Ed0,
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
