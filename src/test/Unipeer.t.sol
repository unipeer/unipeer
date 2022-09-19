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
    event Evidence(
        IArbitrator indexed _arbitrator,
        uint256 indexed _evidenceGroupID,
        address indexed _party,
        string _evidence
    );
    event Dispute(
        IArbitrator indexed _arbitrator,
        uint256 indexed _disputeID,
        uint256 _metaEvidenceID,
        uint256 _evidenceGroupID
    );
    event MetaEvidence(uint256 indexed _metaEvidenceID, string _evidence);

    // IArbitrable
    event Ruling(IArbitrator indexed _arbitrator, uint256 indexed _disputeID, uint256 _ruling);

    event AdminTransferred(address indexed previousAdmin, address indexed newAdmin);
    event FeeWithdrawn(uint256 amount);
    event PaymentMethodUpdate(uint16 indexed paymentID, string paymentName, uint256 metaEvidenceID);
    event SellerPaymentMethod(address indexed sender, uint16 paymentID, string paymentAddress, uint256 feeRate);
    event SellerPaymentDisabled(address indexed sender, uint16 paymentID);
    event SellerDeposit(address indexed sender, IERC20 token, uint256 amount);
    event SellerWithdraw(address indexed sender, IERC20 token, uint256 amount);
    event BuyOrder(
        uint256 indexed orderID,
        address buyer,
        uint16 paymentID,
        address seller,
        IERC20 token,
        uint256 amount,
        uint256 feeAmount
    );
    event Paid(uint256 indexed orderID);
    event OrderComplete(uint256 indexed orderID);
    event OrderResolved(uint256 indexed orderID);
    event TimedOutByBuyer(uint256 indexed orderID);
    event TimedOutBySeller(uint256 indexed orderID);

    address public admin = address(1);
    address public user = address(2);
    address public seller = address(3);
    address public buyer = address(4);

    IERC20 Dai;
    SimpleCentralizedArbitrator arbitrator;
    Unipeer unipeer;

    uint16 constant META_ID = 0;
    uint16 constant PAYMENT_ID = 0;
    uint16 constant ORDER_ID = 0;
    uint256 constant SELLER_BALANCE = 100_000 ether;
    uint256 constant SELLER_FEE = 0;

    function setUp() public {
        Dai = new ERC20Mock("Dai", "DAI", seller, SELLER_BALANCE);
        arbitrator = new SimpleCentralizedArbitrator();

        unipeer = new Unipeer(
            admin,
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
    }

    function setUpPaymentMethod() public {
        startHoax(admin);
        vm.expectEmit(true, true, false, true);
        emit MetaEvidence(META_ID, "ipfs://test");
        unipeer.addMetaEvidence("ipfs://test");
        unipeer.addPaymentMethod("PayPal", META_ID, Dai);
        vm.stopPrank();
    }

    function setUpSeller() public {
        setUpPaymentMethod();
        startHoax(seller);
        unipeer.acceptPaymentMethod(PAYMENT_ID, "seller@paypal.me", SELLER_FEE);
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
        emit SellerPaymentMethod(seller, PAYMENT_ID, "seller@paypal.me", SELLER_FEE);
        unipeer.acceptPaymentMethod(PAYMENT_ID, "seller@paypal.me", SELLER_FEE);
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

        uint256 sellerFeeRate = unipeer.getPaymentMethodSellerFeeRate(PAYMENT_ID, seller);
        uint256 feeRate = unipeer.tradeFeeRate() + sellerFeeRate;
        (uint256 tradeFees, ) = unipeer.calculateFees(amount, feeRate);

        vm.expectEmit(true, true, false, true);
        emit BuyOrder(ORDER_ID, buyer, PAYMENT_ID, seller, Dai, amount, tradeFees);
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
        emit PaymentMethodUpdate(1, "CashApp", META_ID);
        unipeer.addPaymentMethod("CashApp", META_ID, Dai);
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

        (uint256 fees, uint256 amount) = unipeer.getOrderFeeAmount(ORDER_ID);

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
        emit Dispute(arb, ORDER_ID, META_ID, ORDER_ID);
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

        (uint256 fees, uint256 amount) = unipeer.getOrderFeeAmount(ORDER_ID);

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
    // *             Arbitrator            * //
    // ************************************* //

    function testSubmitEvidence() public {
        testDisputeOrder();
        vm.stopPrank();

        vm.expectEmit(true, true, false, true);
        emit Evidence(arbitrator, ORDER_ID, address(this), "ipfs://test");
        unipeer.submitEvidence(ORDER_ID, "ipfs://test");
    }

    function testRule() public {
        testDisputeOrder();
        vm.stopPrank();

        vm.expectEmit(true, true, false, true);
        emit Ruling(arbitrator, ORDER_ID, 1);
        arbitrator.rule(ORDER_ID, 1);
    }

    function testCannotBeCalledByAnyoneRule() public {
        testDisputeOrder();
        vm.stopPrank();

        hoax(admin);
        vm.expectRevert("Only arbitrator");
        unipeer.rule(ORDER_ID, 1);
    }

    function testFailRuleOnNonExistingDispute() public {
        testConfirmPaid();
        vm.stopPrank();

        arbitrator.rule(ORDER_ID, 1);
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

        assertEq(unipeer.metaEvidenceUpdates(), 0);
        assertEq(unipeer.totalPaymentMethods(), 0);
        unipeer.addMetaEvidence("ipfs://test");
        vm.expectEmit(true, true, false, true);
        emit PaymentMethodUpdate(PAYMENT_ID, "PayPal", META_ID);
        unipeer.addPaymentMethod("PayPal", META_ID, Dai);
        assertEq(unipeer.totalPaymentMethods(), 1);

        vm.expectRevert("Invalid Meta Evidence ID");
        unipeer.addPaymentMethod("PayPal", META_ID + 1, Dai);
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
        unipeer.updatePaymentMetaEvidence(PAYMENT_ID, META_ID + 1);
    }

    function testUpdatePaymentName() public {
        testAddPaymentMethod();
        unipeer.updatePaymentName(PAYMENT_ID, "PayPal US");
    }

    function testUpdateTokenEnabled() public {
        testAddPaymentMethod();
        unipeer.updateTokenEnabled(PAYMENT_ID, Dai, false);
    }

    function testCannotCallAdminOnlyFunctions() public {
        vm.expectRevert("Only Admin");
        unipeer.changeAdmin(user);
        vm.expectRevert("Only Admin");
        unipeer.changeArbitrator(IArbitrator(user), bytes("1"));
        vm.expectRevert("Only Admin");
        unipeer.addMetaEvidence("ipfs://test");
        vm.expectRevert("Only Admin");
        unipeer.addPaymentMethod("PayPal", META_ID, Dai);
        vm.expectRevert("Only Admin");
        unipeer.updatePaymentMetaEvidence(PAYMENT_ID, 0);
        vm.expectRevert("Only Admin");
        unipeer.updatePaymentName(PAYMENT_ID, "PayPal US");
        vm.expectRevert("Only Admin");
        unipeer.updateTokenEnabled(PAYMENT_ID, Dai, false);
        vm.expectRevert("Only Admin");
        unipeer.changeBuyerTimeout(100);
        vm.expectRevert("Only Admin");
        unipeer.changeSellerTimeout(100);
        vm.expectRevert("Only Admin");
        unipeer.changeFees(100);
        vm.expectRevert("Only Admin");
        unipeer.withdrawFees(0, payable(user));
    }
}
