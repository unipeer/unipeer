#[doc = "`Caveat(address,bytes)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Caveat {
    pub enforcer: ethers::core::types::Address,
    pub terms: ethers::core::types::Bytes,
}
#[doc = "`Delegation(address,bytes32,(address,bytes)[])`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Delegation {
    pub delegate: ethers::core::types::Address,
    pub authority: [u8; 32],
    pub caveats: ::std::vec::Vec<Caveat>,
}
#[doc = "`Invocations(((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256))`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Invocations {
    pub batch: ::std::vec::Vec<Invocation>,
    pub replay_protection: ReplayProtection,
}
#[doc = "`SignedDelegation((address,bytes32,(address,bytes)[]),bytes)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct SignedDelegation {
    pub delegation: Delegation,
    pub signature: ethers::core::types::Bytes,
}
#[doc = "`Invocation((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Invocation {
    pub transaction: Transaction,
    pub authority: ::std::vec::Vec<SignedDelegation>,
}
#[doc = "`ReplayProtection(uint256,uint256)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct ReplayProtection {
    pub nonce: ethers::core::types::U256,
    pub queue: ethers::core::types::U256,
}
#[doc = "`Transaction(address,uint256,bytes)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct Transaction {
    pub to: ethers::core::types::Address,
    pub gas_limit: ethers::core::types::U256,
    pub data: ethers::core::types::Bytes,
}
#[doc = "`SignedInvocation((((address,uint256,bytes),((address,bytes32,(address,bytes)[]),bytes)[])[],(uint256,uint256)),bytes)`"]
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ethers :: contract :: EthAbiType,
    ethers :: contract :: EthAbiCodec,
)]
pub struct SignedInvocation {
    pub invocations: Invocations,
    pub signature: ethers::core::types::Bytes,
}
