use std::collections::HashMap;

use webb::evm::{
    contract::protocol_solidity::FungibleTokenWrapperContract,
    ethers::{signers::LocalWallet, types::Address},
};
use webb_proposals::TypedChainId;

pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
}

pub struct ZkComponents {
    pub wasm: Vec<u8>,
    pub zkey: Vec<u8>,
    pub witness_calculator: Vec<u8>,
}

pub struct VAnchorBridgeDeploymentArgs<M> {
    chain_ids: Vec<TypedChainId>,
    token_configs: HashMap<TypedChainId, TokenConfig>,
    webb_tokens: HashMap<TypedChainId, FungibleTokenWrapperContract<M>>,
    vanchor_inputs: HashMap<TypedChainId, Vec<Address>>,
    deployers: HashMap<TypedChainId, LocalWallet>,
    initial_governors: HashMap<TypedChainId, Address>,
    small_circuit_components: ZkComponents,
    large_circuit_components: ZkComponents,
}
