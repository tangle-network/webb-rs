#[cfg(test)]
mod tests {
    use ark_ff::{BigInteger, PrimeField};
    use webb::evm::ethers::core::rand::thread_rng;

    use webb::evm::{
        contract::protocol_solidity::{
            vanchor_base::VAnchorBaseContract,
            variable_anchor_tree::VAnchorTreeContract,
        },
        ethers::{
            contract::{Contract, ContractInstance},
            signers::{LocalWallet, Signer},
        },
    };

    use crate::utils::setup_utxos;
    use crate::{
        v_bridge::{TokenConfig, VAnchorBridgeDeploymentConfig},
        LocalEvmChain,
    };

    #[tokio::test]
    async fn test_vanchor_deposit() {
        let token_config = TokenConfig::default();
        let receiver_wallet =
            LocalWallet::new(&mut thread_rng()).with_chain_id(5002u32);
        // Deploy Hermes chain.
        let hermes_chain =
            LocalEvmChain::new(5001, String::from("Hermes"), None);
        let secret_key = hermes_chain.keys()[0].clone();
        let deployer_wallet1 =
            LocalWallet::from(secret_key).with_chain_id(5001u32);

        let hermes_bridge_config = VAnchorBridgeDeploymentConfig::builder()
            .deployer(deployer_wallet1.clone())
            .token_config(token_config.clone())
            .max_edges(2)
            .build();
        let hermes_bridge = hermes_bridge_config
            .deploy_variable_anchor_bridge(&hermes_chain)
            .await
            .unwrap();

        // create vanchor
        let vanchor = VAnchorTreeContract::new(
            hermes_bridge.vanchor,
            hermes_chain.client(),
        );

        let sender = deployer_wallet1.address();
        let receiver = receiver_wallet.address();
        let typed_source_chain_id = hermes_chain.typed_chain_id();
        let types_target_chain_id = hermes_chain.typed_chain_id();
        let ext_amount = 10_i128;
        let public_amount = 10_i128;
        let fee = 0_i128;

        let input_chain_ids = [typed_source_chain_id, types_target_chain_id];
        let input_amounts = [0, 0];
        let input_indices = [0, 0];
        let output_chain_ids = [typed_source_chain_id, types_target_chain_id];
        let output_amount = [10, 0];
        let output_indices = [0, 0];

        let input_utxos =
            setup_utxos(input_chain_ids, input_amounts, Some(input_indices));
        let output_utxos =
            setup_utxos(output_chain_ids, output_amount, Some(output_indices));

        let encrypted_commitment1 =
            output_utxos[0].commitment.into_repr().to_bytes_be();
        let encrypted_commitment2 =
            output_utxos[1].commitment.into_repr().to_bytes_be();

        // Shutdown chains.
        hermes_chain.shutdown();
    }
}
