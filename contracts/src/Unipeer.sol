// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.10;

contract Unipeer {

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
    mapping(address => mapping(uint16 => uint256)) indexCounter;

    event NewPaymentMethod(uint16 methodId, string paymentName);

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
        for (uint256 i = 0 ; i < _tokens.length; i++) {
            pm.tokenWhitelist[_tokens[i]] = true;
        }

        emit NewPaymentMethod(numOfMethods - 1, _paymentName);
    }

    function addPaymentToken(uint16 paymentId, address token) public {

    }

    function updatePaymentAddress(uint16 paymentId, string calldata paymentAddress) public {
        require(paymentId < numOfMethods);
    }
}
