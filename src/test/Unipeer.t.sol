// SPDX-License-Identifier: GPL-2.0
pragma solidity 0.8.17;

import "forge-std/Test.sol";

import "oz/interfaces/IERC20.sol";
import "oz/mocks/ERC20Mock.sol";
import "kleros/IArbitrator.sol";
// import "delegatable/DelegatableRelay.sol";

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
    event OrderBuy(
        uint256 indexed orderID,
        address indexed buyer,
        address indexed seller,
        uint16 paymentID,
        string paymentAddress,
        IERC20 token,
        uint256 amount,
        uint256 feeAmount,
        uint256 sellerFeeAmount
    );
    event OrderPaid(uint256 indexed orderID);
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

    ERC20Mock Dai;
    Arbitrator arbitrator;
    // DelegatableRelay relay = new DelegatableRelay();
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

        unipeer = new Unipeer({
            _admin: admin,
            _relay: address(99),
            _version: "1",
            _arbitrator: arbitrator,
            _arbitratorExtraData: bytes(""),
            _buyerTimeout: 10 seconds,
            _sellerTimeout: 10 seconds,
            _sharedStakeMultiplier: SHARED_MULTI,
            _winnerStakeMultiplier: WIN_MULTI,
            _loserStakeMultiplier: LOSE_MULTI,
            _tradeFeeRate: 5
        });
    }

    function setUpPaymentMethod() public {
        startHoax(admin);
        vm.expectEmit(true, true, false, true);
        emit MetaEvidence(META_ID, "ipfs://test");
        unipeer.addMetaEvidence("ipfs://test");
        unipeer.addPaymentMethod("PayPal", META_ID, Dai);
        vm.stopPrank();
    }

    function setUpSeller(uint96 amount) public {
        setUpPaymentMethod();
        startHoax(seller);
        unipeer.updateSellerPaymentMethod(PAYMENT_ID, "seller@paypal.me", SELLER_FEE);
        Dai.mint(seller, amount);
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
        assertEq(unipeer.getPaymentMethodToken(PAYMENT_ID, Dai), true);
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

    function testDepositTokens(uint96 amount) public {
        setUpPaymentMethod();

        startHoax(seller);
        Dai.mint(seller, amount);
        Dai.approve(address(unipeer), amount);
        assertEq(Dai.balanceOf(seller), SELLER_BALANCE + amount);

        vm.expectEmit(true, true, false, true);
        emit Transfer(seller, address(unipeer), amount);
        vm.expectEmit(true, true, false, true);
        emit SellerDeposit(seller, Dai, amount);
        unipeer.depositTokens(PAYMENT_ID, Dai, amount);

        assertEq(Dai.balanceOf(seller), SELLER_BALANCE);
        assertEq(unipeer.tokenBalance(seller, Dai), amount);
    }

    function testWithdrawTokens(uint96 amount) public {
        testDepositTokens(amount);

        vm.expectEmit(true, true, false, true);
        emit SellerWithdraw(seller, Dai, amount);
        unipeer.withdrawTokens(Dai, amount);
        assertEq(Dai.balanceOf(seller), SELLER_BALANCE + amount);
        assertEq(unipeer.tokenBalance(seller, Dai), 0);
    }

    function testCannotWithdrawMoreTokensThanBalance(uint96 amount) public {
        testDepositTokens(amount);

        vm.expectRevert("Not enough balance to withdraw");
        unipeer.withdrawTokens(Dai, uint256(amount) + 1);
    }

    // ************************************* //
    // *           Order (Buyer)           * //
    // ************************************* //

    function testBuyOrder(uint96 amount) public {
        setUpSeller(amount);

        startHoax(buyer);
        uint256 oldBalance = unipeer.tokenBalance(seller, Dai);

        uint256 arbFees =
            unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());

        uint256 sellerFeeRate = unipeer.getPaymentMethodSellerFeeRate(PAYMENT_ID, seller);
        uint256 tradeFees = amount * unipeer.tradeFeeRate() / MULTIPLIER_DIVISOR;
        uint256 sellerFees = amount * sellerFeeRate / MULTIPLIER_DIVISOR;

        vm.expectEmit(true, true, false, true);
        emit OrderBuy(
            ORDER_ID, buyer, seller, PAYMENT_ID, "seller@paypal.me", Dai, amount, tradeFees, sellerFees
            );
        unipeer.buyOrder{value: arbFees}({
            _paymentID: PAYMENT_ID,
            _seller: seller,
            _token: Dai,
            _amount: amount
        });

        uint256 newBalance = unipeer.tokenBalance(seller, Dai);
        assertEq(oldBalance - newBalance, amount);
        assertEq(unipeer.getCountOrders(), 1);
    }

    function testCannotBuyOrderWithOutArbitrationFees(uint96 amount) public {
        setUpSeller(amount);

        startHoax(buyer);

        uint256 arbFees =
            unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());
        vm.expectRevert("Arbitration fees not paid");
        unipeer.buyOrder{value: arbFees - 1}({
            _paymentID: PAYMENT_ID,
            _seller: seller,
            _token: Dai,
            _amount: amount
        });
    }

    function testCannotBuyOrderMoreThanSellerBalance(uint96 amount) public {
        setUpSeller(amount);

        startHoax(buyer);

        uint256 arbFees =
            unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());
        vm.expectRevert("Not enough seller balance");
        unipeer.buyOrder{value: arbFees}({
            _paymentID: PAYMENT_ID,
            _seller: seller,
            _token: Dai,
            _amount: uint256(amount) + 1
        });
    }

    function testCannotBuyOrderWithASellerNonAcceptedToken(uint96 amount) public {
        setUpSeller(amount);

        startHoax(buyer);

        uint256 arbFees =
            unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());
        vm.expectRevert("PaymentMethod: !Token");
        unipeer.buyOrder{value: arbFees}({
            _paymentID: PAYMENT_ID,
            _seller: seller,
            _token: IERC20(address(99)),
            _amount: 1001 ether
        });
    }

    function testCannotBuyOrderWithASellerNonAcceptedPaymentID(uint96 amount) public {
        setUpSeller(amount);

        hoax(admin);
        vm.expectEmit(true, true, false, true);
        emit PaymentMethodUpdate(1, "CashApp", META_ID);
        unipeer.addPaymentMethod("CashApp", META_ID, Dai);
        startHoax(buyer);

        uint256 arbFees =
            unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());
        vm.expectRevert("PaymentMethod: !Seller");
        unipeer.buyOrder{value: arbFees}({
            _paymentID: 1,
            _seller: seller,
            _token: Dai,
            _amount: 1001 ether
        });
    }

    function testSellerFeeRate(uint96 amount) public {
        setUpSeller(amount);
        uint256 arbFees =
            unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());

        uint256 sellerRate = SELLER_FEE + 1000;
        uint256 sellerFee = amount * sellerRate / MULTIPLIER_DIVISOR;

        hoax(buyer);
        unipeer.buyOrder{value: arbFees}({
            _paymentID: PAYMENT_ID,
            _seller: seller,
            _token: Dai,
            _amount: amount
        });
        (, uint256 tradeAmount1) = unipeer.getOrderFeeAmount(ORDER_ID);

        startHoax(seller);
        unipeer.updateSellerPaymentMethod(PAYMENT_ID, "seller@paypal.me", sellerRate);
        Dai.mint(seller, amount);
        Dai.approve(address(unipeer), amount);
        unipeer.depositTokens(PAYMENT_ID, Dai, amount);
        vm.stopPrank();

        hoax(buyer);
        unipeer.buyOrder{value: arbFees}({
            _paymentID: PAYMENT_ID,
            _seller: seller,
            _token: Dai,
            _amount: amount
        });
        (uint256 fees2, uint256 tradeAmount2) = unipeer.getOrderFeeAmount(ORDER_ID + 1);

        assertEq(amount - sellerFee, tradeAmount2 + fees2);
        assertEq(tradeAmount1 - tradeAmount2, sellerFee);
    }

    function testCannotHaveSellerFeeRateMoreThan100Percent() public {
        uint96 amount = 500 ether;
        setUpSeller(amount);

        uint256 sellerRate = SELLER_FEE + MULTIPLIER_DIVISOR;
        uint256 feeRate = unipeer.tradeFeeRate() + sellerRate;
        uint256 tradeFees = amount * feeRate / MULTIPLIER_DIVISOR;
        uint256 arbFees =
            unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());

        hoax(seller);
        unipeer.updateSellerPaymentMethod(PAYMENT_ID, "seller@paypal.me", sellerRate);

        startHoax(buyer);
        vm.expectRevert("Cummulative fees cannot be more than bought amount");
        unipeer.buyOrder{value: arbFees}({
            _paymentID: PAYMENT_ID,
            _seller: seller,
            _token: Dai,
            _amount: amount
        });
    }

    function testConfirmPaid(uint96 amount) public {
        testBuyOrder(amount);

        unipeer.confirmPaid(ORDER_ID);
    }

    function testCannotConfirmPaidByNonBuyer(uint96 amount) public {
        testBuyOrder(amount);
        vm.stopPrank();

        vm.expectRevert("Only Buyer");
        unipeer.confirmPaid(ORDER_ID);
    }

    function testCannotConfirmPaidAfterTimeout(uint96 amount) public {
        testBuyOrder(amount);

        skip(unipeer.buyerTimeout() + 1);
        vm.expectRevert("Payment confirmation timeout");
        unipeer.confirmPaid(ORDER_ID);
    }

    function testTimeoutByBuyer(uint96 amount) public {
        testBuyOrder(amount);

        skip(unipeer.buyerTimeout() + 1);
        vm.expectEmit(true, true, false, true);
        emit TimedOutByBuyer(ORDER_ID);
        unipeer.timeoutByBuyer(ORDER_ID);
    }

    function testCannotConfirmPaidAfterCancel() public {
        testTimeoutByBuyer(1);

        skip(unipeer.buyerTimeout() + 1);
        vm.expectRevert("OrderStatus: !Created");
        unipeer.confirmPaid(ORDER_ID);
    }

    // ************************************* //
    // *           Order (Seller)          * //
    // ************************************* //

    function testCompleteOrder(uint96 amount) public {
        testConfirmPaid(amount);
        vm.stopPrank();

        (uint256 fees, uint256 tradeAmount) = unipeer.getOrderFeeAmount(ORDER_ID);

        startHoax(seller);

        vm.expectEmit(true, true, false, true);
        emit Transfer(address(unipeer), buyer, tradeAmount);
        vm.expectEmit(true, true, false, true);
        emit OrderComplete(ORDER_ID);
        unipeer.completeOrder(ORDER_ID);

        assertEq(unipeer.protocolFees(Dai), fees);
    }

    function testCompleteOrderWithOutConfirm(uint96 amount) public {
        testBuyOrder(amount);
        vm.stopPrank();

        startHoax(seller);
        vm.expectEmit(true, true, false, true);
        emit OrderComplete(ORDER_ID);
        unipeer.completeOrder(ORDER_ID);
    }

    function testCompleteOrderWithOutConfirmAfterTimeout(uint96 amount) public {
        testBuyOrder(amount);
        vm.stopPrank();

        startHoax(seller);
        skip(unipeer.sellerTimeout() + 1);
        vm.expectEmit(true, true, false, true);
        emit OrderComplete(ORDER_ID);
        unipeer.completeOrder(ORDER_ID);
    }

    function testCannotCompleteOrderAfterTimeout(uint96 amount) public {
        testConfirmPaid(amount);
        vm.stopPrank();

        startHoax(seller);
        skip(unipeer.sellerTimeout() + 1);
        vm.expectRevert("Order completed by timeout");
        unipeer.completeOrder(ORDER_ID);
    }

    function testCannotCompleteOrderByNonSeller(uint96 amount) public {
        testConfirmPaid(amount);
        vm.stopPrank();

        vm.expectRevert("Only Seller");
        unipeer.completeOrder(ORDER_ID);
    }

    function testDisputeOrder(uint96 amount) public {
        testConfirmPaid(amount);
        vm.stopPrank();

        uint256 arbFees =
            unipeer.arbitrator().arbitrationCost(unipeer.arbitratorExtraData());

        startHoax(seller);
        vm.expectEmit(true, true, false, true);
        emit Dispute(unipeer.arbitrator(), ORDER_ID, META_ID, ORDER_ID);
        unipeer.disputeOrder{value: arbFees}(ORDER_ID);
    }

    function testCannotDiputeOrderWithoutArbFees(uint96 amount) public {
        testConfirmPaid(amount);
        vm.stopPrank();

        startHoax(seller);
        vm.expectRevert("Arbitration fees not paid");
        unipeer.disputeOrder(ORDER_ID);
    }

    function testCannotDisputeOrderAfterTimeout(uint96 amount) public {
        testConfirmPaid(amount);
        vm.stopPrank();

        startHoax(seller);
        skip(unipeer.sellerTimeout() + 1);
        vm.expectRevert("Order completed by timeout");
        unipeer.disputeOrder(ORDER_ID);
    }

    function testCannotDisputeNonPaidOrder() public {
        testBuyOrder(1);
        vm.stopPrank();

        startHoax(seller);
        vm.expectRevert("OrderStatus: !Paid");
        unipeer.disputeOrder(ORDER_ID);
    }

    function testCannotDisputeOrderByNonSeller() public {
        testBuyOrder(1);
        vm.stopPrank();

        vm.expectRevert("Only Seller");
        unipeer.disputeOrder(ORDER_ID);
    }

    function testTimeoutBySeller(uint96 amount) public {
        testConfirmPaid(amount);
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

        assertEq(unipeer.protocolFees(Dai), fees);
    }

    // ************************************* //
    // *             Arbitrator            * //
    // ************************************* //

    function testSubmitEvidence() public {
        testDisputeOrder(1);
        vm.stopPrank();

        vm.expectEmit(true, true, false, true);
        emit Evidence(arbitrator, ORDER_ID, address(this), "ipfs://test");
        unipeer.submitEvidence(ORDER_ID, "ipfs://test");
    }

    function testRulingBuyer(uint96 amount) public {
        testDisputeOrder(amount);
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

    function testRulingSeller(uint96 amount) public {
        testDisputeOrder(amount);
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

    function testRulingNone(uint96 amount) public {
        testDisputeOrder(amount);
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
        testDisputeOrder(1);
        vm.stopPrank();

        hoax(admin);
        vm.expectRevert("Only arbitrator");
        unipeer.rule(ORDER_ID, 1);
    }

    function testFailRuleOnNonExistingDispute(uint96 amount) public {
        testConfirmPaid(amount);
        vm.stopPrank();

        arbitrator.giveRuling(ORDER_ID, 1);
    }

    function testFundAppealBuyer(uint96 amount) public {
        testDisputeOrder(amount);
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

        (uint256[3] memory paidFees,,, bool appealed) = unipeer.getRoundInfo(ORDER_ID, 0);
        assertEq(paidFees[1], totalCost);
        assertEq(paidFees[2], 0);
        assertEq(appealed, false);

        assertEq(unipeer.getNumberOfRounds(ORDER_ID), 1);
    }

    function testFundAppealBuyerAppealed(uint96 amount) public {
        testFundAppealBuyer(amount);
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

        (uint256[3] memory paidFees,,, bool appealed) = unipeer.getRoundInfo(ORDER_ID, 0);
        assertEq(paidFees[1], totalCost / 2);
        assertEq(paidFees[2], totalCost);
        assertEq(appealed, true);

        assertEq(unipeer.getNumberOfRounds(ORDER_ID), 2);
    }

    function testCannotActuallyAppealWithoutPaying() public {
        testDisputeOrder(1);
        vm.stopPrank();

        arbitrator.giveRuling(ORDER_ID, 1);
        unipeer.fundAppeal(ORDER_ID, Unipeer.Party.Buyer);

        (uint256[3] memory paidFees,,, bool appealed) = unipeer.getRoundInfo(ORDER_ID, 0);
        assertEq(paidFees[1], 0);
        assertEq(appealed, false);
    }

    function testWithdrawFeeAndRewards(uint96 amount) public {
        testRulingBuyer(amount);

        (,, uint256 feeRewards,) = unipeer.getRoundInfo(ORDER_ID, 0);
        assertEq(feeRewards, 0);
    }

    function testWithdrawFeeAndRewardsWithAppeals(uint96 amount) public {
        testFundAppealBuyerAppealed(amount);

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

    function testBatchWithdrawFeeAndRewardsWithAppeals(uint96 amount) public {
        testFundAppealBuyerAppealed(amount);

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

    function testCannotWithdrawFeeAndRewardsWithoutRuling(uint96 amount) public {
        testFundAppealBuyerAppealed(amount);

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

    function testWithdrawFees(uint248 amount) public {
        startHoax(admin);

        vm.expectRevert("Amount more than accrued fees");
        unipeer.withdrawProtocolFees(Dai, 10, payable(user));

        Dai.mint(address(unipeer), amount);
        stdstore.target(address(unipeer)).sig("protocolFees(address)")
        .with_key(address(Dai))
        .checked_write(amount);
        assertEq(unipeer.protocolFees(Dai), amount);
        unipeer.withdrawProtocolFees(Dai, amount, payable(admin));

        vm.expectRevert("Amount more than accrued fees");
        unipeer.withdrawProtocolFees(Dai, 10, payable(user));
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
        unipeer.withdrawProtocolFees(Dai, 0, payable(user));
    }
}
