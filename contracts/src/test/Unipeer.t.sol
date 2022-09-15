// SPDX-License-Identifier: GPL-2.0
pragma solidity 0.8.15;

import "forge-std/Test.sol";
import "forge-std/console2.sol";

import "oz/interfaces/IERC20.sol";
import "oz/mocks/ERC20Mock.sol";
import "kleros/IArbitrator.sol";
import "kleros/examples/SimpleCentralizedArbitrator.sol";

import "../Unipeer.sol";

contract UnipeerTest is Test {
    using stdStorage for StdStorage;

    // IERC20 Events
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event Transfer(address indexed from, address indexed to, uint256 value);

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
    address public seller = address(3);
    address public buyer = address(4);

    IERC20 Dai;
    IArbitrator arbitrator;
    Unipeer unipeer;

    uint256 constant PAYMENT_ID = 1;
    uint256 constant SELLER_BALANCE = 100_000 ether;

    function setUp() public {
        Dai = new ERC20Mock("Dai", "DAI", seller, SELLER_BALANCE);
        arbitrator = new SimpleCentralizedArbitrator();

        unipeer = new Unipeer(
            "1",
            arbitrator,
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

    // ************************************* //
    // *             Seller                * //
    // ************************************* //

    function setUpPaymentMethod() public {
        startHoax(admin);
        unipeer.addMetaEvidence("ipfs://test");
        unipeer.addPaymentMethod("PayPal", 1, Dai);
        vm.stopPrank();
    }

    function testAcceptPaymentMethod() public {
        setUpPaymentMethod();

        startHoax(seller);
        vm.expectEmit(true, true, false, true);
        emit SellerPaymentMethod(seller, 0, "seller@paypal.me");
        unipeer.acceptPaymentMethod(0, "seller@paypal.me");
        assertEq(unipeer.getPaymentMethodAddress(0, seller), "seller@paypal.me");
    }

    function testDisablePaymentMethod() public {
        setUpPaymentMethod();

        startHoax(seller);
        vm.expectEmit(true, true, false, true);
        emit SellerPaymentDisabled(seller, 0);
        unipeer.disablePaymentMethod(0);
        assertEq(unipeer.getPaymentMethodAddress(0, seller), "");
    }

    function testDepositTokens() public {
        setUpPaymentMethod();

        startHoax(seller);
        Dai.approve(address(unipeer), 1000 ether);
        vm.expectEmit(true, true, false, true);
        emit Transfer(seller, address(unipeer), 1000 ether);
        vm.expectEmit(true, true, false, true);
        emit SellerDeposit(seller, Dai, 1000 ether);
        unipeer.depositTokens(0, Dai, 1000 ether);
        assertEq(Dai.balanceOf(seller), 99_000 ether);
        assertEq(unipeer.tokenBalance(seller, Dai), 1000 ether);
    }

    function testWithdrawTokens() public {
        testDepositTokens();

        vm.expectEmit(true, true, false, true);
        emit SellerWithdraw(seller, Dai, 1000 ether);
        unipeer.withdrawTokens(Dai, 1000 ether);
        assertEq(Dai.balanceOf(seller), 100_000 ether);
        assertEq(unipeer.tokenBalance(seller, Dai), 0);
    }

    function testCannotWithdrawMoreTokensThanBalance() public {
        testDepositTokens();

        vm.expectRevert("Not enough balance to withdraw");
        unipeer.withdrawTokens(Dai, 1_001 ether);
    }

    // ************************************* //
    // *           Order (Buyer)           * //
    // ************************************* //

    function testBuyOrder() public {
        testAcceptPaymentMethod();
        vm.stopPrank();
        testDepositTokens();
        vm.stopPrank();

        startHoax(buyer);
        uint256 oldBalance = unipeer.tokenBalance(seller, Dai);

        (uint256 fees, ) = unipeer.getArbitratorData();
        unipeer.buyOrder{value: fees}(0, seller, Dai, 500);

        uint256 newBalance = unipeer.tokenBalance(seller, Dai);
        assertEq(oldBalance - newBalance, 500);
    }

    // ************************************* //
    // *           Admin only              * //
    // ************************************* //

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
