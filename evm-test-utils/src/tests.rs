#[cfg(test)]
mod tests {
    use std::fs::File;

    use ark_circom::read_zkey;
    use ark_ff::{BigInteger, PrimeField};
    use circom_proving::circom_from_folder;
    use webb::evm::contract::protocol_solidity::variable_anchor_tree::{
        CommonExtData, Encryptions, PublicInputs,
    };
    use webb::evm::ethers::contract::ContractError;
    use webb::evm::ethers::core::rand::thread_rng;

    use crate::errors;
    use crate::types::ExtData;
    use crate::utils::{
        deconstruct_public_inputs_el, setup_utxos, setup_vanchor_circuit,
    };
    use crate::{
        v_bridge::{TokenConfig, VAnchorBridgeDeploymentConfig},
        LocalEvmChain,
    };
    use circom_proving::types::Proof as SolidityProof;
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

    #[tokio::test]
    async fn test_vanchor_deposit() {
        let path_2_2 = "/Users/salman01zz/Webb-Tools/webb-rs/solidity-fixtures/vanchor_2/2/circuit_final.zkey";
        let mut file_2_2 = File::open(path_2_2).unwrap();
        let params_2_2 = read_zkey(&mut file_2_2).unwrap();

        let wasm_2_2_path =
            "/Users/salman01zz/Webb-Tools/webb-rs/solidity-fixtures/vanchor_2/2/poseidon_vanchor_2_2.wasm";

        let wc_2_2 = circom_from_folder(wasm_2_2_path);

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
        let public_amount = 10_i128;
        let fee = 0_i128;
        let refund = 0_i128.into();
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

        let leaf0 = input_utxos[0].commitment.into_repr().to_bytes_be();
        let leaf1 = input_utxos[1].commitment.into_repr().to_bytes_be();

        let leaves: Vec<Vec<u8>> = vec![leaf0, leaf1];

        let ext_data = ExtData::builder()
            .recipient(recipient)
            .relayer(relayer)
            .ext_amount(ext_amount)
            .fee(fee.into())
            .refund(refund)
            .token(token)
            .encrypted_output1(encrypted_output1.clone())
            .encrypted_output2(encrypted_output2.clone())
            .build();

        let ext_data_hash = ext_data.hash();
        let root = vanchor.get_last_root().call().await.unwrap();
        let neighbor_roots =
            vanchor.get_latest_neighbor_roots().call().await.unwrap();

        let (proof, public_inputs) = setup_vanchor_circuit(
            public_amount,
            typed_source_chain_id,
            ext_data_hash.0.to_vec(),
            input_utxos,
            output_utxos.clone(),
            root,
            [neighbor_roots[0]],
            leaves,
            &params_2_2,
            wc_2_2,
        );

        let solidity_proof = SolidityProof::try_from(proof).unwrap();
        let proof_bytes = solidity_proof.encode().unwrap();

        let common_ext_data = CommonExtData {
            recipient,
            ext_amount: ext_data.ext_amount.into(),
            relayer,
            fee: ext_data.fee.into(),
            refund,
            token,
        };

        // Deconstructing public inputs
        let (
            _chain_id,
            public_amount,
            root_set,
            nullifiers,
            commitments,
            ext_data_hash,
        ) = deconstruct_public_inputs_el(&public_inputs);

        let flattened_root: Vec<u8> = root_set
            .iter()
            .flat_map(|x| {
                let mut be_bytes = [0u8; 32];
                x.to_big_endian(&mut be_bytes);
                be_bytes
            })
            .collect();
        let public_inputs = PublicInputs {
            roots: flattened_root.into(),
            extension_roots: b"0x".to_vec().into(),
            input_nullifiers: nullifiers,
            output_commitments: commitments,
            public_amount,
            ext_data_hash,
        };

        let encryptions = Encryptions {
            encrypted_output_1: encrypted_output1.into(),
            encrypted_output_2: encrypted_output2.into(),
        };

        let maybe_result = vanchor
            .transact(
                proof_bytes.into(),
                [0u8; 32].into(),
                common_ext_data,
                public_inputs,
                encryptions,
            )
            .call()
            .await;

        match maybe_result {
            Ok(result) => {
                println!("Transaction successful: {:?}", result);
            }
            Err(err) => {
                let desc = err
                    .decode_revert::<String>()
                    .unwrap_or_else(|| format!("{:?}", err));

                println!("Transaction failed: {:?}", desc);
            }
        }

        // Shutdown chains.
        hermes_chain.shutdown();
    }
}
