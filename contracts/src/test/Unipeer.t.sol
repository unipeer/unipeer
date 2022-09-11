// SPDX-License-Identifier: GPL-2.0
pragma solidity 0.8.15;

import "forge-std/Test.sol";
import "oz/interfaces/IERC20.sol";
import "kleros/IArbitrator.sol";

import "../Unipeer.sol";

contract ContractTest is Test {
    using stdStorage for StdStorage;

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

    function testAddMetaEvidence() public {
        assertEq(unipeer.metaEvidenceUpdates(), 0);

        startHoax(admin);
        unipeer.addMetaEvidence("ipfs://test");
        assertEq(unipeer.metaEvidenceUpdates(), 1);

        unipeer.addMetaEvidence("ipfs://test");
        assertEq(unipeer.metaEvidenceUpdates(), 2);
    }

    function testAddPaymentMethod() public {
        startHoax(admin);
        vm.expectRevert("Invalid Meta Evidence ID");
        unipeer.addPaymentMethod("PayPal", 0, dai);

        assertEq(unipeer.metaEvidenceUpdates(), 0);
        assertEq(unipeer.totalPaymentMethods(), 0);
        unipeer.addMetaEvidence("ipfs://test");
        unipeer.addPaymentMethod("PayPal", 1, dai);
        assertEq(unipeer.totalPaymentMethods(), 1);

        vm.expectRevert("Invalid Meta Evidence ID");
        unipeer.addPaymentMethod("PayPal", 2, dai);
    }

    function testWithdrawFees() public {
        startHoax(admin);

        vm.expectRevert("Amount more than accrued fees");
        unipeer.withdrawFees(10, payable(user));
        stdstore
            .target(address(unipeer))
            .sig("protocolFeesSum()")
            .checked_write(120);
        assertEq(unipeer.protocolFeesSum(), 120);
        unipeer.withdrawFees(120, payable(admin));

        vm.expectRevert("Amount more than accrued fees");
        unipeer.withdrawFees(10, payable(user));
    }

    function testChangeArbitrator() public {
        hoax(admin);
        unipeer.changeArbitrator(IArbitrator(user), bytes("1"));
    }

    function testChangeConfirmTimeout() public {
        hoax(admin);
        unipeer.changeConfirmTimeout(100);
    }

    function testChangeOrderTimeout() public {
        hoax(admin);
        unipeer.changeOrderTimeout(100);
    }

    function testChangeFees() public {
        hoax(admin);
        unipeer.changeFees(100);
    }

    function testUpdatePaymentMetaEvidence() public {
        testAddPaymentMethod();
        unipeer.updatePaymentMetaEvidence(0, 1);
    }

    function testUpdatePaymentName() public {
        testAddPaymentMethod();
        unipeer.updatePaymentName(0, "PayPal US");
    }

    function testUpdateTokenEnabled() public {
        testAddPaymentMethod();
        unipeer.updateTokenEnabled(0, dai, false);
    }

    function testAdminOnlyFunctionsRevert() public {
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.changeArbitrator(IArbitrator(user), bytes("1"));
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.addMetaEvidence("ipfs://test");
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.addPaymentMethod("PayPal", 1, dai);
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
