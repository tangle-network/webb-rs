//! Resource Id Update Proposal.
use crate::{ProposalHeader, ResourceId, TargetSystem};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Resource Id Update Proposal.
///
/// The [`ResourceIdUpdateProposal`] maps a new resource Id to a anchor
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct ResourceIdUpdateProposal {
    header: ProposalHeader,
    new_resource_id: ResourceId,
}

impl ResourceIdUpdateProposal {
    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the new resource id.
    #[must_use]
    pub const fn new_resource_id(&self) -> ResourceId {
        self.new_resource_id
    }

    /// Get target system.
    #[must_use]
    pub fn target_system(&self) -> TargetSystem {
        self.new_resource_id().target_system()
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(40 + 40 + 40 + 40);
        let target_system = self.header().resource_id().target_system();

        let target_details = match target_system {
            TargetSystem::Substrate(target) => target,
            _ => unreachable!("Unexpected target system for substrate"),
        };

        out.extend_from_slice(&self.header.to_bytes());
        let call = ExecuteSetResourceProposal {
            r_id: self.new_resource_id(),
        };
        // add pallet index
        out.push(target_details.pallet_index);
        // add call index, it is big-endian encoded from a u32 (4-bytes)
        // the last byte should contain the u8 call index
        out.push(self.header().function_signature().0[3]);
        scale_codec::Encode::encode_to(&call, &mut out);
        out
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl From<ResourceIdUpdateProposal> for Vec<u8> {
    fn from(proposal: ResourceIdUpdateProposal) -> Self {
        proposal.into_bytes()
    }
}

impl TryFrom<Vec<u8>> for ResourceIdUpdateProposal {
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
        let call: ExecuteSetResourceProposal =
            scale_codec::Decode::decode(&mut &value[42..])?;
        let new_resource_id = ResourceId::from(call.r_id);
        let proposal = ResourceIdUpdateProposal {
            header,
            new_resource_id,
        };
        Ok(proposal)
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteSetResourceProposal {
    r_id: ResourceId,
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
        // anchor tree_id
        let new_target = SubstrateTargetSystem::builder()
            .pallet_index(50)
            .tree_id(3)
            .build();
        let new_target_system = TargetSystem::Substrate(new_target);
        let new_resource_id = ResourceId::new(new_target_system, target_chain);
        let proposal = ResourceIdUpdateProposal::builder()
            .header(header)
            .new_resource_id(new_resource_id)
            .build();
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
        "00000000000000000000000000000000000000000032000000020200000000010000000100000001" // header
        "32"                                                               // pallet call
        "01"                                                               // call index
        "0000000000000000000000000000000000000000003200000003020000000001" // new_resource_id
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        // do the reverse of encode
        let bytes = hex_literal::hex!(
        "00000000000000000000000000000000000000000032000000020200000000010000000100000001" // header
        "32"                                                               // pallet call
        "01"                                                               // call index
        "0000000000000000000000000000000000000000003200000003020000000001" // new_resource_id
        );
        let proposal =
            ResourceIdUpdateProposal::try_from(bytes.to_vec()).unwrap();
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
        // anchor target system
        let src_system = SubstrateTargetSystem::builder()
            .pallet_index(50)
            .tree_id(3)
            .build();
        let src_target_system = TargetSystem::Substrate(src_system);
        let new_resource_id = ResourceId::new(src_target_system, target_chain);
        let expected_proposal = ResourceIdUpdateProposal::builder()
            .header(header)
            .new_resource_id(new_resource_id)
            .build();
        assert_eq!(proposal, expected_proposal);
    }
}
