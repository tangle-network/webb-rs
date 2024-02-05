use std::collections::HashMap;

use webb::evm::{
    contract::protocol_solidity::{
        erc20_preset_minter_pauser::ERC20PresetMinterPauserContract,
        fungible_token_wrapper::FungibleTokenWrapperContract, signature_bridge,
    },
    ethers::{contract::EthCall, signers::LocalWallet, types::Address},
};

use crate::errors::{self, Result};
use webb_proposals::{
    FunctionSignature, ResourceId, TargetSystem, TypedChainId,
};

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
pub struct VAnchorBridgeDeployment<M> {
    pub chains: Vec<crate::LocalEvmChain>,
    #[builder(default)]
    pub token_configs: HashMap<TypedChainId, TokenConfig>,
    #[builder(default)]
    pub webb_tokens: HashMap<TypedChainId, FungibleTokenWrapperContract<M>>,
    pub vanchor_inputs: HashMap<TypedChainId, Address>,
    pub deployers: HashMap<TypedChainId, LocalWallet>,
    pub initial_governors: HashMap<TypedChainId, Address>,
}

impl<M> VAnchorBridgeDeployment<M> {
    pub async fn deploy_variable_anchor_bridge(&self) -> Result<()> {
        for chain in &self.chains {
            let chain_id = chain.chain_id();
            let typed_chain_id = TypedChainId::Evm(chain_id);

            let initial_governor = self
                .initial_governors
                .get(&typed_chain_id)
                .ok_or_else(|| errors::Error::NoInitialGovernor { chain_id })?;

            // Step1 Deploy signature bridge contract
            let bridge = chain
                .deploy_signature_bridge(initial_governor.to_owned(), 1)
                .await?;

            // Step2 Deploy anchor handler contract
            let anchor_handler = chain
                .deploy_anchor_handler(bridge.address(), vec![], vec![])
                .await?;

            // Step3 Deploy treasury handler contract
            let treasury_handler = chain
                .deploy_treasury_handler(bridge.address(), vec![], vec![])
                .await?;

            // Step4 Deploy treasury contract
            let treasury =
                chain.deploy_treasury(treasury_handler.address()).await?;

            // Set treasury handler
            let target_system = TargetSystem::ContractAddress(
                treasury_handler.address().to_fixed_bytes(),
            );
            let resource = ResourceId::new(target_system, typed_chain_id);
            let function_signature_bytes =
                signature_bridge::AdminSetResourceWithSignatureCall::selector()
                    .to_vec();
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&function_signature_bytes);
            let function_signature = FunctionSignature::from(buf);
            bridge.admin_set_resource_with_signature(
                resource_id,
                function_sig,
                nonce,
                new_resource_id,
                handler_address,
                sig,
            )
        }

        Ok(())
    }
}
