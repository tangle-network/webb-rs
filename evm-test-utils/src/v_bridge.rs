use webb::evm::{
    contract::protocol_solidity::signature_bridge,
    ethers::{
        contract::EthCall,
        signers::{LocalWallet, Signer},
        types::{Address, H256},
        utils::keccak256,
    },
};

use crate::errors::Result;
use webb_proposals::{
    FunctionSignature, Nonce, ResourceId, TargetSystem, TypedChainId,
};

#[derive(Clone, Debug, typed_builder::TypedBuilder)]
pub struct TokenConfig {
    pub name: String,
    pub symbol: String,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            name: "Tangle Token".to_string(),
            symbol: "tTNT".to_string(),
        }
    }
}

#[derive(Clone, Debug, typed_builder::TypedBuilder)]
pub struct ZkComponents {
    pub wasm: Vec<u8>,
    pub zkey: Vec<u8>,
    pub witness_calculator: Vec<u8>,
}

#[derive(Clone, Debug, typed_builder::TypedBuilder)]
pub struct VAnchorBridgeInfo {
    pub bridge: Address,
    pub vanchor: Address,
    pub vanchor_handler: Address,
    pub treasury: Address,
    pub treasury_handler: Address,
    pub fungible_token_wrapper: Address,
    pub token_wrapper_handler: Address,
}

#[derive(typed_builder::TypedBuilder)]
pub struct VAnchorBridgeDeploymentConfig {
    #[builder(default)]
    pub token_config: TokenConfig,
    pub deployer: LocalWallet,
    pub max_edges: u8,
}

const MERKLE_TREE_LEVELS: u32 = 30;

impl VAnchorBridgeDeploymentConfig {
    pub async fn deploy_variable_anchor_bridge(
        &self,
        chain: &crate::LocalEvmChain,
    ) -> Result<VAnchorBridgeInfo> {
        let chain_id = chain.chain_id();
        let typed_chain_id = TypedChainId::Evm(chain_id);
        let initial_governor = self.deployer.address();

        // Deploy signature bridge contract
        let bridge = chain
            .deploy_signature_bridge(initial_governor.to_owned(), 1)
            .await?;

        // Deploy anchor handler contract
        let anchor_handler = chain
            .deploy_anchor_handler(bridge.address(), vec![], vec![])
            .await?;

        // Deploy treasury handler and treasury contract
        let treasury_handler = chain
            .deploy_treasury_handler(bridge.address(), vec![], vec![])
            .await?;

        let treasury =
            chain.deploy_treasury(treasury_handler.address()).await?;

        // Set treasury handler with singature.
        let treasury_target_system =
            TargetSystem::ContractAddress(treasury.address().to_fixed_bytes());
        let bridge_target_system =
            TargetSystem::ContractAddress(bridge.address().to_fixed_bytes());
        let resource_id = ResourceId::new(bridge_target_system, typed_chain_id);

        let new_resource_id =
            ResourceId::new(treasury_target_system, typed_chain_id);

        let function_sig_bytes =
            signature_bridge::AdminSetResourceWithSignatureCall::selector()
                .to_vec();
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&function_sig_bytes);
        let function_sig = FunctionSignature::from(buf);
        let nonce = bridge
            .proposal_nonce()
            .await?
            .checked_add(1u64.into())
            .unwrap_or_default();
        let nonce = Nonce(nonce.as_u32());
        let mut unsigned_data = Vec::new();
        unsigned_data.extend_from_slice(&resource_id.to_bytes());
        unsigned_data.extend_from_slice(&function_sig.to_bytes());
        unsigned_data.extend_from_slice(&nonce.to_bytes());
        unsigned_data.extend_from_slice(&new_resource_id.to_bytes());
        unsigned_data
            .extend_from_slice(&treasury_handler.address().to_fixed_bytes());

        let hashed_data: H256 = keccak256(&unsigned_data).into();
        let signature = self.deployer.sign_hash(hashed_data)?;

        bridge
            .admin_set_resource_with_signature(
                resource_id.into(),
                function_sig.into(),
                nonce.into(),
                new_resource_id.into(),
                treasury_handler.address(),
                signature.to_vec().into(),
            )
            .call()
            .await?;

        let poseidon_hasher = chain.deploy_poseidon_hasher().await?;

        let verifier = chain.deploy_vanchor_verifier().await?;

        // Deploy token wrapper handler
        let token_wrapper_handler = chain
            .deploy_token_wrapper_handler(bridge.address(), vec![], vec![])
            .await?;

        // Deploy fungible token wrapper
        let fungible_token_wrapper = chain
            .deploy_fungible_token_wrapper(
                self.token_config.clone().name,
                self.token_config.clone().symbol,
                0,
                treasury.address(),
                token_wrapper_handler.address(),
                10000000000000000000000000u128.into(),
                false,
                self.deployer.address(),
            )
            .await?;

        // Set fungible token resource with signature.
        let fungible_token_target_system = TargetSystem::ContractAddress(
            fungible_token_wrapper.address().to_fixed_bytes(),
        );
        let new_resource_id =
            ResourceId::new(fungible_token_target_system, typed_chain_id);
        let bridge_nonce = bridge
            .proposal_nonce()
            .await?
            .checked_add(1u64.into())
            .unwrap_or_default();
        let nonce = Nonce(bridge_nonce.as_u32());

        let mut unsigned_data = Vec::new();
        unsigned_data.extend_from_slice(&resource_id.to_bytes());
        unsigned_data.extend_from_slice(&function_sig.to_bytes());
        unsigned_data.extend_from_slice(&nonce.to_bytes());
        unsigned_data.extend_from_slice(&new_resource_id.to_bytes());
        unsigned_data.extend_from_slice(
            &token_wrapper_handler.address().to_fixed_bytes(),
        );

        let hashed_data: H256 = keccak256(&unsigned_data).into();
        let signature = self.deployer.sign_hash(hashed_data)?;

        bridge
            .admin_set_resource_with_signature(
                resource_id.into(),
                function_sig.into(),
                nonce.into(),
                new_resource_id.into(),
                token_wrapper_handler.address(),
                signature.to_vec().into(),
            )
            .call()
            .await?;

        // Deploy vanchor tree contract.
        let vanchor = chain
            .deploy_vanchor_tree(
                verifier.address(),
                MERKLE_TREE_LEVELS,
                poseidon_hasher.address(),
                anchor_handler.address(),
                fungible_token_wrapper.address(),
                self.max_edges,
                0.into(),
                10000000000000000000000000u128.into(),
            )
            .await?;

        let role = keccak256(b"MINTER_ROLE");
        fungible_token_wrapper
            .grant_role(role, vanchor.address())
            .call()
            .await?;

        let bridge_info = VAnchorBridgeInfo::builder()
            .bridge(bridge.address())
            .vanchor(vanchor.address())
            .vanchor_handler(anchor_handler.address())
            .treasury(treasury.address())
            .treasury_handler(treasury_handler.address())
            .fungible_token_wrapper(fungible_token_wrapper.address())
            .token_wrapper_handler(token_wrapper_handler.address())
            .build();
        Ok(bridge_info)
    }
}

#[cfg(test)]
mod tests {
    use crate::LocalEvmChain;

    use super::*;
    #[tokio::test]
    async fn test_deploy_all_contracts() {
        let token_config = TokenConfig::default();
        let hermes_chain =
            LocalEvmChain::new(5001, String::from("Hermes"), None);
        let secret_key = hermes_chain.keys()[0].clone();
        let deployer_wallet1 =
            LocalWallet::from(secret_key).with_chain_id(5001u32);

        let hermes_bridge_config = VAnchorBridgeDeploymentConfig::builder()
            .deployer(deployer_wallet1)
            .token_config(token_config)
            .max_edges(2)
            .build();
        let _hermes_bridge_info = hermes_bridge_config
            .deploy_variable_anchor_bridge(&hermes_chain)
            .await
            .unwrap();

        hermes_chain.shutdown();
    }
}
