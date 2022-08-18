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
        let proof = provider
            .get_proof(
                NameOrAddress::Address(address_bytes.into()),
                vec![],
                None,
            )
            .await
            .unwrap();
        println!("Got proof: {}", serde_json::to_string(&proof).unwrap());
    }
}
