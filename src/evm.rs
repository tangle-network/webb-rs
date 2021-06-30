pub use ethereum_types;
pub use ethers;

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
