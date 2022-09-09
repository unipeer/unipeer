// SPDX-License-Identifier: GPL-2.0
pragma solidity 0.8.15;

import "forge-std/Test.sol";
import "kleros/IArbitrator.sol";

import '../Unipeer.sol';

contract ContractTest is Test {
    address public admin = address(1);
    address public user = address(2);

    Unipeer unipeer;
    function setUp() public {
        unipeer = new Unipeer(
            "1",
            IArbitrator(address(0)),
            bytes(""),
            10 seconds,
            10 seconds,
            10_000,
            5_000,
            20_000,
            5
        );

        unipeer.transferOwnership(admin);
    }

    function testExample() public {
        assertTrue(true);
    }

    function testAdminOnlyFunctions() public {
        // startHoax(admin);
        // unipeer.changeArbitrator(IArbitrator(user), bytes("1"));
        // vm.stopPrank();

        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.changeArbitrator(IArbitrator(user), bytes("1"));

/*
        vm.expectRevert("only the owner may call this function.");
        priceModel.setCurrentRate(1);
        vm.expectRevert("only the owner may call this function.");
        priceModel.setNegThreshold(1);
        vm.expectRevert("only the owner may call this function.");
        priceModel.setPosThreshold(1);
        vm.expectRevert("only the owner may call this function.");
        priceModel.setMaxRate(1);
        vm.expectRevert("only the owner may call this function.");
        priceModel.setMinRate(1);
        vm.expectRevert("only the owner may call this function.");
        priceModel.setRateStep(1);
        vm.expectRevert("only the owner may call this function.");
        priceModel.setUpdateRateInterval(1);
        vm.expectRevert("only the owner may call this function.");
        priceModel.setPriceFeed(IFeed(address(99)));
        vm.expectRevert("only the owner may call this function.");
        priceModel.setCToken(CTokenInterface(address(99)));
        */
    }
}
