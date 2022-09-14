pub use delegatable_core::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod delegatable_core {
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
    #[doc = "DelegatableCore was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DELEGATABLECORE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct Caveat[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_CAVEAT_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Caveat\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_CAVEAT_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Delegation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_DELEGATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocations\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATIONS_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATION_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ReplayProtection\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_REPLAYPROTECTION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_SIGNEDDELEGATION_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_SIGNEDDELEGATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Transaction\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_TRANSACTION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"intendedSender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation\",\"name\":\"signedDelegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifyDelegationSignature\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct DelegatableCore<M>(ethers::contract::Contract<M>);
    impl<M> Clone for DelegatableCore<M> {
        fn clone(&self) -> Self {
            DelegatableCore(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DelegatableCore<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for DelegatableCore<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DelegatableCore))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> DelegatableCore<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), DELEGATABLECORE_ABI.clone(), client)
                .into()
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
        #[doc = "Calls the contract's `verifyDelegationSignature` (0x8a04499e) function"]
        pub fn verify_delegation_signature(
            &self,
            signed_delegation: SignedDelegation,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([138, 4, 73, 158], (signed_delegation,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for DelegatableCore<M> {
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum DelegatableCoreCalls {
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
        GetNonce(GetNonceCall),
        VerifyDelegationSignature(VerifyDelegationSignatureCall),
    }
    impl ethers::core::abi::AbiDecode for DelegatableCoreCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetCaveatArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCoreCalls::GetCaveatArrayPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetCaveatPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCoreCalls::GetCaveatPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetDelegationPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCoreCalls::GetDelegationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationsPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCoreCalls::GetInvocationsPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCoreCalls::GetInvocationArrayPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCoreCalls::GetInvocationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetReplayprotectionPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCoreCalls::GetReplayprotectionPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetSigneddelegationArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCoreCalls::GetSigneddelegationArrayPackethash(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetSigneddelegationPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCoreCalls::GetSigneddelegationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetTransactionPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCoreCalls::GetTransactionPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DelegatableCoreCalls::GetNonce(decoded));
            }
            if let Ok(decoded) =
                <VerifyDelegationSignatureCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(DelegatableCoreCalls::VerifyDelegationSignature(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for DelegatableCoreCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                DelegatableCoreCalls::GetCaveatArrayPackethash(element) => element.encode(),
                DelegatableCoreCalls::GetCaveatPackethash(element) => element.encode(),
                DelegatableCoreCalls::GetDelegationPackethash(element) => element.encode(),
                DelegatableCoreCalls::GetInvocationsPackethash(element) => element.encode(),
                DelegatableCoreCalls::GetInvocationArrayPackethash(element) => element.encode(),
                DelegatableCoreCalls::GetInvocationPackethash(element) => element.encode(),
                DelegatableCoreCalls::GetReplayprotectionPackethash(element) => element.encode(),
                DelegatableCoreCalls::GetSigneddelegationArrayPackethash(element) => {
                    element.encode()
                }
                DelegatableCoreCalls::GetSigneddelegationPackethash(element) => element.encode(),
                DelegatableCoreCalls::GetTransactionPackethash(element) => element.encode(),
                DelegatableCoreCalls::GetNonce(element) => element.encode(),
                DelegatableCoreCalls::VerifyDelegationSignature(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DelegatableCoreCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DelegatableCoreCalls::GetCaveatArrayPackethash(element) => element.fmt(f),
                DelegatableCoreCalls::GetCaveatPackethash(element) => element.fmt(f),
                DelegatableCoreCalls::GetDelegationPackethash(element) => element.fmt(f),
                DelegatableCoreCalls::GetInvocationsPackethash(element) => element.fmt(f),
                DelegatableCoreCalls::GetInvocationArrayPackethash(element) => element.fmt(f),
                DelegatableCoreCalls::GetInvocationPackethash(element) => element.fmt(f),
                DelegatableCoreCalls::GetReplayprotectionPackethash(element) => element.fmt(f),
                DelegatableCoreCalls::GetSigneddelegationArrayPackethash(element) => element.fmt(f),
                DelegatableCoreCalls::GetSigneddelegationPackethash(element) => element.fmt(f),
                DelegatableCoreCalls::GetTransactionPackethash(element) => element.fmt(f),
                DelegatableCoreCalls::GetNonce(element) => element.fmt(f),
                DelegatableCoreCalls::VerifyDelegationSignature(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetCaveatArrayPackethashCall> for DelegatableCoreCalls {
        fn from(var: GetCaveatArrayPackethashCall) -> Self {
            DelegatableCoreCalls::GetCaveatArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetCaveatPackethashCall> for DelegatableCoreCalls {
        fn from(var: GetCaveatPackethashCall) -> Self {
            DelegatableCoreCalls::GetCaveatPackethash(var)
        }
    }
    impl ::std::convert::From<GetDelegationPackethashCall> for DelegatableCoreCalls {
        fn from(var: GetDelegationPackethashCall) -> Self {
            DelegatableCoreCalls::GetDelegationPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationsPackethashCall> for DelegatableCoreCalls {
        fn from(var: GetInvocationsPackethashCall) -> Self {
            DelegatableCoreCalls::GetInvocationsPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationArrayPackethashCall> for DelegatableCoreCalls {
        fn from(var: GetInvocationArrayPackethashCall) -> Self {
            DelegatableCoreCalls::GetInvocationArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationPackethashCall> for DelegatableCoreCalls {
        fn from(var: GetInvocationPackethashCall) -> Self {
            DelegatableCoreCalls::GetInvocationPackethash(var)
        }
    }
    impl ::std::convert::From<GetReplayprotectionPackethashCall> for DelegatableCoreCalls {
        fn from(var: GetReplayprotectionPackethashCall) -> Self {
            DelegatableCoreCalls::GetReplayprotectionPackethash(var)
        }
    }
    impl ::std::convert::From<GetSigneddelegationArrayPackethashCall> for DelegatableCoreCalls {
        fn from(var: GetSigneddelegationArrayPackethashCall) -> Self {
            DelegatableCoreCalls::GetSigneddelegationArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetSigneddelegationPackethashCall> for DelegatableCoreCalls {
        fn from(var: GetSigneddelegationPackethashCall) -> Self {
            DelegatableCoreCalls::GetSigneddelegationPackethash(var)
        }
    }
    impl ::std::convert::From<GetTransactionPackethashCall> for DelegatableCoreCalls {
        fn from(var: GetTransactionPackethashCall) -> Self {
            DelegatableCoreCalls::GetTransactionPackethash(var)
        }
    }
    impl ::std::convert::From<GetNonceCall> for DelegatableCoreCalls {
        fn from(var: GetNonceCall) -> Self {
            DelegatableCoreCalls::GetNonce(var)
        }
    }
    impl ::std::convert::From<VerifyDelegationSignatureCall> for DelegatableCoreCalls {
        fn from(var: VerifyDelegationSignatureCall) -> Self {
            DelegatableCoreCalls::VerifyDelegationSignature(var)
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
}
