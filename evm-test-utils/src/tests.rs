#[cfg(test)]
mod tests {
    use ark_ff::{BigInteger, PrimeField};
    use webb::evm::ethers::core::rand::thread_rng;

    use webb::evm::ethers::types::U256;
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

    use crate::types::ExtData;
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

        let relayer_Wallet =
            LocalWallet::new(&mut thread_rng()).with_chain_id(5001u32);

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
        let recipient = receiver_wallet.address();
        let relayer = relayer_Wallet.address();
        let typed_source_chain_id = hermes_chain.typed_chain_id();
        let types_target_chain_id = hermes_chain.typed_chain_id();
        let ext_amount = 10_i128;
        let public_amount: U256 = 10_u128.into();
        let fee: U256 = 0_u128.into();
        let refund: U256 = 0_u128.into();
        let token = hermes_bridge.fungible_token_wrapper;

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

        let encrypted_output1 =
            output_utxos[0].commitment.into_repr().to_bytes_be();
        let encrypted_output2 =
            output_utxos[1].commitment.into_repr().to_bytes_be();

        let ext_data = ExtData::builder()
            .recipient(recipient)
            .relayer(relayer)
            .ext_amount(ext_amount)
            .fee(fee)
            .refund(refund)
            .token(token)
            .encrypted_output1(encrypted_output1)
            .encrypted_output2(encrypted_output2)
            .build();

        let ext_data_hash = ext_data.hash();

        // Shutdown chains.
        hermes_chain.shutdown();
    }
}
