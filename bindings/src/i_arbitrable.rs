pub use i_arbitrable::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_arbitrable {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IArbitrable was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IARBITRABLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IArbitrator\",\"name\":\"_arbitrator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_ruling\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Ruling\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_ruling\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rule\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IArbitrable<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IArbitrable<M> {
        fn clone(&self) -> Self {
            IArbitrable(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IArbitrable<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IArbitrable<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IArbitrable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IArbitrable<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IARBITRABLE_ABI.clone(), client).into()
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
        #[doc = "Gets the contract's `Ruling` event"]
        pub fn ruling_filter(&self) -> ethers::contract::builders::Event<M, RulingFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, RulingFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IArbitrable<M> {
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
    #[ethevent(name = "Ruling", abi = "Ruling(address,uint256,uint256)")]
    pub struct RulingFilter {
        #[ethevent(indexed)]
        pub arbitrator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub dispute_id: ethers::core::types::U256,
        pub ruling: ethers::core::types::U256,
    }
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
}
