// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.10;

import "oz/token/ERC20/utils/SafeERC20.sol";

contract Unipeer {
    using SafeERC20 for IERC20;

    address public admin;

    struct PaymentMethod {
        uint8 metaEvidenceId;
        string paymentName;
        bytes extraData; // subcourtId + minJurors;

        mapping(address => bool) tokenEnabled;
        mapping(address => string) paymentAddress;
    }

    uint16 public numOfMethods;
    mapping(uint16 => PaymentMethod) public paymentMethods;

    // tokenBalance[seller][token] = balance
    mapping(address => mapping(address => uint256)) public tokenBalance;

    event PaymentMethodUpdate(
        uint16 paymentId,
        uint8 metaEvidenceId,
        string paymentName,
        bytes extraData
    );
    event SellerPaymentMethod(address sender, uint16 paymentId, string paymentAddress);
    event SellerPaymentDisabled(address sender, uint16 paymentId);
    event SellerDeposit(address sender, address token, uint256 amount);
    event SellerWithdraw(address sender, address token, uint256 amount);
    event BuyOrder(address buyer, uint16 paymentId, address seller, address token, uint256 amount);
    event OrderDispute();
    event OrderComplete();

    modifier onlyAdmin() {
        require(admin == msg.sender, "Access not allowed: Admin only.");
        _;
    }

    constructor(address _admin) {
        admin = _admin;
    }

    /***** Admin Only functions *****/

    function addPaymentMethod(
        uint8 _metaEvidenceId,
        string calldata _paymentName,
        bytes calldata _extraData
    ) external onlyAdmin {
        PaymentMethod storage pm = paymentMethods[numOfMethods++];
        pm.metaEvidenceId = _metaEvidenceId;
        pm.paymentName = _paymentName;
        pm.extraData = _extraData;

        emit PaymentMethodUpdate(numOfMethods - 1, _metaEvidenceId, _paymentName, _extraData);
    }

    function updateMetaEvidenceId(uint16 _paymentId, uint8 _metaEvidenceId) external onlyAdmin {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.metaEvidenceId = _metaEvidenceId;
        emit PaymentMethodUpdate(_paymentId, pm.metaEvidenceId, pm.paymentName, pm.extraData);
    }

    function updatePaymentName(uint16 _paymentId, string calldata _paymentName) external onlyAdmin {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.paymentName = _paymentName;
        emit PaymentMethodUpdate(_paymentId, pm.metaEvidenceId, pm.paymentName, pm.extraData);
    }

    function updateExtraData(uint16 _paymentId, bytes calldata _extraData) external onlyAdmin {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.extraData = _extraData;
        emit PaymentMethodUpdate(_paymentId, pm.metaEvidenceId, pm.paymentName, pm.extraData);
    }

    function updateTokenEnabled(uint16 _paymentId, address _token, bool _enabled) external onlyAdmin {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.tokenEnabled[_token] = _enabled;
    }

    /******** Mutating functions *******/
    /********       Seller       *******/

    function addSupportedPaymentMethod(uint16 _paymentId, string calldata _paymentAddress) external {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.paymentAddress[msg.sender] = _paymentAddress;
        emit SellerPaymentMethod(msg.sender, _paymentId, _paymentAddress);
    }

    function disablePaymentMethod(uint16 _paymentId) external {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.paymentAddress[msg.sender] = "";
        emit SellerPaymentDisabled(msg.sender, _paymentId);
    }

    function depositTokens(uint16 _paymentId, address _token, uint256 _amount) external {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.tokenEnabled[_token] == true, "Token not yet enabled for selling");

        tokenBalance[msg.sender][_token] += _amount;

        IERC20(_token).safeTransferFrom(msg.sender, address(this), _amount);
        emit SellerDeposit(msg.sender, _token, _amount);
    }

    function withdrawTokens(address _token, uint256 _amount) external {
        require(tokenBalance[msg.sender][_token] >= _amount, "Not enough balance to withdraw");

        tokenBalance[msg.sender][_token] -= _amount;

        IERC20(_token).safeTransfer(msg.sender, _amount);
        emit SellerWithdraw(msg.sender, _token, _amount);
    }

    /********       Buyer       *******/

    function placeOrder(uint16 _paymentId, address _seller, address _token, uint256 _amount) external {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(bytes(pm.paymentAddress[_seller]).length != 0, "Seller doesn't accept this payment method");
        require(pm.tokenEnabled[_token] == true, "Token is not enabled for this payment method");
        require(tokenBalance[_seller][_token] >= _amount, "Not enough seller balance");


        // TODO: create arbitration request and lock seller funds.
        emit BuyOrder(msg.sender, _paymentId, _seller, _token, _amount);
    }

    function completeOrder() external {
    }

    /******** View functions *******/

    function totalPaymentMethods() external view returns (uint16){
        return numOfMethods;
    }
}
