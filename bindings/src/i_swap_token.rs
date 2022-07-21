pub use iswaptoken_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod iswaptoken_mod {
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
    #[doc = "ISwapToken was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ISWAPTOKEN_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amountIn\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountOutMin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenIn\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_tokenOut\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amountOutMin\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swap\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct ISwapToken<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ISwapToken<M> {
        fn clone(&self) -> Self {
            ISwapToken(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ISwapToken<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ISwapToken<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ISwapToken))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ISwapToken<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ISWAPTOKEN_ABI.clone(), client).into()
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ISwapToken<M> {
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
    pub enum ISwapTokenCalls {
        GetAmountOutMin(GetAmountOutMinCall),
        Swap(SwapCall),
    }
    impl ethers::core::abi::AbiDecode for ISwapTokenCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetAmountOutMinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ISwapTokenCalls::GetAmountOutMin(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ISwapTokenCalls::Swap(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ISwapTokenCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ISwapTokenCalls::GetAmountOutMin(element) => element.encode(),
                ISwapTokenCalls::Swap(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ISwapTokenCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ISwapTokenCalls::GetAmountOutMin(element) => element.fmt(f),
                ISwapTokenCalls::Swap(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetAmountOutMinCall> for ISwapTokenCalls {
        fn from(var: GetAmountOutMinCall) -> Self {
            ISwapTokenCalls::GetAmountOutMin(var)
        }
    }
    impl ::std::convert::From<SwapCall> for ISwapTokenCalls {
        fn from(var: SwapCall) -> Self {
            ISwapTokenCalls::Swap(var)
        }
    }
}
