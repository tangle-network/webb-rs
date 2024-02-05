use std::collections::HashMap;

use webb::evm::{
    contract::protocol_solidity::{
        fungible_token_wrapper::FungibleTokenWrapperContract, signature_bridge,
    },
    ethers::{
        contract::EthCall,
        signers::{LocalWallet, Signer},
        types::Address,
    },
};

use crate::errors::{self, Result};
use webb_proposals::{
    evm::SetTreasuryHandlerProposal, FunctionSignature, Nonce, ProposalHeader,
    ProposalTrait, ResourceId, TargetSystem, TypedChainId,
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

#[derive(typed_builder::TypedBuilder)]
pub struct VAnchorBridgeDeployment {
    pub chains: Vec<crate::LocalEvmChain>,
    #[builder(default)]
    pub token_configs: HashMap<TypedChainId, TokenConfig>,
    pub vanchor_inputs: HashMap<TypedChainId, Address>,
    pub deployers: HashMap<TypedChainId, LocalWallet>,
    pub initial_governors: HashMap<TypedChainId, Address>,
}

impl VAnchorBridgeDeployment {
    pub async fn deploy_variable_anchor_bridge(&self) -> Result<()> {
        for chain in &self.chains {
            let chain_id = chain.chain_id();
            let typed_chain_id = TypedChainId::Evm(chain_id);

            let deployer = self
                .deployers
                .get(&typed_chain_id)
                .ok_or_else(|| errors::Error::NoDeployer { chain_id })?;

            let initial_governor = self
                .initial_governors
                .get(&typed_chain_id)
                .ok_or_else(|| errors::Error::NoInitialGovernor { chain_id })?;

            let token_config: TokenConfig = self
                .token_configs
                .get(&typed_chain_id)
                .map(|config| config.clone())
                .unwrap_or_else(TokenConfig::default)
                .into();

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

            // Step5 Set treasury handler with singature.
            let bridge_target_system = TargetSystem::ContractAddress(
                bridge.address().to_fixed_bytes(),
            );
            let resource_id =
                ResourceId::new(bridge_target_system, typed_chain_id);
            let function_sig_bytes =
                signature_bridge::ExecuteProposalWithSignatureCall::selector()
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
            let header = ProposalHeader::new(resource_id, function_sig, nonce);
            let proposal = SetTreasuryHandlerProposal::new(
                header,
                treasury_handler.address().into(),
            );

            let signature = deployer.sign_message(proposal.to_vec()).await?;

            bridge
                .execute_proposal_with_signature(
                    proposal.to_vec().into(),
                    signature.to_vec().into(),
                )
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
                    token_config.name,
                    token_config.symbol,
                )
                .await?;
        }

        Ok(())
    }
}
