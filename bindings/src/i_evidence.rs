pub use i_evidence::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_evidence {
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
    #[doc = "IEvidence was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IEVIDENCE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IArbitrator\",\"name\":\"_arbitrator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_disputeID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_metaEvidenceID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"_evidenceGroupID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Dispute\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IArbitrator\",\"name\":\"_arbitrator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"_evidenceGroupID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"_party\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"_evidence\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Evidence\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_metaEvidenceID\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"_evidence\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MetaEvidence\",\"outputs\":[],\"anonymous\":false}]") . expect ("invalid abi")
        });
    pub struct IEvidence<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IEvidence<M> {
        fn clone(&self) -> Self {
            IEvidence(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IEvidence<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IEvidence<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IEvidence))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IEvidence<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IEVIDENCE_ABI.clone(), client).into()
        }
        #[doc = "Gets the contract's `Dispute` event"]
        pub fn dispute_filter(&self) -> ethers::contract::builders::Event<M, DisputeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Evidence` event"]
        pub fn evidence_filter(&self) -> ethers::contract::builders::Event<M, EvidenceFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MetaEvidence` event"]
        pub fn meta_evidence_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MetaEvidenceFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, IEvidenceEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IEvidence<M> {
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
    #[ethevent(name = "MetaEvidence", abi = "MetaEvidence(uint256,string)")]
    pub struct MetaEvidenceFilter {
        #[ethevent(indexed)]
        pub meta_evidence_id: ethers::core::types::U256,
        pub evidence: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IEvidenceEvents {
        DisputeFilter(DisputeFilter),
        EvidenceFilter(EvidenceFilter),
        MetaEvidenceFilter(MetaEvidenceFilter),
    }
    impl ethers::contract::EthLogDecode for IEvidenceEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DisputeFilter::decode_log(log) {
                return Ok(IEvidenceEvents::DisputeFilter(decoded));
            }
            if let Ok(decoded) = EvidenceFilter::decode_log(log) {
                return Ok(IEvidenceEvents::EvidenceFilter(decoded));
            }
            if let Ok(decoded) = MetaEvidenceFilter::decode_log(log) {
                return Ok(IEvidenceEvents::MetaEvidenceFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IEvidenceEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IEvidenceEvents::DisputeFilter(element) => element.fmt(f),
                IEvidenceEvents::EvidenceFilter(element) => element.fmt(f),
                IEvidenceEvents::MetaEvidenceFilter(element) => element.fmt(f),
            }
        }
    }
}
