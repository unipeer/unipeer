// SPDX-License-Identifier: GPL-2.0
pragma solidity 0.8.15;

import "forge-std/Test.sol";
import "forge-std/console2.sol";

import "oz/interfaces/IERC20.sol";
import "kleros/IArbitrator.sol";

import "../Unipeer.sol";

contract UnipeerTest is Test {
    using stdStorage for StdStorage;

    event FeeWithdrawn(uint256 amount);
    event PaymentMethodUpdate(uint16 paymentID, string paymentName, uint256 metaEvidenceID);
    event SellerPaymentMethod(address sender, uint16 paymentID, string paymentAddress);
    event SellerPaymentDisabled(address sender, uint16 paymentID);
    event SellerDeposit(address sender, IERC20 token, uint256 amount);
    event SellerWithdraw(address sender, IERC20 token, uint256 amount);
    event BuyOrder(
        uint256 orderID,
        address buyer,
        uint16 paymentID,
        address seller,
        IERC20 token,
        uint256 amount,
        uint256 feeAmount
    );
    event Paid(uint256 orderID);
    event OrderComplete(uint256 orderID);
    event OrderResolved(uint256 orderID);
    event TimedOutByBuyer(uint256 orderID);
    event TimedOutBySeller(uint256 orderID);

    address public admin = address(1);
    address public user = address(2);

    IERC20 Dai = IERC20(address(99));

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

    function setUpPaymentMethod() public {
        startHoax(admin);
        unipeer.addMetaEvidence("ipfs://test");
        unipeer.addPaymentMethod("PayPal", 1, Dai);
        vm.stopPrank();
    }

    function testAcceptPaymentMethod() public {
        setUpPaymentMethod();

        vm.expectEmit(true, true, false, true);
        emit SellerPaymentMethod(address(this), 0, "seller@paypal.me");
        unipeer.acceptPaymentMethod(0, "seller@paypal.me");
        assertEq(unipeer.getPaymentMethodAddress(0, address(this)), "seller@paypal.me");
    }

    function testDisablePaymentMethod() public {
        setUpPaymentMethod();

        vm.expectEmit(true, true, false, true);
        emit SellerPaymentDisabled(address(this), 0);
        unipeer.disablePaymentMethod(0);
        assertEq(unipeer.getPaymentMethodAddress(0, address(this)), "");
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
        unipeer.addPaymentMethod("PayPal", 0, Dai);

        assertEq(unipeer.metaEvidenceUpdates(), 0);
        assertEq(unipeer.totalPaymentMethods(), 0);
        unipeer.addMetaEvidence("ipfs://test");
        unipeer.addPaymentMethod("PayPal", 1, Dai);
        assertEq(unipeer.totalPaymentMethods(), 1);

        vm.expectRevert("Invalid Meta Evidence ID");
        unipeer.addPaymentMethod("PayPal", 2, Dai);
    }

    function testWithdrawFees() public {
        startHoax(admin);

        vm.expectRevert("Amount more than accrued fees");
        unipeer.withdrawFees(10, payable(user));
        stdstore.target(address(unipeer)).sig("protocolFeesSum()").checked_write(120);
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
        unipeer.updateTokenEnabled(0, Dai, false);
    }

    function testCannotCallAdminOnlyFunctions() public {
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.changeArbitrator(IArbitrator(user), bytes("1"));
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.addMetaEvidence("ipfs://test");
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.addPaymentMethod("PayPal", 1, Dai);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.updatePaymentMetaEvidence(0, 0);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.updatePaymentName(0, "PayPal US");
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.updateTokenEnabled(0, Dai, false);
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
