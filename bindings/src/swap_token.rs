pub use swaptoken_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod swaptoken_mod {
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
    #[doc = "SwapToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static SWAPTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amountIn\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountOutMin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swap\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static SWAPTOKEN_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610927806100206000396000f3fe6080604052600436106100295760003560e01c80633c50eec11461002e578063d5bcb9b514610060575b600080fd5b34801561003a57600080fd5b5061004e610049366004610650565b610075565b60405190815260200160405180910390f35b61007361006e36600461068c565b6102d9565b005b600060606001600160a01b03851673c02aaa39b223fe8d0a0e5c4f27ead9083c756cc214806100c057506001600160a01b03841673c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2145b1561015257604080516002808252606082018352909160208301908036833701905050905084816000815181106100f9576100f96106f9565b60200260200101906001600160a01b031690816001600160a01b031681525050838160018151811061012d5761012d6106f9565b60200260200101906001600160a01b031690816001600160a01b031681525050610224565b6040805160038082526080820190925290602082016060803683370190505090508481600081518110610187576101876106f9565b60200260200101906001600160a01b031690816001600160a01b03168152505073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2816001815181106101cf576101cf6106f9565b60200260200101906001600160a01b031690816001600160a01b0316815250508381600281518110610203576102036106f9565b60200260200101906001600160a01b031690816001600160a01b0316815250505b60405163d06ca61f60e01b8152600090737a250d5630b4cf539739df2c5dacb4c659f2488d9063d06ca61f906102609087908690600401610753565b600060405180830381865afa15801561027d573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526102a59190810190610774565b905080600183516102b69190610832565b815181106102c6576102c66106f9565b6020026020010151925050509392505050565b6001600160a01b03851673c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2148061032057506001600160a01b03841673c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2145b6103705760405162461bcd60e51b815260206004820152601a60248201527f4d757374206f6e65206f6620746f6b656e732069732057455448000000000000604482015260640160405180910390fd5b604080516002808252606080830184529260208301908036833701905050905085816000815181106103a4576103a46106f9565b60200260200101906001600160a01b031690816001600160a01b03168152505084816001815181106103d8576103d86106f9565b6001600160a01b039283166020918202929092010152861673c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2141561049757604051637ff36ab560e01b8152737a250d5630b4cf539739df2c5dacb4c659f2488d90637ff36ab590349061044a908790869088904290600401610857565b60006040518083038185885af1158015610468573d6000803e3d6000fd5b50505050506040513d6000823e601f3d908101601f191682016040526104919190810190610774565b5061062c565b6001600160a01b03851673c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2141561062c576040516323b872dd60e01b8152336004820152306024820152604481018590526001600160a01b038716906323b872dd906064016020604051808303816000875af115801561050f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610533919061088c565b5060405163095ea7b360e01b8152737a250d5630b4cf539739df2c5dacb4c659f2488d6004820152602481018590526001600160a01b0387169063095ea7b3906044016020604051808303816000875af1158015610595573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105b9919061088c565b506040516318cbafe560e01b8152737a250d5630b4cf539739df2c5dacb4c659f2488d906318cbafe5906105f990879087908690889042906004016108b5565b600060405180830381600087803b15801561061357600080fd5b505af1158015610627573d6000803e3d6000fd5b505050505b505050505050565b80356001600160a01b038116811461064b57600080fd5b919050565b60008060006060848603121561066557600080fd5b61066e84610634565b925061067c60208501610634565b9150604084013590509250925092565b600080600080600060a086880312156106a457600080fd5b6106ad86610634565b94506106bb60208701610634565b935060408601359250606086013591506106d760808701610634565b90509295509295909350565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b600081518084526020808501945080840160005b838110156107485781516001600160a01b031687529582019590820190600101610723565b509495945050505050565b82815260406020820152600061076c604083018461070f565b949350505050565b6000602080838503121561078757600080fd5b825167ffffffffffffffff8082111561079f57600080fd5b818501915085601f8301126107b357600080fd5b8151818111156107c5576107c56106e3565b8060051b604051601f19603f830116810181811085821117156107ea576107ea6106e3565b60405291825284820192508381018501918883111561080857600080fd5b938501935b828510156108265784518452938501939285019261080d565b98975050505050505050565b60008282101561085257634e487b7160e01b600052601160045260246000fd5b500390565b848152608060208201526000610870608083018661070f565b6001600160a01b03949094166040830152506060015292915050565b60006020828403121561089e57600080fd5b815180151581146108ae57600080fd5b9392505050565b85815284602082015260a0604082015260006108d460a083018661070f565b6001600160a01b039490941660608301525060800152939250505056fea2646970667358221220b08e4a1577df78d9143fe39a993b2f1e271244be70a0b1da4a9238c77a4c01cb64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    pub struct SwapToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for SwapToken<M> {
        fn clone(&self) -> Self {
            SwapToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for SwapToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for SwapToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(SwapToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> SwapToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), SWAPTOKEN_ABI.clone(), client).into()
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
                SWAPTOKEN_ABI.clone(),
                SWAPTOKEN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `getAmountOutMin` (0x3c50eec1) function"]
        pub fn get_amount_out_min(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([60, 80, 238, 193], (token_in, token_out, amount_in))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swap` (0xd5bcb9b5) function"]
        pub fn swap(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            amount_out_min: ethers::core::types::U256,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 188, 185, 181],
                    (token_in, token_out, amount_in, amount_out_min, to),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for SwapToken<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getAmountOutMin`function with signature `getAmountOutMin(address,address,uint256)` and selector `[60, 80, 238, 193]`"]
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
        name = "getAmountOutMin",
        abi = "getAmountOutMin(address,address,uint256)"
    )]
    pub struct GetAmountOutMinCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swap`function with signature `swap(address,address,uint256,uint256,address)` and selector `[213, 188, 185, 181]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "swap", abi = "swap(address,address,uint256,uint256,address)")]
    pub struct SwapCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub amount_out_min: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum SwapTokenCalls {
        GetAmountOutMin(GetAmountOutMinCall),
        Swap(SwapCall),
    }
    impl ethers::core::abi::AbiDecode for SwapTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetAmountOutMinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SwapTokenCalls::GetAmountOutMin(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(SwapTokenCalls::Swap(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for SwapTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                SwapTokenCalls::GetAmountOutMin(element) => element.encode(),
                SwapTokenCalls::Swap(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for SwapTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                SwapTokenCalls::GetAmountOutMin(element) => element.fmt(f),
                SwapTokenCalls::Swap(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetAmountOutMinCall> for SwapTokenCalls {
        fn from(var: GetAmountOutMinCall) -> Self {
            SwapTokenCalls::GetAmountOutMin(var)
        }
    }
    impl ::std::convert::From<SwapCall> for SwapTokenCalls {
        fn from(var: SwapCall) -> Self {
            SwapTokenCalls::Swap(var)
        }
    }
}
