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

    // IEvidence Events
    event Dispute(
        IArbitrator indexed _arbitrator,
        uint256 indexed _disputeID,
        uint256 _metaEvidenceID,
        uint256 _evidenceGroupID
    );

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

    uint16 constant PAYMENT_ID = 0;
    uint16 constant ORDER_ID = 0;
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

    function setUpPaymentMethod() public {
        startHoax(admin);
        unipeer.addMetaEvidence("ipfs://test");
        unipeer.addPaymentMethod("PayPal", 1, Dai);
        vm.stopPrank();
    }

    function setUpSeller() public {
        setUpPaymentMethod();
        startHoax(seller);
        unipeer.acceptPaymentMethod(PAYMENT_ID, "seller@paypal.me");
        uint256 amount = 1000 ether;
        Dai.approve(address(unipeer), amount);
        unipeer.depositTokens(PAYMENT_ID, Dai, amount);
        vm.stopPrank();
    }

    // ************************************* //
    // *             Seller                * //
    // ************************************* //

    function testAcceptPaymentMethod() public {
        setUpPaymentMethod();

        startHoax(seller);
        vm.expectEmit(true, true, false, true);
        emit SellerPaymentMethod(seller, PAYMENT_ID, "seller@paypal.me");
        unipeer.acceptPaymentMethod(PAYMENT_ID, "seller@paypal.me");
        assertEq(unipeer.getPaymentMethodAddress(PAYMENT_ID, seller), "seller@paypal.me");
    }

    function testDisablePaymentMethod() public {
        setUpPaymentMethod();

        startHoax(seller);
        vm.expectEmit(true, true, false, true);
        emit SellerPaymentDisabled(seller, PAYMENT_ID);
        unipeer.disablePaymentMethod(PAYMENT_ID);
        assertEq(unipeer.getPaymentMethodAddress(PAYMENT_ID, seller), "");
    }

    function testDepositTokens() public {
        setUpPaymentMethod();

        startHoax(seller);
        uint256 amount = 1000 ether;
        Dai.approve(address(unipeer), amount);
        vm.expectEmit(true, true, false, true);
        emit Transfer(seller, address(unipeer), amount);
        vm.expectEmit(true, true, false, true);
        emit SellerDeposit(seller, Dai, amount);
        unipeer.depositTokens(PAYMENT_ID, Dai, amount);
        assertEq(Dai.balanceOf(seller), SELLER_BALANCE - amount);
        assertEq(unipeer.tokenBalance(seller, Dai), amount);
    }

    function testWithdrawTokens() public {
        testDepositTokens();

        vm.expectEmit(true, true, false, true);
        emit SellerWithdraw(seller, Dai, 1000 ether);
        unipeer.withdrawTokens(Dai, 1000 ether);
        assertEq(Dai.balanceOf(seller), SELLER_BALANCE);
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
        setUpSeller();

        startHoax(buyer);
        uint256 oldBalance = unipeer.tokenBalance(seller, Dai);
        uint256 amount = 500 ether;

        (uint256 arbFees, ) = unipeer.getArbitratorData();
        (uint256 tradeFees, ) = unipeer.calculateFees(amount);

        vm.expectEmit(true, true, false, true);
        emit BuyOrder(0, buyer, PAYMENT_ID, seller, Dai, amount, tradeFees);
        unipeer.buyOrder{value: arbFees}(PAYMENT_ID, seller, Dai, amount);

        uint256 newBalance = unipeer.tokenBalance(seller, Dai);
        assertEq(oldBalance - newBalance, amount);
    }

    function testCannotBuyOrderWithOutArbitrationFees() public {
        setUpSeller();

        startHoax(buyer);

        (uint256 arbFees, ) = unipeer.getArbitratorData();
        vm.expectRevert("Arbitration fees need to be paid");
        unipeer.buyOrder{value: arbFees - 1}(PAYMENT_ID, seller, Dai, 500 ether);
    }

    function testCannotBuyOrderMoreThanSellerBalance() public {
        setUpSeller();

        startHoax(buyer);

        (uint256 arbFees, ) = unipeer.getArbitratorData();
        vm.expectRevert("Not enough seller balance");
        unipeer.buyOrder{value: arbFees}(PAYMENT_ID, seller, Dai, 1001 ether);
    }

    function testCannotBuyOrderWithASellerNonAcceptedToken() public {
        setUpSeller();

        startHoax(buyer);

        (uint256 arbFees, ) = unipeer.getArbitratorData();
        vm.expectRevert("Token is not enabled for this payment method");
        unipeer.buyOrder{value: arbFees}(PAYMENT_ID, seller, IERC20(address(99)), 1001 ether);
    }

    function testCannotBuyOrderWithASellerNonAcceptedPaymentID() public {
        setUpSeller();

        hoax(admin);
        vm.expectEmit(true, true, false, true);
        emit PaymentMethodUpdate(1, "CashApp", 1);
        unipeer.addPaymentMethod("CashApp", 1, Dai);
        startHoax(buyer);

        (uint256 arbFees, ) = unipeer.getArbitratorData();
        vm.expectRevert("Seller doesn't accept this payment method");
        unipeer.buyOrder{value: arbFees}(1, seller, Dai, 1001 ether);
    }

    function testConfirmPaid() public {
        testBuyOrder();

        unipeer.confirmPaid(ORDER_ID);
    }

    function testCannotConfirmPaidByNonBuyer() public {
        testBuyOrder();
        vm.stopPrank();

        vm.expectRevert("Only Buyer");
        unipeer.confirmPaid(ORDER_ID);
    }

    function testCannotConfirmPaidAfterTimeout() public {
        testBuyOrder();

        skip(unipeer.buyerTimeout() + 1);
        vm.expectRevert("Payment confirmation period is over");
        unipeer.confirmPaid(ORDER_ID);
    }

    function testTimeoutByBuyer() public {
        testBuyOrder();

        skip(unipeer.buyerTimeout() + 1);
        vm.expectEmit(true, true, false, true);
        emit TimedOutByBuyer(ORDER_ID);
        unipeer.timeoutByBuyer(ORDER_ID);
    }

    function testCannotConfirmPaidAfterCancel() public {
        testTimeoutByBuyer();

        skip(unipeer.buyerTimeout() + 1);
        vm.expectRevert("Order already cancelled, completed or disputed");
        unipeer.confirmPaid(ORDER_ID);
    }

    // ************************************* //
    // *           Order (Seller)          * //
    // ************************************* //

    function testCompleteOrder() public {
        testConfirmPaid();
        vm.stopPrank();

        uint256 amount = unipeer.getOrderAmountAfterFees(ORDER_ID);
        uint256 fees = unipeer.getFeeAmount(ORDER_ID);

        startHoax(seller);

        vm.expectEmit(true, true, false, true);
        emit Transfer(address(unipeer), buyer, amount);
        vm.expectEmit(true, true, false, true);
        emit OrderComplete(ORDER_ID);
        unipeer.completeOrder(ORDER_ID);

        assertEq(unipeer.protocolFeesSum(), fees);
    }

    function testCompleteOrderWithOutConfirm() public {
        testBuyOrder();
        vm.stopPrank();

        startHoax(seller);
        vm.expectEmit(true, true, false, true);
        emit OrderComplete(ORDER_ID);
        unipeer.completeOrder(ORDER_ID);
    }

    function testCompleteOrderWithOutConfirmAfterTimeout() public {
        testBuyOrder();
        vm.stopPrank();

        startHoax(seller);
        skip(unipeer.sellerTimeout() + 1);
        vm.expectEmit(true, true, false, true);
        emit OrderComplete(ORDER_ID);
        unipeer.completeOrder(ORDER_ID);
    }

    function testCannotCompleteOrderAfterTimeout() public {
        testConfirmPaid();
        vm.stopPrank();

        startHoax(seller);
        skip(unipeer.sellerTimeout() + 1);
        vm.expectRevert("Order already completed by timeout");
        unipeer.completeOrder(ORDER_ID);
    }

    function testCannotCompleteOrderByNonSeller() public {
        testConfirmPaid();
        vm.stopPrank();

        vm.expectRevert("Only Seller");
        unipeer.completeOrder(ORDER_ID);
    }

    function testDisputeOrder() public {
        testConfirmPaid();
        vm.stopPrank();

        (uint256 arbFees, ) = unipeer.getArbitratorData();
        IArbitrator arb = unipeer.getArbitrator();

        startHoax(seller);
        vm.expectEmit(true, true, false, true);
        emit Dispute(arb, ORDER_ID, ORDER_ID, ORDER_ID);
        unipeer.disputeOrder{value: arbFees}(ORDER_ID);
    }

    function testCannotDiputeOrderWithoutArbFees() public {
        testConfirmPaid();
        vm.stopPrank();

        startHoax(seller);
        vm.expectRevert("Arbitration fees need to be paid");
        unipeer.disputeOrder(ORDER_ID);
    }

    function testCannotDisputeOrderAfterTimeout() public {
        testConfirmPaid();
        vm.stopPrank();

        startHoax(seller);
        skip(unipeer.sellerTimeout() + 1);
        vm.expectRevert("Order already completed by timeout");
        unipeer.disputeOrder(ORDER_ID);
    }

    function testCannotDisputeNonPaidOrder() public {
        testBuyOrder();
        vm.stopPrank();

        startHoax(seller);
        vm.expectRevert("Cannot dispute a not yet paid order");
        unipeer.disputeOrder(ORDER_ID);
    }

    function testCannotDisputeOrderByNonSeller() public {
        testBuyOrder();
        vm.stopPrank();

        vm.expectRevert("Only Seller");
        unipeer.disputeOrder(ORDER_ID);
    }

    function testTimeoutBySeller() public {
        testConfirmPaid();
        vm.stopPrank();

        uint256 amount = unipeer.getOrderAmountAfterFees(ORDER_ID);
        uint256 fees = unipeer.getFeeAmount(ORDER_ID);

        skip(unipeer.sellerTimeout() + 1);
        vm.expectEmit(true, true, false, true);
        emit Transfer(address(unipeer), buyer, amount);
        vm.expectEmit(true, true, false, true);
        emit TimedOutBySeller(ORDER_ID);
        vm.expectEmit(true, true, false, true);
        emit OrderComplete(ORDER_ID);
        unipeer.timeoutBySeller(ORDER_ID);

        assertEq(unipeer.protocolFeesSum(), fees);
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
        vm.expectEmit(true, true, false, true);
        emit PaymentMethodUpdate(PAYMENT_ID, "PayPal", 1);
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

    function testChangeBuyerTimeout() public {
        hoax(admin);
        unipeer.changeBuyerTimeout(100);
    }

    function testChangeSellerTimeout() public {
        hoax(admin);
        unipeer.changeSellerTimeout(100);
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
        unipeer.changeBuyerTimeout(100);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.changeSellerTimeout(100);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.changeFees(100);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.withdrawFees(0, payable(user));
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.transferOwnership(user);
        vm.expectRevert("Ownable: caller is not the owner");
        unipeer.renounceOwnership();
    }
}
