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

    // tokenBalance[token][seller] = balance
    mapping(address => mapping(address => uint256)) public tokenBalance;

    event NewPaymentMethod(uint16 paymentId, string paymentName);
    event SellerDeposit(address sender, address token, uint256 amount);
    event SellerWithdraw(address sender, address token, uint256 amount);

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

        emit NewPaymentMethod(numOfMethods - 1, _paymentName);
    }

    function updateMetaEvidenceId(uint16 _paymentId, uint8 _metaEvidenceId) external onlyAdmin {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.metaEvidenceId = _metaEvidenceId;
    }

    function updatePaymentName(uint16 _paymentId, string calldata _paymentName) external onlyAdmin {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.paymentName = _paymentName;
    }

    function updateExtraData(uint16 _paymentId, bytes calldata _extraData) external onlyAdmin {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.extraData = _extraData;
    }

    function updateTokenEnabled(uint16 _paymentId, address _token, bool _enabled) external onlyAdmin {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.tokenEnabled[_token] = _enabled;
    }

    /******** Mutating functions *******/

    function addSupportedPaymentMethod(uint16 _paymentId, string calldata _paymentAddress) external {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.paymentAddress[msg.sender] = _paymentAddress;
    }

    function disablePaymentMethod(uint16 _paymentId) external {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.metaEvidenceId != 0, "Invalid Payment Id");

        pm.paymentAddress[msg.sender] = "";
    }

    function depositTokens(uint16 _paymentId, address _token, uint256 _amount) external {
        PaymentMethod storage pm = paymentMethods[_paymentId];
        require(pm.tokenEnabled[_token] == true, "Token not yet enabled for selling");

        tokenBalance[_token][msg.sender] += _amount;

        IERC20(_token).safeTransferFrom(msg.sender, address(this), _amount);
        emit SellerDeposit(msg.sender, _token, _amount);
    }

    function withdrawTokens(address _token, uint256 _amount) external {
        require(tokenBalance[_token][msg.sender] >= _amount, "Not enough balance to withdraw");

        tokenBalance[_token][msg.sender] -= _amount;

        IERC20(_token).safeTransfer(msg.sender, _amount);
        emit SellerWithdraw(msg.sender, _token, _amount);
    }
}
