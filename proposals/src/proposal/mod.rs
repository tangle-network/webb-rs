#[cfg(feature = "evm")]
pub mod evm;
#[cfg(feature = "substrate")]
pub mod substrate;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// The `Proposal` trait is used to abstract over the different proposals for
/// all the different chains.
pub trait Proposal {
    /// Get the proposal header.
    fn header(&self) -> crate::ProposalHeader;
    /// Convert the proposal into bytes.
    ///
    /// Note: This also includes the proposal header.
    fn to_vec(&self) -> Vec<u8>;
}

/// a helper macro to implement the `Proposal` trait for a given proposal.
macro_rules! impl_proposal_for  {
    ($t:path) => {
        impl $crate::proposal::Proposal for $t {
            fn header(&self) -> $crate::ProposalHeader {
                self.header()
            }

            fn to_vec(&self) -> Vec<u8> {
                self.to_bytes().into()
            }
        }
    };
    ($($rest:path),* $(,)?) => {
        $(impl_proposal_for!($rest);)*
    };
}

#[cfg(feature = "evm")]
impl_proposal_for! {
    crate::proposal::evm::AnchorUpdateProposal,
    crate::proposal::evm::TokenAddProposal,
    crate::proposal::evm::TokenRemoveProposal,
    crate::proposal::evm::WrappingFeeUpdateProposal,
    crate::proposal::evm::MinWithdrawalLimitProposal,
    crate::proposal::evm::MaxDepositLimitProposal,
    crate::proposal::evm::ResourceIdUpdateProposal,
    crate::proposal::evm::SetTreasuryHandlerProposal,
    crate::proposal::evm::SetVerifierProposal,
    crate::proposal::evm::FeeRecipientUpdateProposal,
    crate::proposal::evm::RescueTokensProposal,
}

#[cfg(feature = "substrate")]
impl_proposal_for! {
    crate::proposal::substrate::AnchorUpdateProposal,
    crate::proposal::substrate::TokenAddProposal,
    crate::proposal::substrate::TokenRemoveProposal,
    crate::proposal::substrate::WrappingFeeUpdateProposal,
}
