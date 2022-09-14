pub use delegatable::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod delegatable {
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
    #[doc = "Delegatable was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DELEGATABLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct Caveat[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_CAVEAT_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Caveat\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_CAVEAT_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Delegation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_DELEGATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocations\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATIONS_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATION_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ReplayProtection\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_REPLAYPROTECTION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_SIGNEDDELEGATION_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_SIGNEDDELEGATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Transaction\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_TRANSACTION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"contractInvoke\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"domainHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDelegationTypedDataHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"contractName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"version\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"chainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"verifyingContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getEIP712DomainHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocations\",\"name\":\"invocations\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInvocationsTypedDataHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"intendedSender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedInvocation[]\",\"name\":\"signedInvocations\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Invocations\",\"name\":\"invocations\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"invoke\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation\",\"name\":\"signedDelegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifyDelegationSignature\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedInvocation\",\"name\":\"signedInvocation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocations\",\"name\":\"invocations\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifyInvocationSignature\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct Delegatable<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Delegatable<M> {
        fn clone(&self) -> Self {
            Delegatable(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Delegatable<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Delegatable<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Delegatable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Delegatable<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), DELEGATABLE_ABI.clone(), client).into()
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
        #[doc = "Calls the contract's `contractInvoke` (0x5c6d9f0c) function"]
        pub fn contract_invoke(
            &self,
            batch: ::std::vec::Vec<Invocation>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 109, 159, 12], batch)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `domainHash` (0xdfe86ac5) function"]
        pub fn domain_hash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([223, 232, 106, 197], ())
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
        #[doc = "Calls the contract's `invoke` (0x2fad7efc) function"]
        pub fn invoke(
            &self,
            signed_invocations: ::std::vec::Vec<SignedInvocation>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 173, 126, 252], signed_invocations)
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Delegatable<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum DelegatableCalls {
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
        ContractInvoke(ContractInvokeCall),
        DomainHash(DomainHashCall),
        GetDelegationTypedDataHash(GetDelegationTypedDataHashCall),
        GetEIP712DomainHash(GetEIP712DomainHashCall),
        GetInvocationsTypedDataHash(GetInvocationsTypedDataHashCall),
        GetNonce(GetNonceCall),
        Invoke(InvokeCall),
        VerifyDelegationSignature(VerifyDelegationSignatureCall),
        VerifyInvocationSignature(VerifyInvocationSignatureCall),
    }
    impl ethers::core::abi::AbiDecode for DelegatableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetCaveatArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCalls::GetCaveatArrayPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetCaveatPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCalls::GetCaveatPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetDelegationPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCalls::GetDelegationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationsPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCalls::GetInvocationsPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCalls::GetInvocationArrayPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCalls::GetInvocationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetReplayprotectionPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCalls::GetReplayprotectionPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetSigneddelegationArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCalls::GetSigneddelegationArrayPackethash(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetSigneddelegationPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCalls::GetSigneddelegationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetTransactionPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCalls::GetTransactionPackethash(decoded));
            }
            if let Ok(decoded) =
                <ContractInvokeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCalls::ContractInvoke(decoded));
            }
            if let Ok(decoded) =
                <DomainHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCalls::DomainHash(decoded));
            }
            if let Ok(decoded) =
                <GetDelegationTypedDataHashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCalls::GetDelegationTypedDataHash(decoded));
            }
            if let Ok(decoded) =
                <GetEIP712DomainHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCalls::GetEIP712DomainHash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationsTypedDataHashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCalls::GetInvocationsTypedDataHash(decoded));
            }
            if let Ok(decoded) =
                <GetNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCalls::GetNonce(decoded));
            }
            if let Ok(decoded) = <InvokeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCalls::Invoke(decoded));
            }
            if let Ok(decoded) =
                <VerifyDelegationSignatureCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCalls::VerifyDelegationSignature(decoded));
            }
            if let Ok(decoded) =
                <VerifyInvocationSignatureCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCalls::VerifyInvocationSignature(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for DelegatableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                DelegatableCalls::GetCaveatArrayPackethash(element) => element.encode(),
                DelegatableCalls::GetCaveatPackethash(element) => element.encode(),
                DelegatableCalls::GetDelegationPackethash(element) => element.encode(),
                DelegatableCalls::GetInvocationsPackethash(element) => element.encode(),
                DelegatableCalls::GetInvocationArrayPackethash(element) => element.encode(),
                DelegatableCalls::GetInvocationPackethash(element) => element.encode(),
                DelegatableCalls::GetReplayprotectionPackethash(element) => element.encode(),
                DelegatableCalls::GetSigneddelegationArrayPackethash(element) => element.encode(),
                DelegatableCalls::GetSigneddelegationPackethash(element) => element.encode(),
                DelegatableCalls::GetTransactionPackethash(element) => element.encode(),
                DelegatableCalls::ContractInvoke(element) => element.encode(),
                DelegatableCalls::DomainHash(element) => element.encode(),
                DelegatableCalls::GetDelegationTypedDataHash(element) => element.encode(),
                DelegatableCalls::GetEIP712DomainHash(element) => element.encode(),
                DelegatableCalls::GetInvocationsTypedDataHash(element) => element.encode(),
                DelegatableCalls::GetNonce(element) => element.encode(),
                DelegatableCalls::Invoke(element) => element.encode(),
                DelegatableCalls::VerifyDelegationSignature(element) => element.encode(),
                DelegatableCalls::VerifyInvocationSignature(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DelegatableCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DelegatableCalls::GetCaveatArrayPackethash(element) => element.fmt(f),
                DelegatableCalls::GetCaveatPackethash(element) => element.fmt(f),
                DelegatableCalls::GetDelegationPackethash(element) => element.fmt(f),
                DelegatableCalls::GetInvocationsPackethash(element) => element.fmt(f),
                DelegatableCalls::GetInvocationArrayPackethash(element) => element.fmt(f),
                DelegatableCalls::GetInvocationPackethash(element) => element.fmt(f),
                DelegatableCalls::GetReplayprotectionPackethash(element) => element.fmt(f),
                DelegatableCalls::GetSigneddelegationArrayPackethash(element) => element.fmt(f),
                DelegatableCalls::GetSigneddelegationPackethash(element) => element.fmt(f),
                DelegatableCalls::GetTransactionPackethash(element) => element.fmt(f),
                DelegatableCalls::ContractInvoke(element) => element.fmt(f),
                DelegatableCalls::DomainHash(element) => element.fmt(f),
                DelegatableCalls::GetDelegationTypedDataHash(element) => element.fmt(f),
                DelegatableCalls::GetEIP712DomainHash(element) => element.fmt(f),
                DelegatableCalls::GetInvocationsTypedDataHash(element) => element.fmt(f),
                DelegatableCalls::GetNonce(element) => element.fmt(f),
                DelegatableCalls::Invoke(element) => element.fmt(f),
                DelegatableCalls::VerifyDelegationSignature(element) => element.fmt(f),
                DelegatableCalls::VerifyInvocationSignature(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetCaveatArrayPackethashCall> for DelegatableCalls {
        fn from(var: GetCaveatArrayPackethashCall) -> Self {
            DelegatableCalls::GetCaveatArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetCaveatPackethashCall> for DelegatableCalls {
        fn from(var: GetCaveatPackethashCall) -> Self {
            DelegatableCalls::GetCaveatPackethash(var)
        }
    }
    impl ::std::convert::From<GetDelegationPackethashCall> for DelegatableCalls {
        fn from(var: GetDelegationPackethashCall) -> Self {
            DelegatableCalls::GetDelegationPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationsPackethashCall> for DelegatableCalls {
        fn from(var: GetInvocationsPackethashCall) -> Self {
            DelegatableCalls::GetInvocationsPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationArrayPackethashCall> for DelegatableCalls {
        fn from(var: GetInvocationArrayPackethashCall) -> Self {
            DelegatableCalls::GetInvocationArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationPackethashCall> for DelegatableCalls {
        fn from(var: GetInvocationPackethashCall) -> Self {
            DelegatableCalls::GetInvocationPackethash(var)
        }
    }
    impl ::std::convert::From<GetReplayprotectionPackethashCall> for DelegatableCalls {
        fn from(var: GetReplayprotectionPackethashCall) -> Self {
            DelegatableCalls::GetReplayprotectionPackethash(var)
        }
    }
    impl ::std::convert::From<GetSigneddelegationArrayPackethashCall> for DelegatableCalls {
        fn from(var: GetSigneddelegationArrayPackethashCall) -> Self {
            DelegatableCalls::GetSigneddelegationArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetSigneddelegationPackethashCall> for DelegatableCalls {
        fn from(var: GetSigneddelegationPackethashCall) -> Self {
            DelegatableCalls::GetSigneddelegationPackethash(var)
        }
    }
    impl ::std::convert::From<GetTransactionPackethashCall> for DelegatableCalls {
        fn from(var: GetTransactionPackethashCall) -> Self {
            DelegatableCalls::GetTransactionPackethash(var)
        }
    }
    impl ::std::convert::From<ContractInvokeCall> for DelegatableCalls {
        fn from(var: ContractInvokeCall) -> Self {
            DelegatableCalls::ContractInvoke(var)
        }
    }
    impl ::std::convert::From<DomainHashCall> for DelegatableCalls {
        fn from(var: DomainHashCall) -> Self {
            DelegatableCalls::DomainHash(var)
        }
    }
    impl ::std::convert::From<GetDelegationTypedDataHashCall> for DelegatableCalls {
        fn from(var: GetDelegationTypedDataHashCall) -> Self {
            DelegatableCalls::GetDelegationTypedDataHash(var)
        }
    }
    impl ::std::convert::From<GetEIP712DomainHashCall> for DelegatableCalls {
        fn from(var: GetEIP712DomainHashCall) -> Self {
            DelegatableCalls::GetEIP712DomainHash(var)
        }
    }
    impl ::std::convert::From<GetInvocationsTypedDataHashCall> for DelegatableCalls {
        fn from(var: GetInvocationsTypedDataHashCall) -> Self {
            DelegatableCalls::GetInvocationsTypedDataHash(var)
        }
    }
    impl ::std::convert::From<GetNonceCall> for DelegatableCalls {
        fn from(var: GetNonceCall) -> Self {
            DelegatableCalls::GetNonce(var)
        }
    }
    impl ::std::convert::From<InvokeCall> for DelegatableCalls {
        fn from(var: InvokeCall) -> Self {
            DelegatableCalls::Invoke(var)
        }
    }
    impl ::std::convert::From<VerifyDelegationSignatureCall> for DelegatableCalls {
        fn from(var: VerifyDelegationSignatureCall) -> Self {
            DelegatableCalls::VerifyDelegationSignature(var)
        }
    }
    impl ::std::convert::From<VerifyInvocationSignatureCall> for DelegatableCalls {
        fn from(var: VerifyInvocationSignatureCall) -> Self {
            DelegatableCalls::VerifyInvocationSignature(var)
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
}
