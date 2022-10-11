// SPDX-License-Identifier: GPL-2.0
pragma solidity 0.8.17;

import "oz/token/ERC20/utils/SafeERC20.sol";
import "oz/utils/math/Math.sol";
import "kleros/erc-1497/IEvidence.sol";
import "kleros/IArbitrator.sol";
// import "delegatable/interfaces/IDelegatable.sol";

contract Unipeer is IArbitrable, IEvidence {
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

    struct PaymentMethod {
        // Stores the meta evidence ID specific to the payment method
        // that is to be used in disputes.
        uint256 metaEvidenceID;
        // User friendly name used to identify the payment method.
        string paymentName;
        // Tokens that are accepted via this payment method.
        // tokenEnabled[token] = true
        mapping(IERC20 => bool) tokenEnabled;
        // The payment address of a seller for this payment method that
        // a buyer will make payments to.
        // paymentAddress[seller] = "example@paypal.me"
        mapping(address => string) paymentAddress;
        // Seller's feeRate for a payment method
        // in MULTIPLIER_DIVISOR basis points
        mapping(address => uint256) feeRate;
    }

    struct Order {
        uint256 amount;
        // The calculated fees from tradeFeeRate
        uint256 fee;
        // The fee buyer has paid for arbitration at the time of placing an order.
        uint256 buyerCost;
        // The fee seller has paid for raising a dispute.
        uint256 sellerCost;
        // If dispute exists, the ID of the dispute.
        uint256 disputeID;
        uint256 lastInteraction;
        address payable buyer;
        address payable seller;
        IERC20 token;
        uint16 paymentID;
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
        IArbitrator arbitrator;
        bytes extraData;
        bool resolved;
    }

    // ************************************* //
    // *             Storage               * //
    // ************************************* //

    address public admin;
    // A DelegatableRelay contract used to verify signatures.
    // We use an external deploy contract instead of inheriting it to
    // workaround contract size limits.
    // For more details, see: https://github.com/delegatable/delegatable-sol
    address public relay;
    // Total non-withdrawn fees accumulated by the protocol.
    uint256 public protocolFeesSum;
    // The fee rate applicable to trades,
    // max to the MULTIPLIER_DIVISOR decimal
    uint256 public tradeFeeRate;

    uint16 public totalPaymentMethods;
    // List of Payment Methods by paymentID
    mapping(uint16 => PaymentMethod) public paymentMethods;

    // Holds the total/count of Meta Evidence updates.
    uint256 public metaEvidenceUpdates;
    uint256 public buyerTimeout;
    uint256 public sellerTimeout;

    // tokenBalance[seller][token] = balance
    mapping(address => mapping(IERC20 => uint256)) public tokenBalance;
    // List of dispute details by disputeID
    mapping(uint256 => DisputeData) private disputes;

    // List of orders by orderID
    Order[] public orders;

    // Address of the trusted arbitrator to solve disputes.
    IArbitrator public arbitrator;
    // Extra data for the arbitrator.
    bytes public arbitratorExtraData;

    // ************************************* //
    // *             Modifiers             * //
    // ************************************* //

    modifier onlyAdmin() {
        _onlyAdmin();
        _;
    }

    // ************************************* //
    // *             Events                * //
    // ************************************* //

    event AdminTransferred(address indexed previousAdmin, address indexed newAdmin);
    event FeeWithdrawn(uint256 amount);
    event PaymentMethodUpdate(
        uint16 indexed paymentID, string paymentName, uint256 metaEvidenceID
    );
    event PaymentMethodTokenEnabled(uint16 indexed paymentID, IERC20 token);
    event PaymentMethodTokenDisabled(uint16 indexed paymentID, IERC20 token);
    event SellerPaymentMethod(
        address indexed sender, uint16 paymentID, string paymentAddress, uint256 feeRate
    );
    event SellerDeposit(address indexed sender, IERC20 token, uint256 amount);
    event SellerWithdraw(address indexed sender, IERC20 token, uint256 amount);
    event OrderBuy(
        uint256 indexed orderID,
        address buyer,
        uint16 paymentID,
        address seller,
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
     * @param _relay The DelegatableRelay contract address.
     * @param _arbitrator The arbitrator of the contract.
     * @param _arbitratorExtraData Extra data for the arbitrator.
     * @param _buyerTimeout The payment timeout for the buyer.
     * @param _sellerTimeout The interaction timeout for the seller.
     * @param _sharedStakeMultiplier Multiplier of the appeal cost that the
     *          submitter must pay for a round when there is no winner/loser in
     *          the previous round. In basis points.
     * @param _winnerStakeMultiplier Multiplier of the appeal cost that the winner
     *          has to pay for a round. In basis points.
     * @param _loserStakeMultiplier Multiplier of the appeal cost that the loser
     *          has to pay for a round. In basis points.
     * @param _tradeFeeRate The feeRate by MULTIPLE_DIVISOR for accruing
     *          protocol fees.
     */
    constructor(
        address _admin,
        address _relay,
        string memory _version,
        IArbitrator _arbitrator,
        bytes memory _arbitratorExtraData,
        uint256 _buyerTimeout,
        uint256 _sellerTimeout,
        uint256 _sharedStakeMultiplier,
        uint256 _winnerStakeMultiplier,
        uint256 _loserStakeMultiplier,
        uint256 _tradeFeeRate
    ) {
        admin = _admin;
        relay = _relay;
        arbitrator = _arbitrator;
        arbitratorExtraData = _arbitratorExtraData;
        buyerTimeout = _buyerTimeout;
        sellerTimeout = _sellerTimeout;
        sharedStakeMultiplier = _sharedStakeMultiplier;
        winnerStakeMultiplier = _winnerStakeMultiplier;
        loserStakeMultiplier = _loserStakeMultiplier;
        tradeFeeRate = _tradeFeeRate;
    }

    // ************************************* //
    // *            Mutating               * //
    // ************************************* //

    // ************************************* //
    // *           Admin only              * //
    // ************************************* //

    function changeAdmin(address _newAdmin) external onlyAdmin {
        address oldAdmin = admin;
        admin = _newAdmin;
        emit AdminTransferred(oldAdmin, _newAdmin);
    }

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
        arbitrator = _arbitrator;
        arbitratorExtraData = _arbitratorExtraData;
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
        metaEvidenceID = metaEvidenceUpdates++;
        emit MetaEvidence(metaEvidenceID, _metaEvidence);
    }

    function addPaymentMethod(
        string calldata _paymentName,
        uint256 _metaEvidenceID,
        IERC20 _initalEnabledToken
    ) external onlyAdmin {
        require(_metaEvidenceID < metaEvidenceUpdates, "Invalid Meta Evidence ID");
        PaymentMethod storage pm = paymentMethods[totalPaymentMethods++];
        pm.paymentName = _paymentName;
        pm.metaEvidenceID = _metaEvidenceID;
        pm.tokenEnabled[_initalEnabledToken] = true;

        emit PaymentMethodUpdate(totalPaymentMethods - 1, _paymentName, _metaEvidenceID);
        emit PaymentMethodTokenEnabled(totalPaymentMethods - 1, _initalEnabledToken);
    }

    function updatePaymentMetaEvidence(uint16 _paymentID, uint256 _metaEvidenceID)
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

        if (_enabled)
            emit PaymentMethodTokenEnabled(_paymentID, _token);
        else
            emit PaymentMethodTokenDisabled(_paymentID, _token);
    }

    function changeBuyerTimeout(uint256 _timeout) external onlyAdmin {
        buyerTimeout = _timeout;
    }

    function changeSellerTimeout(uint256 _timeout) external onlyAdmin {
        sellerTimeout = _timeout;
    }

    function changeFees(uint256 _feeRate) external onlyAdmin {
        require(_feeRate <= MULTIPLIER_DIVISOR, "fees cannot be more than 100%");
        tradeFeeRate = _feeRate;
    }

    function withdrawFees(uint256 _amount, address payable _to) external onlyAdmin {
        require(_amount <= protocolFeesSum, "Amount more than accrued fees");
        protocolFeesSum -= _amount;
        _to.send(_amount);
    }

    // ************************************* //
    // *             Seller                * //
    // ************************************* //

    /**
     * @notice Accept or disable a payment methods
     * by a seller for orders.
     * A payment method can be disabled by setting it
     * to an empty string.
     * @param _paymentID The ID of the payment method
     * @param _paymentAddress The payment address the seller will
     *  accept off-chain payments on
     * @param _feeRate The fee rate charged by the seller for
     *  this payment method
     */
    function updateSellerPaymentMethod(
        uint16 _paymentID,
        string calldata _paymentAddress,
        uint256 _feeRate
    ) external {
        require(_paymentID < totalPaymentMethods, "Payment method does not exist.");
        address _seller = _msgSender();

        PaymentMethod storage pm = paymentMethods[_paymentID];
        pm.paymentAddress[_seller] = _paymentAddress;
        pm.feeRate[_seller] = _feeRate;

        emit SellerPaymentMethod(_seller, _paymentID, _paymentAddress, _feeRate);
    }

    function depositTokens(uint16 _paymentID, IERC20 _token, uint256 _amount) external {
        require(_paymentID < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentID];
        require(pm.tokenEnabled[_token] == true, "Token not enabled for selling");

        tokenBalance[_msgSender()][_token] += _amount;
        _token.safeTransferFrom(_msgSender(), address(this), _amount);

        emit SellerDeposit(_msgSender(), _token, _amount);
    }

    function withdrawTokens(IERC20 _token, uint256 _amount) external {
        require(
            tokenBalance[_msgSender()][_token] >= _amount, "Not enough balance to withdraw"
        );

        tokenBalance[_msgSender()][_token] -= _amount;

        _token.safeTransfer(_msgSender(), _amount);
        emit SellerWithdraw(_msgSender(), _token, _amount);
    }

    // ************************************* //
    // *           Order (Buyer)           * //
    // ************************************* //

    function buyOrder(uint16 _paymentID, address _seller, IERC20 _token, uint256 _amount)
        external
        payable
    {
        require(_paymentID < totalPaymentMethods, "PaymentMethod: !Exist");

        PaymentMethod storage pm = paymentMethods[_paymentID];
        require(bytes(pm.paymentAddress[_seller]).length != 0, "PaymentMethod: !Seller");
        require(pm.tokenEnabled[_token] == true, "PaymentMethod: !Token");
        require(tokenBalance[_seller][_token] >= _amount, "Not enough seller balance");

        require(msg.value >= arbitrator.arbitrationCost(arbitratorExtraData), "Arbitration fees not paid");

        (uint256 _fee,) = _calculateFees(_amount, tradeFeeRate);
        (uint256 _sellerFee,) = _calculateFees(_amount, pm.feeRate[_seller]);
        uint256 tradeAmount = _amount - _sellerFee;
        require(_amount >= _sellerFee + _fee, "Cummulative fees cannot be more than bought amount");

        orders.push(
            Order({
                paymentID: _paymentID,
                buyer: payable(_msgSender()),
                seller: payable(_seller),
                token: _token,
                amount: tradeAmount,
                fee: _fee,
                buyerCost: msg.value,
                sellerCost: 0,
                disputeID: 0,
                lastInteraction: block.timestamp,
                status: Status.Created
            })
        );

        // Lock seller funds.
        // We exclude the seller fee here itself
        // since it will remain with the seller
        // in any scenario.
        tokenBalance[_seller][_token] -= tradeAmount;

        emit OrderBuy(
            orders.length - 1,
            _msgSender(),
            _paymentID,
            _seller,
            _token,
            _amount,
            _fee,
            _sellerFee
            );
    }

    function confirmPaid(uint256 _orderID) external {
        require(_orderID < orders.length, "Invalid Order ID");

        Order storage order = orders[_orderID];
        require(order.buyer == _msgSender(), "Only Buyer");
        require(order.status == Status.Created, "OrderStatus: !Created");
        require(
            order.lastInteraction + buyerTimeout >= block.timestamp,
            "Payment confirmation timeout"
        );

        order.lastInteraction = block.timestamp;
        order.status = Status.Paid;

        emit OrderPaid(_orderID);
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
        require(_orderID < orders.length, "Invalid Order ID");

        Order storage order = orders[_orderID];
        require(order.seller == _msgSender(), "Only Seller");
        require(order.status < Status.Completed, "OrderStatus: !<Completed");
        // Lets the seller mark an order as complete before waiting for
        // the buyer to confirmPaid every time
        // since the buyer would make the payment off-chain,
        // the seller can counter-factually complete the order.
        require(
            order.status == Status.Created
                || order.lastInteraction + sellerTimeout >= block.timestamp,
            "Order completed by timeout"
        );

        _markOrderComplete(order);
        emit OrderComplete(_orderID);
    }

    function disputeOrder(uint256 _orderID) external payable {
        require(_orderID < orders.length, "Invalid Order ID");

        Order storage order = orders[_orderID];
        require(order.seller == _msgSender(), "Only Seller");
        require(order.status == Status.Paid, "OrderStatus: !Paid");
        require(
            order.lastInteraction + sellerTimeout >= block.timestamp,
            "Order completed by timeout"
        );

        // Seller can overpay to draw more jurors.
        require(msg.value >= arbitrator.arbitrationCost(arbitratorExtraData), "Arbitration fees not paid");

        order.sellerCost = msg.value;
        order.status = Status.Disputed;
        order.disputeID = arbitrator.createDispute{value: msg.value}(
            RULING_OPTIONS, arbitratorExtraData
        );

        DisputeData storage dispute = disputes[order.disputeID];
        dispute.orderID = _orderID;
        dispute.arbitrator = arbitrator;
        dispute.extraData = arbitratorExtraData;

        PaymentMethod storage pm = paymentMethods[order.paymentID];

        emit Dispute(arbitrator, order.disputeID, pm.metaEvidenceID, _orderID);
    }

    // ************************************* //
    // *             Anonymous             * //
    // ************************************* //

    function timeoutByBuyer(uint256 _orderID) external {
        require(_orderID < orders.length, "Invalid Order ID");

        Order storage order = orders[_orderID];
        require(order.status == Status.Created, "OrderStatus: !Created");
        require(
            order.lastInteraction + buyerTimeout < block.timestamp,
            "Confirmation period NOT timedout"
        );

        uint256 amount = order.amount;
        uint256 buyerCost = order.buyerCost;

        order.amount = 0;
        order.fee = 0;
        order.buyerCost = 0;
        order.status = Status.Cancelled;

        // Unlock seller funds
        tokenBalance[order.seller][order.token] += amount;

        // Return the buyers arbitration fees.
        order.buyer.send(buyerCost);
        emit TimedOutByBuyer(_orderID);
    }

    function timeoutBySeller(uint256 _orderID) external {
        require(_orderID < orders.length, "Invalid Order ID");

        Order storage order = orders[_orderID];
        require(order.status == Status.Paid, "OrderStatus: !Paid");
        require(
            order.lastInteraction + sellerTimeout < block.timestamp,
            "Completion period NOT timedout"
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
     * @param _orderID The index of the order. Used as EvidenceGroupID
     * @param _evidence A link to an evidence using its URI.
     */
    function submitEvidence(uint256 _orderID, string calldata _evidence) external {
        require(_orderID < orders.length, "Invalid Order ID");

        Order memory order = orders[_orderID];
        require(order.status < Status.Resolved, "Dispute is resolved");
        DisputeData storage dispute = disputes[order.disputeID];

        emit Evidence(dispute.arbitrator, _orderID, _msgSender(), _evidence);
    }

    /**
     * @dev Takes up to the total amount required to fund a side of an appeal.
     * Reimburses the rest. Creates an appeal if both sides are fully funded.
     * @param _orderID The ID of the buy order.
     * @param _side The party that pays the appeal fee.
     */
    function fundAppeal(uint256 _orderID, Party _side) external payable {
        require(_orderID < orders.length, "Invalid Order ID");

        Order storage order = orders[_orderID];
        require(_side != Party.None, "Wrong party.");
        require(order.status == Status.Disputed, "No dispute to appeal");

        DisputeData storage dispute = disputes[order.disputeID];
        (uint256 appealPeriodStart, uint256 appealPeriodEnd) =
            dispute.arbitrator.appealPeriod(order.disputeID);
        require(
            block.timestamp >= appealPeriodStart && block.timestamp < appealPeriodEnd,
            "Funding not within appeal period"
        );

        uint256 multiplier;
        uint256 winner = dispute.arbitrator.currentRuling(order.disputeID);
        if (winner == uint256(_side)) {
            multiplier = winnerStakeMultiplier;
        } else if (winner == 0) {
            multiplier = sharedStakeMultiplier;
        } else {
            require(
                block.timestamp < (appealPeriodEnd + appealPeriodStart) / 2,
                "The loser must pay during the first half of the appeal period"
            );
            multiplier = loserStakeMultiplier;
        }

        Round storage round = dispute.rounds[dispute.lastRoundID];
        require(_side != round.sideFunded, "Appeal fee paid");

        uint256 appealCost = dispute.arbitrator.appealCost(order.disputeID, dispute.extraData);
        uint256 totalCost = appealCost + ((appealCost * multiplier) / MULTIPLIER_DIVISOR);

        {
            // Take up to the amount necessary to fund the current round at the current costs.
            uint256 contribution; // Amount contributed.
            uint256 remainingETH; // Remaining ETH to send back.
            (contribution, remainingETH) = _calculateContribution(
                msg.value, totalCost - round.paidFees[uint256(_side)]
            );
            round.contributions[_msgSender()][uint256(_side)] += contribution;
            round.paidFees[uint256(_side)] += contribution;

            emit AppealContribution(_orderID, _side, _msgSender(), contribution);

            // Reimburse leftover ETH if any.
            // Deliberate use of send in order to not block the contract in case of reverting fallback.
            if (remainingETH > 0) {
                payable(_msgSender()).send(remainingETH);
            }
        }

        if (round.paidFees[uint256(_side)] >= totalCost) {
            if (round.sideFunded == Party.None) {
                round.sideFunded = _side;
            } else {
                // Both sides are fully funded. Create an appeal.
                dispute.arbitrator.appeal{value: appealCost}(
                    order.disputeID, dispute.extraData
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

        require(msg.sender == address(dispute.arbitrator), "Only arbitrator");
        require(dispute.resolved != true, " Dispute already resolved");

        Round storage round = dispute.rounds[dispute.lastRoundID];

        // If only one side paid its fees we assume the ruling to be in its favor.
        if (round.sideFunded == Party.Buyer) {
            dispute.ruling = Party.Buyer;
        } else if (round.sideFunded == Party.Seller) {
            dispute.ruling = Party.Seller;
        } else {
            dispute.ruling = Party(_ruling);
        }
        dispute.resolved = true;

        _executeRuling(_disputeID);

        emit Ruling(dispute.arbitrator, _disputeID, uint256(dispute.ruling));
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
    ) external {
        Order storage order = orders[_orderID];
        require(order.status == Status.Resolved, "The order must be resolved.");
        DisputeData storage dispute = disputes[order.disputeID];
        require(dispute.orderID == _orderID, "Undisputed order");

        uint256 reward = _withdrawFeesAndRewards(
            _beneficiary, _orderID, _round, uint256(dispute.ruling)
        );
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
    ) external {
        Order storage order = orders[_orderID];
        require(order.status == Status.Resolved, "The order must be resolved.");
        DisputeData storage dispute = disputes[order.disputeID];
        require(dispute.orderID == _orderID, "Undisputed order");
        uint256 finalRuling = uint256(dispute.ruling);

        uint256 reward;
        uint256 totalRounds = dispute.lastRoundID;
        for (
            uint256 i = _cursor;
            i <= totalRounds && (_count == 0 || i < _cursor + _count);
            i++
        ) {
            reward += _withdrawFeesAndRewards(_beneficiary, _orderID, i, finalRuling);
        }
        _beneficiary.send(reward); // It is the user responsibility to accept ETH.
    }

    // ************************************* //
    // *              Views                * //
    // ************************************* //

    function getOrderFeeAmount(uint256 _orderID)
        external
        view
        returns (uint256 fee, uint256 tradeAmount)
    {
        Order storage order = orders[_orderID];
        fee = order.fee;
        tradeAmount = order.amount - fee;
    }

    function getPaymentMethodSellerFeeRate(uint16 _paymentID, address _seller)
        external
        view
        returns (uint256 fee)
    {
        PaymentMethod storage pm = paymentMethods[_paymentID];
        fee = pm.feeRate[_seller];
    }

    function getPaymentMethodAddress(uint16 _paymentID, address _seller)
        external
        view
        returns (string memory)
    {
        PaymentMethod storage pm = paymentMethods[_paymentID];
        return pm.paymentAddress[_seller];
    }

    function getPaymentMethodToken(uint16 _paymentID, IERC20 _token)
        external
        view
        returns (bool)
    {
        PaymentMethod storage pm = paymentMethods[_paymentID];
        return pm.tokenEnabled[_token];
    }

    function getCountOrders() external view returns (uint256) {
        return orders.length;
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
            round.paidFees,
            round.sideFunded,
            round.feeRewards,
            _round != dispute.lastRoundID
        );
    }

    // ************************************* //
    // *            Internal               * //
    // ************************************* //

    function _onlyAdmin() private view {
        require(admin == msg.sender, "Only Admin");
    }

    /**
     * @dev Gets the trade amount after fees
     * @return fee The fee amount according to the tradeFee rate.
     * @return tradeAmount The amount minus fees to be transferred to the buyer.
     */
    function _calculateFees(uint256 _amount, uint256 _feeRate)
        internal
        pure
        returns (uint256 fee, uint256 tradeAmount)
    {
        fee = _amount * _feeRate / MULTIPLIER_DIVISOR;
        tradeAmount = _amount - fee;
    }

    function _executeRuling(uint256 _disputeID) internal {
        DisputeData storage dispute = disputes[_disputeID];
        Order storage order = orders[dispute.orderID];

        uint256 amount = order.amount;
        uint256 fee = order.fee;
        uint256 buyerCost = order.buyerCost;
        uint256 sellerCost = order.sellerCost;
        uint256 tradeAmount = amount - fee;

        order.amount = 0;
        order.fee = 0;
        order.buyerCost = 0;
        order.sellerCost = 0;
        order.status = Status.Resolved;

        // Collect the fees.
        // We don't collect in case of inconclusive ruling
        if (dispute.ruling != Party.None) {
            protocolFeesSum += fee;
        }

        if (dispute.ruling == Party.Buyer) {
            order.buyer.send(buyerCost);
            // non-safe transfer used here to prevent blocking on revert
            order.token.transfer(order.buyer, tradeAmount);
        } else if (dispute.ruling == Party.Seller) {
            order.seller.send(sellerCost);
            tokenBalance[order.seller][order.token] += amount;
        } else {
            // `buyerCost` and `sellerCost` are equal to the arbitration cost.
            // We take the min of the two in case someone overpaid.
            uint256 splitArbitrationFee = Math.min(buyerCost, sellerCost) / 2;
            order.buyer.send(splitArbitrationFee);
            order.seller.send(splitArbitrationFee);

            uint256 splitAmount = amount / 2;
            // non-safe transfer used here to prevent blocking on revert
            order.token.transfer(order.buyer, splitAmount);
            tokenBalance[order.seller][order.token] += splitAmount;
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
    ) internal returns (uint256 reward) {
        Order storage order = orders[_orderID];
        DisputeData storage dispute = disputes[order.disputeID];
        Round storage round = dispute.rounds[_round];
        uint256[3] storage contributionTo = round.contributions[_beneficiary];
        uint256 lastRound = dispute.lastRoundID;

        if (_round == lastRound) {
            // Allow to reimburse if funding was unsuccessful.
            reward = contributionTo[uint256(Party.Buyer)]
                + contributionTo[uint256(Party.Seller)];
        } else if (_finalRuling == uint256(Party.None)) {
            // Reimburse unspent fees proportionally if there is no winner and loser.
            uint256 totalFeesPaid = round.paidFees[uint256(Party.Buyer)]
                + round.paidFees[uint256(Party.Seller)];
            uint256 totalBeneficiaryContributions = contributionTo[uint256(Party.Buyer)]
                + contributionTo[uint256(Party.Seller)];
            reward = totalFeesPaid > 0
                ? (totalBeneficiaryContributions * round.feeRewards) / totalFeesPaid
                : 0;
        } else {
            // Reward the winner.
            reward = round.paidFees[_finalRuling] > 0
                ? (contributionTo[_finalRuling] * round.feeRewards)
                    / round.paidFees[_finalRuling]
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
        uint256 fee = order.fee;
        uint256 buyerCost = order.buyerCost;
        uint256 tradeAmount = amount - fee;

        order.amount = 0;
        order.fee = 0;
        order.buyerCost = 0;
        order.status = Status.Completed;

        // Collect the fees
        protocolFeesSum += fee;

        // Return the buyers arbitration fees.
        order.buyer.send(buyerCost);

        // Actually close the order by
        // transferring the bought tokens
        order.token.safeTransfer(order.buyer, tradeAmount);
    }

    function _msgSender() internal view returns (address sender) {
        if (msg.sender == address(relay)) {
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
    }
}
