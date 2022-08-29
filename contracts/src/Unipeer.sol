// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.10;

import "oz/token/ERC20/utils/SafeERC20.sol";
import "kleros/IArbitrable.sol";
import "kleros/erc-1497/IEvidence.sol";
import "kleros/IArbitrator.sol";

contract Unipeer is IArbitrable, IEvidence {
    using SafeERC20 for IERC20;

    /* Enums */

    enum Party {
        None, // Party per default when there is no challenger or requester. Also used for unconclusive ruling.
        Buyer, // Party that placed the buy order.
        Seller// Party that provided funds that were locked in a order.
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
        bool disputed;
        bool resolved;
        address buyer;
        address seller;
        IERC20 token;
        uint256 amount;
        // If dispute exists, the ID of the dispute.
        uint256 disputeID;
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
        uint disputeID; // The ID of the dispute related to the challenge.
        Party ruling; // Ruling given by the arbitrator of the dispute.
        uint16 lastRoundID; // The ID of the last round.
        mapping(uint => Round) rounds; // Tracks the info of each funding round of the challenge.
    }

    /* Storage */

    address public admin;

    uint16 public totalPaymentMethods;
    mapping(uint16 => PaymentMethod) public paymentMethods;

    // Stores the arbitrator data of the contract.
    // Updated each time the data is changed.
    ArbitratorData[] public arbitratorDataList;
    // Holds the total/count of Meta Evidence updates.
    uint256 metaEvidenceUpdates;

    // tokenBalance[seller][token] = balance
    mapping(address => mapping(IERC20 => uint256)) public tokenBalance;

    /* Modifiers */

    modifier onlyAdmin() {
        require(admin == msg.sender, "Access not allowed: Admin only.");
        _;
    }

    /* Events */

    event PaymentMethodUpdate(
        uint16 paymentId,
        string paymentName,
        uint256 metaEvidenceID
    );
    event SellerPaymentMethod(address sender, uint16 paymentId, string paymentAddress);
    event SellerPaymentDisabled(address sender, uint16 paymentId);
    event SellerDeposit(address sender, IERC20 token, uint256 amount);
    event SellerWithdraw(address sender, IERC20 token, uint256 amount);
    event BuyOrder(address buyer, uint16 paymentId, address seller, IERC20 token, uint256 amount);
    event OrderDispute();
    event OrderComplete();

    constructor(address _admin, IArbitrator _arbitrator, bytes memory _arbitratorExtraData) {
        admin = _admin;
        arbitratorDataList.push(ArbitratorData({
            arbitrator: _arbitrator,
            arbitratorExtraData: _arbitratorExtraData
        }));
    }

    /***** Admin Only functions *****/

    /** @dev Change the arbitrator to be used for disputes.
     *  The arbitrator is trusted to support appeal period and not reenter.
     *  @param _arbitrator The new trusted arbitrator to be used in the next requests.
     *  @param _arbitratorExtraData The extra data used by the new arbitrator.
     */
    function changeArbitrator(IArbitrator _arbitrator, bytes calldata _arbitratorExtraData) external onlyAdmin {
        arbitratorDataList.push(ArbitratorData({
            arbitrator: _arbitrator,
            arbitratorExtraData: _arbitratorExtraData
        }));
    }

    /** @dev Add a new meta evidence used for disputes.
     *  @param _metaEvidence The meta evidence to be used for future disputes
     *  requests.
     *  returns metaEvidenceID The ID of the associated meta evidence that
     *  can be then linked to a payment method.
     */
    function addMetaEvidence(string calldata _metaEvidence) external onlyAdmin returns (uint256 metaEvidenceID) {
        metaEvidenceID = metaEvidenceUpdates + 1;
        emit MetaEvidence(metaEvidenceID, _metaEvidence);
    }

    function addPaymentMethod(
        string calldata _paymentName,
        uint8 _metaEvidenceID,
        IERC20 _initalEnabledToken
    ) external onlyAdmin {
        require(_metaEvidenceID <= metaEvidenceUpdates, "Invalid Meta Evidence ID");
        PaymentMethod storage pm = paymentMethods[totalPaymentMethods++];
        pm.paymentName = _paymentName;
        pm.metaEvidenceID = _metaEvidenceID;
        pm.tokenEnabled[_initalEnabledToken] = true;

        // emit MetaEvidence(0, _metaEvidence);
        emit PaymentMethodUpdate(totalPaymentMethods - 1, _paymentName, _metaEvidenceID);
    }

    function updateMetaEvidenceID(uint16 _paymentId, uint8 _metaEvidenceID) external onlyAdmin {
        require(_paymentId < totalPaymentMethods, "Payment method does not exist.");
        require(_metaEvidenceID <= metaEvidenceUpdates, "Invalid Meta Evidence ID");

        PaymentMethod storage pm = paymentMethods[_paymentId];
        pm.metaEvidenceID = _metaEvidenceID;

        emit PaymentMethodUpdate(_paymentId, pm.paymentName, pm.metaEvidenceID);
    }

    function updatePaymentName(uint16 _paymentId, string calldata _paymentName) external onlyAdmin {
        require(_paymentId < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentId];
        pm.paymentName = _paymentName;

        emit PaymentMethodUpdate(_paymentId, pm.paymentName, pm.metaEvidenceID);
    }

    function updateTokenEnabled(uint16 _paymentId, IERC20 _token, bool _enabled) external onlyAdmin {
        require(_paymentId < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentId];
        pm.tokenEnabled[_token] = _enabled;
    }

    /******** Mutating functions *******/
    /********       Seller       *******/

    function acceptPaymentMethod(uint16 _paymentId, string calldata _paymentAddress) external {
        require(_paymentId < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentId];
        pm.paymentAddress[msg.sender] = _paymentAddress;

        emit SellerPaymentMethod(msg.sender, _paymentId, _paymentAddress);
    }

    function disablePaymentMethod(uint16 _paymentId) external {
        require(_paymentId < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentId];
        pm.paymentAddress[msg.sender] = "";

        emit SellerPaymentDisabled(msg.sender, _paymentId);
    }

    function depositTokens(uint16 _paymentId, IERC20 _token, uint256 _amount) external {
        require(_paymentId < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.tokenEnabled[_token] == true, "Token not yet enabled for selling");

        tokenBalance[msg.sender][_token] += _amount;
        _token.safeTransferFrom(msg.sender, address(this), _amount);

        emit SellerDeposit(msg.sender, _token, _amount);
    }

    function withdrawTokens(IERC20 _token, uint256 _amount) external {
        require(tokenBalance[msg.sender][_token] >= _amount, "Not enough balance to withdraw");

        tokenBalance[msg.sender][_token] -= _amount;

        _token.safeTransfer(msg.sender, _amount);
        emit SellerWithdraw(msg.sender, _token, _amount);
    }

    /********       Buyer       *******/

    function placeOrder(uint16 _paymentId, address _seller, IERC20 _token, uint256 _amount) external {
        require(_paymentId < totalPaymentMethods, "Payment method does not exist.");

        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(bytes(pm.paymentAddress[_seller]).length != 0, "Seller doesn't accept this payment method");
        require(pm.tokenEnabled[_token] == true, "Token is not enabled for this payment method");
        require(tokenBalance[_seller][_token] >= _amount, "Not enough seller balance");

        // TODO: create arbitration request and lock seller funds.
        emit BuyOrder(msg.sender, _paymentId, _seller, _token, _amount);
    }

    function completeOrder() external {
    }

    /********       Arbitrator       *******/

    /** @dev Give a ruling for a dispute. Can only be called by the arbitrator. TRUSTED.
     *  Account for the situation where the winner loses a case due to paying less appeal fees than expected.
     *  @param _disputeID ID of the dispute in the arbitrator contract.
     *  @param _ruling Ruling given by the arbitrator. Note that 0 is reserved for "Refused to arbitrate".
     */
    function rule(uint _disputeID, uint _ruling) public {
    }

    /******** View functions *******/
}
