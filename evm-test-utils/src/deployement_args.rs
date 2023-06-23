use std::collections::HashMap;

use webb::evm::{
    contract::protocol_solidity::{
        erc20_preset_minter_pauser::ERC20PresetMinterPauserContract,
        fungible_token_wrapper::FungibleTokenWrapperContract,
    },
    ethers::{signers::LocalWallet, types::Address},
};
use webb_proposals::TypedChainId;

#[derive(Clone, Debug, typed_builder::TypedBuilder)]
pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
}

#[derive(Clone, Debug, typed_builder::TypedBuilder)]
pub struct ZkComponents {
    pub wasm: Vec<u8>,
    pub zkey: Vec<u8>,
    pub witness_calculator: Vec<u8>,
}

#[derive(typed_builder::TypedBuilder)]
pub struct VAnchorBridgeDeploymentArgs<M> {
    pub chain_ids: Vec<TypedChainId>,
    #[builder(default)]
    pub token_configs: HashMap<TypedChainId, TokenConfig>,
    #[builder(default)]
    pub webb_tokens: HashMap<TypedChainId, FungibleTokenWrapperContract<M>>,
    pub vanchor_inputs: HashMap<TypedChainId, Address>,
    pub deployers: HashMap<TypedChainId, LocalWallet>,
    pub initial_governors: HashMap<TypedChainId, Address>,
}

#[derive(typed_builder::TypedBuilder)]
pub struct VBridgeDeploymentArgs<M> {
    pub chains: Vec<crate::LocalEvmChain>,
    pub tokens: Vec<ERC20PresetMinterPauserContract<M>>,
    pub deployers: Vec<LocalWallet>,
}
