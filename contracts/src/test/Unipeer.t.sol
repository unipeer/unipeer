// SPDX-License-Identifier: GPL-2.0
pragma solidity 0.8.15;

import "forge-std/Test.sol";
import "oz/interfaces/IERC20.sol";
import "kleros/IArbitrator.sol";

import '../Unipeer.sol';

contract ContractTest is Test {
    address public admin = address(1);
    address public user = address(2);

    IERC20 dai = IERC20(address(99));

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

    function testAdminOnlyFunctions() public {
        startHoax(admin);
        unipeer.changeArbitrator(IArbitrator(user), bytes("1"));
        unipeer.addMetaEvidence("ipfs://test");
        unipeer.addPaymentMethod("PayPal", 0, dai);
        unipeer.updatePaymentMetaEvidence(0, 0);
        unipeer.updatePaymentName(0, "PayPal US");
        unipeer.updateTokenEnabled(0, dai, false);
        unipeer.changeConfirmTimeout(100);
        unipeer.changeOrderTimeout(100);
        unipeer.changeFees(100);
        // TODO: test independently
        unipeer.withdrawFees(0, payable(user));
        vm.stopPrank();

        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.changeArbitrator(IArbitrator(user), bytes("1"));
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.addMetaEvidence("ipfs://test");
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.addPaymentMethod("PayPal", 0, dai);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.updatePaymentMetaEvidence(0, 0);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.updatePaymentName(0, "PayPal US");
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.updateTokenEnabled(0, dai, false);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.changeConfirmTimeout(100);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.changeOrderTimeout(100);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.changeFees(100);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.withdrawFees(0, payable(user));
    }
}
