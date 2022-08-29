pub use treasury_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod treasury_contract {
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
    #[doc = "TreasuryContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TREASURYCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_treasuryHandler\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountToRescue\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rescueTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newHandler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setHandler\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]") . expect ("invalid abi")
    });
    pub struct TreasuryContract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for TreasuryContract<M> {
        fn clone(&self) -> Self {
            TreasuryContract(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for TreasuryContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for TreasuryContract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TreasuryContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> TreasuryContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                TREASURYCONTRACT_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `proposalNonce` (0xcc3c74a1) function"]
        pub fn proposal_nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([204, 60, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rescueTokens` (0x622c77d9) function"]
        pub fn rescue_tokens(
            &self,
            token_address: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount_to_rescue: ethers::core::types::U256,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [98, 44, 119, 217],
                    (token_address, to, amount_to_rescue, nonce),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHandler` (0x72c1ad03) function"]
        pub fn set_handler(
            &self,
            new_handler: ethers::core::types::Address,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 193, 173, 3], (new_handler, nonce))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for TreasuryContract<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `proposalNonce` function with signature `proposalNonce()` and selector `[204, 60, 116, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "proposalNonce", abi = "proposalNonce()")]
    pub struct ProposalNonceCall;
    #[doc = "Container type for all input parameters for the `rescueTokens` function with signature `rescueTokens(address,address,uint256,uint32)` and selector `[98, 44, 119, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "rescueTokens",
        abi = "rescueTokens(address,address,uint256,uint32)"
    )]
    pub struct RescueTokensCall {
        pub token_address: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount_to_rescue: ethers::core::types::U256,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `setHandler` function with signature `setHandler(address,uint32)` and selector `[114, 193, 173, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "setHandler", abi = "setHandler(address,uint32)")]
    pub struct SetHandlerCall {
        pub new_handler: ethers::core::types::Address,
        pub nonce: u32,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum TreasuryContractCalls {
        ProposalNonce(ProposalNonceCall),
        RescueTokens(RescueTokensCall),
        SetHandler(SetHandlerCall),
    }
    impl ethers::core::abi::AbiDecode for TreasuryContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TreasuryContractCalls::ProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <RescueTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TreasuryContractCalls::RescueTokens(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TreasuryContractCalls::SetHandler(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TreasuryContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                TreasuryContractCalls::ProposalNonce(element) => {
                    element.encode()
                }
                TreasuryContractCalls::RescueTokens(element) => {
                    element.encode()
                }
                TreasuryContractCalls::SetHandler(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TreasuryContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TreasuryContractCalls::ProposalNonce(element) => element.fmt(f),
                TreasuryContractCalls::RescueTokens(element) => element.fmt(f),
                TreasuryContractCalls::SetHandler(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ProposalNonceCall> for TreasuryContractCalls {
        fn from(var: ProposalNonceCall) -> Self {
            TreasuryContractCalls::ProposalNonce(var)
        }
    }
    impl ::std::convert::From<RescueTokensCall> for TreasuryContractCalls {
        fn from(var: RescueTokensCall) -> Self {
            TreasuryContractCalls::RescueTokens(var)
        }
    }
    impl ::std::convert::From<SetHandlerCall> for TreasuryContractCalls {
        fn from(var: SetHandlerCall) -> Self {
            TreasuryContractCalls::SetHandler(var)
        }
    }
    #[doc = "Container type for all return fields from the `proposalNonce` function with signature `proposalNonce()` and selector `[204, 60, 116, 161]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct ProposalNonceReturn(pub ethers::core::types::U256);
}
