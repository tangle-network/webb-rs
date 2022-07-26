#[cfg(feature = "evm")]
pub mod evm;
#[cfg(feature = "substrate")]
pub mod substrate;

#[cfg(feature = "cosmwasm")]
pub mod cosmwasm;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// The `Proposal` trait is used to abstract over the different proposals for
/// all the different chains.
pub trait ProposalTrait {
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
        impl $crate::proposal::ProposalTrait for $t {
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
    crate::proposal::substrate::ResourceIdUpdateProposal,
    crate::proposal::substrate::TokenAddProposal,
    crate::proposal::substrate::TokenRemoveProposal,
    crate::proposal::substrate::WrappingFeeUpdateProposal,
}

#[cfg(feature = "cosmwasm")]
impl_proposal_for! {
    crate::proposal::cosmwasm::AnchorUpdateProposal,
    crate::proposal::cosmwasm::TokenAddProposal,
    crate::proposal::cosmwasm::TokenRemoveProposal,
    crate::proposal::cosmwasm::WrappingFeeUpdateProposal,
    crate::proposal::cosmwasm::MinWithdrawalLimitProposal,
    crate::proposal::cosmwasm::MaxDepositLimitProposal,
    crate::proposal::cosmwasm::ResourceIdUpdateProposal,
    crate::proposal::cosmwasm::FeeRecipientUpdateProposal,
    crate::proposal::cosmwasm::SetTreasuryHandlerProposal,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "scale",
    derive(scale_info::TypeInfo, scale_codec::Encode, scale_codec::Decode)
)]
/// Proposal enum
pub enum Proposal {
    /// Represents a signed proposal
    Signed {
        /// Kind of the proposal
        kind: ProposalKind,
        /// Proposal data
        data: Vec<u8>,
        /// Proposal signature
        signature: Vec<u8>,
    },
    /// Represent an unsigned proposal
    Unsigned {
        /// Kind of the proposal
        kind: ProposalKind,
        /// Proposal data
        data: Vec<u8>,
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(
    feature = "scale",
    derive(scale_info::TypeInfo, scale_codec::Encode, scale_codec::Decode)
)]
#[cfg_attr(feature = "max-encoded-len", derive(scale_codec::MaxEncodedLen))]
/// Proposal kind enum
pub enum ProposalKind {
    /// Refresh proposal for DKG rotation
    Refresh,
    /// Proposer set update for emergency fallback mechanism
    ProposerSetUpdate,
    /// EVM transaction proposal
    EVM,
    /// Anchor update proposal for linking anchors together
    AnchorUpdate,
    /// Token add proposal for adding new tokens to the asset application
    TokenAdd,
    /// Token remove proposal for removing tokens from the asset application
    TokenRemove,
    /// Wrapping fee update proposal for changing the wrapping fee of the asset application
    WrappingFeeUpdate,
    /// Resource id update for any connection or application
    ResourceIdUpdate,
    /// Rescue tokens proposal for rescuing tokens from the asset application
    RescueTokens,
    /// Max deposit limit proposal for changing the max deposit limit of the asset application
    MaxDepositLimitUpdate,
    /// Min withdrawal limit proposal for changing the min withdrawal limit of the asset application
    MinWithdrawalLimitUpdate,
    /// Set verifier proposal for changing the zero-knowledge verifier of an application
    SetVerifier,
    /// Set treasury handler proposal for changing the treasury handler of the asset application
    SetTreasuryHandler,
    /// Set fee recipient proposal for changing the fee recipient of the asset application
    FeeRecipientUpdate,
}

impl Proposal {
    /// Returns the proposal data
    pub fn data(&self) -> &Vec<u8> {
        match self {
            Proposal::Signed { data, .. } | Proposal::Unsigned { data, .. } => {
                data
            }
        }
    }

    /// Returns the proposal signature or None if it is unsigned
    pub fn signature(&self) -> Option<Vec<u8>> {
        match self {
            Proposal::Signed { signature, .. } => Some(signature.clone()),
            Proposal::Unsigned { .. } => None,
        }
    }

    /// Returns the proposal kind
    pub fn kind(&self) -> ProposalKind {
        match self {
            Proposal::Signed { kind, .. } | Proposal::Unsigned { kind, .. } => {
                kind.clone()
            }
        }
    }

    /// Returns a boolean indicating if the proposal is signed
    pub fn is_signed(&self) -> bool {
        matches!(self, Proposal::Signed { .. })
    }

    /// Returns a boolean indicating if the proposal is unsigned
    pub fn is_unsigned(&self) -> bool {
        matches!(self, Proposal::Unsigned { .. })
    }
}
