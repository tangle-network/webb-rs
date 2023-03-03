mod evm;

#[cfg(test)]
#[ignore("requires RPC endpoint")]
mod tests {
    use crate::evm::verify_trie_proof::*;
    use ethereum_types::H256 as EthTypeH256;
    use ethers::prelude::*;

    #[tokio::test]
    async fn get_proof_works() {
        let provider = Provider::<Http>::try_from("...").unwrap();

        let address_vec = hex::decode(
            "6fC21092DA55B392b045eD78F4732bff3C580e2c".to_lowercase(),
        )
        .unwrap();
        let mut address_bytes = [0u8; 20];
        address_bytes.copy_from_slice(&address_vec);

        let slot = hex::decode(
            "0000000000000000000000000000000000000000000000000000000000000000",
        );
        let mut slot_bytes = [0u8; 32];
        slot_bytes.copy_from_slice(&slot.unwrap());

        let latest_block_number = provider.get_block_number().await.unwrap();

        let latest_block_hash = provider
            .get_block(latest_block_number)
            .await
            .unwrap()
            .unwrap()
            .hash
            .unwrap();

        let proof = provider
            .get_proof(
                NameOrAddress::Address(address_bytes.into()),
                vec![H256(slot_bytes.into())],
                Some(BlockId::Hash(latest_block_hash)),
            )
            .await
            .unwrap();
        println!("Got proof: {}", serde_json::to_string(&proof).unwrap());

        let storage_hash = (&proof).storage_hash;

        let first_storage_proof = &(&proof).storage_proof[0].proof;
        let first_storage_key = &(&proof).storage_proof[0].key;

        TrieProver::verify_trie_proof(
            EthTypeH256::from(storage_hash.as_fixed_bytes()),
            TrieProver::keccak_256(first_storage_key.clone().as_bytes())
                .to_vec(),
            first_storage_proof
                .iter()
                .map(|x| x.to_vec())
                .collect::<Vec<Vec<u8>>>(),
        );
    }
}
