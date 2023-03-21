use std::sync::Arc;

use futures::prelude::*;
use webb::evm::contract::protocol_solidity::*;
use webb::evm::ethers;
use webb_proposals::TypedChainId;

use crate::deployement_args::VAnchorBridgeDeploymentArgs;
use crate::errors::Result;

type EthersClient = ethers::providers::Provider<ethers::providers::Http>;
type SignerEthersClient = ethers::middleware::SignerMiddleware<
    EthersClient,
    ethers::signers::LocalWallet,
>;

pub struct LocalEvmChain {
    typed_chain_id: TypedChainId,
    name: String,
    client: Arc<SignerEthersClient>,
    anvil_node_handle: anvil::NodeHandle,
    anvil_eth_api: anvil::eth::EthApi,
    account_generator: anvil::AccountGenerator,
}

impl LocalEvmChain {
    pub async fn new(chain_id: u32, name: String) -> Self {
        let typed_chain_id = TypedChainId::Evm(chain_id);
        let (anvil_eth_api, anvil_node_handle, account_generator) =
            Self::spawn_anvil_node(typed_chain_id, None).await;
        let signer =
            anvil_node_handle.dev_wallets().next().expect("dev wallet");
        let client =
            SignerEthersClient::new(anvil_node_handle.http_provider(), signer);
        // start the node
        Self {
            typed_chain_id,
            name,
            client: Arc::new(client),
            anvil_node_handle,
            anvil_eth_api,
            account_generator,
        }
    }

    pub async fn new_with_chain_state(
        chain_id: u32,
        name: String,
        state: anvil::eth::backend::db::SerializableState,
    ) -> Self {
        let typed_chain_id = TypedChainId::Evm(chain_id);
        let (anvil_eth_api, anvil_node_handle, account_generator) =
            Self::spawn_anvil_node(typed_chain_id, Some(state)).await;
        let signer =
            anvil_node_handle.dev_wallets().next().expect("dev wallet");
        let client =
            SignerEthersClient::new(anvil_node_handle.http_provider(), signer);
        Self {
            typed_chain_id,
            name,
            client: Arc::new(client),
            anvil_node_handle,
            anvil_eth_api,
            account_generator,
        }
    }

    pub fn chain_id(&self) -> TypedChainId {
        self.typed_chain_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn anvil_eth_api(&self) -> anvil::eth::EthApi {
        self.anvil_eth_api.clone()
    }

    pub fn account_generator(&self) -> anvil::AccountGenerator {
        self.account_generator.clone()
    }

    pub fn client(&self) -> Arc<SignerEthersClient> {
        self.client.clone()
    }

    pub async fn node_state(
        &self,
    ) -> anvil::eth::backend::db::SerializableState {
        self.anvil_eth_api
            .serialized_state()
            .await
            .expect("valid state")
    }

    pub fn shutdown(mut self) {
        let maybe_signal = self.anvil_node_handle.shutdown_signal_mut().take();
        match maybe_signal {
            Some(signal) => {
                signal.fire().expect("signal fired");
            }
            None => {
                // no signal, maybe the node was already shut down
            }
        }
    }

    pub async fn deploy_token(
        &self,
        name: String,
        symbol: String,
    ) -> Result<ERC20PresetMinterPauserContract<SignerEthersClient>> {
        ERC20PresetMinterPauserContract::deploy(
            self.client.clone(),
            (name, symbol),
        )?
        .confirmations(0usize)
        .send()
        .map_err(Into::into)
        .await
    }

    pub async fn deploy_variable_anchor_bridge(
        &self,
        args: VAnchorBridgeDeploymentArgs<SignerEthersClient>,
    ) -> SignatureBridgeContract<SignerEthersClient> {
        todo!()
    }

    async fn spawn_anvil_node(
        typed_chain_id: TypedChainId,
        state: Option<anvil::eth::backend::db::SerializableState>,
    ) -> (
        anvil::eth::EthApi,
        anvil::NodeHandle,
        anvil::AccountGenerator,
    ) {
        let generator = anvil::AccountGenerator::new(20)
            .chain_id(typed_chain_id.underlying_chain_id());
        let config = anvil::NodeConfig::default()
            .with_tracing(false)
            .silent()
            .with_chain_id(Some(typed_chain_id.underlying_chain_id()))
            .with_port(crate::random_port::random_port())
            .with_account_generator(generator.clone())
            .with_init_state(state)
            .with_genesis_balance(
                ethers::utils::parse_ether(1000).expect("valid ether"),
            );
        let (anvil_eth_api, anvil_node_handle) = anvil::spawn(config).await;
        (anvil_eth_api, anvil_node_handle, generator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_be_able_to_deploy_token() -> Result<()> {
        let chain = LocalEvmChain::new(5001, String::from("Hermes")).await;
        let token = chain
            .deploy_token(String::from("Test"), String::from("TST"))
            .await?;
        let name = token.name().call().await?;
        assert_eq!(name, "Test");
        let symbol = token.symbol().call().await?;
        assert_eq!(symbol, "TST");
        chain.shutdown();
        Ok(())
    }

    #[tokio::test]
    async fn should_load_old_state() -> Result<()> {
        let chain = LocalEvmChain::new(5001, String::from("Hermes")).await;
        let token = chain
            .deploy_token(String::from("Test"), String::from("TST"))
            .await?;
        let name = token.name().call().await?;
        assert_eq!(name, "Test");
        let state = chain.node_state().await;
        chain.shutdown();

        let chain = LocalEvmChain::new_with_chain_state(
            5001,
            String::from("Hermes"),
            state,
        )
        .await;
        let token = ERC20PresetMinterPauserContract::new(
            token.address(),
            chain.client(),
        );
        let name = token.name().call().await?;
        assert_eq!(name, "Test");
        chain.shutdown();
        Ok(())
    }
}
