//!  Maximum Deposit Limit Proposal.
use crate::target_system::TargetSystem;
use crate::ProposalHeader;
use scale_codec::Encode;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

///  Maximum Deposit Limit Proposal.
///
/// The [`MaxDepositLimitProposal`] updates the maximum deposit amount allowed
/// on the variable anchor system.
///
/// The format of the proposal looks like:
/// ```text
/// ┌────────────────────┬──────────┬────────┬─────────────────┬─────────┐
/// │                    │          │        │                 │         │
/// │ ProposalHeader     │ PalletId │ CallId │ MaxDepositLimit │  Nonce  │
/// │       40B          │     1B   │    1B  │      16B        │    4B   │
/// └────────────────────┴──────────┴────────┴─────────────────┴─────────┘
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct MaxDepositLimitProposal {
    header: ProposalHeader,
    max_deposit_limit: u128,
}

impl MaxDepositLimitProposal {
    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the maximum deposit limit.
    #[must_use]
    pub const fn max_deposit_limit(&self) -> u128 {
        self.max_deposit_limit
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let target_system = self.header().resource_id().target_system();

        let target_details = match target_system {
            TargetSystem::Substrate(target) => target,
            _ => unreachable!("Unexpected target system for substrate"),
        };
        // add proposal header 40B
        out.extend_from_slice(&self.header.to_bytes());

        let call = ExecuteMaxDepositLimitProposal {
            max_deposit_limit: self.max_deposit_limit,
            nonce: self.header().nonce().to_u32(),
        };
        // add pallet index
        out.push(target_details.pallet_index);
        // add call index, it is big-endian encoded from a u32 (4-bytes)
        // the last byte should contain the u8 call index
        out.push(self.header().function_signature().0[3]);
        // encode the rest of the call
        out.extend_from_slice(&call.encode());
        out
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl From<MaxDepositLimitProposal> for Vec<u8> {
    fn from(proposal: MaxDepositLimitProposal) -> Self {
        proposal.into_bytes()
    }
}

impl TryFrom<Vec<u8>> for MaxDepositLimitProposal {
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

        let call: ExecuteMaxDepositLimitProposal =
            scale_codec::Decode::decode(&mut &value[42..])?;

        let max_deposit_limit = call.max_deposit_limit;
        let proposal = MaxDepositLimitProposal {
            header,
            max_deposit_limit,
        };
        Ok(proposal)
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteMaxDepositLimitProposal {
    max_deposit_limit: u128,
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
            .pallet_index(35)
            .tree_id(2)
            .build();
        let target_system = TargetSystem::Substrate(target);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature = FunctionSignature::new([0, 0, 0, 0]);
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);

        let proposal = MaxDepositLimitProposal::builder()
            .header(header)
            .max_deposit_limit(100_000_u128)
            .build();
        let bytes = proposal.to_bytes();
        let expected = concat!(
            "00000000000000000000000000000000000000000023000000020200000000010000000000000001", // header
            "23",                               // pallet index
            "00",                               // call index
            "a0860100000000000000000000000000", // max_deposit limit
            "01000000"                          // nonce
        );
        assert_eq!(hex::encode(bytes), expected);
    }

    #[test]
    fn decode() {
        let proposal_bytes = hex_literal::hex!(
            "00000000000000000000000000000000000000000023000000020200000000010000000000000001" // header
            "23"                                // pallet index
            "00"                                // call index
            "a0860100000000000000000000000000"  // max_deposit limit
            "01000000"                          // nonce
        );

        let proposal =
            MaxDepositLimitProposal::try_from(proposal_bytes.to_vec()).unwrap();
        let target = SubstrateTargetSystem::builder()
            .pallet_index(35)
            .tree_id(2)
            .build();
        assert_eq!(
            proposal.header.resource_id(),
            ResourceId::new(
                TargetSystem::Substrate(target),
                TypedChainId::Substrate(1)
            )
        );
        assert_eq!(proposal.max_deposit_limit(), 100_000_u128);
        assert_eq!(proposal.header().nonce().to_u32(), 0x0001);
    }
}
