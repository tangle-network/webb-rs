#[cfg(feature = "evm")]
pub mod evm;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// The `Proposal` trait is used to abstract over the different proposals for
/// all the different chains.
#[allow(clippy::module_name_repetitions)]
pub trait ProposalTrait {
    /// Get the function signature.
    fn function_sig(&self) -> crate::FunctionSignature;
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
#[allow(clippy::exhaustive_enums)]
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
    /// Set daily withdrawal limit proposal for changing the daily withdrawal
    /// limits.
    SetDailyWithdrawalLimit,
}

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

    /// Returns a boolean indicating if proposal is AnchorUpdate kind.
    pub fn is_anchor_update_proposal(&self)-> bool{
        match self.kind()  {
            ProposalKind::AnchorUpdate => true,
            _=> false
        }
    }

}

#[cfg(test)]
mod test 
{   
    use scale_codec::{Encode, Decode};
    use crate::{Proposal, ProposalKind};

    
    #[test]
    pub fn test_decode_proposal_from_bytes(){
        let anchor_update_proposa_bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe00000001000102030405060708090a0b0c0d"
            "0e0f101112131415161718191a1b1c1d1e1f"
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000001"
        );
        let proposal = Proposal::Unsigned { 
            kind: ProposalKind::AnchorUpdate,
            data: anchor_update_proposa_bytes.to_vec() 
        };

        let proposal_bytes = proposal.encode();

        let decoded_proposal = Proposal::decode(&mut proposal_bytes.as_slice()).unwrap();
        assert_eq!(decoded_proposal.kind(), proposal.kind());
        assert_eq!(decoded_proposal.data(), proposal.data());
        assert_eq!(decoded_proposal.is_unsigned(), true);
        assert_eq!(decoded_proposal.is_anchor_update_proposal(), true);
    }


}
