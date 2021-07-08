use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

use ethereum_types::H256;
use rand::Rng;

pub use ethereum_types;
pub use ethers;

pub type H248 = [u8; 31];
pub type Preimage = [u8; 62];

#[derive(Debug, Clone, Copy)]
pub struct Deposit {
    pub preimage: Preimage,
    pub commitment: H256,
    pub nullifier_hash: H256,
    pub secret: H248,
    pub nullifier: H248,
}

impl Deposit {
    fn from_preimage(preimage: Preimage) -> Deposit {
        let nullifier: H248 = preimage[..31]
            .try_into()
            .expect("31 bytes is already there.");
        let secret: H248 = preimage[31..]
            .try_into()
            .expect("31 bytes is already there.");
        let commitment =
            H256::from(pedersen::hash(&preimage).expect("32 bytes"));
        let nullifier_hash =
            H256::from(pedersen::hash(&nullifier).expect("32 bytes"));
        Deposit {
            preimage,
            commitment,
            nullifier_hash,
            secret,
            nullifier,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Note {
    pub currency: String,
    pub amount: u64,
    pub chain_id: u64,
    pub preimage: Preimage,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum NoteError {
    #[error("Invalid Note Length")]
    InvalidNoteLength,
    #[error(transparent)]
    FromHex(#[from] hex::FromHexError),
    #[error("Invalid Note Prefix expected `anchor`")]
    InvalidNotePrefix,
    #[error("Invalid Note Preimage expected 124hex/62bytes.")]
    InvalidNotePreimage,
    #[error(transparent)]
    ParseIntValue(#[from] std::num::ParseIntError),
}

impl FromStr for Note {
    type Err = NoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        // check note length.
        parts
            .len()
            .eq(&5)
            .then(Default::default)
            .ok_or(NoteError::InvalidNoteLength)?;
        // check note prefix.
        parts[0]
            .eq("anchor")
            .then(Default::default)
            .ok_or(NoteError::InvalidNotePrefix)?;
        let currency = parts[1].to_owned();
        let amount = parts[2].parse::<u64>()?;
        let chain_id = parts[3].parse::<u64>()?;
        let mut preimage: Preimage = [0; 62];
        hex::decode_to_slice(&parts[4][2..], &mut preimage)
            .map_err(|_| NoteError::InvalidNotePreimage)?;
        Ok(Self {
            currency,
            amount,
            chain_id,
            preimage,
        })
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "anchor-{}-{}-{}-0x{}",
            self.currency,
            self.amount,
            self.chain_id,
            hex::encode(&self.preimage)
        )
    }
}

impl From<Note> for Deposit {
    fn from(note: Note) -> Self {
        Self::from_preimage(note.preimage)
    }
}

impl Note {
    pub fn builder() -> NoteBuilder {
        Default::default()
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct NoteBuilder {
    currency: String,
    amount: u64,
    chain_id: u64,
}

impl NoteBuilder {
    pub fn with_currency(self, currency: impl Into<String>) -> Self {
        let mut this = self;
        this.currency = currency.into();
        this
    }

    pub fn with_amount(self, amount: impl Into<u64>) -> Self {
        let mut this = self;
        this.amount = amount.into();
        this
    }

    pub fn with_chain_id(self, chain_id: impl Into<u64>) -> Self {
        let mut this = self;
        this.chain_id = chain_id.into();
        this
    }

    pub fn build<R: Rng>(self, rng: &mut R) -> Note {
        let mut preimage: [u8; 62] = [0; 62];
        rng.fill(&mut preimage[..]);
        Note {
            preimage,
            chain_id: self.chain_id,
            amount: self.amount,
            currency: self.currency,
        }
    }
}

#[cfg(all(test, feature = "integration-tests"))]
mod tests {
    use anyhow::Context;
    use ethers::abi::Tokenizable;
    use ethers::prelude::*;
    use ethers::utils::{Ganache, GanacheInstance};
    use rand::SeedableRng;

    use std::convert::TryFrom;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::time::Duration;

    use super::*;
    use crate::contracts::anchor::AnchorContract;

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
        let hasher_factory = create_contract_factory(
            "contracts/build/Hasher.json",
            client.clone(),
        )
        .context("create hasher factory")?;
        let verifier_factory = create_contract_factory(
            "contracts/build/Verifier.json",
            client.clone(),
        )
        .context("create verifier factory")?;
        let native_anchor_factory = create_contract_factory(
            "contracts/build/NativeAnchor.json",
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
        let wallet = LocalWallet::from(key).set_chain_id(1337u64);
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
        let receipt = result.await?;
        println!("Tx: {:?}", receipt.transaction_hash);
        Ok(())
    }
}
