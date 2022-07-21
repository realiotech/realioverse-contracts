pub use ilandbank_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod ilandbank_mod {
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
    #[doc = "ILandBank was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ILANDBANK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_buyer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"buyLandFromBank\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_seller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sellLandToBank\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_beneficiary\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct ILandBank<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ILandBank<M> {
        fn clone(&self) -> Self {
            ILandBank(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ILandBank<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ILandBank<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ILandBank))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ILandBank<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ILANDBANK_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `buyLandFromBank` (0x036b68b7) function"]
        pub fn buy_land_from_bank(
            &self,
            buyer: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 107, 104, 183], (buyer, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sellLandToBank` (0xd33f1b1d) function"]
        pub fn sell_land_to_bank(
            &self,
            seller: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 63, 27, 29], (seller, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xf3fef3a3) function"]
        pub fn withdraw(
            &self,
            beneficiary: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 254, 243, 163], (beneficiary, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ILandBank<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `buyLandFromBank`function with signature `buyLandFromBank(address,uint256)` and selector `[3, 107, 104, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "buyLandFromBank", abi = "buyLandFromBank(address,uint256)")]
    pub struct BuyLandFromBankCall {
        pub buyer: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `sellLandToBank`function with signature `sellLandToBank(address,uint256)` and selector `[211, 63, 27, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "sellLandToBank", abi = "sellLandToBank(address,uint256)")]
    pub struct SellLandToBankCall {
        pub seller: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(address,uint256)` and selector `[243, 254, 243, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(address,uint256)")]
    pub struct WithdrawCall {
        pub beneficiary: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ILandBankCalls {
        BuyLandFromBank(BuyLandFromBankCall),
        SellLandToBank(SellLandToBankCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for ILandBankCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BuyLandFromBankCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILandBankCalls::BuyLandFromBank(decoded));
            }
            if let Ok(decoded) =
                <SellLandToBankCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILandBankCalls::SellLandToBank(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ILandBankCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ILandBankCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ILandBankCalls::BuyLandFromBank(element) => element.encode(),
                ILandBankCalls::SellLandToBank(element) => element.encode(),
                ILandBankCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ILandBankCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ILandBankCalls::BuyLandFromBank(element) => element.fmt(f),
                ILandBankCalls::SellLandToBank(element) => element.fmt(f),
                ILandBankCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BuyLandFromBankCall> for ILandBankCalls {
        fn from(var: BuyLandFromBankCall) -> Self {
            ILandBankCalls::BuyLandFromBank(var)
        }
    }
    impl ::std::convert::From<SellLandToBankCall> for ILandBankCalls {
        fn from(var: SellLandToBankCall) -> Self {
            ILandBankCalls::SellLandToBank(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for ILandBankCalls {
        fn from(var: WithdrawCall) -> Self {
            ILandBankCalls::Withdraw(var)
        }
    }
}
