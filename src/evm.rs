use std::convert::TryInto;

use ethereum_types::H256;
use rand::Rng;

pub use ethereum_types;
pub use ethers;

pub type H248 = [u8; 31];

#[derive(Debug, Clone, Copy)]
pub struct Deposit {
    pub preimage: [u8; 62],
    pub commitment: H256,
    pub nullifier_hash: H256,
    pub secret: H248,
    pub nullifier: H248,
}

impl Deposit {
    pub fn generate<R: Rng>(rng: &mut R) -> Deposit {
        let mut preimage: [u8; 62] = [0; 62];
        rng.fill(&mut preimage[..]);
        let nullifier: H248 = preimage[0..31]
            .try_into()
            .expect("31 bytes is already there.");
        let secret: H248 = preimage[31..62]
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

#[cfg(all(test, feature = "integration-tests"))]
mod tests {
    use ethers::prelude::*;
    use std::convert::TryFrom;
    use std::str::FromStr;
    use std::sync::Arc;
    use std::time::Duration;

    use super::*;
    use crate::contracts::anchor::AnchorContract;

    type DynError = Box<dyn std::error::Error + Sync + Send + 'static>;

    struct BeresheetNetwork;
    impl BeresheetNetwork {
        fn endpoint() -> &'static str {
            "http://beresheet3.edgewa.re:9933"
        }

        fn contract_10_tedg_address() -> Address {
            Address::from_str("0x5f771fc87F87DB48C9fB11aA228D833226580689")
                .unwrap()
        }
    }

    #[tokio::test]
    async fn contract_deposit() -> Result<(), DynError> {
        let provider =
            Provider::<Http>::try_from(BeresheetNetwork::endpoint())?
                .interval(Duration::from_millis(10u64));
        // TODO(shekohex): handle wallets.
        let client = Arc::new(provider);
        let contract = AnchorContract::new(
            BeresheetNetwork::contract_10_tedg_address(),
            client.clone(),
        );
        // create a deposit call with 10tEDG
        let deposit = contract.deposit([0; 32]).value(10);
        let result = deposit.send().await?;
        let inblock = dbg!(result).await?;
        dbg!(inblock);
        Ok(())
    }
}
