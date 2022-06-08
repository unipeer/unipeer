pub use unipeer_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod unipeer_mod {
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
    #[doc = "Unipeer was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNIPEER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Deposit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"paymentId\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"paymentName\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPaymentMethod\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Withdraw\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"_metaEvidenceId\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_paymentName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_extraData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"_tokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPaymentMethod\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_paymentId\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"depositTokens\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numOfMethods\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_paymentId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_paymentAddress\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePaymentAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_paymentId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"_tokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"bool[]\",\"name\":\"_enabled\",\"type\":\"bool[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePaymentToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawTokens\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static UNIPEER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610e4c806100206000396000f3fe608060405234801561001057600080fd5b506004361061007d5760003560e01c806383d508a41161005b57806383d508a4146100bd57806390e6fecb146100d0578063a7cc6a06146100e3578063f851a4401461011057600080fd5b806306b091f9146100825780635ecddfe8146100975780637455ad85146100aa575b600080fd5b610095610090366004610924565b61013b565b005b6100956100a53660046109a9565b61019c565b6100956100b8366004610a41565b6101e8565b6100956100cb366004610b26565b610394565b6100956100de366004610c26565b6104b1565b6000546100f890600160a01b900461ffff1681565b60405161ffff90911681526020015b60405180910390f35b600054610123906001600160a01b031681565b6040516001600160a01b039091168152602001610107565b61014f6001600160a01b038316338361057a565b604080513381526001600160a01b03841660208201529081018290527f9b1bfa7fa9ee420a16e124f794c35ac9f90472acc99140eb2f6447c714cad8eb9060600160405180910390a15050565b60005461ffff600160a01b9091048116908416106101b957600080fd5b33600090815260026020908152604080832061ffff8716845290915290206101e290838361086f565b50505050565b6000546001600160a01b031633146102475760405162461bcd60e51b815260206004820152601f60248201527f416363657373206e6f7420616c6c6f7765643a2041646d696e206f6e6c792e0060448201526064015b60405180910390fd5b600080546001908290600160a01b900461ffff1681601461026783610c78565b82546101009290920a61ffff8181021990931691831602179091551681526020810191909152604001600020805460ff191660ff8a1617815590506102b060018201888861086f565b506102bf60028201868661086f565b5060005b828110156103345760018260030160008686858181106102e5576102e5610c9a565b90506020020160208101906102fa9190610cb0565b6001600160a01b031681526020810191909152604001600020805460ff19169115159190911790558061032c81610ccb565b9150506102c3565b506000547f02eee3a5b72644922ad46f7c8834672d47b331ba43789607a0be0d2e1880dd0b9061037190600190600160a01b900461ffff16610ce6565b888860405161038293929190610d09565b60405180910390a15050505050505050565b6000546001600160a01b031633146103ee5760405162461bcd60e51b815260206004820152601f60248201527f416363657373206e6f7420616c6c6f7765643a2041646d696e206f6e6c792e00604482015260640161023e565b60005461ffff600160a01b90910481169085161061040b57600080fd5b61ffff84166000908152600160205260408120905b838110156104a95782818151811061043a5761043a610c9a565b602002602001015182600301600087878581811061045a5761045a610c9a565b905060200201602081019061046f9190610cb0565b6001600160a01b031681526020810191909152604001600020805460ff1916911515919091179055806104a181610ccb565b915050610420565b505050505050565b3360008181526003602090815260408083206001600160a01b03881680855290835281842087905584845260048352818420818552835290832080546001810182559084529190922060108204018054600f9092166002026101000a61ffff818102199093169286160291909117905561052c9130856105e2565b604080513381526001600160a01b03851660208201529081018390527f5548c837ab068cf56a2c2479df0882a4922fd203edb7517321831d95078c5f629060600160405180910390a1505050565b6040516001600160a01b0383166024820152604481018290526105dd90849063a9059cbb60e01b906064015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b03199093169290921790915261061a565b505050565b6040516001600160a01b03808516602483015283166044820152606481018290526101e29085906323b872dd60e01b906084016105a6565b600061066f826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166106ec9092919063ffffffff16565b8051909150156105dd578080602001905181019061068d9190610d43565b6105dd5760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b606482015260840161023e565b60606106fb8484600085610705565b90505b9392505050565b6060824710156107665760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b606482015260840161023e565b6001600160a01b0385163b6107bd5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e7472616374000000604482015260640161023e565b600080866001600160a01b031685876040516107d99190610d8c565b60006040518083038185875af1925050503d8060008114610816576040519150601f19603f3d011682016040523d82523d6000602084013e61081b565b606091505b509150915061082b828286610836565b979650505050505050565b606083156108455750816106fe565b8251156108555782518084602001fd5b8160405162461bcd60e51b815260040161023e9190610da8565b82805461087b90610ddb565b90600052602060002090601f01602090048101928261089d57600085556108e3565b82601f106108b65782800160ff198235161785556108e3565b828001600101855582156108e3579182015b828111156108e35782358255916020019190600101906108c8565b506108ef9291506108f3565b5090565b5b808211156108ef57600081556001016108f4565b80356001600160a01b038116811461091f57600080fd5b919050565b6000806040838503121561093757600080fd5b61094083610908565b946020939093013593505050565b803561ffff8116811461091f57600080fd5b60008083601f84011261097257600080fd5b50813567ffffffffffffffff81111561098a57600080fd5b6020830191508360208285010111156109a257600080fd5b9250929050565b6000806000604084860312156109be57600080fd5b6109c78461094e565b9250602084013567ffffffffffffffff8111156109e357600080fd5b6109ef86828701610960565b9497909650939450505050565b60008083601f840112610a0e57600080fd5b50813567ffffffffffffffff811115610a2657600080fd5b6020830191508360208260051b85010111156109a257600080fd5b60008060008060008060006080888a031215610a5c57600080fd5b873560ff81168114610a6d57600080fd5b9650602088013567ffffffffffffffff80821115610a8a57600080fd5b610a968b838c01610960565b909850965060408a0135915080821115610aaf57600080fd5b610abb8b838c01610960565b909650945060608a0135915080821115610ad457600080fd5b50610ae18a828b016109fc565b989b979a50959850939692959293505050565b634e487b7160e01b600052604160045260246000fd5b8015158114610b1857600080fd5b50565b803561091f81610b0a565b60008060008060608587031215610b3c57600080fd5b610b458561094e565b935060208086013567ffffffffffffffff80821115610b6357600080fd5b610b6f89838a016109fc565b90965094506040880135915080821115610b8857600080fd5b818801915088601f830112610b9c57600080fd5b813581811115610bae57610bae610af4565b8060051b604051601f19603f83011681018181108582111715610bd357610bd3610af4565b60405291825284820192508381018501918b831115610bf157600080fd5b938501935b82851015610c1657610c0785610b1b565b84529385019392850192610bf6565b989b979a50959850505050505050565b600080600060608486031215610c3b57600080fd5b610c4484610908565b925060208401359150610c596040850161094e565b90509250925092565b634e487b7160e01b600052601160045260246000fd5b600061ffff80831681811415610c9057610c90610c62565b6001019392505050565b634e487b7160e01b600052603260045260246000fd5b600060208284031215610cc257600080fd5b6106fe82610908565b6000600019821415610cdf57610cdf610c62565b5060010190565b600061ffff83811690831681811015610d0157610d01610c62565b039392505050565b61ffff8416815260406020820152816040820152818360608301376000818301606090810191909152601f909201601f1916010192915050565b600060208284031215610d5557600080fd5b81516106fe81610b0a565b60005b83811015610d7b578181015183820152602001610d63565b838111156101e25750506000910152565b60008251610d9e818460208701610d60565b9190910192915050565b6020815260008251806020840152610dc7816040850160208701610d60565b601f01601f19169190910160400192915050565b600181811c90821680610def57607f821691505b60208210811415610e1057634e487b7160e01b600052602260045260246000fd5b5091905056fea2646970667358221220fca2bc11b789632a12355f50c3aba0143a48ceba3bf12dc436fea18baa0c26c964736f6c634300080a0033" . parse () . expect ("invalid bytecode")
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
        ) -> Result<
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
        #[doc = "Calls the contract's `addPaymentMethod` (0x7455ad85) function"]
        pub fn add_payment_method(
            &self,
            meta_evidence_id: u8,
            payment_name: String,
            extra_data: ethers::core::types::Bytes,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [116, 85, 173, 133],
                    (meta_evidence_id, payment_name, extra_data, tokens),
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
        #[doc = "Calls the contract's `depositTokens` (0x90e6fecb) function"]
        pub fn deposit_tokens(
            &self,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            payment_id: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 230, 254, 203], (token, amount, payment_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `numOfMethods` (0xa7cc6a06) function"]
        pub fn num_of_methods(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([167, 204, 106, 6], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updatePaymentAddress` (0x5ecddfe8) function"]
        pub fn update_payment_address(
            &self,
            payment_id: u16,
            payment_address: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 205, 223, 232], (payment_id, payment_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updatePaymentToken` (0x83d508a4) function"]
        pub fn update_payment_token(
            &self,
            payment_id: u16,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
            enabled: ::std::vec::Vec<bool>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 213, 8, 164], (payment_id, tokens, enabled))
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
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(&self) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewPaymentMethod` event"]
        pub fn new_payment_method_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewPaymentMethodFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdraw` event"]
        pub fn withdraw_filter(&self) -> ethers::contract::builders::Event<M, WithdrawFilter> {
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,uint256)")]
    pub struct DepositFilter {
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
    #[ethevent(name = "NewPaymentMethod", abi = "NewPaymentMethod(uint16,string)")]
    pub struct NewPaymentMethodFilter {
        pub payment_id: u16,
        pub payment_name: String,
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
    #[ethevent(name = "Withdraw", abi = "Withdraw(address,address,uint256)")]
    pub struct WithdrawFilter {
        pub sender: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UnipeerEvents {
        DepositFilter(DepositFilter),
        NewPaymentMethodFilter(NewPaymentMethodFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ethers::contract::EthLogDecode for UnipeerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(UnipeerEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = NewPaymentMethodFilter::decode_log(log) {
                return Ok(UnipeerEvents::NewPaymentMethodFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(UnipeerEvents::WithdrawFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for UnipeerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UnipeerEvents::DepositFilter(element) => element.fmt(f),
                UnipeerEvents::NewPaymentMethodFilter(element) => element.fmt(f),
                UnipeerEvents::WithdrawFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addPaymentMethod`function with signature `addPaymentMethod(uint8,string,bytes,address[])` and selector `[116, 85, 173, 133]`"]
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
        abi = "addPaymentMethod(uint8,string,bytes,address[])"
    )]
    pub struct AddPaymentMethodCall {
        pub meta_evidence_id: u8,
        pub payment_name: String,
        pub extra_data: ethers::core::types::Bytes,
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `admin`function with signature `admin()` and selector `[248, 81, 164, 64]`"]
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
    #[doc = "Container type for all input parameters for the `depositTokens`function with signature `depositTokens(address,uint256,uint16)` and selector `[144, 230, 254, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "depositTokens", abi = "depositTokens(address,uint256,uint16)")]
    pub struct DepositTokensCall {
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub payment_id: u16,
    }
    #[doc = "Container type for all input parameters for the `numOfMethods`function with signature `numOfMethods()` and selector `[167, 204, 106, 6]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "numOfMethods", abi = "numOfMethods()")]
    pub struct NumOfMethodsCall;
    #[doc = "Container type for all input parameters for the `updatePaymentAddress`function with signature `updatePaymentAddress(uint16,string)` and selector `[94, 205, 223, 232]`"]
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
        name = "updatePaymentAddress",
        abi = "updatePaymentAddress(uint16,string)"
    )]
    pub struct UpdatePaymentAddressCall {
        pub payment_id: u16,
        pub payment_address: String,
    }
    #[doc = "Container type for all input parameters for the `updatePaymentToken`function with signature `updatePaymentToken(uint16,address[],bool[])` and selector `[131, 213, 8, 164]`"]
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
        name = "updatePaymentToken",
        abi = "updatePaymentToken(uint16,address[],bool[])"
    )]
    pub struct UpdatePaymentTokenCall {
        pub payment_id: u16,
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub enabled: ::std::vec::Vec<bool>,
    }
    #[doc = "Container type for all input parameters for the `withdrawTokens`function with signature `withdrawTokens(address,uint256)` and selector `[6, 176, 145, 249]`"]
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
        AddPaymentMethod(AddPaymentMethodCall),
        Admin(AdminCall),
        DepositTokens(DepositTokensCall),
        NumOfMethods(NumOfMethodsCall),
        UpdatePaymentAddress(UpdatePaymentAddressCall),
        UpdatePaymentToken(UpdatePaymentTokenCall),
        WithdrawTokens(WithdrawTokensCall),
    }
    impl ethers::core::abi::AbiDecode for UnipeerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
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
                <DepositTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::DepositTokens(decoded));
            }
            if let Ok(decoded) =
                <NumOfMethodsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::NumOfMethods(decoded));
            }
            if let Ok(decoded) =
                <UpdatePaymentAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::UpdatePaymentAddress(decoded));
            }
            if let Ok(decoded) =
                <UpdatePaymentTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UnipeerCalls::UpdatePaymentToken(decoded));
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
                UnipeerCalls::AddPaymentMethod(element) => element.encode(),
                UnipeerCalls::Admin(element) => element.encode(),
                UnipeerCalls::DepositTokens(element) => element.encode(),
                UnipeerCalls::NumOfMethods(element) => element.encode(),
                UnipeerCalls::UpdatePaymentAddress(element) => element.encode(),
                UnipeerCalls::UpdatePaymentToken(element) => element.encode(),
                UnipeerCalls::WithdrawTokens(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UnipeerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UnipeerCalls::AddPaymentMethod(element) => element.fmt(f),
                UnipeerCalls::Admin(element) => element.fmt(f),
                UnipeerCalls::DepositTokens(element) => element.fmt(f),
                UnipeerCalls::NumOfMethods(element) => element.fmt(f),
                UnipeerCalls::UpdatePaymentAddress(element) => element.fmt(f),
                UnipeerCalls::UpdatePaymentToken(element) => element.fmt(f),
                UnipeerCalls::WithdrawTokens(element) => element.fmt(f),
            }
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
    impl ::std::convert::From<DepositTokensCall> for UnipeerCalls {
        fn from(var: DepositTokensCall) -> Self {
            UnipeerCalls::DepositTokens(var)
        }
    }
    impl ::std::convert::From<NumOfMethodsCall> for UnipeerCalls {
        fn from(var: NumOfMethodsCall) -> Self {
            UnipeerCalls::NumOfMethods(var)
        }
    }
    impl ::std::convert::From<UpdatePaymentAddressCall> for UnipeerCalls {
        fn from(var: UpdatePaymentAddressCall) -> Self {
            UnipeerCalls::UpdatePaymentAddress(var)
        }
    }
    impl ::std::convert::From<UpdatePaymentTokenCall> for UnipeerCalls {
        fn from(var: UpdatePaymentTokenCall) -> Self {
            UnipeerCalls::UpdatePaymentToken(var)
        }
    }
    impl ::std::convert::From<WithdrawTokensCall> for UnipeerCalls {
        fn from(var: WithdrawTokensCall) -> Self {
            UnipeerCalls::WithdrawTokens(var)
        }
    }
}
