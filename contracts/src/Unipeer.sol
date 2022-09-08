// SPDX-License-Identifier: GPL-2.0
pragma solidity 0.8.15;

import "oz/access/Ownable.sol";
import "oz/token/ERC20/utils/SafeERC20.sol";
import "oz/utils/math/Math.sol";
import "kleros/IArbitrable.sol";
import "kleros/erc-1497/IEvidence.sol";
import "kleros/IArbitrator.sol";
import "delegatable/Delegatable.sol";

contract Unipeer is IArbitrable, IEvidence, Delegatable, Ownable {
    using SafeERC20 for IERC20;

    // ************************************* //
    // *      Constants and immutable      * //
    // ************************************* //

    uint256 private constant RULING_OPTIONS = 2; // The amount of non 0 choices the arbitrator can give.
    uint256 private constant MULTIPLIER_DIVISOR = 10_000; // Divisor parameter for multipliers.

    // Multiplier for calculating the appeal fee that must be paid by the
    // submitter in the case where there is no winner or loser
    // (e.g. when the arbitrator ruled "refuse to arbitrate").
    uint256 public immutable sharedStakeMultiplier;
    // Multiplier for calculating the appeal fee of the party that won the previous round.
    uint256 public immutable winnerStakeMultiplier;
    // Multiplier for calculating the appeal fee of the party that lost the previous round.
    uint256 public immutable loserStakeMultiplier;

    // ************************************* //
    // *              Enums                * //
    // ************************************* //

    enum Party {
        None, // Party per default when there is no challenger or requester. Also used for unconclusive ruling.
        Buyer, // Party that placed the buy order.
        Seller // Party that provided funds that were locked in a order.
    }

    enum Status {
        Created, // The buy order has been created and seller funds locked.
        Paid, // The buy has marked making the off-chain payment to seller.
        Completed, // The seller confirms receiving the off-chain payment.
        Cancelled, // The buyer never made the off-chain payment
        Disputed, // The seller has raised a dispute, claiming to have not received the payment.
        Resolved // The dispute has been successfully resolved.
    }

    // ************************************* //
    // *              Structs              * //
    // ************************************* //

    struct ArbitratorData {
        // Address of the trusted arbitrator to solve disputes.
        IArbitrator arbitrator;
        // Extra data for the arbitrator.
        bytes arbitratorExtraData;
    }

    struct PaymentMethod {
        // User friendly name used to identify the payment method.
        string paymentName;
        // Stores the meta evidence ID specific to the payment method
        // that is to be used in disputes.
        uint256 metaEvidenceID;
        // Tokens that are accepted via this payment method.
        // tokenEnabled[token] = true
        mapping(IERC20 => bool) tokenEnabled;
        // The payment address of a seller for this payment method that
        // a buyer will make payments to.
        // paymentAddress[seller] = "example@paypal.me"
        mapping(address => string) paymentAddress;
    }

    struct Order {
        address payable buyer;
        address payable seller;
        IERC20 token;
        uint256 amount;
        // The fee buyer has paid for arbitration at the time of placing an order.
        uint256 buyerFee;
        // The fee seller has paid for raising a dispute.
        uint256 sellerFee;
        // If dispute exists, the ID of the dispute.
        uint256 disputeID;
        uint256 arbitratorID;
        uint256 lastInteraction;
        Status status;
    }

    // Some arrays below have 3 elements to map with the Party enums for better readability:
    // - 0: is unused, matches `Party.None`.
    // - 1: for `Party.Buyer`.
    // - 2: for `Party.Seller`.
    struct Round {
        uint256[3] paidFees; // Tracks the fees paid by each side in this round.
        // Stores the side that successfully paid the appeal fees in the latest round.
        // Note that if both sides have paid a new round is created.
        Party sideFunded;
        // Sum of reimbursable fees and stake rewards available to the parties
        // that made contributions to the side that ultimately wins a dispute.
        uint256 feeRewards;
        mapping(address => uint256[3]) contributions; // Maps contributors to their contributions for each side.
    }

    struct DisputeData {
        uint256 orderID; // The ID of the order related to the dispute.
        Party ruling; // Ruling given by the arbitrator of the dispute.
        uint16 lastRoundID; // The ID of the last round.
        mapping(uint256 => Round) rounds; // Tracks the info of each funding round of the challenge.
    }

    // ************************************* //
    // *             Storage               * //
    // ************************************* //

    address public admin;
    // Total non-withdrawn fees accumulated by the protocol.
    uint256 public protocolFeesSum;
    // The fee rate applicable to trades,
    // max to the MULTIPLIER_DIVISOR decimal
    uint256 public tradeFees;

    uint16 public totalPaymentMethods;
    // List of Payment Methods by paymentID
    mapping(uint16 => PaymentMethod) public paymentMethods;

    // Stores the arbitrator data of the contract.
    // Updated each time the data is changed.
    ArbitratorData[] public arbitratorDataList;
    // Holds the total/count of Meta Evidence updates.
    uint256 metaEvidenceUpdates;
    uint256 confirmTimeout;
    uint256 orderTimeout;

    // tokenBalance[seller][token] = balance
    mapping(address => mapping(IERC20 => uint256)) public tokenBalance;
    // List of dispute details by disputeID
    mapping(uint256 => DisputeData) public disputes;
    // List of orders by orderID
    Order[] public orders;

    // ************************************* //
    // *             Modifiers             * //
    // ************************************* //

    modifier onlyAdmin() {
        _checkOwner();
        _;
    }

    // ************************************* //
    // *             Events                * //
    // ************************************* //

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

    /**
     * @dev To be emitted when the appeal fees of one of the parties are fully funded.
     * @param _orderID The ID of the respective order.
     * @param _party The party that is fully funded.
     */
    event HasPaidAppealFee(uint256 indexed _orderID, Party _party);

    /**
     * @dev To be emitted when someone contributes to the appeal process.
     * @param _orderID The ID of the respective order.
     * @param _party The party which received the contribution.
     * @param _contributor The address of the contributor.
     * @param _amount The amount contributed.
     */
    event AppealContribution(
        uint256 indexed _orderID, Party _party, address _contributor, uint256 _amount
    );

    /**
     * @dev Constructor.
     * @param _admin The administrator of the contract.
     * @param _arbitrator The arbitrator of the contract.
     * @param _arbitratorExtraData Extra data for the arbitrator.
     * @param _confirmTimeout The payment timeout for the buyer.
     * @param _orderTimeout The general interaction timeout for the parties.
     * @param _sharedStakeMultiplier Multiplier of the appeal cost that the
     * submitter must pay for a round when there is no winner/loser in
     * the previous round. In basis points.
     * @param _winnerStakeMultiplier Multiplier of the appeal cost that the winner
     * has to pay for a round. In basis points.
     * @param _loserStakeMultiplier Multiplier of the appeal cost that the loser
     * has to pay for a round. In basis points.
     */
    constructor(
        string memory version,
        address _admin,
        IArbitrator _arbitrator,
        bytes memory _arbitratorExtraData,
        uint256 _confirmTimeout,
        uint256 _orderTimeout,
        uint256 _sharedStakeMultiplier,
        uint256 _winnerStakeMultiplier,
        uint256 _loserStakeMultiplier,
        uint256 _tradeFees
    )
        Delegatable("Unipeer", version)
    {
        admin = _admin;
        arbitratorDataList.push(
            ArbitratorData({arbitrator: _arbitrator, arbitratorExtraData: _arbitratorExtraData})
        );
        confirmTimeout = _confirmTimeout;
        orderTimeout = _orderTimeout;
        sharedStakeMultiplier = _sharedStakeMultiplier;
        winnerStakeMultiplier = _winnerStakeMultiplier;
        loserStakeMultiplier = _loserStakeMultiplier;
        tradeFees = _tradeFees;
    }

    // ************************************* //
    // *            Mutating               * //
    // ************************************* //

    // ************************************* //
    // *           Admin only              * //
    // ************************************* //

    /**
     * @dev Change the arbitrator to be used for disputes.
     * The arbitrator is trusted to support appeal period and not reenter.
     * @param _arbitrator The new trusted arbitrator to be used in the next requests.
     * @param _arbitratorExtraData The extra data used by the new arbitrator.
     */
    function changeArbitrator(IArbitrator _arbitrator, bytes calldata _arbitratorExtraData)
        external
        onlyAdmin
    {
        arbitratorDataList.push(
            ArbitratorData({arbitrator: _arbitrator, arbitratorExtraData: _arbitratorExtraData})
        );
    }

    /**
     * @dev Add a new meta evidence used for disputes.
     * @param _metaEvidence The meta evidence to be used for future disputes
     * requests.
     * returns metaEvidenceID The ID of the associated meta evidence that
     * can be then linked to a payment method.
     */
    function addMetaEvidence(string calldata _metaEvidence)
        external
        onlyAdmin
        returns (uint256 metaEvidenceID)
    {
        metaEvidenceID = metaEvidenceUpdates + 1;
        emit MetaEvidence(metaEvidenceID, _metaEvidence);
    }

    function addPaymentMethod(
        string calldata _paymentName,
        uint8 _metaEvidenceID,
        IERC20 _initalEnabledToken
    )
        external
        onlyAdmin
    {
        require(_metaEvidenceID <= metaEvidenceUpdates, "Invalid Meta Evidence ID");
        PaymentMethod storage pm = paymentMethods[totalPaymentMethods++];
        pm.paymentName = _paymentName;
        pm.metaEvidenceID = _metaEvidenceID;
        pm.tokenEnabled[_initalEnabledToken] = true;

        emit PaymentMethodUpdate(totalPaymentMethods - 1, _paymentName, _metaEvidenceID);
    }

    function updateMetaEvidenceID(uint16 _paymentID, uint8 _metaEvidenceID)
        external
        onlyAdmin
    {
        require(_paymentID < totalPaymentMethods, "Payment method does not exist.");
        require(_metaEvidenceID <= metaEvidenceUpdates, "Invalid Meta Evidence ID");

        PaymentMethod storage pm = paymentMethods[_paymentID];
        pm.metaEvidenceID = _metaEvidenceID;

        emit PaymentMethodUpdate(_paymentID, pm.paymentName, pm.metaEvidenceID);
    }

    function updatePaymentName(uint16 _paymentID, string calldata _paymentName)
        external
        onlyAdmin
    {
        require(_paymentID < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentID];
        pm.paymentName = _paymentName;

        emit PaymentMethodUpdate(_paymentID, pm.paymentName, pm.metaEvidenceID);
    }

    function updateTokenEnabled(uint16 _paymentID, IERC20 _token, bool _enabled)
        external
        onlyAdmin
    {
        require(_paymentID < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentID];
        pm.tokenEnabled[_token] = _enabled;
    }

    function changeConfirmTimeout(uint256 _timeout) external onlyAdmin {
        confirmTimeout = _timeout;
    }

    function changeOrderTimeout(uint256 _timeout) external onlyAdmin {
        orderTimeout = _timeout;
    }

    function changeFees(uint256 _fees) external onlyAdmin {
        require(_fees < MULTIPLIER_DIVISOR, "fees cannot be more than 100%");
        tradeFees = _fees;
    }

    function withdrawFees(uint256 _amount, address payable _to) external onlyAdmin {
        require(_amount <= protocolFeesSum, "Cannot withdraw more than available fees");
        protocolFeesSum -= _amount;
        _to.transfer(_amount);
    }

    // ************************************* //
    // *             Seller                * //
    // ************************************* //

    function acceptPaymentMethod(uint16 _paymentID, string calldata _paymentAddress)
        external
    {
        require(_paymentID < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentID];
        pm.paymentAddress[msg.sender] = _paymentAddress;

        emit SellerPaymentMethod(msg.sender, _paymentID, _paymentAddress);
    }

    function disablePaymentMethod(uint16 _paymentID) external {
        require(_paymentID < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentID];
        pm.paymentAddress[msg.sender] = "";

        emit SellerPaymentDisabled(msg.sender, _paymentID);
    }

    function depositTokens(uint16 _paymentID, IERC20 _token, uint256 _amount) external {
        require(_paymentID < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentID];
        require(pm.tokenEnabled[_token] == true, "Token not yet enabled for selling");

        tokenBalance[msg.sender][_token] += _amount;
        _token.safeTransferFrom(msg.sender, address(this), _amount);

        emit SellerDeposit(msg.sender, _token, _amount);
    }

    function withdrawTokens(IERC20 _token, uint256 _amount) external {
        require(
            tokenBalance[msg.sender][_token] >= _amount, "Not enough balance to withdraw"
        );

        tokenBalance[msg.sender][_token] -= _amount;

        _token.safeTransfer(msg.sender, _amount);
        emit SellerWithdraw(msg.sender, _token, _amount);
    }

    // ************************************* //
    // *           Order (Buyer)           * //
    // ************************************* //

    function buyOrder(uint16 _paymentID, address _seller, IERC20 _token, uint256 _amount)
        external
        payable
    {
        require(_paymentID < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentID];
        require(
            bytes(pm.paymentAddress[_seller]).length != 0,
            "Seller doesn't accept this payment method"
        );
        require(
            pm.tokenEnabled[_token] == true, "Token is not enabled for this payment method"
        );
        require(tokenBalance[_seller][_token] >= _amount, "Not enough seller balance");

        ArbitratorData memory arbitratorData =
            arbitratorDataList[arbitratorDataList.length - 1];
        IArbitrator arbitrator = arbitratorData.arbitrator;
        bytes memory arbitratorExtraData = arbitratorData.arbitratorExtraData;
        uint256 arbitrationCost = arbitrator.arbitrationCost(arbitratorExtraData);

        require(msg.value >= arbitrationCost, "Arbitration fees need to be paid");

        orders.push(
            Order({
                buyer: payable(msg.sender),
                seller: payable(_seller),
                token: _token,
                amount: _amount,
                buyerFee: msg.value,
                sellerFee: 0,
                disputeID: 0,
                arbitratorID: arbitratorDataList.length - 1,
                lastInteraction: block.timestamp,
                status: Status.Created
            })
        );
        tokenBalance[_seller][_token] -= _amount;

        (uint256 fee, uint256 tradeAmount) = buyQuoteWithFees(_amount);
        emit BuyOrder(
            orders.length - 1, msg.sender, _paymentID, _seller, _token, tradeAmount, fee
            );
    }

    function confirmPaid(uint256 _orderID) external {
        Order storage order = orders[_orderID];
        require(order.buyer == msg.sender, "Only buyer");
        require(
            order.status == Status.Created, "Order already cancelled, completed or disputed"
        );
        require(
            order.lastInteraction + confirmTimeout >= block.timestamp,
            "Payment confirmation period is over"
        );

        order.lastInteraction = block.timestamp;
        order.status = Status.Paid;

        emit Paid(_orderID);
    }

    // ************************************* //
    // *           Order (Seller)          * //
    // ************************************* //

    /**
     * @notice Called by a seller to mark an order complete
     * It can timeout and default to confirmed if the seller doesn't
     * respond.
     */
    function completeOrder(uint256 _orderID) external {
        Order storage order = orders[_orderID];
        require(order.seller == msg.sender, "Only seller");
        require(
            order.status < Status.Completed, "Order already cancelled, completed or disputed"
        );
        require(
            order.status == Status.Created || order.lastInteraction + orderTimeout >= block.timestamp,
            "Order completed by timeout"
        );

        _markOrderComplete(order);
        emit OrderComplete(_orderID);
    }

    function disputeOrder(uint256 _orderID) external payable {
        Order storage order = orders[_orderID];
        require(order.seller == msg.sender, "Only seller");
        require(order.status == Status.Paid, "Cannot dispute a not yet paid order");
        require(
            order.lastInteraction + orderTimeout >= block.timestamp,
            "Order already completed by timeout"
        );

        ArbitratorData memory arbitratorData =
            arbitratorDataList[arbitratorDataList.length - 1];
        IArbitrator arbitrator = arbitratorData.arbitrator;
        bytes memory arbitratorExtraData = arbitratorData.arbitratorExtraData;
        uint256 arbitrationCost = arbitrator.arbitrationCost(arbitratorExtraData);

        // Seller can overpay to draw more jurors.
        require(msg.value >= arbitrationCost, "Arbitration fees need to be paid");

        order.sellerFee = msg.value;
        order.status = Status.Disputed;
        order.disputeID =
            arbitrator.createDispute{value: msg.value}(RULING_OPTIONS, arbitratorExtraData);

        DisputeData storage dispute = disputes[order.disputeID];
        dispute.orderID = _orderID;

        emit Dispute(arbitrator, order.disputeID, _orderID, _orderID);
    }

    // ************************************* //
    // *             Anonymous             * //
    // ************************************* //

    function timeoutByBuyer(uint256 _orderID) external {
        Order storage order = orders[_orderID];
        require(
            order.status == Status.Created,
            "Order can only be cancelled immediately after creation"
        );
        require(
            order.lastInteraction + confirmTimeout < block.timestamp,
            "Confirmation period has not yet timed out"
        );

        uint256 amount = order.amount;
        uint256 buyerFee = order.buyerFee;

        order.amount = 0;
        order.buyerFee = 0;
        order.status = Status.Cancelled;

        // Unlock seller funds
        tokenBalance[order.seller][order.token] += amount;

        // Return the buyers arbitration fees.
        order.buyer.send(buyerFee);
        emit TimedOutByBuyer(_orderID);
    }

    function timeoutBySeller(uint256 _orderID) external {
        Order storage order = orders[_orderID];
        require(
            order.status == Status.Paid,
            "Order can only be cancelled immediately after creation"
        );
        require(
            order.lastInteraction + orderTimeout < block.timestamp,
            "Order completion period has not yet timed out"
        );

        _markOrderComplete(order);
        emit TimedOutBySeller(_orderID);
        emit OrderComplete(_orderID);
    }

    // ************************************* //
    // *             Arbitrator            * //
    // ************************************* //

    /**
     * @dev Submit a reference to evidence. EVENT.
     * @param _orderID The index of the order.
     * @param _evidence A link to an evidence using its URI.
     */
    function submitEvidence(uint256 _orderID, string calldata _evidence) external {
        Order memory order = orders[_orderID];
        require(
            order.status < Status.Resolved,
            "Must not send evidence if the dispute is resolved."
        );

        ArbitratorData memory arbitratorData = arbitratorDataList[order.arbitratorID];
        emit Evidence(arbitratorData.arbitrator, _orderID, msg.sender, _evidence);
    }

    /**
     * @dev Takes up to the total amount required to fund a side of an appeal.
     * Reimburses the rest. Creates an appeal if both sides are fully funded.
     * @param _orderID The ID of the buy order.
     * @param _side The party that pays the appeal fee.
     */
    function fundAppeal(uint256 _orderID, Party _side) external payable {
        Order storage order = orders[_orderID];
        require(_side != Party.None, "Wrong party.");
        require(order.status == Status.Disputed, "No dispute to appeal");

        ArbitratorData memory arbitratorData = arbitratorDataList[order.arbitratorID];
        (uint256 appealPeriodStart, uint256 appealPeriodEnd) =
            arbitratorData.arbitrator.appealPeriod(order.disputeID);
        require(
            block.timestamp >= appealPeriodStart && block.timestamp < appealPeriodEnd,
            "Funding must be made within the appeal period."
        );

        uint256 multiplier;
        uint256 winner = arbitratorData.arbitrator.currentRuling(order.disputeID);
        if (winner == uint256(_side)) {
            multiplier = winnerStakeMultiplier;
        } else if (winner == 0) {
            multiplier = sharedStakeMultiplier;
        } else {
            require(
                block.timestamp < (appealPeriodEnd + appealPeriodStart) / 2,
                "The loser must pay during the first half of the appeal period."
            );
            multiplier = loserStakeMultiplier;
        }

        DisputeData storage dispute = disputes[order.disputeID];
        Round storage round = dispute.rounds[dispute.lastRoundID];
        require(_side != round.sideFunded, "Appeal fee has already been paid.");

        uint256 appealCost = arbitratorData.arbitrator.appealCost(
            order.disputeID, arbitratorData.arbitratorExtraData
        );
        uint256 totalCost = appealCost + ((appealCost * multiplier) / MULTIPLIER_DIVISOR);

        {
            // Take up to the amount necessary to fund the current round at the current costs.
            uint256 contribution; // Amount contributed.
            uint256 remainingETH; // Remaining ETH to send back.
            (contribution, remainingETH) =
                _calculateContribution(msg.value, totalCost - round.paidFees[uint256(_side)]);
            round.contributions[msg.sender][uint256(_side)] += contribution;
            round.paidFees[uint256(_side)] += contribution;

            emit AppealContribution(_orderID, _side, msg.sender, contribution);

            // Reimburse leftover ETH if any.
            // Deliberate use of send in order to not block the contract in case of reverting fallback.
            if (remainingETH > 0) {
                payable(msg.sender).send(remainingETH);
            }
        }

        if (round.paidFees[uint256(_side)] >= totalCost) {
            if (round.sideFunded == Party.None) {
                round.sideFunded = _side;
            } else {
                // Both sides are fully funded. Create an appeal.
                arbitratorData.arbitrator.appeal{value: appealCost}(
                    order.disputeID, arbitratorData.arbitratorExtraData
                );
                round.feeRewards = (
                    round.paidFees[uint256(Party.Buyer)]
                        + round.paidFees[uint256(Party.Seller)]
                ) - appealCost;

                dispute.lastRoundID++;
                round.sideFunded = Party.None;
            }
            emit HasPaidAppealFee(_orderID, _side);
        }
    }

    /**
     * @dev Give a ruling for a dispute. Can only be called by the arbitrator. TRUSTED.
     * Account for the situation where the winner loses a case due to paying less appeal fees than expected.
     * @param _disputeID ID of the dispute in the arbitrator contract.
     * @param _ruling Ruling given by the arbitrator. Note that 0 is reserved for "Refused to arbitrate".
     */
    function rule(uint256 _disputeID, uint256 _ruling) external {
        DisputeData storage dispute = disputes[_disputeID];
        Order storage order = orders[dispute.orderID];
        IArbitrator arbitrator = arbitratorDataList[order.arbitratorID].arbitrator;

        require(msg.sender == address(arbitrator), "Only arbitrator");
        require(_ruling <= RULING_OPTIONS, "Invalid ruling.");
        require(order.status != Status.Resolved, " Dispute already resolved.");

        Round storage round = dispute.rounds[dispute.lastRoundID];

        // If only one side paid its fees we assume the ruling to be in its favor.
        if (round.sideFunded == Party.Buyer) {
            dispute.ruling = Party.Buyer;
        } else if (round.sideFunded == Party.Seller) {
            dispute.ruling = Party.Seller;
        } else {
            dispute.ruling = Party(_ruling);
        }

        _executeRuling(_disputeID);

        emit Ruling(arbitrator, _disputeID, uint256(dispute.ruling));
    }

    /**
     * @dev Withdraws contributions of appeal rounds. Reimburses contributions
     * if the appeal was not fully funded.
     * If the appeal was fully funded, sends the fee stake rewards and reimbursements
     * proportional to the contributions made to the winner of a dispute.
     * @param _beneficiary The address that made contributions.
     * @param _orderID The ID of the resolved order.
     * @param _round The round from which to withdraw.
     */
    function withdrawFeesAndRewards(
        address payable _beneficiary,
        uint256 _orderID,
        uint256 _round
    )
        external
    {
        Order storage order = orders[_orderID];
        require(order.status == Status.Resolved, "The order must be resolved.");
        DisputeData storage dispute = disputes[order.disputeID];
        require(dispute.orderID == _orderID, "Undisputed order");

        uint256 reward =
            _withdrawFeesAndRewards(_beneficiary, _orderID, _round, uint256(dispute.ruling));
        _beneficiary.send(reward); // It is the user responsibility to accept ETH.
    }

    /**
     * @dev Withdraws contributions of multiple appeal rounds at once.
     * This function is O(n) where n is the number of rounds.
     * This could exceed the gas limit, therefore this function should be used
     * only as a utility and not be relied upon by other contracts.
     * @param _beneficiary The address that made contributions.
     * @param _orderID The ID of the resolved order.
     * @param _cursor The round from where to start withdrawing.
     * @param _count The number of rounds to iterate. If set to 0 or a value
     * larger than the number of rounds, iterates until the last round.
     */
    function batchRoundWithdraw(
        address payable _beneficiary,
        uint256 _orderID,
        uint256 _cursor,
        uint256 _count
    )
        external
    {
        Order storage order = orders[_orderID];
        require(order.status == Status.Resolved, "The order must be resolved.");
        DisputeData storage dispute = disputes[order.disputeID];
        require(dispute.orderID == _orderID, "Undisputed order");
        uint256 finalRuling = uint256(dispute.ruling);

        uint256 reward;
        uint256 totalRounds = dispute.lastRoundID;
        for (
            uint256 i = _cursor; i <= totalRounds && (_count == 0 || i < _cursor + _count); i++
        ) {
            reward += _withdrawFeesAndRewards(_beneficiary, _orderID, i, finalRuling);
        }
        _beneficiary.send(reward); // It is the user responsibility to accept ETH.
    }

    // ************************************* //
    // *              Views                * //
    // ************************************* //

    /**
     * @dev Gets the trade amount after fees
     * @return fee The fee amount according to the tradeFee rate.
     * @return tradeAmount The amount minus fees to be transferred to the buyer.
     */
    function buyQuoteWithFees(uint256 _amount)
        public
        view
        returns (uint256 fee, uint256 tradeAmount)
    {
        fee = _amount * tradeFees / MULTIPLIER_DIVISOR;
        tradeAmount = _amount - fee;
    }

    function calculateFee(uint256 _amount) public view returns (uint256) {
        (uint256 fee,) = buyQuoteWithFees(_amount);
        return fee;
    }

    function getFeeAmount(uint256 _orderID) external view returns (uint256) {
        Order storage order = orders[_orderID];
        return calculateFee(order.amount);
    }

    function getCountOrders() external view returns (uint256) {
        return orders.length;
    }

    /**
     * @dev Returns the sum of withdrawable wei from appeal rounds.
     * This function is O(n), where n is the number of rounds of the order.
     * This could exceed the gas limit, therefore this function should only
     * be used for interface display and not by other contracts.
     * @param _orderID The index of the order.
     * @param _beneficiary The contributor for which to query.
     * @return total The total amount of wei available to withdraw.
     */
    function amountWithdrawable(uint256 _orderID, address _beneficiary)
        external
        view
        returns (uint256 total)
    {
        Order storage order = orders[_orderID];
        DisputeData storage dispute = disputes[order.disputeID];
        if (order.status != Status.Resolved) {
            return total;
        }
        if (dispute.orderID != _orderID) {
            return total;
        }
        uint256 finalRuling = uint256(dispute.ruling);

        uint256 totalRounds = dispute.lastRoundID;
        for (uint256 i = 0; i <= totalRounds; i++) {
            Round storage round = dispute.rounds[i];
            if (i == totalRounds - 1) {
                total += round.contributions[_beneficiary][uint256(Party.Buyer)]
                    + round.contributions[_beneficiary][uint256(Party.Seller)];
            } else if (finalRuling == uint256(Party.None)) {
                uint256 totalFeesPaid = round.paidFees[uint256(Party.Buyer)]
                    + round.paidFees[uint256(Party.Seller)];
                uint256 totalBeneficiaryContributions = round.contributions[_beneficiary][uint256(
                    Party.Buyer
                )] + round.contributions[_beneficiary][uint256(Party.Seller)];
                total +=
                    totalFeesPaid > 0
                    ? (totalBeneficiaryContributions * round.feeRewards) / totalFeesPaid
                    : 0;
            } else {
                total +=
                    round.paidFees[finalRuling] > 0
                    ? (round.contributions[_beneficiary][finalRuling] * round.feeRewards)
                        / round.paidFees[finalRuling]
                    : 0;
            }
        }
    }

    /**
     * @dev Gets the number of rounds of the specific order.
     * @param _orderID The ID of the order.
     * @return The number of rounds.
     */
    function getNumberOfRounds(uint256 _orderID) external view returns (uint256) {
        Order storage order = orders[_orderID];
        DisputeData storage dispute = disputes[order.disputeID];
        return dispute.lastRoundID + 1;
    }

    /**
     * @dev Gets the contributions made by a party for a given round of the appeal.
     * @param _orderID The ID of the order.
     * @param _round The position of the round.
     * @param _contributor The address of the contributor.
     * @return contributions The contributions.
     */
    function getContributions(uint256 _orderID, uint256 _round, address _contributor)
        external
        view
        returns (uint256[3] memory contributions)
    {
        Order storage order = orders[_orderID];
        DisputeData storage dispute = disputes[order.disputeID];
        Round storage round = dispute.rounds[_round];
        contributions = round.contributions[_contributor];
    }

    /**
     * @dev Gets the information on a round of a order.
     * @param _orderID The ID of the order.
     * @param _round The round to query.
     * @return paidFees
     * @return sideFunded
     * @return feeRewards
     * @return appealed
     */
    function getRoundInfo(uint256 _orderID, uint256 _round)
        external
        view
        returns (
            uint256[3] memory paidFees,
            Party sideFunded,
            uint256 feeRewards,
            bool appealed
        )
    {
        Order storage order = orders[_orderID];
        DisputeData storage dispute = disputes[order.disputeID];
        Round storage round = dispute.rounds[_round];
        return (
            round.paidFees, round.sideFunded, round.feeRewards, _round != dispute.lastRoundID
        );
    }

    // ************************************* //
    // *            Internal               * //
    // ************************************* //

    function _executeRuling(uint256 _disputeID) internal {
        DisputeData storage dispute = disputes[_disputeID];
        Order storage order = orders[dispute.orderID];

        uint256 amount = order.amount;
        uint256 buyerFee = order.buyerFee;
        uint256 sellerFee = order.sellerFee;

        order.amount = 0;
        order.buyerFee = 0;
        order.sellerFee = 0;
        order.status = Status.Resolved;

        (uint256 fee, uint256 tradeAmount) = buyQuoteWithFees(amount);
        protocolFeesSum += fee;

        if (dispute.ruling == Party.Buyer) {
            order.buyer.send(buyerFee);
            // non-safe transfer used here to prevent blocking on revert
            order.token.transfer(order.buyer, tradeAmount);
        } else if (dispute.ruling == Party.Seller) {
            order.seller.send(sellerFee);
            tokenBalance[order.seller][order.token] += amount;
        } else {
            // `buyerFee` and `sellerFee` are equal to the arbitration cost.
            // We take the min of the two in case someone overpaid.
            uint256 splitArbitrationFee = Math.min(buyerFee, sellerFee) / 2;
            order.buyer.send(splitArbitrationFee);
            order.seller.send(splitArbitrationFee);

            uint256 splitAmount = amount / 2;
            // non-safe transfer used here to prevent blocking on revert
            order.token.transfer(order.buyer, splitAmount);
            order.token.transfer(order.seller, splitAmount);
        }

        emit OrderResolved(dispute.orderID);
    }

    /**
     * @dev Updates contributions of appeal rounds which are going to be withdrawn.
     * Caller functions MUST:
     * (1) check that the order is valid and Resolved
     * (2) send the rewards to the _beneficiary.
     * @param _beneficiary The address that made contributions.
     * @param _orderID The ID of the resolved order.
     * @param _round The round from which to withdraw.
     * @param _finalRuling The final ruling of this order.
     * @return reward The amount of wei available to withdraw from _round.
     */
    function _withdrawFeesAndRewards(
        address _beneficiary,
        uint256 _orderID,
        uint256 _round,
        uint256 _finalRuling
    )
        internal
        returns (uint256 reward)
    {
        Order storage order = orders[_orderID];
        DisputeData storage dispute = disputes[order.disputeID];
        Round storage round = dispute.rounds[_round];
        uint256[3] storage contributionTo = round.contributions[_beneficiary];
        uint256 lastRound = dispute.lastRoundID;

        if (_round == lastRound) {
            // Allow to reimburse if funding was unsuccessful.
            reward =
                contributionTo[uint256(Party.Buyer)] + contributionTo[uint256(Party.Seller)];
        } else if (_finalRuling == uint256(Party.None)) {
            // Reimburse unspent fees proportionally if there is no winner and loser.
            uint256 totalFeesPaid =
                round.paidFees[uint256(Party.Buyer)] + round.paidFees[uint256(Party.Seller)];
            uint256 totalBeneficiaryContributions =
                contributionTo[uint256(Party.Buyer)] + contributionTo[uint256(Party.Seller)];
            reward =
                totalFeesPaid > 0
                ? (totalBeneficiaryContributions * round.feeRewards) / totalFeesPaid
                : 0;
        } else {
            // Reward the winner.
            reward =
                round.paidFees[_finalRuling] > 0
                ? (contributionTo[_finalRuling] * round.feeRewards) / round.paidFees[_finalRuling]
                : 0;
        }
        contributionTo[uint256(Party.Buyer)] = 0;
        contributionTo[uint256(Party.Seller)] = 0;
    }

    /**
     * @dev Returns the contribution value and remainder from available ETH and required amount.
     * @param _available The amount of ETH available for the contribution.
     * @param _requiredAmount The amount of ETH required for the contribution.
     * @return taken The amount of ETH taken.
     * @return remainder The amount of ETH left from the contribution.
     */
    function _calculateContribution(uint256 _available, uint256 _requiredAmount)
        internal
        pure
        returns (uint256 taken, uint256 remainder)
    {
        // Take whatever is available, return 0 as leftover ETH.
        if (_requiredAmount > _available) {
            return (_available, 0);
        }

        remainder = _available - _requiredAmount;
        return (_requiredAmount, remainder);
    }

    function _markOrderComplete(Order storage order) internal {
        uint256 amount = order.amount;
        uint256 buyerFee = order.buyerFee;

        order.amount = 0;
        order.buyerFee = 0;
        order.status = Status.Completed;

        (uint256 fee, uint256 tradeAmount) = buyQuoteWithFees(amount);
        protocolFeesSum += fee;

        // Return the buyers arbitration fees.
        order.buyer.send(buyerFee);

        // Actually close the order by
        // transferring the bought tokens
        order.token.safeTransfer(order.buyer, tradeAmount);
    }

    function _msgSender()
        internal
        view
        override (DelegatableCore, Context)
        returns (address sender)
    {
        if (msg.sender == address(this)) {
            bytes memory array = msg.data;
            uint256 index = msg.data.length;
            assembly {
                // Load the 32 bytes word from memory with the address on the lower 20 bytes, and mask those.
                sender :=
                    and(mload(add(array, index)), 0xffffffffffffffffffffffffffffffffffffffff)
            }
        } else {
            sender = msg.sender;
        }
        return sender;
    }
}
