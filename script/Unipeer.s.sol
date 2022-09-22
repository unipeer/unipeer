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

        IArbitrator arbitrator = IArbitrator(0xEF2689DB6A7b3AB383Fb14Ff7d9C2254C248103f);
        Unipeer unipeer = new Unipeer(
            0x3b434e0D2a6C7F53d5C556D7BAeE8942c351Cf1a,
            address(0),
            "1",
            arbitrator,
            bytes(""),
            30 minutes,
            30 minutes,
            SHARED_MULTI,
            WIN_MULTI,
            LOSE_MULTI,
            0
        );

        vm.stopBroadcast();
    }
}
