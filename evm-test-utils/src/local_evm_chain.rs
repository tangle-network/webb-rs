use std::sync::Arc;

use futures::prelude::*;
use webb::evm::contract::protocol_solidity::{
    anchor_handler::AnchorHandlerContract,
    erc20_preset_minter_pauser::ERC20PresetMinterPauserContract,
    poseidon_hasher::PoseidonHasherContract, poseidon_hasher_factory,
    poseidon_t3::PoseidonT3Contract, poseidon_t4::PoseidonT4Contract,
    poseidon_t6::PoseidonT6Contract, signature_bridge::SignatureBridgeContract,
    treasury::TreasuryContract, treasury_handler::TreasuryHandlerContract,
};
use webb::evm::ethers;
use webb_proposals::TypedChainId;

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
        let maybe_signal =
            Option::take(self.anvil_node_handle.shutdown_signal_mut());
        if let Some(signal) = maybe_signal {
            signal.fire().expect("signal fired");
        }
    }

    /// Deploy a new ERC20 token.
    ///
    /// # Errors
    ///
    /// This function will return an error if the deployment fails.
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

    /// Deploy a new Signature Bridge.
    ///
    /// # Errors
    ///
    /// This function will return an error if the deployment fails.
    pub async fn deploy_signature_bridge(
        &self,
        initial_governor: ethers::types::Address,
        nonce: u32,
    ) -> Result<SignatureBridgeContract<SignerEthersClient>> {
        SignatureBridgeContract::deploy(
            self.client.clone(),
            (initial_governor, nonce),
        )?
        .confirmations(0usize)
        .send()
        .map_err(Into::into)
        .await
    }

    pub async fn deploy_poseidon_hasher(
        &self,
    ) -> Result<PoseidonHasherContract<SignerEthersClient>> {
        let t3 = PoseidonT3Contract::deploy(self.client.clone(), ())?
            .confirmations(0usize)
            .send()
            .await?;
        let t4 = PoseidonT4Contract::deploy(self.client.clone(), ())?
            .confirmations(0usize)
            .send()
            .await?;
        let t6 = PoseidonT6Contract::deploy(self.client.clone(), ())?
            .confirmations(0usize)
            .send()
            .await?;

        let hasher_factory = poseidon_hasher_factory::create(
            t3.address(),
            t4.address(),
            t6.address(),
            self.client.clone(),
        )?;
        let contract = hasher_factory
            .deploy(())?
            .confirmations(0usize)
            .send()
            .await?;
        let hasher = PoseidonHasherContract::new(
            contract.address(),
            self.client.clone(),
        );
        Ok(hasher)
    }

    /// Deploy a new Anchor Handler.
    ///
    /// # Errors
    ///
    /// This function will return an error if the deployment fails.
    pub async fn deploy_anchor_handler(
        &self,
        bridge_contract_address: ethers::types::Address,
        initial_resource_ids: Vec<webb_proposals::ResourceId>,
        initial_contract_addresses: Vec<ethers::types::Address>,
    ) -> Result<AnchorHandlerContract<SignerEthersClient>> {
        let initial_r_ids = initial_resource_ids
            .iter()
            .map(webb_proposals::ResourceId::to_bytes)
            .collect::<Vec<_>>();
        AnchorHandlerContract::deploy(
            self.client.clone(),
            (
                bridge_contract_address,
                initial_r_ids,
                initial_contract_addresses,
            ),
        )?
        .confirmations(0usize)
        .send()
        .map_err(Into::into)
        .await
    }

    /// Deploy a new Treasury Handler.
    ///
    /// # Errors
    ///
    /// This function will return an error if the deployment fails.
    pub async fn deploy_tresury_handler(
        &self,
        bridge_contract_address: ethers::types::Address,
        initial_resource_ids: Vec<webb_proposals::ResourceId>,
        initial_contract_addresses: Vec<ethers::types::Address>,
    ) -> Result<TreasuryHandlerContract<SignerEthersClient>> {
        let initial_r_ids = initial_resource_ids
            .iter()
            .map(webb_proposals::ResourceId::to_bytes)
            .collect::<Vec<_>>();
        TreasuryHandlerContract::deploy(
            self.client.clone(),
            (
                bridge_contract_address,
                initial_r_ids,
                initial_contract_addresses,
            ),
        )?
        .confirmations(0usize)
        .send()
        .map_err(Into::into)
        .await
    }

    /// Deploy a new Treasury.
    ///
    /// # Errors
    ///
    /// This function will return an error if the deployment fails.
    pub async fn deploy_tresury(
        &self,
        treasury_handler_contract_address: ethers::types::Address,
    ) -> Result<TreasuryContract<SignerEthersClient>> {
        TreasuryContract::deploy(
            self.client.clone(),
            treasury_handler_contract_address,
        )?
        .confirmations(0usize)
        .send()
        .map_err(Into::into)
        .await
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
    use webb::evm::ethers::types::U256;

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
    async fn should_be_able_to_deploy_hasher() -> Result<()> {
        let chain = LocalEvmChain::new(5001, String::from("Hermes")).await;
        let hasher = chain.deploy_poseidon_hasher().await?;
        let hash = hasher
            .hash_left_right(U256::from(1), U256::from(2))
            .call()
            .await?;
        let expected_result = U256::from_str_radix(
            "115cc0f5e7d690413df64c6b9662e9cf2a3617f2743245519e19607a4417189a",
            16,
        );
        assert_eq!(hash, expected_result.unwrap());
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
