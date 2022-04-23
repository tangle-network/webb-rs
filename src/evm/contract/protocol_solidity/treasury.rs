pub use treasurycontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod treasurycontract_mod {
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
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_treasuryHandler\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"proposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"address payable\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountToRescue\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"name\":\"rescueTokens\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newHandler\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\"}],\"name\":\"setHandler\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"stateMutability\":\"payable\",\"type\":\"receive\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct TreasuryContract<M>(ethers::contract::Contract<M>);
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
    impl<'a, M: ethers::providers::Middleware> TreasuryContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                TREASURYCONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
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
        #[doc = "Calls the contract's `rescueTokens` (0x17a8a975) function"]
        pub fn rescue_tokens(
            &self,
            token_address: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount_to_rescue: ethers::core::types::U256,
            nonce: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [23, 168, 169, 117],
                    (token_address, to, amount_to_rescue, nonce),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHandler` (0xe29b2c6a) function"]
        pub fn set_handler(
            &self,
            new_handler: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 155, 44, 106], (new_handler, nonce))
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "Container type for all input parameters for the `proposalNonce`function with signature `proposalNonce()` and selector `[204, 60, 116, 161]`"]
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
    #[doc = "Container type for all input parameters for the `rescueTokens`function with signature `rescueTokens(address,address,uint256,uint256)` and selector `[23, 168, 169, 117]`"]
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
        abi = "rescueTokens(address,address,uint256,uint256)"
    )]
    pub struct RescueTokensCall {
        pub token_address: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount_to_rescue: ethers::core::types::U256,
        pub nonce: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setHandler`function with signature `setHandler(address,uint256)` and selector `[226, 155, 44, 106]`"]
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
    #[ethcall(name = "setHandler", abi = "setHandler(address,uint256)")]
    pub struct SetHandlerCall {
        pub new_handler: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
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
}
