// SPDX-License-Identifier: GPL-2.0
pragma solidity 0.8.15;

import "forge-std/Test.sol";

import "oz/interfaces/IERC20.sol";
import "oz/mocks/ERC20Mock.sol";
import "kleros/IArbitrator.sol";
import "delegatable/DelegatableRelay.sol";

import "./Arbitrator.sol";
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
    event Ruling(
        IArbitrator indexed _arbitrator, uint256 indexed _disputeID, uint256 _ruling
    );
    event AppealDecision(uint256 indexed _disputeID, IArbitrable indexed _arbitrable);

    event AdminTransferred(address indexed previousAdmin, address indexed newAdmin);
    event FeeWithdrawn(uint256 amount);
    event PaymentMethodUpdate(
        uint16 indexed paymentID, string paymentName, uint256 metaEvidenceID
    );
    event SellerPaymentMethod(
        address indexed sender, uint16 paymentID, string paymentAddress, uint256 feeRate
    );
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
        uint256 feeAmount,
        uint256 sellerFeeAmount
    );
    event Paid(uint256 indexed orderID);
    event OrderComplete(uint256 indexed orderID);
    event OrderResolved(uint256 indexed orderID);
    event TimedOutByBuyer(uint256 indexed orderID);
    event TimedOutBySeller(uint256 indexed orderID);
    event HasPaidAppealFee(uint256 indexed _orderID, Unipeer.Party _party);
    event AppealContribution(
        uint256 indexed _orderID,
        Unipeer.Party _party,
        address _contributor,
        uint256 _amount
    );

    address public admin = address(1);
    address public user = address(2);
    address public seller = address(3);
    address public buyer = address(4);

    IERC20 Dai;
    Arbitrator arbitrator;
    DelegatableRelay relay;
    Unipeer unipeer;

    uint16 constant META_ID = 0;
    uint16 constant PAYMENT_ID = 0;
    uint16 constant ORDER_ID = 0;
    uint256 constant SELLER_BALANCE = 100_000 ether;
    uint256 constant SELLER_FEE = 0;

    uint256 constant APPEAL_WINDOW = 3 minutes;
    uint256 constant MULTIPLIER_DIVISOR = 10_000;
    uint256 constant SHARED_MULTI = 10_000;
    uint256 constant WIN_MULTI = 5000;
    uint256 constant LOSE_MULTI = 20_000;

    function setUp() public {
        Dai = new ERC20Mock("Dai", "DAI", seller, SELLER_BALANCE);
        arbitrator = new Arbitrator();
        relay  = new DelegatableRelay();

        unipeer = new Unipeer(
            admin,
            relay,
            arbitrator,
            bytes(""),
            10 seconds,
            10 seconds,
            SHARED_MULTI,
            WIN_MULTI,
            LOSE_MULTI,
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
        unipeer.updateSellerPaymentMethod(PAYMENT_ID, "seller@paypal.me", SELLER_FEE);
        uint256 amount = 1000 ether;
        Dai.approve(address(unipeer), amount);
        unipeer.depositTokens(PAYMENT_ID, Dai, amount);
        vm.stopPrank();
    }

    // ************************************* //
    // *             Seller                * //
    // ************************************* //

    function testUpdateSellerPaymentMethod() public {
        setUpPaymentMethod();

        startHoax(seller);
        vm.expectEmit(true, true, false, true);
        emit SellerPaymentMethod(seller, PAYMENT_ID, "seller@paypal.me", SELLER_FEE);
        unipeer.updateSellerPaymentMethod(PAYMENT_ID, "seller@paypal.me", SELLER_FEE);
        assertEq(unipeer.getPaymentMethodAddress(PAYMENT_ID, seller), "seller@paypal.me");
        assertEq(unipeer.getPaymentMethodSellerFeeRate(PAYMENT_ID, seller), SELLER_FEE);
    }

    function testDisablePaymentMethod() public {
        setUpPaymentMethod();

        startHoax(seller);
        vm.expectEmit(true, true, false, true);
        emit SellerPaymentMethod(seller, PAYMENT_ID, "", 0);
        unipeer.updateSellerPaymentMethod(PAYMENT_ID, "", 0);
        assertEq(unipeer.getPaymentMethodAddress(PAYMENT_ID, seller), "");

        vm.expectRevert("PaymentMethod: !Seller");
        unipeer.buyOrder(PAYMENT_ID, seller, Dai, 1);
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
        unipeer.withdrawTokens(Dai, 1001 ether);
    }

    // ************************************* //
    // *           Order (Buyer)           * //
    // ************************************* //

    function testBuyOrder() public {
        setUpSeller();

        startHoax(buyer);
        uint256 oldBalance = unipeer.tokenBalance(seller, Dai);
        uint256 amount = 500 ether;

        uint256 arbFees = unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());

        uint256 sellerFeeRate = unipeer.getPaymentMethodSellerFeeRate(PAYMENT_ID, seller);
        uint256 tradeFees = amount * unipeer.tradeFeeRate() / MULTIPLIER_DIVISOR;
        uint256 sellerFees = amount * sellerFeeRate / MULTIPLIER_DIVISOR;

        vm.expectEmit(true, true, false, true);
        emit BuyOrder(ORDER_ID, buyer, PAYMENT_ID, seller, Dai, amount, tradeFees, sellerFees);
        unipeer.buyOrder{value: arbFees}(PAYMENT_ID, seller, Dai, amount);

        uint256 newBalance = unipeer.tokenBalance(seller, Dai);
        assertEq(oldBalance - newBalance, amount);
    }

    function testCannotBuyOrderWithOutArbitrationFees() public {
        setUpSeller();

        startHoax(buyer);

        uint256 arbFees = unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());
        vm.expectRevert("Arbitration fees not paid");
        unipeer.buyOrder{value: arbFees - 1}(PAYMENT_ID, seller, Dai, 500 ether);
    }

    function testCannotBuyOrderMoreThanSellerBalance() public {
        setUpSeller();

        startHoax(buyer);

        uint256 arbFees = unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());
        vm.expectRevert("Not enough seller balance");
        unipeer.buyOrder{value: arbFees}(PAYMENT_ID, seller, Dai, 1001 ether);
    }

    function testCannotBuyOrderWithASellerNonAcceptedToken() public {
        setUpSeller();

        startHoax(buyer);

        uint256 arbFees = unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());
        vm.expectRevert("PaymentMethod: !Token");
        unipeer.buyOrder{value: arbFees}(
            PAYMENT_ID, seller, IERC20(address(99)), 1001 ether
        );
    }

    function testCannotBuyOrderWithASellerNonAcceptedPaymentID() public {
        setUpSeller();

        hoax(admin);
        vm.expectEmit(true, true, false, true);
        emit PaymentMethodUpdate(1, "CashApp", META_ID);
        unipeer.addPaymentMethod("CashApp", META_ID, Dai);
        startHoax(buyer);

        uint256 arbFees = unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());
        vm.expectRevert("PaymentMethod: !Seller");
        unipeer.buyOrder{value: arbFees}(1, seller, Dai, 1001 ether);
    }

    function testSellerFeeRate() public {
        setUpSeller();
        uint256 arbFees = unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());

        uint256 amount = 500 ether;
        uint256 sellerRate = SELLER_FEE + 1000;
        uint256 sellerFee = amount * sellerRate / MULTIPLIER_DIVISOR;

        hoax(buyer);
        unipeer.buyOrder{value: arbFees}(PAYMENT_ID, seller, Dai, amount);
        (, uint256 tradeAmount1) = unipeer.getOrderFeeAmount(ORDER_ID);

        hoax(seller);
        unipeer.updateSellerPaymentMethod(PAYMENT_ID, "seller@paypal.me", sellerRate);

        hoax(buyer);
        unipeer.buyOrder{value: arbFees}(PAYMENT_ID, seller, Dai, amount);
        (uint256 fees2, uint256 tradeAmount2) = unipeer.getOrderFeeAmount(ORDER_ID + 1);

        assertEq(amount - sellerFee, tradeAmount2 + fees2);
        assertEq(tradeAmount1 - tradeAmount2, sellerFee);
    }

    function testFailSellerFeeRateMoreThan100Percent() public {
        setUpSeller();

        uint256 amount = 500 ether;
        uint256 sellerRate = SELLER_FEE + MULTIPLIER_DIVISOR;
        uint256 feeRate = unipeer.tradeFeeRate() + sellerRate;
        uint256 tradeFees = amount * feeRate / MULTIPLIER_DIVISOR;
        uint256 arbFees = unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());

        hoax(seller);
        unipeer.updateSellerPaymentMethod(PAYMENT_ID, "seller@paypal.me", sellerRate);

        hoax(buyer);
        unipeer.buyOrder{value: arbFees}(PAYMENT_ID, seller, Dai, amount);

        (uint256 fees, uint256 tradeAmount) = unipeer.getOrderFeeAmount(ORDER_ID);
        assertEq(fees, tradeFees);
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
        vm.expectRevert("Payment confirmation timeout");
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
        vm.expectRevert("OrderStatus: !Created");
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
        vm.expectRevert("Order completed by timeout");
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

        uint256 arbFees = unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());

        startHoax(seller);
        vm.expectEmit(true, true, false, true);
        emit Dispute(unipeer.arbitrator(), ORDER_ID, META_ID, ORDER_ID);
        unipeer.disputeOrder{value: arbFees}(ORDER_ID);
    }

    function testCannotDiputeOrderWithoutArbFees() public {
        testConfirmPaid();
        vm.stopPrank();

        startHoax(seller);
        vm.expectRevert("Arbitration fees not paid");
        unipeer.disputeOrder(ORDER_ID);
    }

    function testCannotDisputeOrderAfterTimeout() public {
        testConfirmPaid();
        vm.stopPrank();

        startHoax(seller);
        skip(unipeer.sellerTimeout() + 1);
        vm.expectRevert("Order completed by timeout");
        unipeer.disputeOrder(ORDER_ID);
    }

    function testCannotDisputeNonPaidOrder() public {
        testBuyOrder();
        vm.stopPrank();

        startHoax(seller);
        vm.expectRevert("OrderStatus: !Paid");
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

    function testRulingBuyer() public {
        testDisputeOrder();
        vm.stopPrank();

        (, uint256 tradeAmount) = unipeer.getOrderFeeAmount(ORDER_ID);
        uint256 arbFees = arbitrator.arbitrationCost("");
        uint256 oldBalanceBuyer = address(buyer).balance;
        uint256 oldBalanceSeller = address(seller).balance;

        arbitrator.giveRuling(ORDER_ID, 1);
        skip(APPEAL_WINDOW + 1);
        vm.expectEmit(true, true, false, true);
        emit Transfer(address(unipeer), buyer, tradeAmount);
        vm.expectEmit(true, true, false, true);
        emit OrderResolved(ORDER_ID);
        vm.expectEmit(true, true, false, true);
        emit Ruling(arbitrator, ORDER_ID, 1);
        arbitrator.executeRuling(ORDER_ID);

        uint256 newBalanceBuyer = address(buyer).balance;
        uint256 newBalanceSeller = address(seller).balance;
        assertEq(newBalanceBuyer - oldBalanceBuyer, arbFees);
        assertEq(newBalanceSeller - oldBalanceSeller, 0);
    }

    function testRulingSeller() public {
        testDisputeOrder();
        vm.stopPrank();

        (uint256 fee, uint256 tradeAmount) = unipeer.getOrderFeeAmount(ORDER_ID);
        uint256 arbFees = arbitrator.arbitrationCost("");
        uint256 oldBalanceBuyer = address(buyer).balance;
        uint256 oldBalanceSeller = address(seller).balance;
        uint256 oldTokenBalance = unipeer.tokenBalance(seller, Dai);

        arbitrator.giveRuling(ORDER_ID, 2);
        skip(APPEAL_WINDOW + 1);
        vm.expectEmit(true, true, false, true);
        emit OrderResolved(ORDER_ID);
        vm.expectEmit(true, true, false, true);
        emit Ruling(arbitrator, ORDER_ID, 2);
        arbitrator.executeRuling(ORDER_ID);

        uint256 newBalanceBuyer = address(buyer).balance;
        uint256 newBalanceSeller = address(seller).balance;
        uint256 newTokenBalance = unipeer.tokenBalance(seller, Dai);
        assertEq(newBalanceBuyer - oldBalanceBuyer, 0);
        assertEq(newBalanceSeller - oldBalanceSeller, arbFees);
        assertEq(newTokenBalance - oldTokenBalance, tradeAmount + fee);
    }

    function testRulingNone() public {
        testDisputeOrder();
        vm.stopPrank();

        (uint256 fee, uint256 tradeAmount) = unipeer.getOrderFeeAmount(ORDER_ID);
        uint256 arbFees = arbitrator.arbitrationCost("");
        uint256 oldBalanceBuyer = address(buyer).balance;
        uint256 oldBalanceSeller = address(seller).balance;
        uint256 oldTokenBalance = unipeer.tokenBalance(seller, Dai);

        arbitrator.giveRuling(ORDER_ID, 0);
        skip(APPEAL_WINDOW + 1);
        vm.expectEmit(true, true, false, true);
        emit Transfer(address(unipeer), buyer, (tradeAmount + fee) / 2);
        vm.expectEmit(true, true, false, true);
        emit OrderResolved(ORDER_ID);
        vm.expectEmit(true, true, false, true);
        emit Ruling(arbitrator, ORDER_ID, 0);
        arbitrator.executeRuling(ORDER_ID);

        uint256 newBalanceBuyer = address(buyer).balance;
        uint256 newBalanceSeller = address(seller).balance;
        uint256 newTokenBalance = unipeer.tokenBalance(seller, Dai);
        assertEq(newBalanceBuyer - oldBalanceBuyer, arbFees / 2);
        assertEq(newBalanceSeller - oldBalanceSeller, arbFees / 2);
        assertEq(newTokenBalance - oldTokenBalance, (tradeAmount + fee) / 2);
    }

    function testRuleCannotBeCalledByAnyone() public {
        testDisputeOrder();
        vm.stopPrank();

        hoax(admin);
        vm.expectRevert("Only arbitrator");
        unipeer.rule(ORDER_ID, 1);
    }

    function testFailRuleOnNonExistingDispute() public {
        testConfirmPaid();
        vm.stopPrank();

        arbitrator.giveRuling(ORDER_ID, 1);
    }

    function testFundAppealBuyer() public {
        testDisputeOrder();
        vm.stopPrank();

        uint256 appealCost = arbitrator.appealCost(ORDER_ID, "");
        uint256 totalCost = appealCost + ((appealCost * WIN_MULTI) / MULTIPLIER_DIVISOR);
        arbitrator.giveRuling(ORDER_ID, 1);

        vm.expectEmit(true, true, false, true);
        emit AppealContribution(ORDER_ID, Unipeer.Party.Buyer, buyer, totalCost);
        vm.expectEmit(true, true, false, true);
        emit HasPaidAppealFee(ORDER_ID, Unipeer.Party.Buyer);
        // Appeal as the buyer
        hoax(buyer);
        unipeer.fundAppeal{value: totalCost}(ORDER_ID, Unipeer.Party.Buyer);

        uint256[3] memory contrib = unipeer.getContributions(ORDER_ID, 0, buyer);
        assertEq(contrib[0], 0);
        assertEq(contrib[1], totalCost);
        assertEq(contrib[2], 0);

        (uint256[3] memory paidFees, ,, bool appealed) =
            unipeer.getRoundInfo(ORDER_ID, 0);
        assertEq(paidFees[1], totalCost);
        assertEq(paidFees[2], 0);
        assertEq(appealed, false);

        assertEq(unipeer.getNumberOfRounds(ORDER_ID), 1);
    }

    function testFundAppealBuyerAppealed() public {
        testFundAppealBuyer();
        vm.stopPrank();

        uint256 appealCost = arbitrator.appealCost(ORDER_ID, "");
        uint256 totalCost = appealCost + ((appealCost * LOSE_MULTI) / MULTIPLIER_DIVISOR);

        vm.expectEmit(true, true, false, true);
        emit AppealContribution(ORDER_ID, Unipeer.Party.Seller, seller, totalCost);
        vm.expectEmit(true, true, false, true);
        emit AppealDecision(ORDER_ID, unipeer);
        vm.expectEmit(true, true, false, true);
        emit HasPaidAppealFee(ORDER_ID, Unipeer.Party.Seller);
        // Appeal as the seller
        hoax(seller);
        unipeer.fundAppeal{value: totalCost}(ORDER_ID, Unipeer.Party.Seller);

        uint256[3] memory contrib = unipeer.getContributions(ORDER_ID, 0, seller);
        assertEq(contrib[0], 0);
        assertEq(contrib[1], 0);
        assertEq(contrib[2], totalCost);

        (uint256[3] memory paidFees, ,, bool appealed) =
            unipeer.getRoundInfo(ORDER_ID, 0);
        assertEq(paidFees[1], totalCost / 2);
        assertEq(paidFees[2], totalCost);
        assertEq(appealed, true);

        assertEq(unipeer.getNumberOfRounds(ORDER_ID), 2);
    }

    function testCannotActuallyAppealWithoutPaying() public {
        testDisputeOrder();
        vm.stopPrank();

        arbitrator.giveRuling(ORDER_ID, 1);
        unipeer.fundAppeal(ORDER_ID, Unipeer.Party.Buyer);

        (uint256[3] memory paidFees, ,, bool appealed) =
            unipeer.getRoundInfo(ORDER_ID, 0);
        assertEq(paidFees[1], 0);
        assertEq(appealed, false);
    }

    function testWithdrawFeeAndRewards() public {
        testRulingBuyer();

        (,, uint256 feeRewards,) = unipeer.getRoundInfo(ORDER_ID, 0);
        assertEq(feeRewards, 0);
    }

    function testWithdrawFeeAndRewardsWithAppeals() public {
        testFundAppealBuyerAppealed();

        vm.expectEmit(true, true, false, true);
        emit Ruling(arbitrator, ORDER_ID, 1);
        arbitrator.giveRuling(ORDER_ID, 1);
        skip(APPEAL_WINDOW + 1);
        arbitrator.executeRuling(ORDER_ID);

        (,, uint256 feeRewards,) = unipeer.getRoundInfo(ORDER_ID, 0);
        assertTrue(feeRewards != 0, "feeRewards is zero");

        uint256 oldBalance = address(seller).balance;
        unipeer.withdrawFeesAndRewards(payable(seller), ORDER_ID, 0);
        uint256 newBalance = address(seller).balance;
        assertEq(newBalance - oldBalance, 0);

        oldBalance = address(buyer).balance;
        unipeer.withdrawFeesAndRewards(payable(buyer), ORDER_ID, 0);
        newBalance = address(buyer).balance;
        assertEq(newBalance - oldBalance, feeRewards);
    }

    function testBatchWithdrawFeeAndRewardsWithAppeals() public {
        testFundAppealBuyerAppealed();

        vm.expectEmit(true, true, false, true);
        emit Ruling(arbitrator, ORDER_ID, 1);
        arbitrator.giveRuling(ORDER_ID, 1);
        skip(APPEAL_WINDOW + 1);
        arbitrator.executeRuling(ORDER_ID);

        (,, uint256 feeRewards,) = unipeer.getRoundInfo(ORDER_ID, 0);
        assertTrue(feeRewards != 0, "feeRewards is zero");

        uint256 oldBalance = address(buyer).balance;
        unipeer.batchRoundWithdraw(payable(buyer), ORDER_ID, 0, 0);
        uint256 newBalance = address(buyer).balance;
        assertEq(newBalance - oldBalance, feeRewards);
    }

    function testCannotWithdrawFeeAndRewardsWithoutRuling() public {
        testFundAppealBuyerAppealed();

        arbitrator.giveRuling(ORDER_ID, 1);

        vm.expectRevert("The order must be resolved.");
        unipeer.withdrawFeesAndRewards(payable(seller), ORDER_ID, 0);
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

    function testChangeAdmin() public {
        hoax(admin);
        unipeer.changeAdmin(user);
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
