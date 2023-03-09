#![allow(clippy::exhaustive_enums)]
//! Fee Recipient Update Proposal.
use crate::target_system::TargetSystem;
use crate::ProposalHeader;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Fee Recipient Update Proposal.
///
/// The [`FeeRecipientUpdateProposal`] updates the recipient of the wrapping
/// fees.
///
/// The format of the proposal looks like:
/// ```text
/// ┌────────────────────┬──────────┬────────┬──────────────┬──────────────┬─────────┐
/// │                    │          │        │              │              │         │
/// │ ProposalHeader     │ PalletId │ CallId │ FeeRecipient │ Pool ShareId │  Nonce  │
/// │       40B          │     1B   │    1B  │     32B      │   4B         │    4B   │
/// └────────────────────┴──────────┴────────┴──────────────┴──────────────┴─────────┘
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct FeeRecipientUpdateProposal {
    header: ProposalHeader,
    fee_recipient: [u8; 32],
    pool_share_id: u32,
}

impl FeeRecipientUpdateProposal {
    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the pool share id.
    /// The pool share id is the id of the pool whose fee recipient which
    /// will be updated
    #[must_use]
    pub const fn pool_share_id(&self) -> u32 {
        self.pool_share_id
    }

    /// Get the fee recipient.
    #[must_use]
    pub fn fee_recipient(&self) -> [u8; 32] {
        self.fee_recipient
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(120);
        let target_system = self.header().resource_id().target_system();

        let TargetSystem::Substrate(target_details) = target_system else { unreachable!("Unexpected target system for substrate") };

        // add proposal header 40B
        out.extend_from_slice(&self.header.to_bytes());

        let call = ExecuteFeeRecipientUpdateProposal {
            fee_recipient: self.fee_recipient,
            pool_share_id: self.pool_share_id,
            nonce: self.header().nonce().to_u32(),
        };

        // add pallet index
        out.push(target_details.pallet_index);
        // add call index, it is big-endian encoded from a u32 (4-bytes)
        // the last byte should contain the u8 call index
        out.push(self.header().function_signature.0[3]);
        scale_codec::Encode::encode_to(&call, &mut out);
        out
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl From<FeeRecipientUpdateProposal> for Vec<u8> {
    fn from(proposal: FeeRecipientUpdateProposal) -> Self {
        proposal.into_bytes()
    }
}

impl TryFrom<Vec<u8>> for FeeRecipientUpdateProposal {
    type Error = scale_codec::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        // parse header bytes
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        let parsed_header =
            value.get(0..ProposalHeader::LENGTH).ok_or_else(|| {
                scale_codec::Error::from(
                    "invaid proposal: invalid length of proposal",
                )
            })?;
        header_bytes.copy_from_slice(parsed_header);
        let header = ProposalHeader::from(header_bytes);

        // parse encoded proposal call
        let call: ExecuteFeeRecipientUpdateProposal =
            scale_codec::Decode::decode(&mut &value[42..])?;
        let fee_recipient = call.fee_recipient;
        let pool_share_id = call.pool_share_id;
        let proposal = FeeRecipientUpdateProposal {
            header,
            fee_recipient,
            pool_share_id,
        };
        Ok(proposal)
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteFeeRecipientUpdateProposal {
    fee_recipient: [u8; 32],
    pool_share_id: u32,
    nonce: u32,
}

#[cfg(test)]
mod tests {
    use crate::{
        FunctionSignature, Nonce, ResourceId, SubstrateTargetSystem,
        TargetSystem, TypedChainId,
    };

    use super::*;

    #[test]
    fn encode() {
        let target = SubstrateTargetSystem::builder()
            .pallet_index(50)
            .tree_id(2)
            .build();
        let target_system = TargetSystem::Substrate(target);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature = FunctionSignature::new([0, 0, 0, 1]);
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let fee_recipient = [1u8; 32];
        let pool_share_id = 0x0001;
        let proposal = FeeRecipientUpdateProposal::builder()
            .header(header)
            .fee_recipient(fee_recipient)
            .pool_share_id(pool_share_id)
            .build();
        let bytes = proposal.to_bytes();
        let expected = concat!(
          "00000000000000000000000000000000000000000032000000020200000000010000000100000001", // header
          "32",                                                                   // pallet index
          "01",                                                                   // call index
          "0101010101010101010101010101010101010101010101010101010101010101",     // fee recipient                                                          // call index
          "01000000",                                                             // pool share id
          "01000000",                                                             // nonce
        );
        let bytes_hex = hex::encode(bytes);
        assert_eq!(bytes_hex, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "00000000000000000000000000000000000000000032000000020200000000010000000100000001" // header
            "32"                                                                   // pallet index
            "01"                                                                   // call index
            "0101010101010101010101010101010101010101010101010101010101010101"     // fee recipient                                                          // call index
            "01000000"                                                             // pool share id
            "01000000"                                                             // nonce
        );

        let proposal =
            FeeRecipientUpdateProposal::try_from(bytes.to_vec()).unwrap();
        let target = SubstrateTargetSystem::builder()
            .pallet_index(50)
            .tree_id(2)
            .build();
        assert_eq!(
            proposal.header.resource_id(),
            ResourceId::new(
                TargetSystem::Substrate(target),
                TypedChainId::Substrate(1)
            )
        );
        assert_eq!(proposal.fee_recipient(), [1u8; 32]);
        assert_eq!(proposal.pool_share_id(), 0x0001);
        assert_eq!(proposal.header().nonce().to_u32(), 0x0001);
    }
}
