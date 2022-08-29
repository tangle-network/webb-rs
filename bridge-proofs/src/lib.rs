mod evm;

#[cfg(test)]
mod tests {
    use ethers::prelude::*;

    #[tokio::test]
    async fn get_proof_works() {
        let provider =
            Provider::<Http>::try_from("http://127.0.0.1:5001").unwrap();

        let block = provider.get_block(1u64).await.unwrap();
        println!("Got block: {}", serde_json::to_string(&block).unwrap());

        let address_vec =
            hex::decode("bfce6B877Ebff977bB6e80B24FbBb7bC4eBcA4df").unwrap();
        let mut address_bytes = [0u8; 20];
        address_bytes.copy_from_slice(&address_vec);
        println!("Got address: {:?}, {:?}", address_bytes, address_vec.len());
        let slot = hex::decode(
            "ac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b",
        )
        .unwrap();

        let block_number = provider.get_block_number().await.unwrap();

        let block = provider.get_block(block_number).await.unwrap();
        println!("Got block: {}", serde_json::to_string(&block).unwrap());

        let mut slot_bytes = [0u8; 32];
        slot_bytes.copy_from_slice(&slot);
        let proof = provider
            .get_proof(
                NameOrAddress::Address(address_bytes.into()),
                vec![H256(slot_bytes.into())],
                None,
            )
            .await
            .unwrap();
        println!("Got proof: {}", serde_json::to_string(&proof).unwrap());
    }
}
