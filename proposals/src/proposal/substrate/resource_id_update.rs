//! Resource Id Update Proposal.
use crate::{ProposalHeader, ResourceId};
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
    tree_id: u32,
    #[builder(default = 50)]
    pallet_index: u8,
    #[builder(default = 1)]
    call_index: u8,
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

    /// Get the new resource id.
    #[must_use]
    pub const fn tree_id(&self) -> u32 {
        self.tree_id
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(40 + 40 + 40);
        out.extend_from_slice(&self.header.to_bytes());
        let call = ExecuteSetResourceProposal {
            r_id: self.new_resource_id(),
            tree_id: self.tree_id(),
        };
        // add pallet index
        out.push(self.pallet_index);
        // add call index
        out.push(self.call_index);
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

        // parse pallet index
        let pallet_index = value.get(40).copied().ok_or_else(|| {
            scale_codec::Error::from("invalid proposal: missing pallet index")
        })?;
        // parse call index
        let call_index = value.get(41).copied().ok_or_else(|| {
            scale_codec::Error::from("invalid proposal: missing call index")
        })?;
        // parse encoded proposal call
        let call: ExecuteSetResourceProposal =
            scale_codec::Decode::decode(&mut &value[42..])?;
        let new_resource_id = ResourceId::from(call.r_id);
        let tree_id = call.tree_id;
        let proposal = ResourceIdUpdateProposal {
            header,
            new_resource_id,
            tree_id,
            pallet_index,
            call_index,
        };
        Ok(proposal)
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteSetResourceProposal {
    r_id: ResourceId,
    tree_id: u32,
}

#[cfg(test)]
mod tests {
    use crate::{
        FunctionSignature, Nonce, ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    #[test]
    fn encode() {
        let target_system = TargetSystem::new_tree_id(2);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let new_target_system = TargetSystem::new_tree_id(5);
        let new_resource_id = ResourceId::new(new_target_system, target_chain);
        let anchor_tree_id = 9;
        let proposal = ResourceIdUpdateProposal::builder()
            .header(header)
            .new_resource_id(new_resource_id)
            .tree_id(anchor_tree_id)
            .build();

        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
        "0000000000000000000000000000000000000000000000000002020000000001cafebabe00000001" // header
        "3201" // pallet call, index call
        "0000000000000000000000000000000000000000000000000005020000000001" // new_resource_id
        "09000000" // tree id
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        // do the reverse of encode
        let bytes = hex_literal::hex!(
        "0000000000000000000000000000000000000000000000000002020000000001cafebabe00000001" // header
        "3201" // pallet call, index call
        "0000000000000000000000000000000000000000000000000005020000000001" // new_resource_id
        "09000000" // tree id
        );
        let proposal =
            ResourceIdUpdateProposal::try_from(bytes.to_vec()).unwrap();
        let target_system = TargetSystem::new_tree_id(2);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let new_target_system = TargetSystem::new_tree_id(5);
        let new_resource_id = ResourceId::new(new_target_system, target_chain);
        let anchor_tree_id = 9;
        let expected_proposal = ResourceIdUpdateProposal::builder()
            .header(header)
            .new_resource_id(new_resource_id)
            .tree_id(anchor_tree_id)
            .build();
        assert_eq!(proposal, expected_proposal);
    }
}
