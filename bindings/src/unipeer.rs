pub use unipeer::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod unipeer {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "Unipeer was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNIPEER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_version\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"contract IArbitrator\",\"name\":\"_arbitrator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_arbitratorExtraData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_confirmTimeout\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_orderTimeout\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_sharedStakeMultiplier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_winnerStakeMultiplier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_loserStakeMultiplier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_tradeFees\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"enum Unipeer.Party\",\"name\":\"_party\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"_contributor\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AppealContribution\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"orderID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"buyer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"paymentID\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"seller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IERC20\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"feeAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BuyOrder\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IArbitrator\",\"name\":\"_arbitrator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_metaEvidenceID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"_evidenceGroupID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Dispute\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IArbitrator\",\"name\":\"_arbitrator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_evidenceGroupID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"_party\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"_evidence\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Evidence\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeeWithdrawn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"enum Unipeer.Party\",\"name\":\"_party\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"HasPaidAppealFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_metaEvidenceID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"_evidence\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MetaEvidence\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"orderID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OrderComplete\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"orderID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OrderResolved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"orderID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paid\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"paymentID\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"paymentName\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"metaEvidenceID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentMethodUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IArbitrator\",\"name\":\"_arbitrator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_ruling\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Ruling\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IERC20\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SellerDeposit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"paymentID\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SellerPaymentDisabled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint16\",\"name\":\"paymentID\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"paymentAddress\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SellerPaymentMethod\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IERC20\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SellerWithdraw\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"orderID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TimedOutByBuyer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"orderID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TimedOutBySeller\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct Caveat[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_CAVEAT_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Caveat\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_CAVEAT_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Delegation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_DELEGATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocations\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATIONS_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATION_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ReplayProtection\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_REPLAYPROTECTION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_SIGNEDDELEGATION_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_SIGNEDDELEGATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Transaction\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_TRANSACTION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_paymentID\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_paymentAddress\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptPaymentMethod\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_metaEvidence\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addMetaEvidence\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"metaEvidenceID\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_paymentName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_metaEvidenceID\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"_initalEnabledToken\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPaymentMethod\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_beneficiary\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"amountWithdrawable\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"total\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"arbitratorDataList\",\"outputs\":[{\"internalType\":\"contract IArbitrator\",\"name\":\"arbitrator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"arbitratorExtraData\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"_beneficiary\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_cursor\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_count\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"batchRoundWithdraw\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_paymentID\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_seller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"buyOrder\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"buyQuoteWithFees\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tradeAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calculateFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IArbitrator\",\"name\":\"_arbitrator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_arbitratorExtraData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeArbitrator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_timeout\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeConfirmTimeout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_fees\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_timeout\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeOrderTimeout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"completeOrder\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"confirmPaid\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"confirmTimeout\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"contractInvoke\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_paymentID\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"depositTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_paymentID\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disablePaymentMethod\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"disputeOrder\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"disputes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"orderID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum Unipeer.Party\",\"name\":\"ruling\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"lastRoundID\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"domainHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"enforceCaveat\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum Unipeer.Party\",\"name\":\"_side\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"fundAppeal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_round\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_contributor\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getContributions\",\"outputs\":[{\"internalType\":\"uint256[3]\",\"name\":\"contributions\",\"type\":\"uint256[3]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCountOrders\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDelegationTypedDataHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"contractName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"version\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"chainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"verifyingContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getEIP712DomainHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFeeAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocations\",\"name\":\"invocations\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInvocationsTypedDataHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"intendedSender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNumberOfRounds\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_round\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoundInfo\",\"outputs\":[{\"internalType\":\"uint256[3]\",\"name\":\"paidFees\",\"type\":\"uint256[3]\",\"components\":[]},{\"internalType\":\"enum Unipeer.Party\",\"name\":\"sideFunded\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeRewards\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"appealed\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedInvocation[]\",\"name\":\"signedInvocations\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Invocations\",\"name\":\"invocations\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"invoke\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"loserStakeMultiplier\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"metaEvidenceUpdates\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"orderTimeout\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"orders\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"buyer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"seller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"buyerFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"sellerFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"disputeID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"arbitratorID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastInteraction\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum Unipeer.Status\",\"name\":\"status\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paymentMethods\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"paymentName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"metaEvidenceID\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"protocolFeesSum\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_ruling\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rule\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sharedStakeMultiplier\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_evidence\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitEvidence\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"timeoutByBuyer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"timeoutBySeller\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalPaymentMethods\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tradeFees\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_paymentID\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_metaEvidenceID\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePaymentMetaEvidence\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_paymentID\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_paymentName\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePaymentName\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_paymentID\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateTokenEnabled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation\",\"name\":\"signedDelegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifyDelegationSignature\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedInvocation\",\"name\":\"signedInvocation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocations\",\"name\":\"invocations\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifyInvocationSignature\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"winnerStakeMultiplier\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"_to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"_beneficiary\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_orderID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_round\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawFeesAndRewards\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawTokens\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static UNIPEER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6101006040523480156200001257600080fd5b506040516200769538038062007695833981016040819052620000359162000352565b6040805180820190915260078152662ab734b832b2b960c91b6020820152896200006862000062620001a3565b62000201565b8151602080840191909120825182840120604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818601528082019390935260608301919091524660808301523060a0808401919091528151808403909101815260c0909201905280519101206080525050604080518082019091526001600160a01b038981168252602082018981526007805460018101825560009190915283517fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688600290920291820180546001600160a01b0319169190941617835590517fa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c689909101906200017b9082620004b0565b505050600995909555600a9390935560a09190915260c05260e052600455506200057c915050565b6000303303620001fb57600080368080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152505050503601516001600160a01b03169150620001fe9050565b50335b90565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b634e487b7160e01b600052604160045260246000fd5b60006001600160401b038084111562000284576200028462000251565b604051601f8501601f19908116603f01168101908282118183101715620002af57620002af62000251565b81604052809350858152868686011115620002c957600080fd5b600092505b85831015620002ee578285015160208483010152602083019250620002ce565b8583111562000301576000602087830101525b5050509392505050565b80516001600160a01b03811681146200032357600080fd5b919050565b600082601f8301126200033a57600080fd5b6200034b8383516020850162000267565b9392505050565b60008060008060008060008060006101208a8c0312156200037257600080fd5b89516001600160401b03808211156200038a57600080fd5b818c0191508c601f8301126200039f57600080fd5b620003b08d83516020850162000267565b9a50620003c060208d016200030b565b995060408c0151915080821115620003d757600080fd5b50620003e68c828d0162000328565b97505060608a0151955060808a0151945060a08a0151935060c08a0151925060e08a015191506101008a015190509295985092959850929598565b600181811c908216806200043657607f821691505b6020821081036200045757634e487b7160e01b600052602260045260246000fd5b50919050565b601f821115620004ab57600081815260208120601f850160051c81016020861015620004865750805b601f850160051c820191505b81811015620004a75782815560010162000492565b5050505b505050565b81516001600160401b03811115620004cc57620004cc62000251565b620004e481620004dd845462000421565b846200045d565b602080601f8311600181146200051c5760008415620005035750858301515b600019600386901b1c1916600185901b178555620004a7565b600085815260208120601f198616915b828110156200054d578886015182559484019460019091019084016200052c565b50858210156200056c5787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b60805160a05160c05160e0516170bc620005d96000396000818161053501526118a201526000818161092d01526117c001526000818161063901526117f0015260008181610da001528181612b8b0152613b2201526170bc6000f3fe6080604052600436106104105760003560e01c80637b577b581161021e578063ab8024a611610123578063d327c1eb116100ab578063ec0e71ba1161007a578063ec0e71ba14610dc2578063f2fde38b14610df0578063f851a44014610e10578063fbef704514610e30578063fc6f8f1614610e4657600080fd5b8063d327c1eb14610cb7578063d5f3d85614610d4e578063dd2cc3f314610d6e578063dfe86ac514610d8e57600080fd5b8063b6adaaff116100f2578063b6adaaff14610c21578063ba20600414610c41578063ba7079ca14610c57578063c189fde614610c77578063caced6c514610c9757600080fd5b8063ab8024a614610bab578063abaa14ff14610bc1578063ae1a38ed14610be1578063b6694b7b14610c0157600080fd5b80639704122c116101a6578063a09970b211610175578063a09970b214610aed578063a1d7783014610b00578063a2e01f7514610b35578063a6a7f0eb14610b55578063a85c38ef14610b7557600080fd5b80639704122c14610a7857806397182ed614610a9857806398f1c16214610ab857806399a5d74714610acd57600080fd5b806389535803116101ed578063895358031461098f57806389c46caf146109d25780638a04499e146109f25780638a9bb02a14610a2a5780638da5cb5b14610a5a57600080fd5b80637b577b58146108fb5780637b9433831461091b57806386c61ce51461094f578063870026f11461096f57600080fd5b80634ffd43c3116103245780636c2b1253116102ac5780636f9658031161027b5780636f96580314610866578063715018a6146108865780637234eefe1461089b578063736f7ce7146108bb5780637a982eb6146108db57600080fd5b80636c2b1253146107f05780636cda375b146108105780636cdc090f146108305780636e99a23a1461084657600080fd5b80635c6d9f0c116102f35780635c6d9f0c146107505780635cf1b24a1461077057806360b6d7681461079057806364ac92f2146107b057806368c76ffd146107c357600080fd5b80634ffd43c3146106a95780635068de4c146106bf57806350a4f450146106df578063564a565d146106ff57600080fd5b80632f52a2fd116103a7578063382a2b8611610376578063382a2b86146105e75780633a48182114610607578063416583411461062757806346639cca1461065b5780634b2dd8f41461068957600080fd5b80632f52a2fd146105575780632fad7efc14610577578063311a6c56146105a7578063339ac67c146105c757600080fd5b80631049334f116103e35780631049334f146104b85780631165542b146104f057806312b3a2c0146105105780631d5120851461052357600080fd5b806301e6e5151461041557806306b091f91461043757806308aaf6b0146104575780630cd4384d1461048a575b600080fd5b34801561042157600080fd5b50610435610430366004615929565b610e66565b005b34801561044357600080fd5b50610435610452366004615957565b610fd3565b34801561046357600080fd5b50610477610472366004615adc565b611128565b6040519081526020015b60405180910390f35b34801561049657600080fd5b506104aa6104a5366004615b27565b6111a8565b604051610481929190615b9a565b3480156104c457600080fd5b506104776104d3366004615bbc565b600b60209081526000928352604080842090915290825290205481565b3480156104fc57600080fd5b5061047761050b366004615bf5565b61124c565b61043561051e366004615c1a565b61148a565b34801561052f57600080fd5b506104777f000000000000000000000000000000000000000000000000000000000000000081565b34801561056357600080fd5b50610477610572366004615d59565b611d20565b34801561058357600080fd5b50610597610592366004615dd1565b611d8b565b6040519015158152602001610481565b3480156105b357600080fd5b506104356105c2366004615e12565b611e52565b3480156105d357600080fd5b506104356105e2366004615e34565b6120ec565b3480156105f357600080fd5b50610435610602366004615929565b612283565b34801561061357600080fd5b50610477610622366004616089565b6123c4565b34801561063357600080fd5b506104777f000000000000000000000000000000000000000000000000000000000000000081565b34801561066757600080fd5b506005546106769061ffff1681565b60405161ffff9091168152602001610481565b34801561069557600080fd5b506104356106a43660046160bd565b61243b565b3480156106b557600080fd5b5061047760045481565b3480156106cb57600080fd5b506105976106da36600461613d565b6125ca565b3480156106eb57600080fd5b506104356106fa366004615929565b612b08565b34801561070b57600080fd5b5061074161071a366004615929565b600c602052600090815260409020805460019091015460ff811690610100900461ffff1683565b604051610481939291906161dc565b34801561075c57600080fd5b5061059761076b366004615dd1565b612b15565b34801561077c57600080fd5b5061047761078b366004616202565b612b29565b34801561079c57600080fd5b506104776107ab3660046162bf565b612b86565b6104356107be3660046162f3565b612bd6565b3480156107cf57600080fd5b506107e36107de366004616342565b613178565b604051610481919061639e565b3480156107fc57600080fd5b5061047761080b3660046163ac565b613213565b34801561081c57600080fd5b5061043561082b366004615929565b613270565b34801561083c57600080fd5b5061047760085481565b34801561085257600080fd5b506104356108613660046163e0565b6132ce565b34801561087257600080fd5b50610477610881366004616415565b61340e565b34801561089257600080fd5b5061043561345f565b3480156108a757600080fd5b506104776108b63660046162bf565b613473565b3480156108c757600080fd5b506104776108d6366004616431565b6134b1565b3480156108e757600080fd5b506104356108f6366004616476565b613519565b34801561090757600080fd5b506104776109163660046164dc565b61365c565b34801561092757600080fd5b506104777f000000000000000000000000000000000000000000000000000000000000000081565b34801561095b57600080fd5b5061043561096a366004616510565b6136c4565b34801561097b57600080fd5b5061043561098a366004615929565b6137a6565b34801561099b57600080fd5b506104776109aa366004615957565b6001600160a01b03919091166000908152600160209081526040808320938352929052205490565b3480156109de57600080fd5b506104356109ed366004615b27565b613939565b3480156109fe57600080fd5b50610a12610a0d3660046163ac565b613a00565b6040516001600160a01b039091168152602001610481565b348015610a3657600080fd5b50610a4a610a45366004615e12565b613a2a565b6040516104819493929190616543565b348015610a6657600080fd5b506000546001600160a01b0316610a12565b348015610a8457600080fd5b50610477610a93366004615929565b613aea565b348015610aa457600080fd5b50610477610ab3366004615d59565b613b1d565b348015610ac457600080fd5b50600d54610477565b348015610ad957600080fd5b50610477610ae8366004615929565b613b4a565b610435610afb366004615929565b613b5e565b348015610b0c57600080fd5b50610b20610b1b366004615929565b613f64565b60408051928352602083019190915201610481565b348015610b4157600080fd5b50610477610b50366004616575565b613f95565b348015610b6157600080fd5b50610435610b703660046165a9565b614001565b348015610b8157600080fd5b50610b95610b90366004615929565b614291565b6040516104819a999897969594939291906165f4565b348015610bb757600080fd5b5061047760095481565b348015610bcd57600080fd5b50610435610bdc366004616674565b614305565b348015610bed57600080fd5b50610435610bfc3660046166b2565b614375565b348015610c0d57600080fd5b50610477610c1c3660046166eb565b6143fd565b348015610c2d57600080fd5b50610435610c3c366004615929565b61445d565b348015610c4d57600080fd5b50610477600a5481565b348015610c6357600080fd5b50610435610c72366004616720565b6145cc565b348015610c8357600080fd5b50610435610c92366004615929565b61467e565b348015610ca357600080fd5b50610a12610cb236600461677f565b61468b565b348015610cc357600080fd5b50610477610cd23660046167b3565b8351602094850120835193850193909320604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8188015280820195909552606085019190915260808401929092526001600160a01b031660a0808401919091528151808403909101815260c09092019052805191012090565b348015610d5a57600080fd5b50610435610d693660046166b2565b6146b5565b348015610d7a57600080fd5b50610435610d89366004615bf5565b614765565b348015610d9a57600080fd5b506104777f000000000000000000000000000000000000000000000000000000000000000081565b348015610dce57600080fd5b50610de2610ddd366004615929565b6147ff565b604051610481929190616827565b348015610dfc57600080fd5b50610435610e0b36600461684b565b6148c1565b348015610e1c57600080fd5b50600254610a12906001600160a01b031681565b348015610e3c57600080fd5b5061047760035481565b348015610e5257600080fd5b50610477610e61366004615929565b61493a565b6000600d8281548110610e7b57610e7b616868565b90600052602060002090600a02019050610e936149a6565b81546001600160a01b03908116911614610ee15760405162461bcd60e51b815260206004820152600a60248201526927b7363c90313abcb2b960b11b60448201526064015b60405180910390fd5b6000600982015460ff166005811115610efc57610efc6161b2565b14610f195760405162461bcd60e51b8152600401610ed89061687e565b426009548260080154610f2c91906168e2565b1015610f865760405162461bcd60e51b815260206004820152602360248201527f5061796d656e7420636f6e6669726d6174696f6e20706572696f64206973206f6044820152623b32b960e91b6064820152608401610ed8565b42600882015560098101805460ff191660011790556040518281527f581d416ae9dff30c9305c2b35cb09ed5991897ab97804db29ccf92678e953160906020015b60405180910390a15050565b80600b6000610fe06149a6565b6001600160a01b039081168252602080830193909352604091820160009081209187168152925290205410156110585760405162461bcd60e51b815260206004820152601e60248201527f4e6f7420656e6f7567682062616c616e636520746f20776974686472617700006044820152606401610ed8565b80600b60006110656149a6565b6001600160a01b03166001600160a01b031681526020019081526020016000206000846001600160a01b03166001600160a01b0316815260200190815260200160002060008282546110b791906168fa565b909155506110d990506110c86149a6565b6001600160a01b0384169083614a02565b7f5c9993d2dd44f9c0f3ee9d50711f139df4c5139f8886d073d6666663a14fa6396111026149a6565b604080516001600160a01b03928316815291851660208301528101839052606001610fc7565b6000807f80ad7e1b04ee6d994a125f4714ca0720908bd80ed16063ec8aee4b88e9253e2d8360000151846020015180519060200120604051602001611189939291909283526001600160a01b03919091166020830152604082015260600190565b60408051601f1981840301815291905280516020909101209392505050565b6006602052600090815260409020805481906111c390616911565b80601f01602080910402602001604051908101604052809291908181526020018280546111ef90616911565b801561123c5780601f106112115761010080835404028352916020019161123c565b820191906000526020600020905b81548152906001019060200180831161121f57829003601f168201915b5050505050908060010154905082565b600080600d848154811061126257611262616868565b600091825260208083206006600a90930201918201548352600c905260409091209091506005600983015460ff1660058111156112a1576112a16161b2565b146112ad575050611484565b805485146112bc575050611484565b600181015460009060ff1660028111156112d8576112d86161b2565b6001830154909150610100900461ffff1660005b81811161147e576000818152600285016020526040902061130e6001846168fa565b8203611354576001600160a01b03881660009081526005820160205260409020600281015460019091015461134391906168e2565b61134d90886168e2565b965061146b565b836113e25760008160020154826001015461136f91906168e2565b6001600160a01b038a1660009081526005840160205260408120600281015460019091015492935090916113a391906168e2565b9050600082116113b45760006113cf565b818360040154826113c5919061694b565b6113cf919061696a565b6113d9908a6168e2565b9850505061146b565b60008185600381106113f6576113f6616868565b01541161140457600061145e565b80846003811061141657611416616868565b015460048201546001600160a01b038a1660009081526005840160205260409020866003811061144857611448616868565b0154611454919061694b565b61145e919061696a565b61146890886168e2565b96505b50806114768161698c565b9150506112ec565b50505050505b92915050565b6000600d838154811061149f5761149f616868565b600091825260208220600a9091020191508260028111156114c2576114c26161b2565b036114fe5760405162461bcd60e51b815260206004820152600c60248201526b2bb937b733903830b93a3c9760a11b6044820152606401610ed8565b6004600982015460ff166005811115611519576115196161b2565b1461155d5760405162461bcd60e51b8152602060048201526014602482015273139bc8191a5cdc1d5d19481d1bc8185c1c19585b60621b6044820152606401610ed8565b6000600782600701548154811061157657611576616868565b60009182526020918290206040805180820190915260029092020180546001600160a01b0316825260018101805492939192918401916115b590616911565b80601f01602080910402602001604051908101604052809291908181526020018280546115e190616911565b801561162e5780601f106116035761010080835404028352916020019161162e565b820191906000526020600020905b81548152906001019060200180831161161157829003601f168201915b505050505081525050905060008082600001516001600160a01b031663afe15cfb85600601546040518263ffffffff1660e01b815260040161167291815260200190565b6040805180830381865afa15801561168e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116b291906169a5565b915091508142101580156116c557508042105b6117285760405162461bcd60e51b815260206004820152602e60248201527f46756e64696e67206d757374206265206d6164652077697468696e207468652060448201526d30b83832b0b6103832b934b7b21760911b6064820152608401610ed8565b82516006850154604051631c3db16d60e01b815260009283926001600160a01b0390911691631c3db16d916117639160040190815260200190565b602060405180830381865afa158015611780573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117a491906169c9565b90508660028111156117b8576117b86161b2565b81036117e6577f000000000000000000000000000000000000000000000000000000000000000091506118c4565b80600003611816577f000000000000000000000000000000000000000000000000000000000000000091506118c4565b600261182285856168e2565b61182c919061696a565b42106118a05760405162461bcd60e51b815260206004820152603e60248201527f546865206c6f736572206d7573742070617920647572696e672074686520666960448201527f7273742068616c66206f66207468652061707065616c20706572696f642e00006064820152608401610ed8565b7f000000000000000000000000000000000000000000000000000000000000000091505b60068601546000908152600c602090815260408083206001810154610100900461ffff168452600280820190935292206003810154909160ff90911690811115611910576119106161b2565b896002811115611922576119226161b2565b036119795760405162461bcd60e51b815260206004820152602160248201527f41707065616c206665652068617320616c7265616479206265656e20706169646044820152601760f91b6064820152608401610ed8565b86516006890154602089015160405163791f8b7360e11b81526000936001600160a01b03169263f23f16e6926119b1926004016169e2565b602060405180830381865afa1580156119ce573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119f291906169c9565b90506000612710611a03878461694b565b611a0d919061696a565b611a1790836168e2565b9050600080611a5434868f6002811115611a3357611a336161b2565b60038110611a4357611a43616868565b0154611a4f90866168fa565b614a6a565b909250905081600586016000611a686149a6565b6001600160a01b03166001600160a01b031681526020019081526020016000208e6002811115611a9a57611a9a6161b2565b60038110611aaa57611aaa616868565b016000828254611aba91906168e2565b90915550829050858e6002811115611ad457611ad46161b2565b60038110611ae457611ae4616868565b016000828254611af491906168e2565b909155508e90507f22039e79a27860a038aece214124c8ecef12fdc1334feb8e8a157a0ae16a1db48e611b256149a6565b85604051611b35939291906169fb565b60405180910390a28015611b7357611b4b6149a6565b6001600160a01b03166108fc829081150290604051600060405180830381858888f150505050505b50819050838c6002811115611b8a57611b8a6161b2565b60038110611b9a57611b9a616868565b015410611d12576000600384015460ff166002811115611bbc57611bbc6161b2565b03611bec576003830180548c919060ff19166001836002811115611be257611be26161b2565b0217905550611cd9565b885160068b015460208b015160405163093225f160e31b81526001600160a01b03909316926349912f88928692611c25926004016169e2565b6000604051808303818588803b158015611c3e57600080fd5b505af1158015611c52573d6000803e3d6000fd5b5085935086925060029150611c649050565b60038110611c7457611c74616868565b01548460010154611c8591906168e2565b611c8f91906168fa565b600484015560018481018054610100900461ffff1691611cae83616a23565b825461ffff9182166101009390930a92830291909202199091161790555060038301805460ff191690555b8b7f213276d821d6205d87fa4f4658db4bf5a24a2323b0fdd8ffcd239c445f86a57b8c604051611d099190616a44565b60405180910390a25b505050505050505050505050565b6000807f409f5114779a253e700d775d7845e6efc1e83685ac59868d2df3d4de51c7d62183600001518460200151611d5b866040015161365c565b6040805160208101959095526001600160a01b03909316928401929092526060830152608082015260a001611189565b6000805b82811015611e4b5736848483818110611daa57611daa616868565b9050602002810190611dbc9190616a52565b90506000611dcc610cb283616a72565b9050611e1881878786818110611de457611de4616868565b9050602002810190611df69190616a52565b611e009080616a7e565b602001803603810190611e139190616415565b614a97565b611e35611e258380616a7e565b611e2f9080616a94565b83614b53565b5050508080611e439061698c565b915050611d8f565b5092915050565b6000828152600c602052604081208054600d80549293929091908110611e7a57611e7a616868565b90600052602060002090600a0201905060006007826007015481548110611ea357611ea3616868565b60009182526020909120600290910201546001600160a01b0316905080611ec86149a6565b6001600160a01b031614611f105760405162461bcd60e51b815260206004820152600f60248201526e27b7363c9030b93134ba3930ba37b960891b6044820152606401610ed8565b6002841115611f535760405162461bcd60e51b815260206004820152600f60248201526e24b73b30b634b210393ab634b7339760891b6044820152606401610ed8565b6005600983015460ff166005811115611f6e57611f6e6161b2565b03611fbb5760405162461bcd60e51b815260206004820152601a60248201527f204469737075746520616c7265616479207265736f6c7665642e0000000000006044820152606401610ed8565b600183810154610100900461ffff166000908152600285016020526040902090600382015460ff166002811115611ff457611ff46161b2565b03612011576001848101805460ff191682805b0217905550612082565b6002600382015460ff16600281111561202c5761202c6161b2565b03612048576001808501805460029260ff199091169083612007565b84600281111561205a5761205a6161b2565b60018086018054909160ff199091169083600281111561207c5761207c6161b2565b02179055505b61208b86614f38565b600184015486906001600160a01b038416907f394027a5fa6e098a1191094d1719d6929b9abc535fcc0c8f448d6a4e756222769060ff1660028111156120d3576120d36161b2565b60405190815260200160405180910390a3505050505050565b6000600d848154811061210157612101616868565b60009182526020909120600a9091020190506005600982015460ff16600581111561212e5761212e6161b2565b1461217b5760405162461bcd60e51b815260206004820152601b60248201527f546865206f72646572206d757374206265207265736f6c7665642e00000000006044820152606401610ed8565b60068101546000908152600c60205260409020805485146121d15760405162461bcd60e51b815260206004820152601060248201526f2ab73234b9b83aba32b21037b93232b960811b6044820152606401610ed8565b600181015460009060ff1660028111156121ed576121ed6161b2565b6001830154909150600090610100900461ffff16865b8181111580156122235750861580612223575061222087896168e2565b81105b15612252576122348a8a83876152e0565b61223e90846168e2565b92508061224a8161698c565b915050612203565b506040516001600160a01b038a169083156108fc029084906000818181858888f15050505050505050505050505050565b6000600d828154811061229857612298616868565b60009182526020909120600a9091020190506001600982015460ff1660058111156122c5576122c56161b2565b146122e25760405162461bcd60e51b8152600401610ed890616add565b42600a5482600801546122f591906168e2565b106123585760405162461bcd60e51b815260206004820152602d60248201527f4f7264657220636f6d706c6574696f6e20706572696f6420686173206e6f742060448201526c1e595d081d1a5b5959081bdd5d609a1b6064820152608401610ed8565b61236181615458565b6040518281527fa08ae43f66b3bb6dc42d8d01b32cd576235826cfa50d940d0141e0c885df07c89060200160405180910390a16040518281527f18cc2626d53b298add3ec58253b2a64517d5ab3dca9c8eb4b078a2852705c3d890602001610fc7565b6000606060005b835181101561242c57816123f78583815181106123ea576123ea616868565b6020026020010151612b29565b604051602001612408929190616b33565b604051602081830303815290604052915080806124249061698c565b9150506123cb565b50805160209091012092915050565b60055461ffff908116908416106124645760405162461bcd60e51b8152600401610ed890616b55565b61ffff831660009081526006602090815260408083206001600160a01b0386168452600281019092529091205460ff1615156001146124ef5760405162461bcd60e51b815260206004820152602160248201527f546f6b656e206e6f742079657420656e61626c656420666f722073656c6c696e6044820152606760f81b6064820152608401610ed8565b81600b60006124fc6149a6565b6001600160a01b03166001600160a01b031681526020019081526020016000206000856001600160a01b03166001600160a01b03168152602001908152602001600020600082825461254e91906168e2565b90915550612571905061255f6149a6565b6001600160a01b0385169030856154f2565b7f223333d41820f8f24aa9d9555d0c8e92398a489801fe2bb8ba5d674530a9804261259a6149a6565b604080516001600160a01b039283168152918616602083015281018490526060015b60405180910390a150505050565b6000806125da6040850185616b8c565b6125e991600491600091616bd2565b6125f291616bfc565b90506001600160e01b0319811663f2fde38b60e01b036126605760405162461bcd60e51b8152602060048201526024808201527f7472616e736665724f776e657273686970206973206e6f742064656c6567617460448201526361626c6560e01b6064820152608401610ed8565b6001600160e01b031981166338a80c5360e11b036126cc5760405162461bcd60e51b8152602060048201526024808201527f72656e6f756e63654f776e657273686970206973206e6f742064656c6567617460448201526361626c6560e01b6064820152608401610ed8565b6001600160e01b03198116635d383ce560e11b036127385760405162461bcd60e51b815260206004820152602360248201527f6368616e676541726269747261746f72206973206e6f742064656c6567617461604482015262626c6560e81b6064820152608401610ed8565b6001600160e01b0319811663b6694b7b60e01b036127a35760405162461bcd60e51b815260206004820152602260248201527f6164644d65746145766964656e6365206973206e6f742064656c6567617461626044820152616c6560f01b6064820152608401610ed8565b6001600160e01b03198116633d4c175b60e11b0361280f5760405162461bcd60e51b815260206004820152602360248201527f6164645061796d656e744d6574686f64206973206e6f742064656c6567617461604482015262626c6560e81b6064820152608401610ed8565b6001600160e01b031981166386c61ce560e01b036128845760405162461bcd60e51b815260206004820152602c60248201527f7570646174655061796d656e744d65746145766964656e6365206973206e6f7460448201526b2064656c6567617461626c6560a01b6064820152608401610ed8565b6001600160e01b0319811663ae1a38ed60e01b036128f05760405162461bcd60e51b8152602060048201526024808201527f7570646174655061796d656e744e616d65206973206e6f742064656c6567617460448201526361626c6560e01b6064820152608401610ed8565b6001600160e01b0319811663abaa14ff60e01b0361295e5760405162461bcd60e51b815260206004820152602560248201527f757064617465546f6b656e456e61626c6564206973206e6f742064656c6567616044820152647461626c6560d81b6064820152608401610ed8565b6001600160e01b0319811663050a4f4560e41b036129ce5760405162461bcd60e51b815260206004820152602760248201527f6368616e6765436f6e6669726d54696d656f7574206973206e6f742064656c6560448201526667617461626c6560c81b6064820152608401610ed8565b6001600160e01b031981166360c4fef360e11b03612a3c5760405162461bcd60e51b815260206004820152602560248201527f6368616e67654f7264657254696d656f7574206973206e6f742064656c6567616044820152647461626c6560d81b6064820152608401610ed8565b6001600160e01b03198116636cda375b60e01b03612a9c5760405162461bcd60e51b815260206004820152601d60248201527f6368616e676546656573206973206e6f742064656c6567617461626c650000006044820152606401610ed8565b6001600160e01b0319811663dd2cc3f360e01b03612afc5760405162461bcd60e51b815260206004820152601f60248201527f776974686472617746656573206973206e6f742064656c6567617461626c65006044820152606401610ed8565b50600195945050505050565b612b10615530565b600955565b6000612b22838333614b53565b9392505050565b6000807fd97dd99b404d177890f06a8f0fc8e5ed0333fb2ebb6684360709066e8984f594612b5a8460000151613f95565b612b6785602001516134b1565b6040805160208101949094528301919091526060820152608001611189565b6000807f0000000000000000000000000000000000000000000000000000000000000000612bb384613473565b60405161190160f01b602082015260228101929092526042820152606201611189565b60055461ffff90811690851610612bff5760405162461bcd60e51b8152600401610ed890616b55565b61ffff841660009081526006602090815260408083206001600160a01b0387168452600381019092529091208054612c3690616911565b9050600003612c995760405162461bcd60e51b815260206004820152602960248201527f53656c6c657220646f65736e2774206163636570742074686973207061796d656044820152681b9d081b595d1a1bd960ba1b6064820152608401610ed8565b6001600160a01b038316600090815260028201602052604090205460ff161515600114612d1d5760405162461bcd60e51b815260206004820152602c60248201527f546f6b656e206973206e6f7420656e61626c656420666f72207468697320706160448201526b1e5b595b9d081b595d1a1bd960a21b6064820152608401610ed8565b6001600160a01b038085166000908152600b6020908152604080832093871683529290522054821115612d925760405162461bcd60e51b815260206004820152601960248201527f4e6f7420656e6f7567682073656c6c65722062616c616e6365000000000000006044820152606401610ed8565b6007805460009190612da6906001906168fa565b81548110612db657612db6616868565b60009182526020918290206040805180820190915260029092020180546001600160a01b031682526001810180549293919291840191612df590616911565b80601f0160208091040260200160405190810160405280929190818152602001828054612e2190616911565b8015612e6e5780601f10612e4357610100808354040283529160200191612e6e565b820191906000526020600020905b815481529060010190602001808311612e5157829003601f168201915b5050509190925250508151602083015160405163f7434ea960e01b815293945090929091506000906001600160a01b0384169063f7434ea990612eb5908590600401616c2c565b602060405180830381865afa158015612ed2573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612ef691906169c9565b905080341015612f485760405162461bcd60e51b815260206004820181905260248201527f4172626974726174696f6e2066656573206e65656420746f20626520706169646044820152606401610ed8565b600d604051806101400160405280612f5e6149a6565b6001600160a01b031681526020018a6001600160a01b03168152602001896001600160a01b0316815260200188815260200134815260200160008152602001600081526020016001600780549050612fb691906168fa565b81524260208201526040016000905281546001808201845560009384526020938490208351600a9093020180546001600160a01b03199081166001600160a01b0394851617825594840151818301805487169185169190911790556040840151600282018054909616931692909217909355606082015160038201556080820151600482015560a082015160058083019190915560c0830151600683015560e083015160078301556101008301516008830155610120830151600983018054949593949193909260ff19909216918490811115613095576130956161b2565b021790555050506001600160a01b038089166000908152600b60209081526040808320938b16835292905290812080548892906130d39084906168fa565b9091555060009050806130e588613f64565b600d5491935091507fbe360470ba40d19dfe43cce1e19662a7e0541a977014ee9debc7f9a2924d02179061311b906001906168fa565b6131236149a6565b604080519283526001600160a01b03918216602084015261ffff8f1690830152808d1660608301528b16608082015260a0810183905260c0810184905260e00160405180910390a15050505050505050505050565b61318061590b565b6000600d858154811061319557613195616868565b60009182526020808320600a9290920290910160068101548352600c825260408084208885526002810184528185206001600160a01b03891686526005810190945293819020815160608101928390529295509060039082845b8154815260200190600101908083116131ef57505050505093505050509392505050565b6000807f3c36a06e1d288b0f94f565588317a46ad11bc3c96992109f9a2365a2737259a76132448460000151611d20565b602080860151805190820120604051611189949392019283526020830191909152604082015260600190565b613278615530565b61271081106132c95760405162461bcd60e51b815260206004820152601d60248201527f666565732063616e6e6f74206265206d6f7265207468616e20313030250000006044820152606401610ed8565b600455565b6000600d83815481106132e3576132e3616868565b60009182526020909120600a9091020190506005600982015460ff166005811115613310576133106161b2565b1461335d5760405162461bcd60e51b815260206004820152601b60248201527f546865206f72646572206d757374206265207265736f6c7665642e00000000006044820152606401610ed8565b60068101546000908152600c60205260409020805484146133b35760405162461bcd60e51b815260206004820152601060248201526f2ab73234b9b83aba32b21037b93232b960811b6044820152606401610ed8565b60018101546000906133de9087908790879060ff1660028111156133d9576133d96161b2565b6152e0565b6040519091506001600160a01b0387169082156108fc029083906000818181858888f15050505050505050505050565b6000807fe8d3d963b33868fb116316bc3fd55e8f49123f30e4418f71e140d54b7cd3b2b983600001518460200151604051602001611189939291909283526020830191909152604082015260600190565b613467615530565b61347160006155a9565b565b6000807f08c69a206c06f5334b35ceb1186181a713b21aff02cf66285f375084fbef2eb86134a484600001516123c4565b612b67856020015161340e565b6000606060005b835181101561242c57816134e48583815181106134d7576134d7616868565b6020026020010151613213565b6040516020016134f5929190616b33565b604051602081830303815290604052915080806135119061698c565b9150506134b8565b613521615530565b60ff82161580159061353857506008548260ff1611155b61357f5760405162461bcd60e51b8152602060048201526018602482015277125b9d985b1a590813595d1848115d9a59195b98d948125160421b6044820152606401610ed8565b60058054600091600691839161ffff909116908261359c83616a23565b82546101009290920a61ffff81810219909316918316021790915516815260208101919091526040016000209050806135d6858783616c85565b5060ff83166001808301919091556001600160a01b03831660009081526002830160205260409020805460ff1916821790556005547fa9623e6f99664313c60fa7d1499b5d8f01ef311fa403cb5e6b959e01e18e31569161363a9161ffff16616d44565b86868660405161364d9493929190616d90565b60405180910390a15050505050565b6000606060005b835181101561242c578161368f85838151811061368257613682616868565b6020026020010151611128565b6040516020016136a0929190616b33565b604051602081830303815290604052915080806136bc9061698c565b915050613663565b6136cc615530565b60055461ffff908116908316106136f55760405162461bcd60e51b8152600401610ed890616b55565b6008548160ff1611156137455760405162461bcd60e51b8152602060048201526018602482015277125b9d985b1a590813595d1848115d9a59195b98d948125160421b6044820152606401610ed8565b61ffff82166000908152600660205260409081902060ff831660018201819055915190917fa9623e6f99664313c60fa7d1499b5d8f01ef311fa403cb5e6b959e01e18e315691613799918691859190616dc2565b60405180910390a1505050565b6000600d82815481106137bb576137bb616868565b600091825260208220600a909102019150600982015460ff1660058111156137e5576137e56161b2565b146138025760405162461bcd60e51b8152600401610ed890616add565b42600954826008015461381591906168e2565b106138745760405162461bcd60e51b815260206004820152602960248201527f436f6e6669726d6174696f6e20706572696f6420686173206e6f7420796574206044820152681d1a5b5959081bdd5d60ba1b6064820152608401610ed8565b600381810180546004840180546000938490559083905560098501805460ff191690941790935560018401546001600160a01b039081168352600b60209081526040808520600288015490931685529190528220805491939284926138da9084906168e2565b909155505082546040516001600160a01b039091169082156108fc029083906000818181858888f19350505050507f91e3c154ff27b859a148421f882fdc69c8fa7ef2cdde96a9216fb2bd1f79528b846040516125bc91815260200190565b60055461ffff908116908216106139625760405162461bcd60e51b8152600401610ed890616b55565b61ffff8116600090815260066020908152604080832081519283019091528282529160038301906139916149a6565b6001600160a01b031681526020810191909152604001600020906139b59082616e61565b507fc74b33c06f0c4b5953bed1067cd8fe104c8b58fecccbbb21dcd9e85a47e853256139df6149a6565b604080516001600160a01b03909216825261ffff8516602083015201610fc7565b805160009081613a0f82613b1d565b90506000613a218286602001516155f9565b95945050505050565b613a3261590b565b600080600080600d8781548110613a4b57613a4b616868565b60009182526020808320600a9290920290910160068101548352600c825260408084208a85526002810190935292839020600380820154600483015460018601548751606081019889905295985095969395869560ff90931694919361010090910461ffff168e1415929186919082845b815481526020019060010190808311613abc5750505050509350965096509650965050505092959194509250565b600080600d8381548110613b0057613b00616868565b90600052602060002090600a02019050612b228160030154613b4a565b6000807f0000000000000000000000000000000000000000000000000000000000000000612bb384611d20565b600080613b5683613f64565b509392505050565b6000600d8281548110613b7357613b73616868565b90600052602060002090600a02019050613b8b6149a6565b60018201546001600160a01b03908116911614613bd85760405162461bcd60e51b815260206004820152600b60248201526a27b7363c9039b2b63632b960a91b6044820152606401610ed8565b6001600982015460ff166005811115613bf357613bf36161b2565b14613c4c5760405162461bcd60e51b815260206004820152602360248201527f43616e6e6f7420646973707574652061206e6f74207965742070616964206f726044820152623232b960e91b6064820152608401610ed8565b42600a548260080154613c5f91906168e2565b1015613cb85760405162461bcd60e51b815260206004820152602260248201527f4f7264657220616c726561647920636f6d706c657465642062792074696d656f6044820152611d5d60f21b6064820152608401610ed8565b6007805460009190613ccc906001906168fa565b81548110613cdc57613cdc616868565b60009182526020918290206040805180820190915260029092020180546001600160a01b031682526001810180549293919291840191613d1b90616911565b80601f0160208091040260200160405190810160405280929190818152602001828054613d4790616911565b8015613d945780601f10613d6957610100808354040283529160200191613d94565b820191906000526020600020905b815481529060010190602001808311613d7757829003601f168201915b5050509190925250508151602083015160405163f7434ea960e01b815293945090929091506000906001600160a01b0384169063f7434ea990613ddb908590600401616c2c565b602060405180830381865afa158015613df8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613e1c91906169c9565b905080341015613e6e5760405162461bcd60e51b815260206004820181905260248201527f4172626974726174696f6e2066656573206e65656420746f20626520706169646044820152606401610ed8565b346005860181905560098601805460ff1916600490811790915560405163c13517e160e01b81526001600160a01b0386169263c13517e1929091613eb7916002918891016169e2565b60206040518083038185885af1158015613ed5573d6000803e3d6000fd5b50505050506040513d601f19601f82011682018060405250810190613efa91906169c9565b600686018181556000918252600c6020908152604092839020898155915483518a81529182018a905291926001600160a01b038716917f74baab670a4015ab2f1b467c5252a96141a2573f2908e58a92081e80d3cfde3d910160405180910390a350505050505050565b60008061271060045484613f78919061694b565b613f82919061696a565b9150613f8e82846168fa565b9050915091565b6000807fb16dfdb3b8fa033fe30ac976cd4a50ad256b6811c80d90fcd0b323eec190047d8360000151846020015185604001518051906020012060405160200161118994939291909384526001600160a01b039290921660208401526040830152606082015260800190565b6000600d848154811061401657614016616868565b60009182526020918290206040805161014081018252600a90930290910180546001600160a01b0390811684526001820154811694840194909452600281015490931690820152600382015460608201526004820154608082015260058083015460a0830152600683015460c0830152600783015460e08301526008830154610100830152600983015491929161012084019160ff909116908111156140be576140be6161b2565b60058111156140cf576140cf6161b2565b9052509050600581610120015160058111156140ed576140ed6161b2565b106141555760405162461bcd60e51b815260206004820152603260248201527f4d757374206e6f742073656e642065766964656e63652069662074686520646960448201527139b83aba329034b9903932b9b7b63b32b21760711b6064820152608401610ed8565b600060078260e001518154811061416e5761416e616868565b60009182526020918290206040805180820190915260029092020180546001600160a01b0316825260018101805492939192918401916141ad90616911565b80601f01602080910402602001604051908101604052809291908181526020018280546141d990616911565b80156142265780601f106141fb57610100808354040283529160200191614226565b820191906000526020600020905b81548152906001019060200180831161420957829003601f168201915b50505050508152505090506142396149a6565b6001600160a01b03168582600001516001600160a01b03167fdccf2f8b2cc26eafcd61905cba744cff4b81d14740725f6376390dc6298a6a3c8787604051614282929190616f20565b60405180910390a45050505050565b600d81815481106142a157600080fd5b60009182526020909120600a909102018054600182015460028301546003840154600485015460058601546006870154600788015460088901546009909901546001600160a01b039889169a50968816989790951696939592949193909260ff168a565b61430d615530565b60055461ffff908116908416106143365760405162461bcd60e51b8152600401610ed890616b55565b61ffff90921660009081526006602090815260408083206001600160a01b0390941683526002909301905220805491151560ff19909216919091179055565b61437d615530565b60055461ffff908116908416106143a65760405162461bcd60e51b8152600401610ed890616b55565b61ffff83166000908152600660205260409020806143c5838583616c85565b5060018101546040517fa9623e6f99664313c60fa7d1499b5d8f01ef311fa403cb5e6b959e01e18e3156916125bc9187918591616dc2565b6000614407615530565b600880549060006144178361698c565b919050559050807f61606860eb6c87306811e2695215385101daab53bd6ab4e9f9049aead9363c7d848460405161444f929190616f20565b60405180910390a292915050565b6000600d828154811061447257614472616868565b90600052602060002090600a0201905061448a6149a6565b60018201546001600160a01b039081169116146144d75760405162461bcd60e51b815260206004820152600b60248201526a27b7363c9039b2b63632b960a91b6044820152606401610ed8565b6002600982015460ff1660058111156144f2576144f26161b2565b1061450f5760405162461bcd60e51b8152600401610ed89061687e565b6000600982015460ff16600581111561452a5761452a6161b2565b1480614547575042600a54826008015461454491906168e2565b10155b6145935760405162461bcd60e51b815260206004820152601a60248201527f4f7264657220636f6d706c657465642062792074696d656f75740000000000006044820152606401610ed8565b61459c81615458565b6040518281527f18cc2626d53b298add3ec58253b2a64517d5ab3dca9c8eb4b078a2852705c3d890602001610fc7565b6145d4615530565b60076040518060400160405280856001600160a01b0316815260200184848080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920182905250939094525050835460018082018655948252602091829020845160029092020180546001600160a01b0319166001600160a01b03909216919091178155908301519293909290830191506146769082616e61565b505050505050565b614686615530565b600a55565b60008061469b8360000151612b86565b905060006146ad8285602001516155f9565b949350505050565b60055461ffff908116908416106146de5760405162461bcd60e51b8152600401610ed890616b55565b61ffff83166000908152600660205260408120908390839060038401906147036149a6565b6001600160a01b03168152602081019190915260400160002091614728919083616c85565b507fa636b73d01712b7183ef8ebe3e4fa0b2ecfb26bd6d50fd638bca9b3dc8015d6c6147526149a6565b8585856040516125bc9493929190616f34565b61476d615530565b6003548211156147bf5760405162461bcd60e51b815260206004820152601d60248201527f416d6f756e74206d6f7265207468616e206163637275656420666565730000006044820152606401610ed8565b81600360008282546147d191906168fa565b90915550506040516001600160a01b0382169083156108fc029084906000818181858888f150505050505050565b6007818154811061480f57600080fd5b6000918252602090912060029091020180546001820180546001600160a01b0390921693509061483e90616911565b80601f016020809104026020016040519081016040528092919081815260200182805461486a90616911565b80156148b75780601f1061488c576101008083540402835291602001916148b7565b820191906000526020600020905b81548152906001019060200180831161489a57829003601f168201915b5050505050905082565b6148c9615530565b6001600160a01b03811661492e5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610ed8565b614937816155a9565b50565b600080600d838154811061495057614950616868565b90600052602060002090600a020190506000600c60008360060154815260200190815260200160002090508060010160019054906101000a900461ffff16600161499a9190616f6d565b61ffff16949350505050565b60003033036149fc57600080368080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152505050503601516001600160a01b031691506149ff9050565b50335b90565b6040516001600160a01b038316602482015260448101829052614a6590849063a9059cbb60e01b906064015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b0319909316929092179091526156d0565b505050565b60008083831115614a8057508290506000614a90565b614a8a83856168fa565b90508291505b9250929050565b60208082015182516001600160a01b0385166000908152600180855260408083208584529095529390205491929091614acf916168e2565b8114614b295760405162461bcd60e51b815260206004820152602360248201527f44656c6567617461626c65436f72653a6e6f6e6365322d6f75742d6f662d6f726044820152623232b960e91b6064820152608401610ed8565b6001600160a01b039093166000908152600160209081526040808320938352929052209190915550565b6000805b83811015613b56576000858583818110614b7357614b73616868565b9050602002810190614b859190616a52565b614b8e90616f93565b9050600080826020015151600003614ba65750849050805b6000805b846020015151811015614e4157600085602001518281518110614bcf57614bcf616868565b602002602001015190506000614be482613a00565b905082600003614bf5578095508594505b846001600160a01b0316816001600160a01b031614614c685760405162461bcd60e51b815260206004820152602960248201527f44656c6567617461626c65436f72653a696e76616c69642d64656c656761746960448201526837b716b9b4b3b732b960b91b6064820152608401610ed8565b815160208101518514614cd75760405162461bcd60e51b815260206004820152603160248201527f44656c6567617461626c65436f72653a696e76616c69642d617574686f726974604482015270792d64656c65676174696f6e2d6c696e6b60781b6064820152608401610ed8565b6000614ce284613213565b905060005b8260400151518161ffff161015614e2457600083604001518261ffff1681518110614d1457614d14616868565b60200260200101516000015190506000816001600160a01b0316635068de4c86604001518561ffff1681518110614d4d57614d4d616868565b6020026020010151602001518e60000151876040518463ffffffff1660e01b8152600401614d7d93929190616f9f565b6020604051808303816000875af1158015614d9c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190614dc09190616ffb565b905080614e0f5760405162461bcd60e51b815260206004820152601f60248201527f44656c6567617461626c65436f72653a6361766561742d72656a6563746564006044820152606401610ed8565b50508080614e1c90616a23565b915050614ce7565b50905195509350829150614e3990508161698c565b915050614baa565b50835180516001600160a01b03163014614eaf5760405162461bcd60e51b815260206004820152602960248201527f44656c6567617461626c65436f72653a696e76616c69642d696e766f636174696044820152681bdb8b5d185c99d95d60ba1b6064820152608401610ed8565b614ec7816000015182604001518360200151876157a2565b965086614f205760405162461bcd60e51b815260206004820152602160248201527f44656c6567617461626c65436f72653a3a657865637574696f6e2d6661696c656044820152601960fa1b6064820152608401610ed8565b50505050508080614f309061698c565b915050614b57565b6000818152600c602052604081208054600d80549293929091908110614f6057614f60616868565b600091825260208220600a91909102016003810180546004830180546005808601805495889055928790559186905560098501805460ff19169092179091559294509280614fad85613f64565b915091508160036000828254614fc391906168e2565b909155505060018781015460ff166002811115614fe257614fe26161b2565b0361508f5785546040516001600160a01b039091169085156108fc029086906000818181858888f1505050506002870154875460405163a9059cbb60e01b81526001600160a01b039182166004820152602481018590529116915063a9059cbb906044016020604051808303816000875af1158015615065573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906150899190616ffb565b506152a1565b6002600188015460ff1660028111156150aa576150aa6161b2565b036151275760018601546040516001600160a01b039091169084156108fc029085906000818181858888f150505060018801546001600160a01b039081166000908152600b6020908152604080832060028d01549094168352929052908120805489945090925061511c9084906168e2565b909155506152a19050565b6000600261513586866157e1565b61513f919061696a565b87546040519192506001600160a01b03169082156108fc029083906000818181858888f150505060018901546040516001600160a01b03909116925083156108fc02915083906000818181858888f193505050505060006002876151a3919061696a565b6002890154895460405163a9059cbb60e01b81526001600160a01b03918216600482015260248101849052929350169063a9059cbb906044016020604051808303816000875af11580156151fb573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061521f9190616ffb565b506002880154600189015460405163a9059cbb60e01b81526001600160a01b0391821660048201526024810184905291169063a9059cbb906044016020604051808303816000875af1158015615279573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061529d9190616ffb565b5050505b86546040519081527f1e622a4d355fee568685ee98938ecae07ca1ab8ba9ec33832db0089485e987fa9060200160405180910390a15050505050505050565b600080600d85815481106152f6576152f6616868565b600091825260208083206006600a90930201918201548352600c815260408084208885526002810183528185206001600160a01b038c1686526005810190935293206001840154929450909161ffff6101009091041680880361536e578160020154826001015461536791906168e2565b955061543c565b866153d65760008360020154846001015461538991906168e2565b90506000836002015484600101546153a191906168e2565b9050600082116153b25760006153cd565b818560040154826153c3919061694b565b6153cd919061696a565b9750505061543c565b60008388600381106153ea576153ea616868565b0154116153f8576000615439565b82876003811061540a5761540a616868565b0154836004015483896003811061542357615423616868565b015461542f919061694b565b615439919061696a565b95505b6000826001015560008260020155509398975050505050505050565b6003810180546004830180546000938490559083905560098401805460ff1916600217905590918061548984613f64565b91509150816003600082825461549f91906168e2565b909155505084546040516001600160a01b039091169084156108fc029085906000818181858888f15050875460028901546154eb94506001600160a01b03908116935016905083614a02565b5050505050565b6040516001600160a01b038085166024830152831660448201526064810182905261552a9085906323b872dd60e01b90608401614a2e565b50505050565b6155386149a6565b6001600160a01b03166155536000546001600160a01b031690565b6001600160a01b0316146134715760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610ed8565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b60008060008084516041146156145760009350505050611484565b50505060208201516040830151606084015160001a601b8110156156405761563d601b82617018565b90505b8060ff16601b1415801561565857508060ff16601c14155b156156695760009350505050611484565b60408051600081526020810180835288905260ff831691810191909152606081018490526080810183905260019060a0016020604051602081039080840390855afa1580156156bc573d6000803e3d6000fd5b505050602060405103519350505050611484565b6000615725826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166157f79092919063ffffffff16565b805190915015614a6557808060200190518101906157439190616ffb565b614a655760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608401610ed8565b60008084836040516020016157b892919061703d565b604051602081830303815290604052905060008082516020840160008a89f19695505050505050565b60008183106157f05781612b22565b5090919050565b60606146ad8484600085856001600160a01b0385163b6158595760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610ed8565b600080866001600160a01b031685876040516158759190617074565b60006040518083038185875af1925050503d80600081146158b2576040519150601f19603f3d011682016040523d82523d6000602084013e6158b7565b606091505b50915091506158c78282866158d2565b979650505050505050565b606083156158e1575081612b22565b8251156158f15782518084602001fd5b8160405162461bcd60e51b8152600401610ed89190616c2c565b60405180606001604052806003906020820280368337509192915050565b60006020828403121561593b57600080fd5b5035919050565b6001600160a01b038116811461493757600080fd5b6000806040838503121561596a57600080fd5b823561597581615942565b946020939093013593505050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156159bb576159bb615983565b60405290565b604051606081016001600160401b03811182821017156159bb576159bb615983565b604051601f8201601f191681016001600160401b0381118282101715615a0b57615a0b615983565b604052919050565b600082601f830112615a2457600080fd5b81356001600160401b03811115615a3d57615a3d615983565b615a50601f8201601f19166020016159e3565b818152846020838601011115615a6557600080fd5b816020850160208301376000918101602001919091529392505050565b600060408284031215615a9457600080fd5b615a9c615999565b90508135615aa981615942565b815260208201356001600160401b03811115615ac457600080fd5b615ad084828501615a13565b60208301525092915050565b600060208284031215615aee57600080fd5b81356001600160401b03811115615b0457600080fd5b6146ad84828501615a82565b803561ffff81168114615b2257600080fd5b919050565b600060208284031215615b3957600080fd5b612b2282615b10565b60005b83811015615b5d578181015183820152602001615b45565b8381111561552a5750506000910152565b60008151808452615b86816020860160208601615b42565b601f01601f19169290920160200192915050565b604081526000615bad6040830185615b6e565b90508260208301529392505050565b60008060408385031215615bcf57600080fd5b8235615bda81615942565b91506020830135615bea81615942565b809150509250929050565b60008060408385031215615c0857600080fd5b823591506020830135615bea81615942565b60008060408385031215615c2d57600080fd5b82359150602083013560038110615bea57600080fd5b60006001600160401b03821115615c5c57615c5c615983565b5060051b60200190565b600082601f830112615c7757600080fd5b81356020615c8c615c8783615c43565b6159e3565b82815260059290921b84018101918181019086841115615cab57600080fd5b8286015b84811015615cea5780356001600160401b03811115615cce5760008081fd5b615cdc8986838b0101615a82565b845250918301918301615caf565b509695505050505050565b600060608284031215615d0757600080fd5b615d0f6159c1565b90508135615d1c81615942565b81526020828101359082015260408201356001600160401b03811115615d4157600080fd5b615d4d84828501615c66565b60408301525092915050565b600060208284031215615d6b57600080fd5b81356001600160401b03811115615d8157600080fd5b6146ad84828501615cf5565b60008083601f840112615d9f57600080fd5b5081356001600160401b03811115615db657600080fd5b6020830191508360208260051b8501011115614a9057600080fd5b60008060208385031215615de457600080fd5b82356001600160401b03811115615dfa57600080fd5b615e0685828601615d8d565b90969095509350505050565b60008060408385031215615e2557600080fd5b50508035926020909101359150565b60008060008060808587031215615e4a57600080fd5b8435615e5581615942565b966020860135965060408601359560600135945092505050565b600060608284031215615e8157600080fd5b615e896159c1565b90508135615e9681615942565b81526020828101359082015260408201356001600160401b03811115615ebb57600080fd5b615d4d84828501615a13565b600060408284031215615ed957600080fd5b615ee1615999565b905081356001600160401b0380821115615efa57600080fd5b615f0685838601615cf5565b83526020840135915080821115615f1c57600080fd5b50615ad084828501615a13565b600082601f830112615f3a57600080fd5b81356020615f4a615c8783615c43565b82815260059290921b84018101918181019086841115615f6957600080fd5b8286015b84811015615cea5780356001600160401b03811115615f8c5760008081fd5b615f9a8986838b0101615ec7565b845250918301918301615f6d565b600060408284031215615fba57600080fd5b615fc2615999565b905081356001600160401b0380821115615fdb57600080fd5b615fe785838601615e6f565b83526020840135915080821115615ffd57600080fd5b50615ad084828501615f29565b600082601f83011261601b57600080fd5b8135602061602b615c8783615c43565b82815260059290921b8401810191818101908684111561604a57600080fd5b8286015b84811015615cea5780356001600160401b0381111561606d5760008081fd5b61607b8986838b0101615fa8565b84525091830191830161604e565b60006020828403121561609b57600080fd5b81356001600160401b038111156160b157600080fd5b6146ad8482850161600a565b6000806000606084860312156160d257600080fd5b6160db84615b10565b925060208401356160eb81615942565b929592945050506040919091013590565b60008083601f84011261610e57600080fd5b5081356001600160401b0381111561612557600080fd5b602083019150836020828501011115614a9057600080fd5b6000806000806060858703121561615357600080fd5b84356001600160401b038082111561616a57600080fd5b616176888389016160fc565b9096509450602087013591508082111561618f57600080fd5b508501606081880312156161a257600080fd5b9396929550929360400135925050565b634e487b7160e01b600052602160045260246000fd5b600381106161d8576161d86161b2565b9052565b838152606081016161f060208301856161c8565b61ffff83166040830152949350505050565b60006020828403121561621457600080fd5b81356001600160401b0381111561622a57600080fd5b6146ad84828501615fa8565b60006040828403121561624857600080fd5b616250615999565b9050813581526020820135602082015292915050565b60006060828403121561627857600080fd5b616280615999565b905081356001600160401b0381111561629857600080fd5b6162a48482850161600a565b8252506162b48360208401616236565b602082015292915050565b6000602082840312156162d157600080fd5b81356001600160401b038111156162e757600080fd5b6146ad84828501616266565b6000806000806080858703121561630957600080fd5b61631285615b10565b9350602085013561632281615942565b9250604085013561633281615942565b9396929550929360600135925050565b60008060006060848603121561635757600080fd5b8335925060208401359150604084013561637081615942565b809150509250925092565b8060005b600381101561552a57815184526020938401939091019060010161637f565b60608101611484828461637b565b6000602082840312156163be57600080fd5b81356001600160401b038111156163d457600080fd5b6146ad84828501615ec7565b6000806000606084860312156163f557600080fd5b833561640081615942565b95602085013595506040909401359392505050565b60006040828403121561642757600080fd5b612b228383616236565b60006020828403121561644357600080fd5b81356001600160401b0381111561645957600080fd5b6146ad84828501615f29565b803560ff81168114615b2257600080fd5b6000806000806060858703121561648c57600080fd5b84356001600160401b038111156164a257600080fd5b6164ae878288016160fc565b90955093506164c1905060208601616465565b915060408501356164d181615942565b939692955090935050565b6000602082840312156164ee57600080fd5b81356001600160401b0381111561650457600080fd5b6146ad84828501615c66565b6000806040838503121561652357600080fd5b61652c83615b10565b915061653a60208401616465565b90509250929050565b60c08101616551828761637b565b61655e60608301866161c8565b83608083015282151560a083015295945050505050565b60006020828403121561658757600080fd5b81356001600160401b0381111561659d57600080fd5b6146ad84828501615e6f565b6000806000604084860312156165be57600080fd5b8335925060208401356001600160401b038111156165db57600080fd5b6165e7868287016160fc565b9497909650939450505050565b6001600160a01b038b811682528a8116602083015289166040820152606081018890526080810187905260a0810186905260c0810185905260e081018490526101008101839052610140810160068310616650576166506161b2565b826101208301529b9a5050505050505050505050565b801515811461493757600080fd5b60008060006060848603121561668957600080fd5b61669284615b10565b925060208401356166a281615942565b9150604084013561637081616666565b6000806000604084860312156166c757600080fd5b6166d084615b10565b925060208401356001600160401b038111156165db57600080fd5b600080602083850312156166fe57600080fd5b82356001600160401b0381111561671457600080fd5b615e06858286016160fc565b60008060006040848603121561673557600080fd5b83356166d081615942565b60006040828403121561675257600080fd5b61675a615999565b905081356001600160401b038082111561677357600080fd5b615f0685838601616266565b60006020828403121561679157600080fd5b81356001600160401b038111156167a757600080fd5b6146ad84828501616740565b600080600080608085870312156167c957600080fd5b84356001600160401b03808211156167e057600080fd5b6167ec88838901615a13565b9550602087013591508082111561680257600080fd5b5061680f87828801615a13565b9350506040850135915060608501356164d181615942565b6001600160a01b03831681526040602082018190526000906146ad90830184615b6e565b60006020828403121561685d57600080fd5b8135612b2281615942565b634e487b7160e01b600052603260045260246000fd5b6020808252602e908201527f4f7264657220616c72656164792063616e63656c6c65642c20636f6d706c657460408201526d1959081bdc88191a5cdc1d5d195960921b606082015260800190565b634e487b7160e01b600052601160045260246000fd5b600082198211156168f5576168f56168cc565b500190565b60008282101561690c5761690c6168cc565b500390565b600181811c9082168061692557607f821691505b60208210810361694557634e487b7160e01b600052602260045260246000fd5b50919050565b6000816000190483118215151615616965576169656168cc565b500290565b60008261698757634e487b7160e01b600052601260045260246000fd5b500490565b60006001820161699e5761699e6168cc565b5060010190565b600080604083850312156169b857600080fd5b505080516020909101519092909150565b6000602082840312156169db57600080fd5b5051919050565b8281526040602082015260006146ad6040830184615b6e565b60608101616a0982866161c8565b6001600160a01b0393909316602082015260400152919050565b600061ffff808316818103616a3a57616a3a6168cc565b6001019392505050565b6020810161148482846161c8565b60008235603e19833603018112616a6857600080fd5b9190910192915050565b60006114843683616740565b60008235605e19833603018112616a6857600080fd5b6000808335601e19843603018112616aab57600080fd5b8301803591506001600160401b03821115616ac557600080fd5b6020019150600581901b3603821315614a9057600080fd5b60208082526036908201527f4f726465722063616e206f6e6c792062652063616e63656c6c656420696d6d656040820152753234b0ba32b63c9030b33a32b91031b932b0ba34b7b760511b606082015260800190565b60008351616b45818460208801615b42565b9190910191825250602001919050565b6020808252601e908201527f5061796d656e74206d6574686f6420646f6573206e6f742065786973742e0000604082015260600190565b6000808335601e19843603018112616ba357600080fd5b8301803591506001600160401b03821115616bbd57600080fd5b602001915036819003821315614a9057600080fd5b60008085851115616be257600080fd5b83861115616bef57600080fd5b5050820193919092039150565b6001600160e01b03198135818116916004851015616c245780818660040360031b1b83161692505b505092915050565b602081526000612b226020830184615b6e565b601f821115614a6557600081815260208120601f850160051c81016020861015616c665750805b601f850160051c820191505b8181101561467657828155600101616c72565b6001600160401b03831115616c9c57616c9c615983565b616cb083616caa8354616911565b83616c3f565b6000601f841160018114616ce45760008515616ccc5750838201355b600019600387901b1c1916600186901b1783556154eb565b600083815260209020601f19861690835b82811015616d155786850135825560209485019460019092019101616cf5565b5086821015616d325760001960f88860031b161c19848701351681555b505060018560011b0183555050505050565b600061ffff83811690831681811015616d5f57616d5f6168cc565b039392505050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b61ffff85168152606060208201526000616dae606083018587616d67565b905060ff8316604083015295945050505050565b61ffff841681526000602060608184015260008554616de081616911565b8060608701526080600180841660008114616e025760018114616e1c57616e4a565b60ff1985168984015283151560051b890183019550616e4a565b8a6000528660002060005b85811015616e425781548b8201860152908301908801616e27565b8a0184019650505b505050505060409390930193909352509392505050565b81516001600160401b03811115616e7a57616e7a615983565b616e8e81616e888454616911565b84616c3f565b602080601f831160018114616ec35760008415616eab5750858301515b600019600386901b1c1916600185901b178555614676565b600085815260208120601f198616915b82811015616ef257888601518255948401946001909101908401616ed3565b5085821015616f105787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b6020815260006146ad602083018486616d67565b6001600160a01b038516815261ffff84166020820152606060408201819052600090616f639083018486616d67565b9695505050505050565b600061ffff808316818516808303821115616f8a57616f8a6168cc565b01949350505050565b60006114843683615fa8565b606081526000616fb26060830186615b6e565b828103602084015260018060a01b03855116815260208501516020820152604085015160606040830152616fe96060830182615b6e565b92505050826040830152949350505050565b60006020828403121561700d57600080fd5b8151612b2281616666565b600060ff821660ff84168060ff03821115617035576170356168cc565b019392505050565b6000835161704f818460208801615b42565b60609390931b6bffffffffffffffffffffffff19169190920190815260140192915050565b60008251616a68818460208701615b4256fea26469706673582212209d931caaada8b73203b76d45fa53d0538c717fc5fd3c4dad9de9f7c9d2a7fa3664736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct Unipeer<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Unipeer<M> {
        fn clone(&self) -> Self {
            Unipeer(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Unipeer<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Unipeer<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Unipeer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Unipeer<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), UNIPEER_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                UNIPEER_ABI.clone(),
                UNIPEER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `GET_CAVEAT_ARRAY_PACKETHASH` (0x7b577b58) function"]
        pub fn get_caveat_array_packethash(
            &self,
            input: ::std::vec::Vec<Caveat>,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([123, 87, 123, 88], input)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `GET_CAVEAT_PACKETHASH` (0x08aaf6b0) function"]
        pub fn get_caveat_packethash(
            &self,
            input: Caveat,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([8, 170, 246, 176], (input,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `GET_DELEGATION_PACKETHASH` (0x2f52a2fd) function"]
        pub fn get_delegation_packethash(
            &self,
            input: Delegation,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([47, 82, 162, 253], (input,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `GET_INVOCATIONS_PACKETHASH` (0x7234eefe) function"]
        pub fn get_invocations_packethash(
            &self,
            input: Invocations,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([114, 52, 238, 254], (input,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `GET_INVOCATION_ARRAY_PACKETHASH` (0x3a481821) function"]
        pub fn get_invocation_array_packethash(
            &self,
            input: ::std::vec::Vec<Invocation>,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([58, 72, 24, 33], input)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `GET_INVOCATION_PACKETHASH` (0x5cf1b24a) function"]
        pub fn get_invocation_packethash(
            &self,
            input: Invocation,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([92, 241, 178, 74], (input,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `GET_REPLAYPROTECTION_PACKETHASH` (0x6f965803) function"]
        pub fn get_replayprotection_packethash(
            &self,
            input: ReplayProtection,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([111, 150, 88, 3], (input,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `GET_SIGNEDDELEGATION_ARRAY_PACKETHASH` (0x736f7ce7) function"]
        pub fn get_signeddelegation_array_packethash(
            &self,
            input: ::std::vec::Vec<SignedDelegation>,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([115, 111, 124, 231], input)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `GET_SIGNEDDELEGATION_PACKETHASH` (0x6c2b1253) function"]
        pub fn get_signeddelegation_packethash(
            &self,
            input: SignedDelegation,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([108, 43, 18, 83], (input,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `GET_TRANSACTION_PACKETHASH` (0xa2e01f75) function"]
        pub fn get_transaction_packethash(
            &self,
            input: Transaction,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 224, 31, 117], (input,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `acceptPaymentMethod` (0xd5f3d856) function"]
        pub fn accept_payment_method(
            &self,
            payment_id: u16,
            payment_address: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 243, 216, 86], (payment_id, payment_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addMetaEvidence` (0xb6694b7b) function"]
        pub fn add_meta_evidence(
            &self,
            meta_evidence: String,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 105, 75, 123], meta_evidence)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addPaymentMethod` (0x7a982eb6) function"]
        pub fn add_payment_method(
            &self,
            payment_name: String,
            meta_evidence_id: u8,
            inital_enabled_token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [122, 152, 46, 182],
                    (payment_name, meta_evidence_id, inital_enabled_token),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `amountWithdrawable` (0x1165542b) function"]
        pub fn amount_withdrawable(
            &self,
            order_id: ethers::core::types::U256,
            beneficiary: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([17, 101, 84, 43], (order_id, beneficiary))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `arbitratorDataList` (0xec0e71ba) function"]
        pub fn arbitrator_data_list(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([236, 14, 113, 186], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `batchRoundWithdraw` (0x339ac67c) function"]
        pub fn batch_round_withdraw(
            &self,
            beneficiary: ethers::core::types::Address,
            order_id: ethers::core::types::U256,
            cursor: ethers::core::types::U256,
            count: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 154, 198, 124], (beneficiary, order_id, cursor, count))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `buyOrder` (0x64ac92f2) function"]
        pub fn buy_order(
            &self,
            payment_id: u16,
            seller: ethers::core::types::Address,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 172, 146, 242], (payment_id, seller, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `buyQuoteWithFees` (0xa1d77830) function"]
        pub fn buy_quote_with_fees(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([161, 215, 120, 48], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculateFee` (0x99a5d747) function"]
        pub fn calculate_fee(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([153, 165, 215, 71], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeArbitrator` (0xba7079ca) function"]
        pub fn change_arbitrator(
            &self,
            arbitrator: ethers::core::types::Address,
            arbitrator_extra_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 112, 121, 202], (arbitrator, arbitrator_extra_data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeConfirmTimeout` (0x50a4f450) function"]
        pub fn change_confirm_timeout(
            &self,
            timeout: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 164, 244, 80], timeout)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeFees` (0x6cda375b) function"]
        pub fn change_fees(
            &self,
            fees: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 218, 55, 91], fees)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeOrderTimeout` (0xc189fde6) function"]
        pub fn change_order_timeout(
            &self,
            timeout: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 137, 253, 230], timeout)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `completeOrder` (0xb6adaaff) function"]
        pub fn complete_order(
            &self,
            order_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 173, 170, 255], order_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `confirmPaid` (0x01e6e515) function"]
        pub fn confirm_paid(
            &self,
            order_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 230, 229, 21], order_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `confirmTimeout` (0xab8024a6) function"]
        pub fn confirm_timeout(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([171, 128, 36, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `contractInvoke` (0x5c6d9f0c) function"]
        pub fn contract_invoke(
            &self,
            batch: ::std::vec::Vec<Invocation>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 109, 159, 12], batch)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositTokens` (0x4b2dd8f4) function"]
        pub fn deposit_tokens(
            &self,
            payment_id: u16,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 45, 216, 244], (payment_id, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disablePaymentMethod` (0x89c46caf) function"]
        pub fn disable_payment_method(
            &self,
            payment_id: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 196, 108, 175], payment_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disputeOrder` (0xa09970b2) function"]
        pub fn dispute_order(
            &self,
            order_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 153, 112, 178], order_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disputes` (0x564a565d) function"]
        pub fn disputes(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::U256, u8, u16)>
        {
            self.0
                .method_hash([86, 74, 86, 93], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `domainHash` (0xdfe86ac5) function"]
        pub fn domain_hash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([223, 232, 106, 197], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enforceCaveat` (0x5068de4c) function"]
        pub fn enforce_caveat(
            &self,
            p0: ethers::core::types::Bytes,
            transaction: Transaction,
            p2: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([80, 104, 222, 76], (p0, transaction, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fundAppeal` (0x12b3a2c0) function"]
        pub fn fund_appeal(
            &self,
            order_id: ethers::core::types::U256,
            side: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 179, 162, 192], (order_id, side))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getContributions` (0x68c76ffd) function"]
        pub fn get_contributions(
            &self,
            order_id: ethers::core::types::U256,
            round: ethers::core::types::U256,
            contributor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, [ethers::core::types::U256; 3usize]>
        {
            self.0
                .method_hash([104, 199, 111, 253], (order_id, round, contributor))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCountOrders` (0x98f1c162) function"]
        pub fn get_count_orders(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([152, 241, 193, 98], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDelegationTypedDataHash` (0x97182ed6) function"]
        pub fn get_delegation_typed_data_hash(
            &self,
            delegation: Delegation,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([151, 24, 46, 214], (delegation,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEIP712DomainHash` (0xd327c1eb) function"]
        pub fn get_eip712_domain_hash(
            &self,
            contract_name: String,
            version: String,
            chain_id: ethers::core::types::U256,
            verifying_contract: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [211, 39, 193, 235],
                    (contract_name, version, chain_id, verifying_contract),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFeeAmount` (0x9704122c) function"]
        pub fn get_fee_amount(
            &self,
            order_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([151, 4, 18, 44], order_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getInvocationsTypedDataHash` (0x60b6d768) function"]
        pub fn get_invocations_typed_data_hash(
            &self,
            invocations: Invocations,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([96, 182, 215, 104], (invocations,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNonce` (0x89535803) function"]
        pub fn get_nonce(
            &self,
            intended_sender: ethers::core::types::Address,
            queue: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([137, 83, 88, 3], (intended_sender, queue))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNumberOfRounds` (0xfc6f8f16) function"]
        pub fn get_number_of_rounds(
            &self,
            order_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([252, 111, 143, 22], order_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoundInfo` (0x8a9bb02a) function"]
        pub fn get_round_info(
            &self,
            order_id: ethers::core::types::U256,
            round: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                [ethers::core::types::U256; 3usize],
                u8,
                ethers::core::types::U256,
                bool,
            ),
        > {
            self.0
                .method_hash([138, 155, 176, 42], (order_id, round))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `invoke` (0x2fad7efc) function"]
        pub fn invoke(
            &self,
            signed_invocations: ::std::vec::Vec<SignedInvocation>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 173, 126, 252], signed_invocations)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `loserStakeMultiplier` (0x1d512085) function"]
        pub fn loser_stake_multiplier(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([29, 81, 32, 133], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `metaEvidenceUpdates` (0x6cdc090f) function"]
        pub fn meta_evidence_updates(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([108, 220, 9, 15], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `orderTimeout` (0xba206004) function"]
        pub fn order_timeout(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([186, 32, 96, 4], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `orders` (0xa85c38ef) function"]
        pub fn orders(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                u8,
            ),
        > {
            self.0
                .method_hash([168, 92, 56, 239], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paymentMethods` (0x0cd4384d) function"]
        pub fn payment_methods(
            &self,
            p0: u16,
        ) -> ethers::contract::builders::ContractCall<M, (String, ethers::core::types::U256)>
        {
            self.0
                .method_hash([12, 212, 56, 77], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `protocolFeesSum` (0xfbef7045) function"]
        pub fn protocol_fees_sum(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([251, 239, 112, 69], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rule` (0x311a6c56) function"]
        pub fn rule(
            &self,
            dispute_id: ethers::core::types::U256,
            ruling: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 26, 108, 86], (dispute_id, ruling))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sharedStakeMultiplier` (0x41658341) function"]
        pub fn shared_stake_multiplier(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([65, 101, 131, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitEvidence` (0xa6a7f0eb) function"]
        pub fn submit_evidence(
            &self,
            order_id: ethers::core::types::U256,
            evidence: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 167, 240, 235], (order_id, evidence))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `timeoutByBuyer` (0x870026f1) function"]
        pub fn timeout_by_buyer(
            &self,
            order_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 0, 38, 241], order_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `timeoutBySeller` (0x382a2b86) function"]
        pub fn timeout_by_seller(
            &self,
            order_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 42, 43, 134], order_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenBalance` (0x1049334f) function"]
        pub fn token_balance(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([16, 73, 51, 79], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalPaymentMethods` (0x46639cca) function"]
        pub fn total_payment_methods(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([70, 99, 156, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tradeFees` (0x4ffd43c3) function"]
        pub fn trade_fees(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([79, 253, 67, 195], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updatePaymentMetaEvidence` (0x86c61ce5) function"]
        pub fn update_payment_meta_evidence(
            &self,
            payment_id: u16,
            meta_evidence_id: u8,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 198, 28, 229], (payment_id, meta_evidence_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updatePaymentName` (0xae1a38ed) function"]
        pub fn update_payment_name(
            &self,
            payment_id: u16,
            payment_name: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 26, 56, 237], (payment_id, payment_name))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateTokenEnabled` (0xabaa14ff) function"]
        pub fn update_token_enabled(
            &self,
            payment_id: u16,
            token: ethers::core::types::Address,
            enabled: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 170, 20, 255], (payment_id, token, enabled))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyDelegationSignature` (0x8a04499e) function"]
        pub fn verify_delegation_signature(
            &self,
            signed_delegation: SignedDelegation,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([138, 4, 73, 158], (signed_delegation,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyInvocationSignature` (0xcaced6c5) function"]
        pub fn verify_invocation_signature(
            &self,
            signed_invocation: SignedInvocation,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([202, 206, 214, 197], (signed_invocation,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `winnerStakeMultiplier` (0x7b943383) function"]
        pub fn winner_stake_multiplier(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([123, 148, 51, 131], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFees` (0xdd2cc3f3) function"]
        pub fn withdraw_fees(
            &self,
            amount: ethers::core::types::U256,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 44, 195, 243], (amount, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFeesAndRewards` (0x6e99a23a) function"]
        pub fn withdraw_fees_and_rewards(
            &self,
            beneficiary: ethers::core::types::Address,
            order_id: ethers::core::types::U256,
            round: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 153, 162, 58], (beneficiary, order_id, round))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawTokens` (0x06b091f9) function"]
        pub fn withdraw_tokens(
            &self,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 176, 145, 249], (token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AppealContribution` event"]
        pub fn appeal_contribution_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AppealContributionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BuyOrder` event"]
        pub fn buy_order_filter(&self) -> ethers::contract::builders::Event<M, BuyOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Dispute` event"]
        pub fn dispute_filter(&self) -> ethers::contract::builders::Event<M, DisputeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Evidence` event"]
        pub fn evidence_filter(&self) -> ethers::contract::builders::Event<M, EvidenceFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FeeWithdrawn` event"]
        pub fn fee_withdrawn_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FeeWithdrawnFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `HasPaidAppealFee` event"]
        pub fn has_paid_appeal_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, HasPaidAppealFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MetaEvidence` event"]
        pub fn meta_evidence_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MetaEvidenceFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OrderComplete` event"]
        pub fn order_complete_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OrderCompleteFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OrderResolved` event"]
        pub fn order_resolved_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OrderResolvedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paid` event"]
        pub fn paid_filter(&self) -> ethers::contract::builders::Event<M, PaidFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PaymentMethodUpdate` event"]
        pub fn payment_method_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PaymentMethodUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Ruling` event"]
        pub fn ruling_filter(&self) -> ethers::contract::builders::Event<M, RulingFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SellerDeposit` event"]
        pub fn seller_deposit_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SellerDepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SellerPaymentDisabled` event"]
        pub fn seller_payment_disabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SellerPaymentDisabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SellerPaymentMethod` event"]
        pub fn seller_payment_method_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SellerPaymentMethodFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SellerWithdraw` event"]
        pub fn seller_withdraw_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SellerWithdrawFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TimedOutByBuyer` event"]
        pub fn timed_out_by_buyer_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TimedOutByBuyerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TimedOutBySeller` event"]
        pub fn timed_out_by_seller_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TimedOutBySellerFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, UnipeerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Unipeer<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "AppealContribution",
        abi = "AppealContribution(uint256,uint8,address,uint256)"
    )]
    pub struct AppealContributionFilter {
        #[ethevent(indexed)]
        pub order_id: ethers::core::types::U256,
        pub party: u8,
        pub contributor: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "BuyOrder",
        abi = "BuyOrder(uint256,address,uint16,address,address,uint256,uint256)"
    )]
    pub struct BuyOrderFilter {
        pub order_id: ethers::core::types::U256,
        pub buyer: ethers::core::types::Address,
        pub payment_id: u16,
        pub seller: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub fee_amount: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Dispute", abi = "Dispute(address,uint256,uint256,uint256)")]
    pub struct DisputeFilter {
        #[ethevent(indexed)]
        pub arbitrator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub dispute_id: ethers::core::types::U256,
        pub meta_evidence_id: ethers::core::types::U256,
        pub evidence_group_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Evidence", abi = "Evidence(address,uint256,address,string)")]
    pub struct EvidenceFilter {
        #[ethevent(indexed)]
        pub arbitrator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub evidence_group_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub party: ethers::core::types::Address,
        pub evidence: String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "FeeWithdrawn", abi = "FeeWithdrawn(uint256)")]
    pub struct FeeWithdrawnFilter {
        pub amount: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "HasPaidAppealFee", abi = "HasPaidAppealFee(uint256,uint8)")]
    pub struct HasPaidAppealFeeFilter {
        #[ethevent(indexed)]
        pub order_id: ethers::core::types::U256,
        pub party: u8,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "MetaEvidence", abi = "MetaEvidence(uint256,string)")]
    pub struct MetaEvidenceFilter {
        #[ethevent(indexed)]
        pub meta_evidence_id: ethers::core::types::U256,
        pub evidence: String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "OrderComplete", abi = "OrderComplete(uint256)")]
    pub struct OrderCompleteFilter {
        pub order_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "OrderResolved", abi = "OrderResolved(uint256)")]
    pub struct OrderResolvedFilter {
        pub order_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Paid", abi = "Paid(uint256)")]
    pub struct PaidFilter {
        pub order_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "PaymentMethodUpdate",
        abi = "PaymentMethodUpdate(uint16,string,uint256)"
    )]
    pub struct PaymentMethodUpdateFilter {
        pub payment_id: u16,
        pub payment_name: String,
        pub meta_evidence_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Ruling", abi = "Ruling(address,uint256,uint256)")]
    pub struct RulingFilter {
        #[ethevent(indexed)]
        pub arbitrator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub dispute_id: ethers::core::types::U256,
        pub ruling: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "SellerDeposit", abi = "SellerDeposit(address,address,uint256)")]
    pub struct SellerDepositFilter {
        pub sender: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "SellerPaymentDisabled",
        abi = "SellerPaymentDisabled(address,uint16)"
    )]
    pub struct SellerPaymentDisabledFilter {
        pub sender: ethers::core::types::Address,
        pub payment_id: u16,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "SellerPaymentMethod",
        abi = "SellerPaymentMethod(address,uint16,string)"
    )]
    pub struct SellerPaymentMethodFilter {
        pub sender: ethers::core::types::Address,
        pub payment_id: u16,
        pub payment_address: String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "SellerWithdraw",
        abi = "SellerWithdraw(address,address,uint256)"
    )]
    pub struct SellerWithdrawFilter {
        pub sender: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "TimedOutByBuyer", abi = "TimedOutByBuyer(uint256)")]
    pub struct TimedOutByBuyerFilter {
        pub order_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "TimedOutBySeller", abi = "TimedOutBySeller(uint256)")]
    pub struct TimedOutBySellerFilter {
        pub order_id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UnipeerEvents {
        AppealContributionFilter(AppealContributionFilter),
        BuyOrderFilter(BuyOrderFilter),
        DisputeFilter(DisputeFilter),
        EvidenceFilter(EvidenceFilter),
        FeeWithdrawnFilter(FeeWithdrawnFilter),
        HasPaidAppealFeeFilter(HasPaidAppealFeeFilter),
        MetaEvidenceFilter(MetaEvidenceFilter),
        OrderCompleteFilter(OrderCompleteFilter),
        OrderResolvedFilter(OrderResolvedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PaidFilter(PaidFilter),
        PaymentMethodUpdateFilter(PaymentMethodUpdateFilter),
        RulingFilter(RulingFilter),
        SellerDepositFilter(SellerDepositFilter),
        SellerPaymentDisabledFilter(SellerPaymentDisabledFilter),
        SellerPaymentMethodFilter(SellerPaymentMethodFilter),
        SellerWithdrawFilter(SellerWithdrawFilter),
        TimedOutByBuyerFilter(TimedOutByBuyerFilter),
        TimedOutBySellerFilter(TimedOutBySellerFilter),
    }
    impl ethers::contract::EthLogDecode for UnipeerEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AppealContributionFilter::decode_log(log) {
                return Ok(UnipeerEvents::AppealContributionFilter(decoded));
            }
            if let Ok(decoded) = BuyOrderFilter::decode_log(log) {
                return Ok(UnipeerEvents::BuyOrderFilter(decoded));
            }
            if let Ok(decoded) = DisputeFilter::decode_log(log) {
                return Ok(UnipeerEvents::DisputeFilter(decoded));
            }
            if let Ok(decoded) = EvidenceFilter::decode_log(log) {
                return Ok(UnipeerEvents::EvidenceFilter(decoded));
            }
            if let Ok(decoded) = FeeWithdrawnFilter::decode_log(log) {
                return Ok(UnipeerEvents::FeeWithdrawnFilter(decoded));
            }
            if let Ok(decoded) = HasPaidAppealFeeFilter::decode_log(log) {
                return Ok(UnipeerEvents::HasPaidAppealFeeFilter(decoded));
            }
            if let Ok(decoded) = MetaEvidenceFilter::decode_log(log) {
                return Ok(UnipeerEvents::MetaEvidenceFilter(decoded));
            }
            if let Ok(decoded) = OrderCompleteFilter::decode_log(log) {
                return Ok(UnipeerEvents::OrderCompleteFilter(decoded));
            }
            if let Ok(decoded) = OrderResolvedFilter::decode_log(log) {
                return Ok(UnipeerEvents::OrderResolvedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(UnipeerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PaidFilter::decode_log(log) {
                return Ok(UnipeerEvents::PaidFilter(decoded));
            }
            if let Ok(decoded) = PaymentMethodUpdateFilter::decode_log(log) {
                return Ok(UnipeerEvents::PaymentMethodUpdateFilter(decoded));
            }
            if let Ok(decoded) = RulingFilter::decode_log(log) {
                return Ok(UnipeerEvents::RulingFilter(decoded));
            }
            if let Ok(decoded) = SellerDepositFilter::decode_log(log) {
                return Ok(UnipeerEvents::SellerDepositFilter(decoded));
            }
            if let Ok(decoded) = SellerPaymentDisabledFilter::decode_log(log) {
                return Ok(UnipeerEvents::SellerPaymentDisabledFilter(decoded));
            }
            if let Ok(decoded) = SellerPaymentMethodFilter::decode_log(log) {
                return Ok(UnipeerEvents::SellerPaymentMethodFilter(decoded));
            }
            if let Ok(decoded) = SellerWithdrawFilter::decode_log(log) {
                return Ok(UnipeerEvents::SellerWithdrawFilter(decoded));
            }
            if let Ok(decoded) = TimedOutByBuyerFilter::decode_log(log) {
                return Ok(UnipeerEvents::TimedOutByBuyerFilter(decoded));
            }
            if let Ok(decoded) = TimedOutBySellerFilter::decode_log(log) {
                return Ok(UnipeerEvents::TimedOutBySellerFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for UnipeerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UnipeerEvents::AppealContributionFilter(element) => element.fmt(f),
                UnipeerEvents::BuyOrderFilter(element) => element.fmt(f),
                UnipeerEvents::DisputeFilter(element) => element.fmt(f),
                UnipeerEvents::EvidenceFilter(element) => element.fmt(f),
                UnipeerEvents::FeeWithdrawnFilter(element) => element.fmt(f),
                UnipeerEvents::HasPaidAppealFeeFilter(element) => element.fmt(f),
                UnipeerEvents::MetaEvidenceFilter(element) => element.fmt(f),
                UnipeerEvents::OrderCompleteFilter(element) => element.fmt(f),
                UnipeerEvents::OrderResolvedFilter(element) => element.fmt(f),
                UnipeerEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                UnipeerEvents::PaidFilter(element) => element.fmt(f),
                UnipeerEvents::PaymentMethodUpdateFilter(element) => element.fmt(f),
                UnipeerEvents::RulingFilter(element) => element.fmt(f),
                UnipeerEvents::SellerDepositFilter(element) => element.fmt(f),
                UnipeerEvents::SellerPaymentDisabledFilter(element) => element.fmt(f),
                UnipeerEvents::SellerPaymentMethodFilter(element) => element.fmt(f),
                UnipeerEvents::SellerWithdrawFilter(element) => element.fmt(f),
                UnipeerEvents::TimedOutByBuyerFilter(element) => element.fmt(f),
                UnipeerEvents::TimedOutBySellerFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `GET_CAVEAT_ARRAY_PACKETHASH` function with signature `GET_CAVEAT_ARRAY_PACKETHASH((address,bytes)[])` and selector `[123, 87, 123, 88]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "GET_CAVEAT_ARRAY_PACKETHASH",
        abi = "GET_CAVEAT_ARRAY_PACKETHASH((address,bytes)[])"
    )]
    pub struct GetCaveatArrayPackethashCall {
        pub input: ::std::vec::Vec<Caveat>,
    }
    #[doc = "Container type for all input parameters for the `GET_CAVEAT_PACKETHASH` function with signature `GET_CAVEAT_PACKETHASH((address,bytes))` and selector `[8, 170, 246, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "GET_CAVEAT_PACKETHASH",
        abi = "GET_CAVEAT_PACKETHASH((address,bytes))"
    )]
    pub struct GetCaveatPackethashCall {
        pub input: Caveat,
    }
    #[doc = "Container type for all input parameters for the `GET_DELEGATION_PACKETHASH` function with signature `GET_DELEGATION_PACKETHASH((address,bytes32,(address,bytes)[]))` and selector `[47, 82, 162, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "GET_DELEGATION_PACKETHASH",
        abi = "GET_DELEGATION_PACKETHASH((address,bytes32,(address,bytes)[]))"
    )]
    pub struct GetDelegationPackethashCall {
        pub input: Delegation,
    }
    #[doc = "Container type for all input parameters for the `GET_INVOCATIONS_PACKETHASH` function with signature `GET_INVOCATIONS_PACKETHASH((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)))` and selector `[114, 52, 238, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "GET_INVOCATIONS_PACKETHASH",
        abi = "GET_INVOCATIONS_PACKETHASH((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)))"
    )]
    pub struct GetInvocationsPackethashCall {
        pub input: Invocations,
    }
    #[doc = "Container type for all input parameters for the `GET_INVOCATION_ARRAY_PACKETHASH` function with signature `GET_INVOCATION_ARRAY_PACKETHASH(((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[])` and selector `[58, 72, 24, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "GET_INVOCATION_ARRAY_PACKETHASH",
        abi = "GET_INVOCATION_ARRAY_PACKETHASH(((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[])"
    )]
    pub struct GetInvocationArrayPackethashCall {
        pub input: ::std::vec::Vec<Invocation>,
    }
    #[doc = "Container type for all input parameters for the `GET_INVOCATION_PACKETHASH` function with signature `GET_INVOCATION_PACKETHASH(((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[]))` and selector `[92, 241, 178, 74]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "GET_INVOCATION_PACKETHASH",
        abi = "GET_INVOCATION_PACKETHASH(((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[]))"
    )]
    pub struct GetInvocationPackethashCall {
        pub input: Invocation,
    }
    #[doc = "Container type for all input parameters for the `GET_REPLAYPROTECTION_PACKETHASH` function with signature `GET_REPLAYPROTECTION_PACKETHASH((uint256,uint256))` and selector `[111, 150, 88, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "GET_REPLAYPROTECTION_PACKETHASH",
        abi = "GET_REPLAYPROTECTION_PACKETHASH((uint256,uint256))"
    )]
    pub struct GetReplayprotectionPackethashCall {
        pub input: ReplayProtection,
    }
    #[doc = "Container type for all input parameters for the `GET_SIGNEDDELEGATION_ARRAY_PACKETHASH` function with signature `GET_SIGNEDDELEGATION_ARRAY_PACKETHASH(((address,bytes32,(address,bytes)[]),bytes)[])` and selector `[115, 111, 124, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "GET_SIGNEDDELEGATION_ARRAY_PACKETHASH",
        abi = "GET_SIGNEDDELEGATION_ARRAY_PACKETHASH(((address,bytes32,(address,bytes)[]),bytes)[])"
    )]
    pub struct GetSigneddelegationArrayPackethashCall {
        pub input: ::std::vec::Vec<SignedDelegation>,
    }
    #[doc = "Container type for all input parameters for the `GET_SIGNEDDELEGATION_PACKETHASH` function with signature `GET_SIGNEDDELEGATION_PACKETHASH(((address,bytes32,(address,bytes)[]),bytes))` and selector `[108, 43, 18, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "GET_SIGNEDDELEGATION_PACKETHASH",
        abi = "GET_SIGNEDDELEGATION_PACKETHASH(((address,bytes32,(address,bytes)[]),bytes))"
    )]
    pub struct GetSigneddelegationPackethashCall {
        pub input: SignedDelegation,
    }
    #[doc = "Container type for all input parameters for the `GET_TRANSACTION_PACKETHASH` function with signature `GET_TRANSACTION_PACKETHASH((address,uint256,bytes))` and selector `[162, 224, 31, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "GET_TRANSACTION_PACKETHASH",
        abi = "GET_TRANSACTION_PACKETHASH((address,uint256,bytes))"
    )]
    pub struct GetTransactionPackethashCall {
        pub input: Transaction,
    }
    #[doc = "Container type for all input parameters for the `acceptPaymentMethod` function with signature `acceptPaymentMethod(uint16,string)` and selector `[213, 243, 216, 86]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "acceptPaymentMethod",
        abi = "acceptPaymentMethod(uint16,string)"
    )]
    pub struct AcceptPaymentMethodCall {
        pub payment_id: u16,
        pub payment_address: String,
    }
    #[doc = "Container type for all input parameters for the `addMetaEvidence` function with signature `addMetaEvidence(string)` and selector `[182, 105, 75, 123]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addMetaEvidence", abi = "addMetaEvidence(string)")]
    pub struct AddMetaEvidenceCall {
        pub meta_evidence: String,
    }
    #[doc = "Container type for all input parameters for the `addPaymentMethod` function with signature `addPaymentMethod(string,uint8,address)` and selector `[122, 152, 46, 182]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "addPaymentMethod",
        abi = "addPaymentMethod(string,uint8,address)"
    )]
    pub struct AddPaymentMethodCall {
        pub payment_name: String,
        pub meta_evidence_id: u8,
        pub inital_enabled_token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    #[doc = "Container type for all input parameters for the `amountWithdrawable` function with signature `amountWithdrawable(uint256,address)` and selector `[17, 101, 84, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "amountWithdrawable",
        abi = "amountWithdrawable(uint256,address)"
    )]
    pub struct AmountWithdrawableCall {
        pub order_id: ethers::core::types::U256,
        pub beneficiary: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `arbitratorDataList` function with signature `arbitratorDataList(uint256)` and selector `[236, 14, 113, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "arbitratorDataList", abi = "arbitratorDataList(uint256)")]
    pub struct ArbitratorDataListCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `batchRoundWithdraw` function with signature `batchRoundWithdraw(address,uint256,uint256,uint256)` and selector `[51, 154, 198, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "batchRoundWithdraw",
        abi = "batchRoundWithdraw(address,uint256,uint256,uint256)"
    )]
    pub struct BatchRoundWithdrawCall {
        pub beneficiary: ethers::core::types::Address,
        pub order_id: ethers::core::types::U256,
        pub cursor: ethers::core::types::U256,
        pub count: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `buyOrder` function with signature `buyOrder(uint16,address,address,uint256)` and selector `[100, 172, 146, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "buyOrder", abi = "buyOrder(uint16,address,address,uint256)")]
    pub struct BuyOrderCall {
        pub payment_id: u16,
        pub seller: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `buyQuoteWithFees` function with signature `buyQuoteWithFees(uint256)` and selector `[161, 215, 120, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "buyQuoteWithFees", abi = "buyQuoteWithFees(uint256)")]
    pub struct BuyQuoteWithFeesCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `calculateFee` function with signature `calculateFee(uint256)` and selector `[153, 165, 215, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "calculateFee", abi = "calculateFee(uint256)")]
    pub struct CalculateFeeCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `changeArbitrator` function with signature `changeArbitrator(address,bytes)` and selector `[186, 112, 121, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "changeArbitrator", abi = "changeArbitrator(address,bytes)")]
    pub struct ChangeArbitratorCall {
        pub arbitrator: ethers::core::types::Address,
        pub arbitrator_extra_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `changeConfirmTimeout` function with signature `changeConfirmTimeout(uint256)` and selector `[80, 164, 244, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "changeConfirmTimeout", abi = "changeConfirmTimeout(uint256)")]
    pub struct ChangeConfirmTimeoutCall {
        pub timeout: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `changeFees` function with signature `changeFees(uint256)` and selector `[108, 218, 55, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "changeFees", abi = "changeFees(uint256)")]
    pub struct ChangeFeesCall {
        pub fees: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `changeOrderTimeout` function with signature `changeOrderTimeout(uint256)` and selector `[193, 137, 253, 230]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "changeOrderTimeout", abi = "changeOrderTimeout(uint256)")]
    pub struct ChangeOrderTimeoutCall {
        pub timeout: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `completeOrder` function with signature `completeOrder(uint256)` and selector `[182, 173, 170, 255]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "completeOrder", abi = "completeOrder(uint256)")]
    pub struct CompleteOrderCall {
        pub order_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `confirmPaid` function with signature `confirmPaid(uint256)` and selector `[1, 230, 229, 21]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "confirmPaid", abi = "confirmPaid(uint256)")]
    pub struct ConfirmPaidCall {
        pub order_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `confirmTimeout` function with signature `confirmTimeout()` and selector `[171, 128, 36, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "confirmTimeout", abi = "confirmTimeout()")]
    pub struct ConfirmTimeoutCall;
    #[doc = "Container type for all input parameters for the `contractInvoke` function with signature `contractInvoke(((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[])` and selector `[92, 109, 159, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "contractInvoke",
        abi = "contractInvoke(((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[])"
    )]
    pub struct ContractInvokeCall {
        pub batch: ::std::vec::Vec<Invocation>,
    }
    #[doc = "Container type for all input parameters for the `depositTokens` function with signature `depositTokens(uint16,address,uint256)` and selector `[75, 45, 216, 244]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "depositTokens", abi = "depositTokens(uint16,address,uint256)")]
    pub struct DepositTokensCall {
        pub payment_id: u16,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `disablePaymentMethod` function with signature `disablePaymentMethod(uint16)` and selector `[137, 196, 108, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "disablePaymentMethod", abi = "disablePaymentMethod(uint16)")]
    pub struct DisablePaymentMethodCall {
        pub payment_id: u16,
    }
    #[doc = "Container type for all input parameters for the `disputeOrder` function with signature `disputeOrder(uint256)` and selector `[160, 153, 112, 178]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "disputeOrder", abi = "disputeOrder(uint256)")]
    pub struct DisputeOrderCall {
        pub order_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `disputes` function with signature `disputes(uint256)` and selector `[86, 74, 86, 93]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "disputes", abi = "disputes(uint256)")]
    pub struct DisputesCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `domainHash` function with signature `domainHash()` and selector `[223, 232, 106, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "domainHash", abi = "domainHash()")]
    pub struct DomainHashCall;
    #[doc = "Container type for all input parameters for the `enforceCaveat` function with signature `enforceCaveat(bytes,(address,uint256,bytes),bytes32)` and selector `[80, 104, 222, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "enforceCaveat",
        abi = "enforceCaveat(bytes,(address,uint256,bytes),bytes32)"
    )]
    pub struct EnforceCaveatCall {
        pub p0: ethers::core::types::Bytes,
        pub transaction: Transaction,
        pub p2: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `fundAppeal` function with signature `fundAppeal(uint256,uint8)` and selector `[18, 179, 162, 192]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "fundAppeal", abi = "fundAppeal(uint256,uint8)")]
    pub struct FundAppealCall {
        pub order_id: ethers::core::types::U256,
        pub side: u8,
    }
    #[doc = "Container type for all input parameters for the `getContributions` function with signature `getContributions(uint256,uint256,address)` and selector `[104, 199, 111, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "getContributions",
        abi = "getContributions(uint256,uint256,address)"
    )]
    pub struct GetContributionsCall {
        pub order_id: ethers::core::types::U256,
        pub round: ethers::core::types::U256,
        pub contributor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCountOrders` function with signature `getCountOrders()` and selector `[152, 241, 193, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCountOrders", abi = "getCountOrders()")]
    pub struct GetCountOrdersCall;
    #[doc = "Container type for all input parameters for the `getDelegationTypedDataHash` function with signature `getDelegationTypedDataHash((address,bytes32,(address,bytes)[]))` and selector `[151, 24, 46, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "getDelegationTypedDataHash",
        abi = "getDelegationTypedDataHash((address,bytes32,(address,bytes)[]))"
    )]
    pub struct GetDelegationTypedDataHashCall {
        pub delegation: Delegation,
    }
    #[doc = "Container type for all input parameters for the `getEIP712DomainHash` function with signature `getEIP712DomainHash(string,string,uint256,address)` and selector `[211, 39, 193, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "getEIP712DomainHash",
        abi = "getEIP712DomainHash(string,string,uint256,address)"
    )]
    pub struct GetEIP712DomainHashCall {
        pub contract_name: String,
        pub version: String,
        pub chain_id: ethers::core::types::U256,
        pub verifying_contract: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getFeeAmount` function with signature `getFeeAmount(uint256)` and selector `[151, 4, 18, 44]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getFeeAmount", abi = "getFeeAmount(uint256)")]
    pub struct GetFeeAmountCall {
        pub order_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getInvocationsTypedDataHash` function with signature `getInvocationsTypedDataHash((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)))` and selector `[96, 182, 215, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "getInvocationsTypedDataHash",
        abi = "getInvocationsTypedDataHash((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)))"
    )]
    pub struct GetInvocationsTypedDataHashCall {
        pub invocations: Invocations,
    }
    #[doc = "Container type for all input parameters for the `getNonce` function with signature `getNonce(address,uint256)` and selector `[137, 83, 88, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNonce", abi = "getNonce(address,uint256)")]
    pub struct GetNonceCall {
        pub intended_sender: ethers::core::types::Address,
        pub queue: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getNumberOfRounds` function with signature `getNumberOfRounds(uint256)` and selector `[252, 111, 143, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNumberOfRounds", abi = "getNumberOfRounds(uint256)")]
    pub struct GetNumberOfRoundsCall {
        pub order_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getRoundInfo` function with signature `getRoundInfo(uint256,uint256)` and selector `[138, 155, 176, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRoundInfo", abi = "getRoundInfo(uint256,uint256)")]
    pub struct GetRoundInfoCall {
        pub order_id: ethers::core::types::U256,
        pub round: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `invoke` function with signature `invoke(((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)),bytes)[])` and selector `[47, 173, 126, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "invoke",
        abi = "invoke(((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)),bytes)[])"
    )]
    pub struct InvokeCall {
        pub signed_invocations: ::std::vec::Vec<SignedInvocation>,
    }
    #[doc = "Container type for all input parameters for the `loserStakeMultiplier` function with signature `loserStakeMultiplier()` and selector `[29, 81, 32, 133]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "loserStakeMultiplier", abi = "loserStakeMultiplier()")]
    pub struct LoserStakeMultiplierCall;
    #[doc = "Container type for all input parameters for the `metaEvidenceUpdates` function with signature `metaEvidenceUpdates()` and selector `[108, 220, 9, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "metaEvidenceUpdates", abi = "metaEvidenceUpdates()")]
    pub struct MetaEvidenceUpdatesCall;
    #[doc = "Container type for all input parameters for the `orderTimeout` function with signature `orderTimeout()` and selector `[186, 32, 96, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "orderTimeout", abi = "orderTimeout()")]
    pub struct OrderTimeoutCall;
    #[doc = "Container type for all input parameters for the `orders` function with signature `orders(uint256)` and selector `[168, 92, 56, 239]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "orders", abi = "orders(uint256)")]
    pub struct OrdersCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `paymentMethods` function with signature `paymentMethods(uint16)` and selector `[12, 212, 56, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "paymentMethods", abi = "paymentMethods(uint16)")]
    pub struct PaymentMethodsCall(pub u16);
    #[doc = "Container type for all input parameters for the `protocolFeesSum` function with signature `protocolFeesSum()` and selector `[251, 239, 112, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "protocolFeesSum", abi = "protocolFeesSum()")]
    pub struct ProtocolFeesSumCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `rule` function with signature `rule(uint256,uint256)` and selector `[49, 26, 108, 86]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "rule", abi = "rule(uint256,uint256)")]
    pub struct RuleCall {
        pub dispute_id: ethers::core::types::U256,
        pub ruling: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `sharedStakeMultiplier` function with signature `sharedStakeMultiplier()` and selector `[65, 101, 131, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "sharedStakeMultiplier", abi = "sharedStakeMultiplier()")]
    pub struct SharedStakeMultiplierCall;
    #[doc = "Container type for all input parameters for the `submitEvidence` function with signature `submitEvidence(uint256,string)` and selector `[166, 167, 240, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "submitEvidence", abi = "submitEvidence(uint256,string)")]
    pub struct SubmitEvidenceCall {
        pub order_id: ethers::core::types::U256,
        pub evidence: String,
    }
    #[doc = "Container type for all input parameters for the `timeoutByBuyer` function with signature `timeoutByBuyer(uint256)` and selector `[135, 0, 38, 241]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "timeoutByBuyer", abi = "timeoutByBuyer(uint256)")]
    pub struct TimeoutByBuyerCall {
        pub order_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `timeoutBySeller` function with signature `timeoutBySeller(uint256)` and selector `[56, 42, 43, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "timeoutBySeller", abi = "timeoutBySeller(uint256)")]
    pub struct TimeoutBySellerCall {
        pub order_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `tokenBalance` function with signature `tokenBalance(address,address)` and selector `[16, 73, 51, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tokenBalance", abi = "tokenBalance(address,address)")]
    pub struct TokenBalanceCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `totalPaymentMethods` function with signature `totalPaymentMethods()` and selector `[70, 99, 156, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "totalPaymentMethods", abi = "totalPaymentMethods()")]
    pub struct TotalPaymentMethodsCall;
    #[doc = "Container type for all input parameters for the `tradeFees` function with signature `tradeFees()` and selector `[79, 253, 67, 195]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "tradeFees", abi = "tradeFees()")]
    pub struct TradeFeesCall;
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `updatePaymentMetaEvidence` function with signature `updatePaymentMetaEvidence(uint16,uint8)` and selector `[134, 198, 28, 229]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "updatePaymentMetaEvidence",
        abi = "updatePaymentMetaEvidence(uint16,uint8)"
    )]
    pub struct UpdatePaymentMetaEvidenceCall {
        pub payment_id: u16,
        pub meta_evidence_id: u8,
    }
    #[doc = "Container type for all input parameters for the `updatePaymentName` function with signature `updatePaymentName(uint16,string)` and selector `[174, 26, 56, 237]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "updatePaymentName", abi = "updatePaymentName(uint16,string)")]
    pub struct UpdatePaymentNameCall {
        pub payment_id: u16,
        pub payment_name: String,
    }
    #[doc = "Container type for all input parameters for the `updateTokenEnabled` function with signature `updateTokenEnabled(uint16,address,bool)` and selector `[171, 170, 20, 255]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "updateTokenEnabled",
        abi = "updateTokenEnabled(uint16,address,bool)"
    )]
    pub struct UpdateTokenEnabledCall {
        pub payment_id: u16,
        pub token: ethers::core::types::Address,
        pub enabled: bool,
    }
    #[doc = "Container type for all input parameters for the `verifyDelegationSignature` function with signature `verifyDelegationSignature(((address,bytes32,(address,bytes)[]),bytes))` and selector `[138, 4, 73, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "verifyDelegationSignature",
        abi = "verifyDelegationSignature(((address,bytes32,(address,bytes)[]),bytes))"
    )]
    pub struct VerifyDelegationSignatureCall {
        pub signed_delegation: SignedDelegation,
    }
    #[doc = "Container type for all input parameters for the `verifyInvocationSignature` function with signature `verifyInvocationSignature(((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)),bytes))` and selector `[202, 206, 214, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "verifyInvocationSignature",
        abi = "verifyInvocationSignature(((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)),bytes))"
    )]
    pub struct VerifyInvocationSignatureCall {
        pub signed_invocation: SignedInvocation,
    }
    #[doc = "Container type for all input parameters for the `winnerStakeMultiplier` function with signature `winnerStakeMultiplier()` and selector `[123, 148, 51, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "winnerStakeMultiplier", abi = "winnerStakeMultiplier()")]
    pub struct WinnerStakeMultiplierCall;
    #[doc = "Container type for all input parameters for the `withdrawFees` function with signature `withdrawFees(uint256,address)` and selector `[221, 44, 195, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdrawFees", abi = "withdrawFees(uint256,address)")]
    pub struct WithdrawFeesCall {
        pub amount: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `withdrawFeesAndRewards` function with signature `withdrawFeesAndRewards(address,uint256,uint256)` and selector `[110, 153, 162, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "withdrawFeesAndRewards",
        abi = "withdrawFeesAndRewards(address,uint256,uint256)"
    )]
    pub struct WithdrawFeesAndRewardsCall {
        pub beneficiary: ethers::core::types::Address,
        pub order_id: ethers::core::types::U256,
        pub round: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdrawTokens` function with signature `withdrawTokens(address,uint256)` and selector `[6, 176, 145, 249]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdrawTokens", abi = "withdrawTokens(address,uint256)")]
    pub struct WithdrawTokensCall {
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UnipeerCalls {
        GetCaveatArrayPackethash(GetCaveatArrayPackethashCall),
        GetCaveatPackethash(GetCaveatPackethashCall),
        GetDelegationPackethash(GetDelegationPackethashCall),
        GetInvocationsPackethash(GetInvocationsPackethashCall),
        GetInvocationArrayPackethash(GetInvocationArrayPackethashCall),
        GetInvocationPackethash(GetInvocationPackethashCall),
        GetReplayprotectionPackethash(GetReplayprotectionPackethashCall),
        GetSigneddelegationArrayPackethash(GetSigneddelegationArrayPackethashCall),
        GetSigneddelegationPackethash(GetSigneddelegationPackethashCall),
        GetTransactionPackethash(GetTransactionPackethashCall),
        AcceptPaymentMethod(AcceptPaymentMethodCall),
        AddMetaEvidence(AddMetaEvidenceCall),
        AddPaymentMethod(AddPaymentMethodCall),
        Admin(AdminCall),
        AmountWithdrawable(AmountWithdrawableCall),
        ArbitratorDataList(ArbitratorDataListCall),
        BatchRoundWithdraw(BatchRoundWithdrawCall),
        BuyOrder(BuyOrderCall),
        BuyQuoteWithFees(BuyQuoteWithFeesCall),
        CalculateFee(CalculateFeeCall),
        ChangeArbitrator(ChangeArbitratorCall),
        ChangeConfirmTimeout(ChangeConfirmTimeoutCall),
        ChangeFees(ChangeFeesCall),
        ChangeOrderTimeout(ChangeOrderTimeoutCall),
        CompleteOrder(CompleteOrderCall),
        ConfirmPaid(ConfirmPaidCall),
        ConfirmTimeout(ConfirmTimeoutCall),
        ContractInvoke(ContractInvokeCall),
        DepositTokens(DepositTokensCall),
        DisablePaymentMethod(DisablePaymentMethodCall),
        DisputeOrder(DisputeOrderCall),
        Disputes(DisputesCall),
        DomainHash(DomainHashCall),
        EnforceCaveat(EnforceCaveatCall),
        FundAppeal(FundAppealCall),
        GetContributions(GetContributionsCall),
        GetCountOrders(GetCountOrdersCall),
        GetDelegationTypedDataHash(GetDelegationTypedDataHashCall),
        GetEIP712DomainHash(GetEIP712DomainHashCall),
        GetFeeAmount(GetFeeAmountCall),
        GetInvocationsTypedDataHash(GetInvocationsTypedDataHashCall),
        GetNonce(GetNonceCall),
        GetNumberOfRounds(GetNumberOfRoundsCall),
        GetRoundInfo(GetRoundInfoCall),
        Invoke(InvokeCall),
        LoserStakeMultiplier(LoserStakeMultiplierCall),
        MetaEvidenceUpdates(MetaEvidenceUpdatesCall),
        OrderTimeout(OrderTimeoutCall),
        Orders(OrdersCall),
        Owner(OwnerCall),
        PaymentMethods(PaymentMethodsCall),
        ProtocolFeesSum(ProtocolFeesSumCall),
        RenounceOwnership(RenounceOwnershipCall),
        Rule(RuleCall),
        SharedStakeMultiplier(SharedStakeMultiplierCall),
        SubmitEvidence(SubmitEvidenceCall),
        TimeoutByBuyer(TimeoutByBuyerCall),
        TimeoutBySeller(TimeoutBySellerCall),
        TokenBalance(TokenBalanceCall),
        TotalPaymentMethods(TotalPaymentMethodsCall),
        TradeFees(TradeFeesCall),
        TransferOwnership(TransferOwnershipCall),
        UpdatePaymentMetaEvidence(UpdatePaymentMetaEvidenceCall),
        UpdatePaymentName(UpdatePaymentNameCall),
        UpdateTokenEnabled(UpdateTokenEnabledCall),
        VerifyDelegationSignature(VerifyDelegationSignatureCall),
        VerifyInvocationSignature(VerifyInvocationSignatureCall),
        WinnerStakeMultiplier(WinnerStakeMultiplierCall),
        WithdrawFees(WithdrawFeesCall),
        WithdrawFeesAndRewards(WithdrawFeesAndRewardsCall),
        WithdrawTokens(WithdrawTokensCall),
    }
    impl ethers::core::abi::AbiDecode for UnipeerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetCaveatArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::GetCaveatArrayPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetCaveatPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::GetCaveatPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetDelegationPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::GetDelegationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationsPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::GetInvocationsPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::GetInvocationArrayPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::GetInvocationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetReplayprotectionPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::GetReplayprotectionPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetSigneddelegationArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::GetSigneddelegationArrayPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetSigneddelegationPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::GetSigneddelegationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetTransactionPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::GetTransactionPackethash(decoded));
            }
            if let Ok(decoded) =
                <AcceptPaymentMethodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::AcceptPaymentMethod(decoded));
            }
            if let Ok(decoded) =
                <AddMetaEvidenceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::AddMetaEvidence(decoded));
            }
            if let Ok(decoded) =
                <AddPaymentMethodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::AddPaymentMethod(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <AmountWithdrawableCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::AmountWithdrawable(decoded));
            }
            if let Ok(decoded) =
                <ArbitratorDataListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::ArbitratorDataList(decoded));
            }
            if let Ok(decoded) =
                <BatchRoundWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::BatchRoundWithdraw(decoded));
            }
            if let Ok(decoded) =
                <BuyOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::BuyOrder(decoded));
            }
            if let Ok(decoded) =
                <BuyQuoteWithFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::BuyQuoteWithFees(decoded));
            }
            if let Ok(decoded) =
                <CalculateFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::CalculateFee(decoded));
            }
            if let Ok(decoded) =
                <ChangeArbitratorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::ChangeArbitrator(decoded));
            }
            if let Ok(decoded) =
                <ChangeConfirmTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::ChangeConfirmTimeout(decoded));
            }
            if let Ok(decoded) =
                <ChangeFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::ChangeFees(decoded));
            }
            if let Ok(decoded) =
                <ChangeOrderTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::ChangeOrderTimeout(decoded));
            }
            if let Ok(decoded) =
                <CompleteOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::CompleteOrder(decoded));
            }
            if let Ok(decoded) =
                <ConfirmPaidCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::ConfirmPaid(decoded));
            }
            if let Ok(decoded) =
                <ConfirmTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::ConfirmTimeout(decoded));
            }
            if let Ok(decoded) =
                <ContractInvokeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::ContractInvoke(decoded));
            }
            if let Ok(decoded) =
                <DepositTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::DepositTokens(decoded));
            }
            if let Ok(decoded) =
                <DisablePaymentMethodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::DisablePaymentMethod(decoded));
            }
            if let Ok(decoded) =
                <DisputeOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::DisputeOrder(decoded));
            }
            if let Ok(decoded) =
                <DisputesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::Disputes(decoded));
            }
            if let Ok(decoded) =
                <DomainHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::DomainHash(decoded));
            }
            if let Ok(decoded) =
                <EnforceCaveatCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::EnforceCaveat(decoded));
            }
            if let Ok(decoded) =
                <FundAppealCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::FundAppeal(decoded));
            }
            if let Ok(decoded) =
                <GetContributionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::GetContributions(decoded));
            }
            if let Ok(decoded) =
                <GetCountOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::GetCountOrders(decoded));
            }
            if let Ok(decoded) =
                <GetDelegationTypedDataHashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::GetDelegationTypedDataHash(decoded));
            }
            if let Ok(decoded) =
                <GetEIP712DomainHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::GetEIP712DomainHash(decoded));
            }
            if let Ok(decoded) =
                <GetFeeAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::GetFeeAmount(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationsTypedDataHashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::GetInvocationsTypedDataHash(decoded));
            }
            if let Ok(decoded) =
                <GetNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::GetNonce(decoded));
            }
            if let Ok(decoded) =
                <GetNumberOfRoundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::GetNumberOfRounds(decoded));
            }
            if let Ok(decoded) =
                <GetRoundInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::GetRoundInfo(decoded));
            }
            if let Ok(decoded) = <InvokeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::Invoke(decoded));
            }
            if let Ok(decoded) =
                <LoserStakeMultiplierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::LoserStakeMultiplier(decoded));
            }
            if let Ok(decoded) =
                <MetaEvidenceUpdatesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::MetaEvidenceUpdates(decoded));
            }
            if let Ok(decoded) =
                <OrderTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::OrderTimeout(decoded));
            }
            if let Ok(decoded) = <OrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::Orders(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PaymentMethodsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::PaymentMethods(decoded));
            }
            if let Ok(decoded) =
                <ProtocolFeesSumCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::ProtocolFeesSum(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RuleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UnipeerCalls::Rule(decoded));
            }
            if let Ok(decoded) =
                <SharedStakeMultiplierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::SharedStakeMultiplier(decoded));
            }
            if let Ok(decoded) =
                <SubmitEvidenceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::SubmitEvidence(decoded));
            }
            if let Ok(decoded) =
                <TimeoutByBuyerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::TimeoutByBuyer(decoded));
            }
            if let Ok(decoded) =
                <TimeoutBySellerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::TimeoutBySeller(decoded));
            }
            if let Ok(decoded) =
                <TokenBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::TokenBalance(decoded));
            }
            if let Ok(decoded) =
                <TotalPaymentMethodsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::TotalPaymentMethods(decoded));
            }
            if let Ok(decoded) =
                <TradeFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::TradeFees(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdatePaymentMetaEvidenceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::UpdatePaymentMetaEvidence(decoded));
            }
            if let Ok(decoded) =
                <UpdatePaymentNameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::UpdatePaymentName(decoded));
            }
            if let Ok(decoded) =
                <UpdateTokenEnabledCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::UpdateTokenEnabled(decoded));
            }
            if let Ok(decoded) =
                <VerifyDelegationSignatureCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::VerifyDelegationSignature(decoded));
            }
            if let Ok(decoded) =
                <VerifyInvocationSignatureCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UnipeerCalls::VerifyInvocationSignature(decoded));
            }
            if let Ok(decoded) =
                <WinnerStakeMultiplierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::WinnerStakeMultiplier(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::WithdrawFees(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFeesAndRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::WithdrawFeesAndRewards(decoded));
            }
            if let Ok(decoded) =
                <WithdrawTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::WithdrawTokens(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UnipeerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UnipeerCalls::GetCaveatArrayPackethash(element) => element.encode(),
                UnipeerCalls::GetCaveatPackethash(element) => element.encode(),
                UnipeerCalls::GetDelegationPackethash(element) => element.encode(),
                UnipeerCalls::GetInvocationsPackethash(element) => element.encode(),
                UnipeerCalls::GetInvocationArrayPackethash(element) => element.encode(),
                UnipeerCalls::GetInvocationPackethash(element) => element.encode(),
                UnipeerCalls::GetReplayprotectionPackethash(element) => element.encode(),
                UnipeerCalls::GetSigneddelegationArrayPackethash(element) => element.encode(),
                UnipeerCalls::GetSigneddelegationPackethash(element) => element.encode(),
                UnipeerCalls::GetTransactionPackethash(element) => element.encode(),
                UnipeerCalls::AcceptPaymentMethod(element) => element.encode(),
                UnipeerCalls::AddMetaEvidence(element) => element.encode(),
                UnipeerCalls::AddPaymentMethod(element) => element.encode(),
                UnipeerCalls::Admin(element) => element.encode(),
                UnipeerCalls::AmountWithdrawable(element) => element.encode(),
                UnipeerCalls::ArbitratorDataList(element) => element.encode(),
                UnipeerCalls::BatchRoundWithdraw(element) => element.encode(),
                UnipeerCalls::BuyOrder(element) => element.encode(),
                UnipeerCalls::BuyQuoteWithFees(element) => element.encode(),
                UnipeerCalls::CalculateFee(element) => element.encode(),
                UnipeerCalls::ChangeArbitrator(element) => element.encode(),
                UnipeerCalls::ChangeConfirmTimeout(element) => element.encode(),
                UnipeerCalls::ChangeFees(element) => element.encode(),
                UnipeerCalls::ChangeOrderTimeout(element) => element.encode(),
                UnipeerCalls::CompleteOrder(element) => element.encode(),
                UnipeerCalls::ConfirmPaid(element) => element.encode(),
                UnipeerCalls::ConfirmTimeout(element) => element.encode(),
                UnipeerCalls::ContractInvoke(element) => element.encode(),
                UnipeerCalls::DepositTokens(element) => element.encode(),
                UnipeerCalls::DisablePaymentMethod(element) => element.encode(),
                UnipeerCalls::DisputeOrder(element) => element.encode(),
                UnipeerCalls::Disputes(element) => element.encode(),
                UnipeerCalls::DomainHash(element) => element.encode(),
                UnipeerCalls::EnforceCaveat(element) => element.encode(),
                UnipeerCalls::FundAppeal(element) => element.encode(),
                UnipeerCalls::GetContributions(element) => element.encode(),
                UnipeerCalls::GetCountOrders(element) => element.encode(),
                UnipeerCalls::GetDelegationTypedDataHash(element) => element.encode(),
                UnipeerCalls::GetEIP712DomainHash(element) => element.encode(),
                UnipeerCalls::GetFeeAmount(element) => element.encode(),
                UnipeerCalls::GetInvocationsTypedDataHash(element) => element.encode(),
                UnipeerCalls::GetNonce(element) => element.encode(),
                UnipeerCalls::GetNumberOfRounds(element) => element.encode(),
                UnipeerCalls::GetRoundInfo(element) => element.encode(),
                UnipeerCalls::Invoke(element) => element.encode(),
                UnipeerCalls::LoserStakeMultiplier(element) => element.encode(),
                UnipeerCalls::MetaEvidenceUpdates(element) => element.encode(),
                UnipeerCalls::OrderTimeout(element) => element.encode(),
                UnipeerCalls::Orders(element) => element.encode(),
                UnipeerCalls::Owner(element) => element.encode(),
                UnipeerCalls::PaymentMethods(element) => element.encode(),
                UnipeerCalls::ProtocolFeesSum(element) => element.encode(),
                UnipeerCalls::RenounceOwnership(element) => element.encode(),
                UnipeerCalls::Rule(element) => element.encode(),
                UnipeerCalls::SharedStakeMultiplier(element) => element.encode(),
                UnipeerCalls::SubmitEvidence(element) => element.encode(),
                UnipeerCalls::TimeoutByBuyer(element) => element.encode(),
                UnipeerCalls::TimeoutBySeller(element) => element.encode(),
                UnipeerCalls::TokenBalance(element) => element.encode(),
                UnipeerCalls::TotalPaymentMethods(element) => element.encode(),
                UnipeerCalls::TradeFees(element) => element.encode(),
                UnipeerCalls::TransferOwnership(element) => element.encode(),
                UnipeerCalls::UpdatePaymentMetaEvidence(element) => element.encode(),
                UnipeerCalls::UpdatePaymentName(element) => element.encode(),
                UnipeerCalls::UpdateTokenEnabled(element) => element.encode(),
                UnipeerCalls::VerifyDelegationSignature(element) => element.encode(),
                UnipeerCalls::VerifyInvocationSignature(element) => element.encode(),
                UnipeerCalls::WinnerStakeMultiplier(element) => element.encode(),
                UnipeerCalls::WithdrawFees(element) => element.encode(),
                UnipeerCalls::WithdrawFeesAndRewards(element) => element.encode(),
                UnipeerCalls::WithdrawTokens(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UnipeerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UnipeerCalls::GetCaveatArrayPackethash(element) => element.fmt(f),
                UnipeerCalls::GetCaveatPackethash(element) => element.fmt(f),
                UnipeerCalls::GetDelegationPackethash(element) => element.fmt(f),
                UnipeerCalls::GetInvocationsPackethash(element) => element.fmt(f),
                UnipeerCalls::GetInvocationArrayPackethash(element) => element.fmt(f),
                UnipeerCalls::GetInvocationPackethash(element) => element.fmt(f),
                UnipeerCalls::GetReplayprotectionPackethash(element) => element.fmt(f),
                UnipeerCalls::GetSigneddelegationArrayPackethash(element) => element.fmt(f),
                UnipeerCalls::GetSigneddelegationPackethash(element) => element.fmt(f),
                UnipeerCalls::GetTransactionPackethash(element) => element.fmt(f),
                UnipeerCalls::AcceptPaymentMethod(element) => element.fmt(f),
                UnipeerCalls::AddMetaEvidence(element) => element.fmt(f),
                UnipeerCalls::AddPaymentMethod(element) => element.fmt(f),
                UnipeerCalls::Admin(element) => element.fmt(f),
                UnipeerCalls::AmountWithdrawable(element) => element.fmt(f),
                UnipeerCalls::ArbitratorDataList(element) => element.fmt(f),
                UnipeerCalls::BatchRoundWithdraw(element) => element.fmt(f),
                UnipeerCalls::BuyOrder(element) => element.fmt(f),
                UnipeerCalls::BuyQuoteWithFees(element) => element.fmt(f),
                UnipeerCalls::CalculateFee(element) => element.fmt(f),
                UnipeerCalls::ChangeArbitrator(element) => element.fmt(f),
                UnipeerCalls::ChangeConfirmTimeout(element) => element.fmt(f),
                UnipeerCalls::ChangeFees(element) => element.fmt(f),
                UnipeerCalls::ChangeOrderTimeout(element) => element.fmt(f),
                UnipeerCalls::CompleteOrder(element) => element.fmt(f),
                UnipeerCalls::ConfirmPaid(element) => element.fmt(f),
                UnipeerCalls::ConfirmTimeout(element) => element.fmt(f),
                UnipeerCalls::ContractInvoke(element) => element.fmt(f),
                UnipeerCalls::DepositTokens(element) => element.fmt(f),
                UnipeerCalls::DisablePaymentMethod(element) => element.fmt(f),
                UnipeerCalls::DisputeOrder(element) => element.fmt(f),
                UnipeerCalls::Disputes(element) => element.fmt(f),
                UnipeerCalls::DomainHash(element) => element.fmt(f),
                UnipeerCalls::EnforceCaveat(element) => element.fmt(f),
                UnipeerCalls::FundAppeal(element) => element.fmt(f),
                UnipeerCalls::GetContributions(element) => element.fmt(f),
                UnipeerCalls::GetCountOrders(element) => element.fmt(f),
                UnipeerCalls::GetDelegationTypedDataHash(element) => element.fmt(f),
                UnipeerCalls::GetEIP712DomainHash(element) => element.fmt(f),
                UnipeerCalls::GetFeeAmount(element) => element.fmt(f),
                UnipeerCalls::GetInvocationsTypedDataHash(element) => element.fmt(f),
                UnipeerCalls::GetNonce(element) => element.fmt(f),
                UnipeerCalls::GetNumberOfRounds(element) => element.fmt(f),
                UnipeerCalls::GetRoundInfo(element) => element.fmt(f),
                UnipeerCalls::Invoke(element) => element.fmt(f),
                UnipeerCalls::LoserStakeMultiplier(element) => element.fmt(f),
                UnipeerCalls::MetaEvidenceUpdates(element) => element.fmt(f),
                UnipeerCalls::OrderTimeout(element) => element.fmt(f),
                UnipeerCalls::Orders(element) => element.fmt(f),
                UnipeerCalls::Owner(element) => element.fmt(f),
                UnipeerCalls::PaymentMethods(element) => element.fmt(f),
                UnipeerCalls::ProtocolFeesSum(element) => element.fmt(f),
                UnipeerCalls::RenounceOwnership(element) => element.fmt(f),
                UnipeerCalls::Rule(element) => element.fmt(f),
                UnipeerCalls::SharedStakeMultiplier(element) => element.fmt(f),
                UnipeerCalls::SubmitEvidence(element) => element.fmt(f),
                UnipeerCalls::TimeoutByBuyer(element) => element.fmt(f),
                UnipeerCalls::TimeoutBySeller(element) => element.fmt(f),
                UnipeerCalls::TokenBalance(element) => element.fmt(f),
                UnipeerCalls::TotalPaymentMethods(element) => element.fmt(f),
                UnipeerCalls::TradeFees(element) => element.fmt(f),
                UnipeerCalls::TransferOwnership(element) => element.fmt(f),
                UnipeerCalls::UpdatePaymentMetaEvidence(element) => element.fmt(f),
                UnipeerCalls::UpdatePaymentName(element) => element.fmt(f),
                UnipeerCalls::UpdateTokenEnabled(element) => element.fmt(f),
                UnipeerCalls::VerifyDelegationSignature(element) => element.fmt(f),
                UnipeerCalls::VerifyInvocationSignature(element) => element.fmt(f),
                UnipeerCalls::WinnerStakeMultiplier(element) => element.fmt(f),
                UnipeerCalls::WithdrawFees(element) => element.fmt(f),
                UnipeerCalls::WithdrawFeesAndRewards(element) => element.fmt(f),
                UnipeerCalls::WithdrawTokens(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetCaveatArrayPackethashCall> for UnipeerCalls {
        fn from(var: GetCaveatArrayPackethashCall) -> Self {
            UnipeerCalls::GetCaveatArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetCaveatPackethashCall> for UnipeerCalls {
        fn from(var: GetCaveatPackethashCall) -> Self {
            UnipeerCalls::GetCaveatPackethash(var)
        }
    }
    impl ::std::convert::From<GetDelegationPackethashCall> for UnipeerCalls {
        fn from(var: GetDelegationPackethashCall) -> Self {
            UnipeerCalls::GetDelegationPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationsPackethashCall> for UnipeerCalls {
        fn from(var: GetInvocationsPackethashCall) -> Self {
            UnipeerCalls::GetInvocationsPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationArrayPackethashCall> for UnipeerCalls {
        fn from(var: GetInvocationArrayPackethashCall) -> Self {
            UnipeerCalls::GetInvocationArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationPackethashCall> for UnipeerCalls {
        fn from(var: GetInvocationPackethashCall) -> Self {
            UnipeerCalls::GetInvocationPackethash(var)
        }
    }
    impl ::std::convert::From<GetReplayprotectionPackethashCall> for UnipeerCalls {
        fn from(var: GetReplayprotectionPackethashCall) -> Self {
            UnipeerCalls::GetReplayprotectionPackethash(var)
        }
    }
    impl ::std::convert::From<GetSigneddelegationArrayPackethashCall> for UnipeerCalls {
        fn from(var: GetSigneddelegationArrayPackethashCall) -> Self {
            UnipeerCalls::GetSigneddelegationArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetSigneddelegationPackethashCall> for UnipeerCalls {
        fn from(var: GetSigneddelegationPackethashCall) -> Self {
            UnipeerCalls::GetSigneddelegationPackethash(var)
        }
    }
    impl ::std::convert::From<GetTransactionPackethashCall> for UnipeerCalls {
        fn from(var: GetTransactionPackethashCall) -> Self {
            UnipeerCalls::GetTransactionPackethash(var)
        }
    }
    impl ::std::convert::From<AcceptPaymentMethodCall> for UnipeerCalls {
        fn from(var: AcceptPaymentMethodCall) -> Self {
            UnipeerCalls::AcceptPaymentMethod(var)
        }
    }
    impl ::std::convert::From<AddMetaEvidenceCall> for UnipeerCalls {
        fn from(var: AddMetaEvidenceCall) -> Self {
            UnipeerCalls::AddMetaEvidence(var)
        }
    }
    impl ::std::convert::From<AddPaymentMethodCall> for UnipeerCalls {
        fn from(var: AddPaymentMethodCall) -> Self {
            UnipeerCalls::AddPaymentMethod(var)
        }
    }
    impl ::std::convert::From<AdminCall> for UnipeerCalls {
        fn from(var: AdminCall) -> Self {
            UnipeerCalls::Admin(var)
        }
    }
    impl ::std::convert::From<AmountWithdrawableCall> for UnipeerCalls {
        fn from(var: AmountWithdrawableCall) -> Self {
            UnipeerCalls::AmountWithdrawable(var)
        }
    }
    impl ::std::convert::From<ArbitratorDataListCall> for UnipeerCalls {
        fn from(var: ArbitratorDataListCall) -> Self {
            UnipeerCalls::ArbitratorDataList(var)
        }
    }
    impl ::std::convert::From<BatchRoundWithdrawCall> for UnipeerCalls {
        fn from(var: BatchRoundWithdrawCall) -> Self {
            UnipeerCalls::BatchRoundWithdraw(var)
        }
    }
    impl ::std::convert::From<BuyOrderCall> for UnipeerCalls {
        fn from(var: BuyOrderCall) -> Self {
            UnipeerCalls::BuyOrder(var)
        }
    }
    impl ::std::convert::From<BuyQuoteWithFeesCall> for UnipeerCalls {
        fn from(var: BuyQuoteWithFeesCall) -> Self {
            UnipeerCalls::BuyQuoteWithFees(var)
        }
    }
    impl ::std::convert::From<CalculateFeeCall> for UnipeerCalls {
        fn from(var: CalculateFeeCall) -> Self {
            UnipeerCalls::CalculateFee(var)
        }
    }
    impl ::std::convert::From<ChangeArbitratorCall> for UnipeerCalls {
        fn from(var: ChangeArbitratorCall) -> Self {
            UnipeerCalls::ChangeArbitrator(var)
        }
    }
    impl ::std::convert::From<ChangeConfirmTimeoutCall> for UnipeerCalls {
        fn from(var: ChangeConfirmTimeoutCall) -> Self {
            UnipeerCalls::ChangeConfirmTimeout(var)
        }
    }
    impl ::std::convert::From<ChangeFeesCall> for UnipeerCalls {
        fn from(var: ChangeFeesCall) -> Self {
            UnipeerCalls::ChangeFees(var)
        }
    }
    impl ::std::convert::From<ChangeOrderTimeoutCall> for UnipeerCalls {
        fn from(var: ChangeOrderTimeoutCall) -> Self {
            UnipeerCalls::ChangeOrderTimeout(var)
        }
    }
    impl ::std::convert::From<CompleteOrderCall> for UnipeerCalls {
        fn from(var: CompleteOrderCall) -> Self {
            UnipeerCalls::CompleteOrder(var)
        }
    }
    impl ::std::convert::From<ConfirmPaidCall> for UnipeerCalls {
        fn from(var: ConfirmPaidCall) -> Self {
            UnipeerCalls::ConfirmPaid(var)
        }
    }
    impl ::std::convert::From<ConfirmTimeoutCall> for UnipeerCalls {
        fn from(var: ConfirmTimeoutCall) -> Self {
            UnipeerCalls::ConfirmTimeout(var)
        }
    }
    impl ::std::convert::From<ContractInvokeCall> for UnipeerCalls {
        fn from(var: ContractInvokeCall) -> Self {
            UnipeerCalls::ContractInvoke(var)
        }
    }
    impl ::std::convert::From<DepositTokensCall> for UnipeerCalls {
        fn from(var: DepositTokensCall) -> Self {
            UnipeerCalls::DepositTokens(var)
        }
    }
    impl ::std::convert::From<DisablePaymentMethodCall> for UnipeerCalls {
        fn from(var: DisablePaymentMethodCall) -> Self {
            UnipeerCalls::DisablePaymentMethod(var)
        }
    }
    impl ::std::convert::From<DisputeOrderCall> for UnipeerCalls {
        fn from(var: DisputeOrderCall) -> Self {
            UnipeerCalls::DisputeOrder(var)
        }
    }
    impl ::std::convert::From<DisputesCall> for UnipeerCalls {
        fn from(var: DisputesCall) -> Self {
            UnipeerCalls::Disputes(var)
        }
    }
    impl ::std::convert::From<DomainHashCall> for UnipeerCalls {
        fn from(var: DomainHashCall) -> Self {
            UnipeerCalls::DomainHash(var)
        }
    }
    impl ::std::convert::From<EnforceCaveatCall> for UnipeerCalls {
        fn from(var: EnforceCaveatCall) -> Self {
            UnipeerCalls::EnforceCaveat(var)
        }
    }
    impl ::std::convert::From<FundAppealCall> for UnipeerCalls {
        fn from(var: FundAppealCall) -> Self {
            UnipeerCalls::FundAppeal(var)
        }
    }
    impl ::std::convert::From<GetContributionsCall> for UnipeerCalls {
        fn from(var: GetContributionsCall) -> Self {
            UnipeerCalls::GetContributions(var)
        }
    }
    impl ::std::convert::From<GetCountOrdersCall> for UnipeerCalls {
        fn from(var: GetCountOrdersCall) -> Self {
            UnipeerCalls::GetCountOrders(var)
        }
    }
    impl ::std::convert::From<GetDelegationTypedDataHashCall> for UnipeerCalls {
        fn from(var: GetDelegationTypedDataHashCall) -> Self {
            UnipeerCalls::GetDelegationTypedDataHash(var)
        }
    }
    impl ::std::convert::From<GetEIP712DomainHashCall> for UnipeerCalls {
        fn from(var: GetEIP712DomainHashCall) -> Self {
            UnipeerCalls::GetEIP712DomainHash(var)
        }
    }
    impl ::std::convert::From<GetFeeAmountCall> for UnipeerCalls {
        fn from(var: GetFeeAmountCall) -> Self {
            UnipeerCalls::GetFeeAmount(var)
        }
    }
    impl ::std::convert::From<GetInvocationsTypedDataHashCall> for UnipeerCalls {
        fn from(var: GetInvocationsTypedDataHashCall) -> Self {
            UnipeerCalls::GetInvocationsTypedDataHash(var)
        }
    }
    impl ::std::convert::From<GetNonceCall> for UnipeerCalls {
        fn from(var: GetNonceCall) -> Self {
            UnipeerCalls::GetNonce(var)
        }
    }
    impl ::std::convert::From<GetNumberOfRoundsCall> for UnipeerCalls {
        fn from(var: GetNumberOfRoundsCall) -> Self {
            UnipeerCalls::GetNumberOfRounds(var)
        }
    }
    impl ::std::convert::From<GetRoundInfoCall> for UnipeerCalls {
        fn from(var: GetRoundInfoCall) -> Self {
            UnipeerCalls::GetRoundInfo(var)
        }
    }
    impl ::std::convert::From<InvokeCall> for UnipeerCalls {
        fn from(var: InvokeCall) -> Self {
            UnipeerCalls::Invoke(var)
        }
    }
    impl ::std::convert::From<LoserStakeMultiplierCall> for UnipeerCalls {
        fn from(var: LoserStakeMultiplierCall) -> Self {
            UnipeerCalls::LoserStakeMultiplier(var)
        }
    }
    impl ::std::convert::From<MetaEvidenceUpdatesCall> for UnipeerCalls {
        fn from(var: MetaEvidenceUpdatesCall) -> Self {
            UnipeerCalls::MetaEvidenceUpdates(var)
        }
    }
    impl ::std::convert::From<OrderTimeoutCall> for UnipeerCalls {
        fn from(var: OrderTimeoutCall) -> Self {
            UnipeerCalls::OrderTimeout(var)
        }
    }
    impl ::std::convert::From<OrdersCall> for UnipeerCalls {
        fn from(var: OrdersCall) -> Self {
            UnipeerCalls::Orders(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for UnipeerCalls {
        fn from(var: OwnerCall) -> Self {
            UnipeerCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PaymentMethodsCall> for UnipeerCalls {
        fn from(var: PaymentMethodsCall) -> Self {
            UnipeerCalls::PaymentMethods(var)
        }
    }
    impl ::std::convert::From<ProtocolFeesSumCall> for UnipeerCalls {
        fn from(var: ProtocolFeesSumCall) -> Self {
            UnipeerCalls::ProtocolFeesSum(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for UnipeerCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            UnipeerCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RuleCall> for UnipeerCalls {
        fn from(var: RuleCall) -> Self {
            UnipeerCalls::Rule(var)
        }
    }
    impl ::std::convert::From<SharedStakeMultiplierCall> for UnipeerCalls {
        fn from(var: SharedStakeMultiplierCall) -> Self {
            UnipeerCalls::SharedStakeMultiplier(var)
        }
    }
    impl ::std::convert::From<SubmitEvidenceCall> for UnipeerCalls {
        fn from(var: SubmitEvidenceCall) -> Self {
            UnipeerCalls::SubmitEvidence(var)
        }
    }
    impl ::std::convert::From<TimeoutByBuyerCall> for UnipeerCalls {
        fn from(var: TimeoutByBuyerCall) -> Self {
            UnipeerCalls::TimeoutByBuyer(var)
        }
    }
    impl ::std::convert::From<TimeoutBySellerCall> for UnipeerCalls {
        fn from(var: TimeoutBySellerCall) -> Self {
            UnipeerCalls::TimeoutBySeller(var)
        }
    }
    impl ::std::convert::From<TokenBalanceCall> for UnipeerCalls {
        fn from(var: TokenBalanceCall) -> Self {
            UnipeerCalls::TokenBalance(var)
        }
    }
    impl ::std::convert::From<TotalPaymentMethodsCall> for UnipeerCalls {
        fn from(var: TotalPaymentMethodsCall) -> Self {
            UnipeerCalls::TotalPaymentMethods(var)
        }
    }
    impl ::std::convert::From<TradeFeesCall> for UnipeerCalls {
        fn from(var: TradeFeesCall) -> Self {
            UnipeerCalls::TradeFees(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for UnipeerCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            UnipeerCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UpdatePaymentMetaEvidenceCall> for UnipeerCalls {
        fn from(var: UpdatePaymentMetaEvidenceCall) -> Self {
            UnipeerCalls::UpdatePaymentMetaEvidence(var)
        }
    }
    impl ::std::convert::From<UpdatePaymentNameCall> for UnipeerCalls {
        fn from(var: UpdatePaymentNameCall) -> Self {
            UnipeerCalls::UpdatePaymentName(var)
        }
    }
    impl ::std::convert::From<UpdateTokenEnabledCall> for UnipeerCalls {
        fn from(var: UpdateTokenEnabledCall) -> Self {
            UnipeerCalls::UpdateTokenEnabled(var)
        }
    }
    impl ::std::convert::From<VerifyDelegationSignatureCall> for UnipeerCalls {
        fn from(var: VerifyDelegationSignatureCall) -> Self {
            UnipeerCalls::VerifyDelegationSignature(var)
        }
    }
    impl ::std::convert::From<VerifyInvocationSignatureCall> for UnipeerCalls {
        fn from(var: VerifyInvocationSignatureCall) -> Self {
            UnipeerCalls::VerifyInvocationSignature(var)
        }
    }
    impl ::std::convert::From<WinnerStakeMultiplierCall> for UnipeerCalls {
        fn from(var: WinnerStakeMultiplierCall) -> Self {
            UnipeerCalls::WinnerStakeMultiplier(var)
        }
    }
    impl ::std::convert::From<WithdrawFeesCall> for UnipeerCalls {
        fn from(var: WithdrawFeesCall) -> Self {
            UnipeerCalls::WithdrawFees(var)
        }
    }
    impl ::std::convert::From<WithdrawFeesAndRewardsCall> for UnipeerCalls {
        fn from(var: WithdrawFeesAndRewardsCall) -> Self {
            UnipeerCalls::WithdrawFeesAndRewards(var)
        }
    }
    impl ::std::convert::From<WithdrawTokensCall> for UnipeerCalls {
        fn from(var: WithdrawTokensCall) -> Self {
            UnipeerCalls::WithdrawTokens(var)
        }
    }
    #[doc = "Container type for all return fields from the `GET_CAVEAT_ARRAY_PACKETHASH` function with signature `GET_CAVEAT_ARRAY_PACKETHASH((address,bytes)[])` and selector `[123, 87, 123, 88]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCaveatArrayPackethashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `GET_CAVEAT_PACKETHASH` function with signature `GET_CAVEAT_PACKETHASH((address,bytes))` and selector `[8, 170, 246, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCaveatPackethashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `GET_DELEGATION_PACKETHASH` function with signature `GET_DELEGATION_PACKETHASH((address,bytes32,(address,bytes)[]))` and selector `[47, 82, 162, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetDelegationPackethashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `GET_INVOCATIONS_PACKETHASH` function with signature `GET_INVOCATIONS_PACKETHASH((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)))` and selector `[114, 52, 238, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetInvocationsPackethashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `GET_INVOCATION_ARRAY_PACKETHASH` function with signature `GET_INVOCATION_ARRAY_PACKETHASH(((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[])` and selector `[58, 72, 24, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetInvocationArrayPackethashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `GET_INVOCATION_PACKETHASH` function with signature `GET_INVOCATION_PACKETHASH(((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[]))` and selector `[92, 241, 178, 74]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetInvocationPackethashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `GET_REPLAYPROTECTION_PACKETHASH` function with signature `GET_REPLAYPROTECTION_PACKETHASH((uint256,uint256))` and selector `[111, 150, 88, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetReplayprotectionPackethashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `GET_SIGNEDDELEGATION_ARRAY_PACKETHASH` function with signature `GET_SIGNEDDELEGATION_ARRAY_PACKETHASH(((address,bytes32,(address,bytes)[]),bytes)[])` and selector `[115, 111, 124, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSigneddelegationArrayPackethashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `GET_SIGNEDDELEGATION_PACKETHASH` function with signature `GET_SIGNEDDELEGATION_PACKETHASH(((address,bytes32,(address,bytes)[]),bytes))` and selector `[108, 43, 18, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetSigneddelegationPackethashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `GET_TRANSACTION_PACKETHASH` function with signature `GET_TRANSACTION_PACKETHASH((address,uint256,bytes))` and selector `[162, 224, 31, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetTransactionPackethashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `addMetaEvidence` function with signature `addMetaEvidence(string)` and selector `[182, 105, 75, 123]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AddMetaEvidenceReturn {
        pub meta_evidence_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AdminReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `amountWithdrawable` function with signature `amountWithdrawable(uint256,address)` and selector `[17, 101, 84, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AmountWithdrawableReturn {
        pub total: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `arbitratorDataList` function with signature `arbitratorDataList(uint256)` and selector `[236, 14, 113, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ArbitratorDataListReturn {
        pub arbitrator: ethers::core::types::Address,
        pub arbitrator_extra_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all return fields from the `buyQuoteWithFees` function with signature `buyQuoteWithFees(uint256)` and selector `[161, 215, 120, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BuyQuoteWithFeesReturn {
        pub fee: ethers::core::types::U256,
        pub trade_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `calculateFee` function with signature `calculateFee(uint256)` and selector `[153, 165, 215, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CalculateFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `confirmTimeout` function with signature `confirmTimeout()` and selector `[171, 128, 36, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ConfirmTimeoutReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `contractInvoke` function with signature `contractInvoke(((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[])` and selector `[92, 109, 159, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ContractInvokeReturn(pub bool);
    #[doc = "Container type for all return fields from the `disputes` function with signature `disputes(uint256)` and selector `[86, 74, 86, 93]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DisputesReturn {
        pub order_id: ethers::core::types::U256,
        pub ruling: u8,
        pub last_round_id: u16,
    }
    #[doc = "Container type for all return fields from the `domainHash` function with signature `domainHash()` and selector `[223, 232, 106, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DomainHashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `enforceCaveat` function with signature `enforceCaveat(bytes,(address,uint256,bytes),bytes32)` and selector `[80, 104, 222, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct EnforceCaveatReturn(pub bool);
    #[doc = "Container type for all return fields from the `getContributions` function with signature `getContributions(uint256,uint256,address)` and selector `[104, 199, 111, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetContributionsReturn {
        pub contributions: [ethers::core::types::U256; 3usize],
    }
    #[doc = "Container type for all return fields from the `getCountOrders` function with signature `getCountOrders()` and selector `[152, 241, 193, 98]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetCountOrdersReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getDelegationTypedDataHash` function with signature `getDelegationTypedDataHash((address,bytes32,(address,bytes)[]))` and selector `[151, 24, 46, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetDelegationTypedDataHashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getEIP712DomainHash` function with signature `getEIP712DomainHash(string,string,uint256,address)` and selector `[211, 39, 193, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetEIP712DomainHashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getFeeAmount` function with signature `getFeeAmount(uint256)` and selector `[151, 4, 18, 44]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetFeeAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getInvocationsTypedDataHash` function with signature `getInvocationsTypedDataHash((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)))` and selector `[96, 182, 215, 104]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetInvocationsTypedDataHashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getNonce` function with signature `getNonce(address,uint256)` and selector `[137, 83, 88, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNonceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getNumberOfRounds` function with signature `getNumberOfRounds(uint256)` and selector `[252, 111, 143, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNumberOfRoundsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getRoundInfo` function with signature `getRoundInfo(uint256,uint256)` and selector `[138, 155, 176, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetRoundInfoReturn {
        pub paid_fees: [ethers::core::types::U256; 3usize],
        pub side_funded: u8,
        pub fee_rewards: ethers::core::types::U256,
        pub appealed: bool,
    }
    #[doc = "Container type for all return fields from the `invoke` function with signature `invoke(((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)),bytes)[])` and selector `[47, 173, 126, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct InvokeReturn {
        pub success: bool,
    }
    #[doc = "Container type for all return fields from the `loserStakeMultiplier` function with signature `loserStakeMultiplier()` and selector `[29, 81, 32, 133]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LoserStakeMultiplierReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `metaEvidenceUpdates` function with signature `metaEvidenceUpdates()` and selector `[108, 220, 9, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MetaEvidenceUpdatesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `orderTimeout` function with signature `orderTimeout()` and selector `[186, 32, 96, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OrderTimeoutReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `orders` function with signature `orders(uint256)` and selector `[168, 92, 56, 239]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OrdersReturn {
        pub buyer: ethers::core::types::Address,
        pub seller: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub buyer_fee: ethers::core::types::U256,
        pub seller_fee: ethers::core::types::U256,
        pub dispute_id: ethers::core::types::U256,
        pub arbitrator_id: ethers::core::types::U256,
        pub last_interaction: ethers::core::types::U256,
        pub status: u8,
    }
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `paymentMethods` function with signature `paymentMethods(uint16)` and selector `[12, 212, 56, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PaymentMethodsReturn {
        pub payment_name: String,
        pub meta_evidence_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `protocolFeesSum` function with signature `protocolFeesSum()` and selector `[251, 239, 112, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ProtocolFeesSumReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `sharedStakeMultiplier` function with signature `sharedStakeMultiplier()` and selector `[65, 101, 131, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SharedStakeMultiplierReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `tokenBalance` function with signature `tokenBalance(address,address)` and selector `[16, 73, 51, 79]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TokenBalanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `totalPaymentMethods` function with signature `totalPaymentMethods()` and selector `[70, 99, 156, 202]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TotalPaymentMethodsReturn(pub u16);
    #[doc = "Container type for all return fields from the `tradeFees` function with signature `tradeFees()` and selector `[79, 253, 67, 195]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TradeFeesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `verifyDelegationSignature` function with signature `verifyDelegationSignature(((address,bytes32,(address,bytes)[]),bytes))` and selector `[138, 4, 73, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VerifyDelegationSignatureReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `verifyInvocationSignature` function with signature `verifyInvocationSignature(((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)),bytes))` and selector `[202, 206, 214, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VerifyInvocationSignatureReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `winnerStakeMultiplier` function with signature `winnerStakeMultiplier()` and selector `[123, 148, 51, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct WinnerStakeMultiplierReturn(pub ethers::core::types::U256);
}
