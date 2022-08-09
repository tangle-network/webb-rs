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
    target_system: TargetSystem,
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
    pub const fn target_system(&self) -> TargetSystem {
        self.target_system
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(40 + 40 + 40 + 40);
        let target_details = self
            .header()
            .resource_id()
            .target_system()
            .get_substrate_target_system();

        out.extend_from_slice(&self.header.to_bytes());
        let call = ExecuteSetResourceProposal {
            r_id: self.new_resource_id(),
            target: self.target_system(),
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
        let target_system = call.target;
        let proposal = ResourceIdUpdateProposal {
            header,
            new_resource_id,
            target_system,
        };
        Ok(proposal)
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteSetResourceProposal {
    r_id: ResourceId,
    target: TargetSystem,
}

#[cfg(test)]
mod tests {
    use crate::{
        FunctionSignature, Nonce, ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    #[test]
    fn encode() {
        let target_system = TargetSystem::substrate_target_system(50, 1, 2);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        // anchor tree_id
        let target_system = TargetSystem::substrate_target_system(50, 1, 3);
        let proposal = ResourceIdUpdateProposal::builder()
            .header(header)
            .new_resource_id(resource_id)
            .target_system(target_system)
            .build();

        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
        "0000000000000000000000000000000000000000320100000002020000000001cafebabe00000001" // header
        "3201" // pallet call, index call
        "0000000000000000000000000000000000000000320100000002020000000001" // new_resource_id
        "01320103000000" // anchor target system
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        // do the reverse of encode
        let bytes = hex_literal::hex!(
        "0000000000000000000000000000000000000000320100000002020000000001cafebabe00000001" // header
        "3201" // pallet call, index call
        "0000000000000000000000000000000000000000320100000002020000000001" // new_resource_id
        "01320103000000" // anchor target system
        );
        let proposal =
            ResourceIdUpdateProposal::try_from(bytes.to_vec()).unwrap();
        let target_system = TargetSystem::substrate_target_system(50, 1, 2);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        // anchor target system
        let target_system = TargetSystem::substrate_target_system(50, 1, 3);
        let expected_proposal = ResourceIdUpdateProposal::builder()
            .header(header)
            .new_resource_id(resource_id)
            .target_system(target_system)
            .build();
        assert_eq!(proposal, expected_proposal);
    }
}
