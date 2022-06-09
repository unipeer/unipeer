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
        mapping(address => bool) tokenWhitelist;
        // address[] tokens;
    }
    uint16 public numOfMethods;
    mapping(uint => PaymentMethod) paymentMethods;

    // paymentAddressBySellerPaymentId[seller][paymentId] = paymentAddress
    mapping(address => mapping(uint16 => string)) paymentAddressBySellerPaymentId;

    // tokenBalanceBySeller[seller][token] = balance
    mapping(address => mapping(address => uint256)) tokenBalanceBySeller;
    // tokenPaymentIdsBySeller[seller][token] = paymentMethodId
    mapping(address => mapping(address => uint16[])) tokenPaymentIdsBySeller;

    // sellerByTokenPaymentId[token][paymentId] = sellers;
    mapping(address => mapping(uint16 => address[])) sellerByTokenPaymentId;
    mapping(address => mapping(uint16 => uint256)) sellerByTokenPaymentIdCounter;

    event NewPaymentMethod(uint16 paymentId, string paymentName);
    event Deposit(address sender, address token, uint256 amount);
    event Withdraw(address sender, address token, uint256 amount);

    modifier onlyAdmin() {
        require(admin == msg.sender, "Access not allowed: Admin only.");
        _;
    }

    function addPaymentMethod(
        uint8 _metaEvidenceId,
        string calldata _paymentName,
        bytes calldata _extraData,
        address[] calldata _tokens
    ) public onlyAdmin {
        PaymentMethod storage pm = paymentMethods[numOfMethods++];
        pm.metaEvidenceId = _metaEvidenceId;
        pm.paymentName = _paymentName;
        pm.extraData = _extraData;
        for (uint256 i = 0; i < _tokens.length; i++) {
            pm.tokenWhitelist[_tokens[i]] = true;
        }

        emit NewPaymentMethod(numOfMethods - 1, _paymentName);
    }

    function updatePaymentToken(uint16 _paymentId, address[] calldata _tokens, bool[] memory _enabled) public onlyAdmin {
        require(_paymentId < numOfMethods);

        PaymentMethod storage pm = paymentMethods[_paymentId];
        for (uint256 i = 0; i < _tokens.length; i++) {
            pm.tokenWhitelist[_tokens[i]] = _enabled[i];
        }
    }

    function updatePaymentAddress(uint16 _paymentId, string calldata _paymentAddress) public {
        require(_paymentId < numOfMethods);

        paymentAddressBySellerPaymentId[msg.sender][_paymentId] = _paymentAddress;
    }

    function addSupportedPaymentId(address _token, uint16 _paymentId) public {
        tokenPaymentIdsBySeller[msg.sender][_token].push(_paymentId); // enum or bitmask?
    }

    function depositTokens(address _token, uint256 _amount) public {
        IERC20(_token).safeTransferFrom(msg.sender, address(this), _amount);

        tokenBalanceBySeller[msg.sender][_token] += _amount;
        emit Deposit(msg.sender, _token, _amount);
    }

    function withdrawTokens(address _token, uint256 _amount) public {
        require(tokenBalanceBySeller[msg.sender][_token] >= _amount, "Not enough balance to withdraw");

        IERC20(_token).safeTransfer(msg.sender, _amount);
        emit Withdraw(msg.sender, _token, _amount);
    }
}
