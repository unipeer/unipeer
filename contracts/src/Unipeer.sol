// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.10;

contract Unipeer {

    struct PaymentMethod {
        string paymentName;
        bytes extraData; // subcourtId + minJurors;
        uint8 metaEvidenceId;
        mapping(address => bool) tokenWhitelist;
    }
    PaymentMethod[] public paymentMethods;

    // paymentAddressBySellerPaymentId[seller][paymentId] = paymentAddress
    mapping(address => mapping(uint16 => string)) paymentAddressBySellerPaymentId;

    // tokenBalanceBySeller[seller][token] = balance
    mapping(address => mapping(address => uint256)) tokenBalanceBySeller;
    // tokenPaymentIdsBySeller[seller][token] = paymentMethodId
    mapping(address => mapping(address => uint16[])) tokenPaymentIdsBySeller;

    // sellerByTokenPaymentId[token][paymentId] = sellers;
    mapping(address => mapping(uint16 => address[])) sellerByTokenPaymentId;
    mapping(address => mapping(uint16 => uint256)) indexCounter;

}
