use webb::evm::ethers;

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
}

pub type Result<T> = std::result::Result<T, Error>;
