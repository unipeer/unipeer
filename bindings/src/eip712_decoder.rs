pub use eip712_decoder::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod eip712_decoder {
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
    #[doc = "EIP712Decoder was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static EIP712DECODER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct Caveat[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_CAVEAT_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Caveat\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_CAVEAT_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Delegation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_DELEGATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocations\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"batch\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"struct ReplayProtection\",\"name\":\"replayProtection\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATIONS_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocation[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATION_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Invocation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Transaction\",\"name\":\"transaction\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"authority\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_INVOCATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ReplayProtection\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queue\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_REPLAYPROTECTION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation[]\",\"name\":\"_input\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_SIGNEDDELEGATION_ARRAY_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SignedDelegation\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Delegation\",\"name\":\"delegation\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"authority\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"struct Caveat[]\",\"name\":\"caveats\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"enforcer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"terms\",\"type\":\"bytes\",\"components\":[]}]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_SIGNEDDELEGATION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Transaction\",\"name\":\"_input\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"GET_TRANSACTION_PACKETHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static EIP712DECODER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610d26806100206000396000f3fe608060405234801561001057600080fd5b506004361061009e5760003560e01c80636f965803116100665780636f965803146101145780637234eefe14610127578063736f7ce71461013a5780637b577b581461014d578063a2e01f751461016057600080fd5b806308aaf6b0146100a35780632f52a2fd146100c85780633a481821146100db5780635cf1b24a146100ee5780636c2b125314610101575b600080fd5b6100b66100b13660046106cd565b610173565b60405190815260200160405180910390f35b6100b66100d636600461081d565b6101f3565b6100b66100e9366004610a69565b61025e565b6100b66100fc366004610a9d565b6102d5565b6100b661010f366004610ad1565b610332565b6100b6610122366004610b35565b61038f565b6100b6610135366004610b58565b6103e0565b6100b6610148366004610bd6565b61041e565b6100b661015b366004610c0a565b610486565b6100b661016e366004610c3e565b6104ee565b6000807f80ad7e1b04ee6d994a125f4714ca0720908bd80ed16063ec8aee4b88e9253e2d83600001518460200151805190602001206040516020016101d4939291909283526001600160a01b03919091166020830152604082015260600190565b60408051601f1981840301815291905280516020909101209392505050565b6000807f409f5114779a253e700d775d7845e6efc1e83685ac59868d2df3d4de51c7d6218360000151846020015161022e8660400151610486565b6040805160208101959095526001600160a01b03909316928401929092526060830152608082015260a0016101d4565b6000606060005b83518110156102c6578161029185838151811061028457610284610c72565b60200260200101516102d5565b6040516020016102a2929190610c88565b604051602081830303815290604052915080806102be90610cc9565b915050610265565b50805160209091012092915050565b6000807fd97dd99b404d177890f06a8f0fc8e5ed0333fb2ebb6684360709066e8984f59461030684600001516104ee565b610313856020015161041e565b60408051602081019490945283019190915260608201526080016101d4565b6000807f3c36a06e1d288b0f94f565588317a46ad11bc3c96992109f9a2365a2737259a761036384600001516101f3565b6020808601518051908201206040516101d4949392019283526020830191909152604082015260600190565b6000807fe8d3d963b33868fb116316bc3fd55e8f49123f30e4418f71e140d54b7cd3b2b9836000015184602001516040516020016101d4939291909283526020830191909152604082015260600190565b6000807f08c69a206c06f5334b35ceb1186181a713b21aff02cf66285f375084fbef2eb8610411846000015161025e565b610313856020015161038f565b6000606060005b83518110156102c6578161045185838151811061044457610444610c72565b6020026020010151610332565b604051602001610462929190610c88565b6040516020818303038152906040529150808061047e90610cc9565b915050610425565b6000606060005b83518110156102c657816104b98583815181106104ac576104ac610c72565b6020026020010151610173565b6040516020016104ca929190610c88565b604051602081830303815290604052915080806104e690610cc9565b91505061048d565b6000807fb16dfdb3b8fa033fe30ac976cd4a50ad256b6811c80d90fcd0b323eec190047d836000015184602001518560400151805190602001206040516020016101d494939291909384526001600160a01b039290921660208401526040830152606082015260800190565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156105925761059261055a565b60405290565b604051606081016001600160401b03811182821017156105925761059261055a565b604051601f8201601f191681016001600160401b03811182821017156105e2576105e261055a565b604052919050565b80356001600160a01b038116811461060157600080fd5b919050565b600082601f83011261061757600080fd5b81356001600160401b038111156106305761063061055a565b610643601f8201601f19166020016105ba565b81815284602083860101111561065857600080fd5b816020850160208301376000918101602001919091529392505050565b60006040828403121561068757600080fd5b61068f610570565b905061069a826105ea565b815260208201356001600160401b038111156106b557600080fd5b6106c184828501610606565b60208301525092915050565b6000602082840312156106df57600080fd5b81356001600160401b038111156106f557600080fd5b61070184828501610675565b949350505050565b60006001600160401b038211156107225761072261055a565b5060051b60200190565b600082601f83011261073d57600080fd5b8135602061075261074d83610709565b6105ba565b82815260059290921b8401810191818101908684111561077157600080fd5b8286015b848110156107b05780356001600160401b038111156107945760008081fd5b6107a28986838b0101610675565b845250918301918301610775565b509695505050505050565b6000606082840312156107cd57600080fd5b6107d5610598565b90506107e0826105ea565b81526020820135602082015260408201356001600160401b0381111561080557600080fd5b6108118482850161072c565b60408301525092915050565b60006020828403121561082f57600080fd5b81356001600160401b0381111561084557600080fd5b610701848285016107bb565b60006060828403121561086357600080fd5b61086b610598565b9050610876826105ea565b81526020820135602082015260408201356001600160401b0381111561089b57600080fd5b61081184828501610606565b6000604082840312156108b957600080fd5b6108c1610570565b905081356001600160401b03808211156108da57600080fd5b6108e6858386016107bb565b835260208401359150808211156108fc57600080fd5b506106c184828501610606565b600082601f83011261091a57600080fd5b8135602061092a61074d83610709565b82815260059290921b8401810191818101908684111561094957600080fd5b8286015b848110156107b05780356001600160401b0381111561096c5760008081fd5b61097a8986838b01016108a7565b84525091830191830161094d565b60006040828403121561099a57600080fd5b6109a2610570565b905081356001600160401b03808211156109bb57600080fd5b6109c785838601610851565b835260208401359150808211156109dd57600080fd5b506106c184828501610909565b600082601f8301126109fb57600080fd5b81356020610a0b61074d83610709565b82815260059290921b84018101918181019086841115610a2a57600080fd5b8286015b848110156107b05780356001600160401b03811115610a4d5760008081fd5b610a5b8986838b0101610988565b845250918301918301610a2e565b600060208284031215610a7b57600080fd5b81356001600160401b03811115610a9157600080fd5b610701848285016109ea565b600060208284031215610aaf57600080fd5b81356001600160401b03811115610ac557600080fd5b61070184828501610988565b600060208284031215610ae357600080fd5b81356001600160401b03811115610af957600080fd5b610701848285016108a7565b600060408284031215610b1757600080fd5b610b1f610570565b9050813581526020820135602082015292915050565b600060408284031215610b4757600080fd5b610b518383610b05565b9392505050565b600060208284031215610b6a57600080fd5b81356001600160401b0380821115610b8157600080fd5b9083019060608286031215610b9557600080fd5b610b9d610570565b823582811115610bac57600080fd5b610bb8878286016109ea565b825250610bc88660208501610b05565b602082015295945050505050565b600060208284031215610be857600080fd5b81356001600160401b03811115610bfe57600080fd5b61070184828501610909565b600060208284031215610c1c57600080fd5b81356001600160401b03811115610c3257600080fd5b6107018482850161072c565b600060208284031215610c5057600080fd5b81356001600160401b03811115610c6657600080fd5b61070184828501610851565b634e487b7160e01b600052603260045260246000fd5b6000835160005b81811015610ca95760208187018101518583015201610c8f565b81811115610cb8576000828501525b509190910191825250602001919050565b600060018201610ce957634e487b7160e01b600052601160045260246000fd5b506001019056fea2646970667358221220b69eb6e6f037cfe3b06e8cb4133223bdf75c3ddb5980cfb7def378dc134171f564736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct EIP712Decoder<M>(ethers::contract::Contract<M>);
    impl<M> Clone for EIP712Decoder<M> {
        fn clone(&self) -> Self {
            EIP712Decoder(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for EIP712Decoder<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for EIP712Decoder<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(EIP712Decoder))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> EIP712Decoder<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), EIP712DECODER_ABI.clone(), client)
                .into()
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
                EIP712DECODER_ABI.clone(),
                EIP712DECODER_BYTECODE.clone().into(),
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
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for EIP712Decoder<M> {
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum EIP712DecoderCalls {
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
    }
    impl ethers::core::abi::AbiDecode for EIP712DecoderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetCaveatArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EIP712DecoderCalls::GetCaveatArrayPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetCaveatPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EIP712DecoderCalls::GetCaveatPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetDelegationPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EIP712DecoderCalls::GetDelegationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationsPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EIP712DecoderCalls::GetInvocationsPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EIP712DecoderCalls::GetInvocationArrayPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetInvocationPackethashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(EIP712DecoderCalls::GetInvocationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetReplayprotectionPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EIP712DecoderCalls::GetReplayprotectionPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetSigneddelegationArrayPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EIP712DecoderCalls::GetSigneddelegationArrayPackethash(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetSigneddelegationPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EIP712DecoderCalls::GetSigneddelegationPackethash(decoded));
            }
            if let Ok(decoded) =
                <GetTransactionPackethashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(EIP712DecoderCalls::GetTransactionPackethash(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for EIP712DecoderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                EIP712DecoderCalls::GetCaveatArrayPackethash(element) => element.encode(),
                EIP712DecoderCalls::GetCaveatPackethash(element) => element.encode(),
                EIP712DecoderCalls::GetDelegationPackethash(element) => element.encode(),
                EIP712DecoderCalls::GetInvocationsPackethash(element) => element.encode(),
                EIP712DecoderCalls::GetInvocationArrayPackethash(element) => element.encode(),
                EIP712DecoderCalls::GetInvocationPackethash(element) => element.encode(),
                EIP712DecoderCalls::GetReplayprotectionPackethash(element) => element.encode(),
                EIP712DecoderCalls::GetSigneddelegationArrayPackethash(element) => element.encode(),
                EIP712DecoderCalls::GetSigneddelegationPackethash(element) => element.encode(),
                EIP712DecoderCalls::GetTransactionPackethash(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for EIP712DecoderCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                EIP712DecoderCalls::GetCaveatArrayPackethash(element) => element.fmt(f),
                EIP712DecoderCalls::GetCaveatPackethash(element) => element.fmt(f),
                EIP712DecoderCalls::GetDelegationPackethash(element) => element.fmt(f),
                EIP712DecoderCalls::GetInvocationsPackethash(element) => element.fmt(f),
                EIP712DecoderCalls::GetInvocationArrayPackethash(element) => element.fmt(f),
                EIP712DecoderCalls::GetInvocationPackethash(element) => element.fmt(f),
                EIP712DecoderCalls::GetReplayprotectionPackethash(element) => element.fmt(f),
                EIP712DecoderCalls::GetSigneddelegationArrayPackethash(element) => element.fmt(f),
                EIP712DecoderCalls::GetSigneddelegationPackethash(element) => element.fmt(f),
                EIP712DecoderCalls::GetTransactionPackethash(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetCaveatArrayPackethashCall> for EIP712DecoderCalls {
        fn from(var: GetCaveatArrayPackethashCall) -> Self {
            EIP712DecoderCalls::GetCaveatArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetCaveatPackethashCall> for EIP712DecoderCalls {
        fn from(var: GetCaveatPackethashCall) -> Self {
            EIP712DecoderCalls::GetCaveatPackethash(var)
        }
    }
    impl ::std::convert::From<GetDelegationPackethashCall> for EIP712DecoderCalls {
        fn from(var: GetDelegationPackethashCall) -> Self {
            EIP712DecoderCalls::GetDelegationPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationsPackethashCall> for EIP712DecoderCalls {
        fn from(var: GetInvocationsPackethashCall) -> Self {
            EIP712DecoderCalls::GetInvocationsPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationArrayPackethashCall> for EIP712DecoderCalls {
        fn from(var: GetInvocationArrayPackethashCall) -> Self {
            EIP712DecoderCalls::GetInvocationArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetInvocationPackethashCall> for EIP712DecoderCalls {
        fn from(var: GetInvocationPackethashCall) -> Self {
            EIP712DecoderCalls::GetInvocationPackethash(var)
        }
    }
    impl ::std::convert::From<GetReplayprotectionPackethashCall> for EIP712DecoderCalls {
        fn from(var: GetReplayprotectionPackethashCall) -> Self {
            EIP712DecoderCalls::GetReplayprotectionPackethash(var)
        }
    }
    impl ::std::convert::From<GetSigneddelegationArrayPackethashCall> for EIP712DecoderCalls {
        fn from(var: GetSigneddelegationArrayPackethashCall) -> Self {
            EIP712DecoderCalls::GetSigneddelegationArrayPackethash(var)
        }
    }
    impl ::std::convert::From<GetSigneddelegationPackethashCall> for EIP712DecoderCalls {
        fn from(var: GetSigneddelegationPackethashCall) -> Self {
            EIP712DecoderCalls::GetSigneddelegationPackethash(var)
        }
    }
    impl ::std::convert::From<GetTransactionPackethashCall> for EIP712DecoderCalls {
        fn from(var: GetTransactionPackethashCall) -> Self {
            EIP712DecoderCalls::GetTransactionPackethash(var)
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
}
