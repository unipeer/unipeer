pub use i_delegatable::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_delegatable {
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
    #[doc = "IDelegatable was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IDELEGATABLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"contractInvoke\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDelegationTypedDataHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"contractName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"version\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"chainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"verifyingContract\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"getEIP712DomainHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocations\",\"name\":\"invocations\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInvocationsTypedDataHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedInvocation[]\",\"name\":\"signedInvocations\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Invocations\",\"name\":\"invocations\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"invoke\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation\",\"name\":\"signedDelegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifyDelegationSignature\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedInvocation\",\"name\":\"signedInvocation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocations\",\"name\":\"invocations\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifyInvocationSignature\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    pub struct IDelegatable<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IDelegatable<M> {
        fn clone(&self) -> Self {
            IDelegatable(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IDelegatable<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IDelegatable<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IDelegatable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IDelegatable<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IDELEGATABLE_ABI.clone(), client).into()
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IDelegatable<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
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
    pub enum IDelegatableCalls {
        ContractInvoke(ContractInvokeCall),
        GetDelegationTypedDataHash(GetDelegationTypedDataHashCall),
        GetEIP712DomainHash(GetEIP712DomainHashCall),
        GetInvocationsTypedDataHash(GetInvocationsTypedDataHashCall),
        Invoke(InvokeCall),
        VerifyDelegationSignature(VerifyDelegationSignatureCall),
        VerifyInvocationSignature(VerifyInvocationSignatureCall),
    }
    impl ethers::core::abi::AbiDecode for IDelegatableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ContractInvokeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDelegatableCalls::ContractInvoke(decoded));
            }
            if let Ok(decoded) =
                <GetDelegationTypedDataHashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IDelegatableCalls::GetDelegationTypedDataHash(decoded));
            }
            if let Ok(decoded) =
                <GetEIP712DomainHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDelegatableCalls::GetEIP712DomainHash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationsTypedDataHashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IDelegatableCalls::GetInvocationsTypedDataHash(decoded));
            }
            if let Ok(decoded) = <InvokeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDelegatableCalls::Invoke(decoded));
            }
            if let Ok(decoded) =
                <VerifyDelegationSignatureCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IDelegatableCalls::VerifyDelegationSignature(decoded));
            }
            if let Ok(decoded) =
                <VerifyInvocationSignatureCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IDelegatableCalls::VerifyInvocationSignature(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IDelegatableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IDelegatableCalls::ContractInvoke(element) => element.encode(),
                IDelegatableCalls::GetDelegationTypedDataHash(element) => element.encode(),
                IDelegatableCalls::GetEIP712DomainHash(element) => element.encode(),
                IDelegatableCalls::GetInvocationsTypedDataHash(element) => element.encode(),
                IDelegatableCalls::Invoke(element) => element.encode(),
                IDelegatableCalls::VerifyDelegationSignature(element) => element.encode(),
                IDelegatableCalls::VerifyInvocationSignature(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IDelegatableCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IDelegatableCalls::ContractInvoke(element) => element.fmt(f),
                IDelegatableCalls::GetDelegationTypedDataHash(element) => element.fmt(f),
                IDelegatableCalls::GetEIP712DomainHash(element) => element.fmt(f),
                IDelegatableCalls::GetInvocationsTypedDataHash(element) => element.fmt(f),
                IDelegatableCalls::Invoke(element) => element.fmt(f),
                IDelegatableCalls::VerifyDelegationSignature(element) => element.fmt(f),
                IDelegatableCalls::VerifyInvocationSignature(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ContractInvokeCall> for IDelegatableCalls {
        fn from(var: ContractInvokeCall) -> Self {
            IDelegatableCalls::ContractInvoke(var)
        }
    }
    impl ::std::convert::From<GetDelegationTypedDataHashCall> for IDelegatableCalls {
        fn from(var: GetDelegationTypedDataHashCall) -> Self {
            IDelegatableCalls::GetDelegationTypedDataHash(var)
        }
    }
    impl ::std::convert::From<GetEIP712DomainHashCall> for IDelegatableCalls {
        fn from(var: GetEIP712DomainHashCall) -> Self {
            IDelegatableCalls::GetEIP712DomainHash(var)
        }
    }
    impl ::std::convert::From<GetInvocationsTypedDataHashCall> for IDelegatableCalls {
        fn from(var: GetInvocationsTypedDataHashCall) -> Self {
            IDelegatableCalls::GetInvocationsTypedDataHash(var)
        }
    }
    impl ::std::convert::From<InvokeCall> for IDelegatableCalls {
        fn from(var: InvokeCall) -> Self {
            IDelegatableCalls::Invoke(var)
        }
    }
    impl ::std::convert::From<VerifyDelegationSignatureCall> for IDelegatableCalls {
        fn from(var: VerifyDelegationSignatureCall) -> Self {
            IDelegatableCalls::VerifyDelegationSignature(var)
        }
    }
    impl ::std::convert::From<VerifyInvocationSignatureCall> for IDelegatableCalls {
        fn from(var: VerifyInvocationSignatureCall) -> Self {
            IDelegatableCalls::VerifyInvocationSignature(var)
        }
    }
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
