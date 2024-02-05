use webb::evm::ethers;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Generic error.
    #[error("{}", _0)]
    Generic(&'static str),
    /// IO error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Smart contract error.
    #[error(transparent)]
    EthersContract(
        #[from]
        ethers::contract::ContractError<
            ethers::providers::Provider<ethers::providers::Http>,
        >,
    ),
    /// Smart contract error.
    #[error(transparent)]
    EthersContract2(
        #[from]
        ethers::contract::ContractError<
            ethers::middleware::SignerMiddleware<
                ethers::providers::Provider<ethers::providers::Http>,
                ethers::signers::LocalWallet,
            >,
        >,
    ),
    /// Initial Governor not defined for given chain
    #[error("Initial Governor not defined for: {:?}", chain_id)]
    NoInitialGovernor {
        /// The chain id of the bridge.
        chain_id: u32,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
