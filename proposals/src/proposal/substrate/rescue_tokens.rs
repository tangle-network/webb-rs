//! Rescue Tokens Proposal.
use crate::target_system::TargetSystem;
use crate::ProposalHeader;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Rescue Tokens Proposal.
///
/// The [`RescueTokensProposal`] rescues tokens from the treasury (fee
/// recipient) to a specified recipient address
///
/// The format of the proposal looks like:
/// ```text
/// ┌────────────────┬──────────┬────────┬──────────┬─────────┬────────┬───────────┬─────────┐
/// │                │          │        │   Pool   │         │        │           │         │
/// │ ProposalHeader │ PalletId │ CallId │  ShareId │ AssetId │ Amount │ Recipient │  Nonce  │
/// │       40B      │     1B   │    1B  │    4B    │    4B   │   16B  │    32B    │    4B   │
/// └────────────────┴──────────┴────────┴──────────┴─────────┴────────┴───────────┴─────────┘
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct RescueTokensProposal {
    header: ProposalHeader,
    pool_share_id: u32,
    asset_id: u32,
    amount: u128,
    recipient: [u8; 32],
}

impl RescueTokensProposal {
    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the pool share id.
    /// The pool share id of the fee recipient from where tokens
    /// will be rescued.
    #[must_use]
    pub const fn pool_share_id(&self) -> u32 {
        self.pool_share_id
    }

    /// Get the asset id.
    /// Asset Id is the currency Id of token which will be rescued.
    #[must_use]
    pub fn asset_id(&self) -> u32 {
        self.asset_id.clone()
    }

    /// Get the recipient address where tokens will be transferred.
    #[must_use]
    pub fn recipient(&self) -> [u8; 32] {
        self.recipient.clone()
    }

    /// Get the total amount to be rescued.
    #[must_use]
    pub fn rescue_amount(&self) -> u128 {
        self.amount.clone()
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(120);
        let target_system = self.header().resource_id().target_system();

        let target_details = match target_system {
            TargetSystem::Substrate(target) => target,
            _ => unreachable!("Unexpected target system for substrate"),
        };

        // add proposal header 40B
        out.extend_from_slice(&self.header.to_bytes());

        let call = ExecuteRescueTokensProposal {
            pool_share_id: self.pool_share_id(),
            asset_id: self.asset_id(),
            amount: self.rescue_amount(),
            recipient: self.recipient(),
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

impl From<RescueTokensProposal> for Vec<u8> {
    fn from(proposal: RescueTokensProposal) -> Self {
        proposal.into_bytes()
    }
}

impl TryFrom<Vec<u8>> for RescueTokensProposal {
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
        let call: ExecuteRescueTokensProposal =
            scale_codec::Decode::decode(&mut &value[42..])?;

        let proposal = RescueTokensProposal {
            header,
            pool_share_id: call.pool_share_id,
            asset_id: call.asset_id,
            amount: call.amount,
            recipient: call.recipient,
        };
        Ok(proposal)
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteRescueTokensProposal {
    pool_share_id: u32,
    asset_id: u32,
    amount: u128,
    recipient: [u8; 32],
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
        let recipient = [1u8; 32];
        let pool_share_id = 0x0001;
        let asset_id = 0x002;
        let proposal = RescueTokensProposal::builder()
            .header(header)
            .pool_share_id(pool_share_id)
            .asset_id(asset_id)
            .amount(100_000_u128)
            .recipient(recipient)
            .build();
        let bytes = proposal.to_bytes();
        let expected = concat!(
            "00000000000000000000000000000000000000000032000000020200000000010000000100000001", // header
            "32",                                                                   // pallet index
            "01",                                                                   // call index
            "01000000",                                                             // pool share id
            "02000000",                                                             // asset id
            "a0860100000000000000000000000000",                                     // rescue amount
            "0101010101010101010101010101010101010101010101010101010101010101",     // recipient                                                          // call index
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
            "01000000"                                                             // pool share id
            "02000000"                                                             // asset id
            "a0860100000000000000000000000000"                                     // rescue amount
            "0101010101010101010101010101010101010101010101010101010101010101"     // recipient                                                          // call index
            "01000000"                                                             // nonce
        );

        let proposal = RescueTokensProposal::try_from(bytes.to_vec()).unwrap();
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
        assert_eq!(proposal.recipient(), [1u8; 32]);
        assert_eq!(proposal.pool_share_id(), 0x0001);
        assert_eq!(proposal.asset_id(), 0x0002);
        assert_eq!(proposal.rescue_amount(), 100_000_u128);
        assert_eq!(proposal.header().nonce().to_u32(), 0x0001);
    }
}
