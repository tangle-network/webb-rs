#[cfg(feature = "evm")]
pub mod evm;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "substrate")]
use frame_support::{pallet_prelude::Get, BoundedVec};

/// The `Proposal` trait is used to abstract over the different proposals for
/// all the different chains.
#[allow(clippy::module_name_repetitions)]
pub trait ProposalTrait {
    /// Get the function signature.
    fn function_sig() -> crate::FunctionSignature;
    /// Get the proposal header.
    fn header(&self) -> crate::ProposalHeader;
    /// Convert the proposal into bytes.
    ///
    /// Note: This also includes the proposal header.
    fn to_vec(&self) -> Vec<u8>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(
    feature = "scale",
    derive(scale_info::TypeInfo, scale_codec::Encode, scale_codec::Decode,)
)]
// only derive `MaxEncodedLen` if both `scale` and `substrate` features are enabled
#[cfg_attr(
    all(feature = "scale", feature = "substrate"),
    derive(scale_codec::MaxEncodedLen)
)]
#[allow(clippy::exhaustive_enums)]
/// Proposal enum
pub enum Proposal<
    #[cfg(feature = "substrate")]
    #[rustfmt::ignore] MaxLength: Get<u32>,
> {
    /// Represents a signed proposal
    Signed {
        /// Kind of the proposal
        kind: ProposalKind,
        /// Proposal data
        #[cfg(feature = "substrate")]
        data: BoundedVec<u8, MaxLength>,
        /// Proposal data
        #[cfg(not(feature = "substrate"))]
        data: Vec<u8>,
        /// Proposal signature
        #[cfg(feature = "substrate")]
        signature: BoundedVec<u8, MaxLength>,
        /// Proposal signature
        #[cfg(not(feature = "substrate"))]
        signature: Vec<u8>,
    },
    /// Represent an unsigned proposal
    Unsigned {
        /// Kind of the proposal
        kind: ProposalKind,
        /// Proposal data
        #[cfg(feature = "substrate")]
        data: BoundedVec<u8, MaxLength>,
        /// Proposal data
        #[cfg(not(feature = "substrate"))]
        data: Vec<u8>,
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(
    feature = "scale",
    derive(
        scale_info::TypeInfo,
        scale_codec::Encode,
        scale_codec::Decode,
        scale_codec::MaxEncodedLen
    )
)]
/// Proposal kind enum
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::exhaustive_enums)]
pub enum ProposalKind {
    /// Refresh proposal for DKG rotation
    Refresh,
    /// Proposer set update for emergency fallback mechanism
    ProposerSetUpdate,
    /// EVM transaction proposal
    EVM,
    /// Anchor create proposal for linking anchors together
    AnchorCreate,
    /// Anchor update proposal for linking anchors together
    AnchorUpdate,
    /// Token add proposal for adding new tokens to the asset application
    TokenAdd,
    /// Token remove proposal for removing tokens from the asset application
    TokenRemove,
    /// Wrapping fee update proposal for changing the wrapping fee of the asset
    /// application
    WrappingFeeUpdate,
    /// Resource id update for any connection or application
    ResourceIdUpdate,
    /// Rescue tokens proposal for rescuing tokens from the asset application
    RescueTokens,
    /// Max deposit limit proposal for changing the max deposit limit of the
    /// asset application
    MaxDepositLimitUpdate,
    /// Min withdrawal limit proposal for changing the min withdrawal limit of
    /// the asset application
    MinWithdrawalLimitUpdate,
    /// Set verifier proposal for changing the zero-knowledge verifier of an
    /// application
    SetVerifier,
    /// Set treasury handler proposal for changing the treasury handler of the
    /// asset application
    SetTreasuryHandler,
    /// Set fee recipient proposal for changing the fee recipient of the asset
    /// application
    FeeRecipientUpdate,
    /// Toggles whether or not the native token is allowed to be wrapped.
    SetNativeAllowed,
}

#[cfg(feature = "substrate")]
impl<MaxLength: Get<u32>> Proposal<MaxLength> {
    /// Returns the proposal data
    #[must_use]
    pub fn data(&self) -> &Vec<u8> {
        match self {
            Proposal::Signed { data, .. } | Proposal::Unsigned { data, .. } => {
                data
            }
        }
    }

    /// Returns the proposal signature or None if it is unsigned
    #[must_use]
    pub fn signature(&self) -> Option<Vec<u8>> {
        match self {
            Proposal::Signed { signature, .. } => {
                Some(signature.clone().into())
            }
            Proposal::Unsigned { .. } => None,
        }
    }

    /// Returns the proposal kind
    #[must_use]
    pub fn kind(&self) -> ProposalKind {
        match self {
            Proposal::Signed { kind, .. } | Proposal::Unsigned { kind, .. } => {
                *kind
            }
        }
    }

    /// Returns a boolean indicating if the proposal is signed
    #[must_use]
    pub fn is_signed(&self) -> bool {
        matches!(self, Proposal::Signed { .. })
    }

    /// Returns a boolean indicating if the proposal is unsigned
    #[must_use]
    pub fn is_unsigned(&self) -> bool {
        matches!(self, Proposal::Unsigned { .. })
    }
}

#[cfg(not(feature = "substrate"))]
impl Proposal {
    /// Returns the proposal data
    #[must_use]
    pub fn data(&self) -> &Vec<u8> {
        match self {
            Proposal::Signed { data, .. } | Proposal::Unsigned { data, .. } => {
                data
            }
        }
    }

    /// Returns the proposal signature or None if it is unsigned
    #[must_use]
    pub fn signature(&self) -> Option<Vec<u8>> {
        match self {
            Proposal::Signed { signature, .. } => Some(signature.clone()),
            Proposal::Unsigned { .. } => None,
        }
    }

    /// Returns the proposal kind
    #[must_use]
    pub fn kind(&self) -> ProposalKind {
        match self {
            Proposal::Signed { kind, .. } | Proposal::Unsigned { kind, .. } => {
                *kind
            }
        }
    }

    /// Returns a boolean indicating if the proposal is signed
    #[must_use]
    pub fn is_signed(&self) -> bool {
        matches!(self, Proposal::Signed { .. })
    }

    /// Returns a boolean indicating if the proposal is unsigned
    #[must_use]
    pub fn is_unsigned(&self) -> bool {
        matches!(self, Proposal::Unsigned { .. })
    }
}
