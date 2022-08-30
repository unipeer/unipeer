// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.10;

import "oz/token/ERC20/utils/SafeERC20.sol";
import "kleros/IArbitrable.sol";
import "kleros/erc-1497/IEvidence.sol";
import "kleros/IArbitrator.sol";

contract Unipeer is IArbitrable, IEvidence {
    using SafeERC20 for IERC20;

    /* Constants and immutable */

    uint256 private constant RULING_OPTIONS = 2; // The amount of non 0 choices the arbitrator can give.
    uint256 private constant MULTIPLIER_DIVISOR = 10000; // Divisor parameter for multipliers.

    /* Enums */

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

    /* Structs */

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
        address buyer;
        address seller;
        IERC20 token;
        uint256 amount;
        // If dispute exists, the ID of the dispute.
        uint256 disputeID;
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

    struct DisputeDetails {
        uint256 disputeID; // The ID of the dispute related to the challenge.
        Party ruling; // Ruling given by the arbitrator of the dispute.
        uint16 lastRoundID; // The ID of the last round.
        mapping(uint256 => Round) rounds; // Tracks the info of each funding round of the challenge.
    }

    /* Storage */

    address public admin;

    uint16 public totalPaymentMethods;
    // List of Payment Methods by paymentID
    mapping(uint16 => PaymentMethod) public paymentMethods;

    // Stores the arbitrator data of the contract.
    // Updated each time the data is changed.
    ArbitratorData[] public arbitratorDataList;
    // Holds the total/count of Meta Evidence updates.
    uint256 metaEvidenceUpdates;
    uint256 confirmationTimeout = 1 days;

    // tokenBalance[seller][token] = balance
    mapping(address => mapping(IERC20 => uint256)) public tokenBalance;
    // List of dispute details by disputeID
    mapping(uint256 => DisputeDetails) public disputes;
    // List of orders by orderID
    Order[] public orders;

    /* Modifiers */

    modifier onlyAdmin() {
        require(admin == msg.sender, "Access not allowed: Admin only.");
        _;
    }

    /* Events */

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
        uint256 amount
    );
    event Paid(uint256 orderID);
    event OrderComplete(uint256 orderID);

    constructor(
        address _admin,
        IArbitrator _arbitrator,
        bytes memory _arbitratorExtraData
    ) {
        admin = _admin;
        arbitratorDataList.push(
            ArbitratorData({arbitrator: _arbitrator, arbitratorExtraData: _arbitratorExtraData})
        );
    }

    /**
     ************** Admin Only functions *************
     */

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

    /**
     ************  Mutating functions  **************
     */
    /**
     ************      Seller          **************
     */

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

    /**
     ************      Buyer          **************
     */

    function buyOrder(uint16 _paymentID, address _seller, IERC20 _token, uint256 _amount)
        external
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

        orders.push(
            Order({
                buyer: msg.sender,
                seller: _seller,
                token: _token,
                amount: _amount,
                disputeID: 0,
                lastInteraction: block.timestamp,
                status: Status.Created
            })
        );
        tokenBalance[_seller][_token] -= _amount;

        emit BuyOrder(orders.length - 1, msg.sender, _paymentID, _seller, _token, _amount);
    }

    function confirmPaid(uint256 _orderID) external {
        Order storage order = orders[_orderID];
        require(order.buyer == msg.sender, "Only buyer can confirm the off-chain payment");
        require(
            order.status == Status.Created, "Order already cancelled, completed or disputed"
        );
        require(
            order.lastInteraction + confirmationTimeout >= block.timestamp,
            "Order confirmation period is over"
        );

        order.lastInteraction = block.timestamp;
        order.status = Status.Paid;

        emit Paid(_orderID);
    }

    function completeOrder(uint256 _orderID) external {
        Order storage order = orders[_orderID];
        require(order.seller == msg.sender, "Only seller mark an order complete");
        require(
            order.status < Status.Completed, "Order already cancelled, completed or disputed"
        );

        order.lastInteraction = block.timestamp;
        order.status = Status.Completed;

        order.token.safeTransfer(order.buyer, order.amount);

        emit OrderComplete(_orderID);
    }

    function cancelOrder(uint256 _orderID) external {
        Order storage order = orders[_orderID];
        require(
            order.status != Status.Created,
            "Order can only be cancelled immediately after creation"
        );
        require(
            order.lastInteraction + confirmationTimeout < block.timestamp,
            "Confirmation period has not yet timed out"
        );

        order.status = Status.Cancelled;
        tokenBalance[order.seller][order.token] += order.amount;
    }

    /**
     * **********   Arbitrator       **************
     */

    /**
     * @dev Give a ruling for a dispute. Can only be called by the arbitrator. TRUSTED.
     * Account for the situation where the winner loses a case due to paying less appeal fees than expected.
     * @param _disputeID ID of the dispute in the arbitrator contract.
     * @param _ruling Ruling given by the arbitrator. Note that 0 is reserved for "Refused to arbitrate".
     */
    function rule(uint256 _disputeID, uint256 _ruling) external {}

    /**
     ************   View functions   **************
     */

    function numOfOrders() external view returns (uint256) {
        return orders.length;
    }
}
