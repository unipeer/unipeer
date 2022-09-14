pub use caveat_enforcer::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod caveat_enforcer {
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
    #[doc = "CaveatEnforcer was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CAVEATENFORCER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"struct Transaction\",\"name\":\"tx\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"delegationHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enforceCaveat\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct CaveatEnforcer<M>(ethers::contract::Contract<M>);
    impl<M> Clone for CaveatEnforcer<M> {
        fn clone(&self) -> Self {
            CaveatEnforcer(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CaveatEnforcer<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CaveatEnforcer<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CaveatEnforcer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> CaveatEnforcer<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CAVEATENFORCER_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `enforceCaveat` (0x5068de4c) function"]
        pub fn enforce_caveat(
            &self,
            terms: ethers::core::types::Bytes,
            tx: Transaction,
            delegation_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([80, 104, 222, 76], (terms, tx, delegation_hash))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for CaveatEnforcer<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
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
        pub terms: ethers::core::types::Bytes,
        pub tx: Transaction,
        pub delegation_hash: [u8; 32],
    }
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
}
