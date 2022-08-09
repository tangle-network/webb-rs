//! Wrapping Fee Update Proposal.
use crate::ProposalHeader;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Wrapping Fee Update Proposal.
///
/// The [`WrappingFeeUpdateProposal`] updates the wrapping fee percentage for a specific pool share.
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct WrappingFeeUpdateProposal {
    header: ProposalHeader,
    #[builder(setter(transform = |v: u128| check_and_validate_wrapping_fee(v)))]
    wrapping_fee_percent: u128,
    into_pool_share_id: u32,
}

impl WrappingFeeUpdateProposal {
    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the wrapping fee percent.
    #[must_use]
    pub const fn wrapping_fee_percent(&self) -> u128 {
        debug_assert!(
            self.wrapping_fee_percent <= 100,
            "wrapping fee percent is too large"
        );
        self.wrapping_fee_percent
    }

    /// Get the pool share id.
    /// The pool share id is the id of the pool share that the wrapping fee will be applied to.
    #[must_use]
    pub const fn pool_share_id(&self) -> u32 {
        self.into_pool_share_id
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(80);
        let target_details = self
            .header()
            .resource_id()
            .target_system()
            .get_substrate_target_system();

        // add proposal header 40B
        out.extend_from_slice(&self.header.to_bytes());

        let call = ExecuteWrappingFeeUpdate {
            r_id: self.header.resource_id().to_bytes(),
            wrapping_fee_percent: self.wrapping_fee_percent,
            into_pool_share_id: self.into_pool_share_id,
        };
        // add pallet index
        out.push(target_details.pallet_index());
        // add call index
        out.push(target_details.call_index());
        scale_codec::Encode::encode_to(&call, &mut out);
        out
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl From<WrappingFeeUpdateProposal> for Vec<u8> {
    fn from(proposal: WrappingFeeUpdateProposal) -> Self {
        proposal.into_bytes()
    }
}

impl TryFrom<Vec<u8>> for WrappingFeeUpdateProposal {
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

        let call: ExecuteWrappingFeeUpdate =
            scale_codec::Decode::decode(&mut &value[42..])?;

        let wrapping_fee_percent = call.wrapping_fee_percent;
        let into_pool_share_id = call.into_pool_share_id;
        let proposal = WrappingFeeUpdateProposal {
            header,
            wrapping_fee_percent,
            into_pool_share_id,
        };
        Ok(proposal)
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteWrappingFeeUpdate {
    r_id: [u8; 32],
    wrapping_fee_percent: u128,
    into_pool_share_id: u32,
}

fn check_and_validate_wrapping_fee(wrapping_fee_percent: u128) -> u128 {
    debug_assert!(
        wrapping_fee_percent <= 100,
        "wrapping fee percent is too large"
    );
    wrapping_fee_percent
}

#[cfg(test)]
mod tests {
    use crate::{
        FunctionSignature, Nonce, ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    #[test]
    fn encode() {
        let target_system = TargetSystem::substrate_target_system(35, 0, 2);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let proposal = WrappingFeeUpdateProposal::builder()
            .header(header)
            .wrapping_fee_percent(5)
            .into_pool_share_id(1)
            .build();
        let bytes = proposal.to_bytes();
        let expected = concat!(
            "0000000000000000000000000000000000000000230000000002020000000001cafebabe00000001", // header
            "23", // pallet index
            "00", // call index
            "0000000000000000000000000000000000000000230000000002020000000001", // resource id
            "05000000000000000000000000000000", // wrapping fee percent
            "01000000"                          // pool share id
        );
        assert_eq!(expected, hex::encode(bytes));
    }

    #[test]
    fn decode() {
        let proposal_bytes = hex_literal::hex!(
          "0000000000000000000000000000000000000000230000000002020000000001cafebabe00000001" // header
          "23" // pallet index
          "00" // call index
          "0000000000000000000000000000000000000000230000000002020000000001" // resource id
          "05000000000000000000000000000000" // wrapping fee percent
          "01000000"  // pool share id
        );

        let proposal =
            WrappingFeeUpdateProposal::try_from(proposal_bytes.to_vec())
                .unwrap();
        assert_eq!(
            proposal.header.resource_id(),
            ResourceId::new(
                TargetSystem::substrate_target_system(35, 0, 2),
                TypedChainId::Substrate(1)
            )
        );
        assert_eq!(proposal.wrapping_fee_percent(), 5);
        assert_eq!(proposal.pool_share_id(), 1);
    }

    #[cfg(debug_assertions)]
    #[should_panic(expected = "wrapping fee percent is too large")]
    #[test]
    fn should_check_wrapping_fee_value() {
        let target_system = TargetSystem::substrate_target_system(35, 0, 2);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let _ = WrappingFeeUpdateProposal::builder()
            .header(header)
            .wrapping_fee_percent(101)
            .into_pool_share_id(1)
            .build();
    }
}
