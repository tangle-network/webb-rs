use webb::evm::ethers;
use webb_proposals::TypedChainId;

pub struct LocalEvmChain {
    typed_chain_id: TypedChainId,
    name: String,
    anvil_node_handle: anvil::NodeHandle,
    anvil_eth_api: anvil::eth::EthApi,
    account_generator: anvil::AccountGenerator,
}

impl LocalEvmChain {
    pub async fn new(chain_id: u32, name: String) -> Self {
        let typed_chain_id = TypedChainId::Evm(chain_id);
        let (anvil_eth_api, anvil_node_handle, account_generator) =
            Self::spawn_anvil_node(typed_chain_id, None).await;
        Self {
            typed_chain_id,
            name,
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
        Self {
            typed_chain_id,
            name,
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
            .with_chain_id(Some(typed_chain_id.underlying_chain_id()))
            .with_port(
                u16::try_from(typed_chain_id.underlying_chain_id())
                    .expect("valid port"),
            )
            .with_account_generator(generator.clone())
            .with_init_state(state)
            .with_genesis_balance(
                ethers::utils::parse_ether(1000).expect("valid ether"),
            );
        let (anvil_eth_api, anvil_node_handle) = anvil::spawn(config).await;
        (anvil_eth_api, anvil_node_handle, generator)
    }
}
