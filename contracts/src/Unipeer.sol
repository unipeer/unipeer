// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.10;

import "oz/token/ERC20/utils/SafeERC20.sol";

contract Unipeer {
    using SafeERC20 for IERC20;

    address public admin;
    mapping(address => bool) public tokenWhitelist;

    struct PaymentMethod {
        uint8 metaEvidenceId;
        string paymentName;
        bytes extraData; // subcourtId + minJurors;
    }
    uint16 public numOfMethods;
    mapping(uint16 => PaymentMethod) public paymentMethods;

    // supportedPaymentIdForToken[seller][paymentId][token]
    mapping(address => mapping(uint16 => mapping(address => bool))) supportedPaymentIdForToken;
    // paymentAddress[seller][paymentId] = paymentAddress
    mapping(address => mapping(uint16 => string)) paymentAddress;

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

    function whitelistToken(address _token, bool _allow) external onlyAdmin {
        tokenWhitelist[_token] = _allow;
    }

    /******** Mutating functions *******/

    function addSupportedPaymentMethod(address _token, uint16 _paymentId, string calldata _paymentAddress) external {
        supportedPaymentIdForToken[msg.sender][_paymentId][_token] = true;
        paymentAddress[msg.sender][_paymentId] = _paymentAddress;
    }

    function disablePaymentMethod(address _token, uint16 _paymentId) external {
        supportedPaymentIdForToken[msg.sender][_paymentId][_token] = false;
    }

    function depositTokens(address _token, uint256 _amount) external {
        require(tokenWhitelist[_token] == true, "Token not yet enabled for selling");
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
