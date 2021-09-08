pub use ethereum_types;
pub use ethers;

pub mod contract;
pub mod merkle_tree;
pub mod note;

#[cfg(all(test, feature = "integration-tests"))]
mod tests {
    use std::convert::TryFrom;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::time::Duration;

    use anyhow::Context;
    use ethers::abi::Tokenizable;
    use ethers::prelude::*;
    use ethers::utils::{Ganache, GanacheInstance};
    use rand::SeedableRng;

    use super::contract::tornado::anchor::AnchorContract;
    use super::note::{Deposit, Note};

    fn create_contract_factory<P: Into<PathBuf>, M: Middleware>(
        path: P,
        client: Arc<M>,
    ) -> anyhow::Result<ContractFactory<M>> {
        let json_file = fs::read_to_string(path.into())
            .context("reading contract json file")?;
        let raw: serde_json::Value =
            serde_json::from_str(&json_file).context("parsing json file")?;
        let abi = serde_json::from_value(raw["abi"].clone())?;
        let bytecode_hex = raw["bytecode"].as_str().expect("bytecode");
        let bytecode = hex::decode(&bytecode_hex[2..])
            .context("decoding bytecode from hex to bytes")?;
        Ok(ContractFactory::new(abi, bytecode.into(), client))
    }
    async fn deploy_anchor_contract<M: Middleware + 'static>(
        client: Arc<M>,
    ) -> anyhow::Result<Address> {
        let hasher_factory =
            create_contract_factory("contracts/tornado/Hasher.json", client.clone())
                .context("create hasher factory")?;
        let verifier_factory =
            create_contract_factory("contracts/tornado/Verifier.json", client.clone())
                .context("create verifier factory")?;
        let native_anchor_factory = create_contract_factory(
            "contracts/tornado/NativeAnchor.json",
            client.clone(),
        )
        .context("create native anchor factory")?;
        let hasher_instance = hasher_factory
            .deploy(())
            .context("deploy hasher")?
            .send()
            .await?;
        let verifier_instance = verifier_factory
            .deploy(())
            .context("deploy verifier")?
            .send()
            .await?;

        let verifier_address = verifier_instance.address().into_token();
        let hasher_address = hasher_instance.address().into_token();
        let denomination = ethers::utils::parse_ether("1")?.into_token();
        let merkle_tree_hight = 20u32.into_token();
        let args = (
            verifier_address,
            hasher_address,
            denomination,
            merkle_tree_hight,
        );
        let native_anchor_instance = native_anchor_factory
            .deploy(args)
            .context("deploy native anchor")?
            .send()
            .await?;
        Ok(native_anchor_instance.address())
    }

    async fn launch_ganache() -> GanacheInstance {
        tokio::task::spawn_blocking(|| Ganache::new().spawn())
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn contract_deposit() -> anyhow::Result<()> {
        let ganache = launch_ganache().await;
        let provider = Provider::<Http>::try_from(ganache.endpoint())?
            .interval(Duration::from_millis(10u64));
        let key = ganache.keys().first().cloned().unwrap();
        let wallet = LocalWallet::from(key);
        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);
        let anchor_contract_address =
            deploy_anchor_contract(client.clone()).await?;
        let contract =
            AnchorContract::new(anchor_contract_address, client.clone());
        let mut rng = rand::rngs::StdRng::from_seed([0u8; 32]);
        let note = Note::builder()
            .with_chain_id(1337u64)
            .with_amount(1u64)
            .with_currency("ETH")
            .build(&mut rng);
        println!("Note: {}", note);
        let deposit: Deposit = note.clone().into();
        let tx = contract
            .deposit(deposit.commitment.into())
            .value(ethers::utils::parse_ether(note.amount)?);
        let result = tx.send().await?;
        let receipt = result.await?.unwrap();
        println!("Tx: {:?}", receipt.transaction_hash);
        Ok(())
    }
}
