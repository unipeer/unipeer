pub use unipeer_mod::*;
#[allow(clippy::too_many_arguments)]
mod unipeer_mod {
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
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"paymentId\",\"type\":\"uint16\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"paymentName\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPaymentMethod\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"_metaEvidenceId\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_paymentName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_extraData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"_tokens\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPaymentMethod\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"_paymentId\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"depositTokens\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numOfMethods\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_paymentId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_paymentAddress\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePaymentAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_paymentId\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"_tokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"bool[]\",\"name\":\"_enabled\",\"type\":\"bool[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePaymentToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawTokens\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static UNIPEER_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610db5806100206000396000f3fe608060405234801561001057600080fd5b506004361061007d5760003560e01c806383d508a41161005b57806383d508a4146100bd57806390e6fecb146100d0578063a7cc6a06146100e3578063f851a4401461011057600080fd5b806306b091f9146100825780635ecddfe8146100975780637455ad85146100aa575b600080fd5b61009561009036600461088d565b61013b565b005b6100956100a5366004610912565b610153565b6100956100b83660046109aa565b61019f565b6100956100cb366004610a8f565b61034b565b6100956100de366004610b8f565b610468565b6000546100f890600160a01b900461ffff1681565b60405161ffff90911681526020015b60405180910390f35b600054610123906001600160a01b031681565b6040516001600160a01b039091168152602001610107565b61014f6001600160a01b03831633836104e8565b5050565b60005461ffff600160a01b90910481169084161061017057600080fd5b33600090815260026020908152604080832061ffff8716845290915290206101999083836107d8565b50505050565b6000546001600160a01b031633146101fe5760405162461bcd60e51b815260206004820152601f60248201527f416363657373206e6f7420616c6c6f7765643a2041646d696e206f6e6c792e0060448201526064015b60405180910390fd5b600080546001908290600160a01b900461ffff1681601461021e83610be1565b82546101009290920a61ffff8181021990931691831602179091551681526020810191909152604001600020805460ff191660ff8a1617815590506102676001820188886107d8565b506102766002820186866107d8565b5060005b828110156102eb57600182600301600086868581811061029c5761029c610c03565b90506020020160208101906102b19190610c19565b6001600160a01b031681526020810191909152604001600020805460ff1916911515919091179055806102e381610c34565b91505061027a565b506000547f02eee3a5b72644922ad46f7c8834672d47b331ba43789607a0be0d2e1880dd0b9061032890600190600160a01b900461ffff16610c4f565b888860405161033993929190610c72565b60405180910390a15050505050505050565b6000546001600160a01b031633146103a55760405162461bcd60e51b815260206004820152601f60248201527f416363657373206e6f7420616c6c6f7765643a2041646d696e206f6e6c792e0060448201526064016101f5565b60005461ffff600160a01b9091048116908516106103c257600080fd5b61ffff84166000908152600160205260408120905b83811015610460578281815181106103f1576103f1610c03565b602002602001015182600301600087878581811061041157610411610c03565b90506020020160208101906104269190610c19565b6001600160a01b031681526020810191909152604001600020805460ff19169115159190911790558061045881610c34565b9150506103d7565b505050505050565b3360008181526003602090815260408083206001600160a01b03881680855290835281842087905584845260048352818420818552835290832080546001810182559084529190922060108204018054600f9092166002026101000a61ffff81810219909316928616029190911790556104e391308561054b565b505050565b6040516001600160a01b0383166024820152604481018290526104e390849063a9059cbb60e01b906064015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152610583565b6040516001600160a01b03808516602483015283166044820152606481018290526101999085906323b872dd60e01b90608401610514565b60006105d8826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166106559092919063ffffffff16565b8051909150156104e357808060200190518101906105f69190610cac565b6104e35760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b60648201526084016101f5565b6060610664848460008561066e565b90505b9392505050565b6060824710156106cf5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b60648201526084016101f5565b6001600160a01b0385163b6107265760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064016101f5565b600080866001600160a01b031685876040516107429190610cf5565b60006040518083038185875af1925050503d806000811461077f576040519150601f19603f3d011682016040523d82523d6000602084013e610784565b606091505b509150915061079482828661079f565b979650505050505050565b606083156107ae575081610667565b8251156107be5782518084602001fd5b8160405162461bcd60e51b81526004016101f59190610d11565b8280546107e490610d44565b90600052602060002090601f016020900481019282610806576000855561084c565b82601f1061081f5782800160ff1982351617855561084c565b8280016001018555821561084c579182015b8281111561084c578235825591602001919060010190610831565b5061085892915061085c565b5090565b5b80821115610858576000815560010161085d565b80356001600160a01b038116811461088857600080fd5b919050565b600080604083850312156108a057600080fd5b6108a983610871565b946020939093013593505050565b803561ffff8116811461088857600080fd5b60008083601f8401126108db57600080fd5b50813567ffffffffffffffff8111156108f357600080fd5b60208301915083602082850101111561090b57600080fd5b9250929050565b60008060006040848603121561092757600080fd5b610930846108b7565b9250602084013567ffffffffffffffff81111561094c57600080fd5b610958868287016108c9565b9497909650939450505050565b60008083601f84011261097757600080fd5b50813567ffffffffffffffff81111561098f57600080fd5b6020830191508360208260051b850101111561090b57600080fd5b60008060008060008060006080888a0312156109c557600080fd5b873560ff811681146109d657600080fd5b9650602088013567ffffffffffffffff808211156109f357600080fd5b6109ff8b838c016108c9565b909850965060408a0135915080821115610a1857600080fd5b610a248b838c016108c9565b909650945060608a0135915080821115610a3d57600080fd5b50610a4a8a828b01610965565b989b979a50959850939692959293505050565b634e487b7160e01b600052604160045260246000fd5b8015158114610a8157600080fd5b50565b803561088881610a73565b60008060008060608587031215610aa557600080fd5b610aae856108b7565b935060208086013567ffffffffffffffff80821115610acc57600080fd5b610ad889838a01610965565b90965094506040880135915080821115610af157600080fd5b818801915088601f830112610b0557600080fd5b813581811115610b1757610b17610a5d565b8060051b604051601f19603f83011681018181108582111715610b3c57610b3c610a5d565b60405291825284820192508381018501918b831115610b5a57600080fd5b938501935b82851015610b7f57610b7085610a84565b84529385019392850192610b5f565b989b979a50959850505050505050565b600080600060608486031215610ba457600080fd5b610bad84610871565b925060208401359150610bc2604085016108b7565b90509250925092565b634e487b7160e01b600052601160045260246000fd5b600061ffff80831681811415610bf957610bf9610bcb565b6001019392505050565b634e487b7160e01b600052603260045260246000fd5b600060208284031215610c2b57600080fd5b61066782610871565b6000600019821415610c4857610c48610bcb565b5060010190565b600061ffff83811690831681811015610c6a57610c6a610bcb565b039392505050565b61ffff8416815260406020820152816040820152818360608301376000818301606090810191909152601f909201601f1916010192915050565b600060208284031215610cbe57600080fd5b815161066781610a73565b60005b83811015610ce4578181015183820152602001610ccc565b838111156101995750506000910152565b60008251610d07818460208701610cc9565b9190910192915050565b6020815260008251806020840152610d30816040850160208701610cc9565b601f01601f19169190910160400192915050565b600181811c90821680610d5857607f821691505b60208210811415610d7957634e487b7160e01b600052602260045260246000fd5b5091905056fea2646970667358221220d2cb962eb21dcdce94185a8d50b37fa6b06d99d00e4d5e1359d5b63c699347c664736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    #[derive(Clone)]
    pub struct Unipeer<M>(ethers::contract::Contract<M>);
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
    impl<'a, M: ethers::providers::Middleware> Unipeer<M> {
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
        #[doc = "Gets the contract's `NewPaymentMethod` event"]
        pub fn new_payment_method_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewPaymentMethodFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, NewPaymentMethodFilter> {
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
    #[ethevent(name = "NewPaymentMethod", abi = "NewPaymentMethod(uint16,string)")]
    pub struct NewPaymentMethodFilter {
        pub payment_id: u16,
        pub payment_name: String,
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
