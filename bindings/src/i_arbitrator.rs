pub use i_arbitrator::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_arbitrator {
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
    #[doc = "IArbitrator was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IARBITRATOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"contract IArbitrable\",\"name\":\"_arbitrable\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AppealDecision\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"contract IArbitrable\",\"name\":\"_arbitrable\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AppealPossible\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"contract IArbitrable\",\"name\":\"_arbitrable\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"DisputeCreation\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_extraData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"appeal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_extraData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"appealCost\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"cost\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"appealPeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"start\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"end\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_extraData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"arbitrationCost\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"cost\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_choices\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_extraData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"createDispute\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"disputeID\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentRuling\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"ruling\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"disputeStatus\",\"outputs\":[{\"internalType\":\"enum IArbitrator.DisputeStatus\",\"name\":\"status\",\"type\":\"uint8\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IArbitrator<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IArbitrator<M> {
        fn clone(&self) -> Self {
            IArbitrator(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IArbitrator<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IArbitrator<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IArbitrator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IArbitrator<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IARBITRATOR_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `appeal` (0x49912f88) function"]
        pub fn appeal(
            &self,
            dispute_id: ethers::core::types::U256,
            extra_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 145, 47, 136], (dispute_id, extra_data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `appealCost` (0xf23f16e6) function"]
        pub fn appeal_cost(
            &self,
            dispute_id: ethers::core::types::U256,
            extra_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([242, 63, 22, 230], (dispute_id, extra_data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `appealPeriod` (0xafe15cfb) function"]
        pub fn appeal_period(
            &self,
            dispute_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([175, 225, 92, 251], dispute_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `arbitrationCost` (0xf7434ea9) function"]
        pub fn arbitration_cost(
            &self,
            extra_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([247, 67, 78, 169], extra_data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createDispute` (0xc13517e1) function"]
        pub fn create_dispute(
            &self,
            choices: ethers::core::types::U256,
            extra_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([193, 53, 23, 225], (choices, extra_data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentRuling` (0x1c3db16d) function"]
        pub fn current_ruling(
            &self,
            dispute_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([28, 61, 177, 109], dispute_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `disputeStatus` (0x10f169e8) function"]
        pub fn dispute_status(
            &self,
            dispute_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([16, 241, 105, 232], dispute_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AppealDecision` event"]
        pub fn appeal_decision_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AppealDecisionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AppealPossible` event"]
        pub fn appeal_possible_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AppealPossibleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DisputeCreation` event"]
        pub fn dispute_creation_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DisputeCreationFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IArbitratorEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IArbitrator<M> {
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
    #[ethevent(name = "AppealDecision", abi = "AppealDecision(uint256,address)")]
    pub struct AppealDecisionFilter {
        #[ethevent(indexed)]
        pub dispute_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub arbitrable: ethers::core::types::Address,
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
    #[ethevent(name = "AppealPossible", abi = "AppealPossible(uint256,address)")]
    pub struct AppealPossibleFilter {
        #[ethevent(indexed)]
        pub dispute_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub arbitrable: ethers::core::types::Address,
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
    #[ethevent(name = "DisputeCreation", abi = "DisputeCreation(uint256,address)")]
    pub struct DisputeCreationFilter {
        #[ethevent(indexed)]
        pub dispute_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub arbitrable: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IArbitratorEvents {
        AppealDecisionFilter(AppealDecisionFilter),
        AppealPossibleFilter(AppealPossibleFilter),
        DisputeCreationFilter(DisputeCreationFilter),
    }
    impl ethers::contract::EthLogDecode for IArbitratorEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AppealDecisionFilter::decode_log(log) {
                return Ok(IArbitratorEvents::AppealDecisionFilter(decoded));
            }
            if let Ok(decoded) = AppealPossibleFilter::decode_log(log) {
                return Ok(IArbitratorEvents::AppealPossibleFilter(decoded));
            }
            if let Ok(decoded) = DisputeCreationFilter::decode_log(log) {
                return Ok(IArbitratorEvents::DisputeCreationFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IArbitratorEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IArbitratorEvents::AppealDecisionFilter(element) => element.fmt(f),
                IArbitratorEvents::AppealPossibleFilter(element) => element.fmt(f),
                IArbitratorEvents::DisputeCreationFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `appeal` function with signature `appeal(uint256,bytes)` and selector `[73, 145, 47, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "appeal", abi = "appeal(uint256,bytes)")]
    pub struct AppealCall {
        pub dispute_id: ethers::core::types::U256,
        pub extra_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `appealCost` function with signature `appealCost(uint256,bytes)` and selector `[242, 63, 22, 230]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "appealCost", abi = "appealCost(uint256,bytes)")]
    pub struct AppealCostCall {
        pub dispute_id: ethers::core::types::U256,
        pub extra_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `appealPeriod` function with signature `appealPeriod(uint256)` and selector `[175, 225, 92, 251]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "appealPeriod", abi = "appealPeriod(uint256)")]
    pub struct AppealPeriodCall {
        pub dispute_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `arbitrationCost` function with signature `arbitrationCost(bytes)` and selector `[247, 67, 78, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "arbitrationCost", abi = "arbitrationCost(bytes)")]
    pub struct ArbitrationCostCall {
        pub extra_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `createDispute` function with signature `createDispute(uint256,bytes)` and selector `[193, 53, 23, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "createDispute", abi = "createDispute(uint256,bytes)")]
    pub struct CreateDisputeCall {
        pub choices: ethers::core::types::U256,
        pub extra_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `currentRuling` function with signature `currentRuling(uint256)` and selector `[28, 61, 177, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "currentRuling", abi = "currentRuling(uint256)")]
    pub struct CurrentRulingCall {
        pub dispute_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `disputeStatus` function with signature `disputeStatus(uint256)` and selector `[16, 241, 105, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "disputeStatus", abi = "disputeStatus(uint256)")]
    pub struct DisputeStatusCall {
        pub dispute_id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IArbitratorCalls {
        Appeal(AppealCall),
        AppealCost(AppealCostCall),
        AppealPeriod(AppealPeriodCall),
        ArbitrationCost(ArbitrationCostCall),
        CreateDispute(CreateDisputeCall),
        CurrentRuling(CurrentRulingCall),
        DisputeStatus(DisputeStatusCall),
    }
    impl ethers::core::abi::AbiDecode for IArbitratorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AppealCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IArbitratorCalls::Appeal(decoded));
            }
            if let Ok(decoded) =
                <AppealCostCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IArbitratorCalls::AppealCost(decoded));
            }
            if let Ok(decoded) =
                <AppealPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IArbitratorCalls::AppealPeriod(decoded));
            }
            if let Ok(decoded) =
                <ArbitrationCostCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IArbitratorCalls::ArbitrationCost(decoded));
            }
            if let Ok(decoded) =
                <CreateDisputeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IArbitratorCalls::CreateDispute(decoded));
            }
            if let Ok(decoded) =
                <CurrentRulingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IArbitratorCalls::CurrentRuling(decoded));
            }
            if let Ok(decoded) =
                <DisputeStatusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IArbitratorCalls::DisputeStatus(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IArbitratorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IArbitratorCalls::Appeal(element) => element.encode(),
                IArbitratorCalls::AppealCost(element) => element.encode(),
                IArbitratorCalls::AppealPeriod(element) => element.encode(),
                IArbitratorCalls::ArbitrationCost(element) => element.encode(),
                IArbitratorCalls::CreateDispute(element) => element.encode(),
                IArbitratorCalls::CurrentRuling(element) => element.encode(),
                IArbitratorCalls::DisputeStatus(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IArbitratorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IArbitratorCalls::Appeal(element) => element.fmt(f),
                IArbitratorCalls::AppealCost(element) => element.fmt(f),
                IArbitratorCalls::AppealPeriod(element) => element.fmt(f),
                IArbitratorCalls::ArbitrationCost(element) => element.fmt(f),
                IArbitratorCalls::CreateDispute(element) => element.fmt(f),
                IArbitratorCalls::CurrentRuling(element) => element.fmt(f),
                IArbitratorCalls::DisputeStatus(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AppealCall> for IArbitratorCalls {
        fn from(var: AppealCall) -> Self {
            IArbitratorCalls::Appeal(var)
        }
    }
    impl ::std::convert::From<AppealCostCall> for IArbitratorCalls {
        fn from(var: AppealCostCall) -> Self {
            IArbitratorCalls::AppealCost(var)
        }
    }
    impl ::std::convert::From<AppealPeriodCall> for IArbitratorCalls {
        fn from(var: AppealPeriodCall) -> Self {
            IArbitratorCalls::AppealPeriod(var)
        }
    }
    impl ::std::convert::From<ArbitrationCostCall> for IArbitratorCalls {
        fn from(var: ArbitrationCostCall) -> Self {
            IArbitratorCalls::ArbitrationCost(var)
        }
    }
    impl ::std::convert::From<CreateDisputeCall> for IArbitratorCalls {
        fn from(var: CreateDisputeCall) -> Self {
            IArbitratorCalls::CreateDispute(var)
        }
    }
    impl ::std::convert::From<CurrentRulingCall> for IArbitratorCalls {
        fn from(var: CurrentRulingCall) -> Self {
            IArbitratorCalls::CurrentRuling(var)
        }
    }
    impl ::std::convert::From<DisputeStatusCall> for IArbitratorCalls {
        fn from(var: DisputeStatusCall) -> Self {
            IArbitratorCalls::DisputeStatus(var)
        }
    }
    #[doc = "Container type for all return fields from the `appealCost` function with signature `appealCost(uint256,bytes)` and selector `[242, 63, 22, 230]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AppealCostReturn {
        pub cost: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `appealPeriod` function with signature `appealPeriod(uint256)` and selector `[175, 225, 92, 251]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AppealPeriodReturn {
        pub start: ethers::core::types::U256,
        pub end: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `arbitrationCost` function with signature `arbitrationCost(bytes)` and selector `[247, 67, 78, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ArbitrationCostReturn {
        pub cost: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `createDispute` function with signature `createDispute(uint256,bytes)` and selector `[193, 53, 23, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CreateDisputeReturn {
        pub dispute_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `currentRuling` function with signature `currentRuling(uint256)` and selector `[28, 61, 177, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CurrentRulingReturn {
        pub ruling: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `disputeStatus` function with signature `disputeStatus(uint256)` and selector `[16, 241, 105, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct DisputeStatusReturn {
        pub status: u8,
    }
}
