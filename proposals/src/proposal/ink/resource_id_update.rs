//! Resource Id Update Proposal.
use crate::{ProposalHeader, ResourceId};

/// Resource Id Update Proposal.
///
/// The [`ResourceIdUpdateProposal`] maps a new resource Id to a handler
/// address.
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct ResourceIdUpdateProposal {
    header: ProposalHeader,
    new_resource_id: ResourceId,
    handler_address: [u8; 32],
    execution_address: [u8; 32],
}

impl ResourceIdUpdateProposal {
    /// Creates a new resource id update proposal.
    #[must_use]
    pub const fn new(
        header: ProposalHeader,
        new_resource_id: ResourceId,
        handler_address: [u8; 32],
        execution_address: [u8; 32],
    ) -> Self {
        Self {
            header,
            new_resource_id,
            handler_address,
            execution_address,
        }
    }

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

    /// Get the handler address.
    #[must_use]
    pub fn handler_address(&self) -> [u8; 32] {
        self.handler_address
    }

    /// Get the execution address.
    #[must_use]
    pub fn execution_address(&self) -> [u8; 32] {
        self.execution_address
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());

        let message = ResourceIdUpdateData {
            resource_id: self.header.resource_id().to_bytes(),
            function_sig: self.header.function_signature().to_bytes(),
            nonce: self.header.nonce().0,
            new_resource_id: self.new_resource_id.to_bytes(),
            handler_addr: self.handler_address,
            execution_context_addr: self.execution_address,
        };
        scale_codec::Encode::encode_to(&message, &mut bytes);

        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl TryFrom<Vec<u8>> for ResourceIdUpdateProposal {
    type Error = scale_codec::Error;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        let parsed_header =
            bytes.get(0..ProposalHeader::LENGTH).ok_or_else(|| {
                scale_codec::Error::from(
                    "invaid proposal: invalid length of proposal",
                )
            })?;

        header_bytes.copy_from_slice(parsed_header);
        let header = ProposalHeader::from(header_bytes);

        let decoded_msg: ResourceIdUpdateData =
            scale_codec::Decode::decode(&mut &bytes[40..])?;

        Ok(Self {
            header,
            new_resource_id: decoded_msg.new_resource_id.into(),
            handler_address: decoded_msg.handler_addr,
            execution_address: decoded_msg.execution_context_addr,
        })
    }
}

impl From<ResourceIdUpdateProposal> for Vec<u8> {
    fn from(proposal: ResourceIdUpdateProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ResourceIdUpdateData {
    pub resource_id: [u8; 32],
    pub function_sig: [u8; 4],
    pub nonce: u32,
    pub new_resource_id: [u8; 32],
    pub handler_addr: [u8; 32],
    pub execution_context_addr: [u8; 32],
}

#[cfg(test)]
mod tests {
    use crate::ink::ink_address_to_target_address;
    use crate::{
        FunctionSignature, Nonce, ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    const TARGET_CONTRACT_ADDR: [u8; 32] = [0u8; 32];
    const NEW_TARGET_CONTRACT_ADDR: [u8; 32] = [1u8; 32];
    const HANDLER_ADDR: [u8; 32] = [2u8; 32];
    const EXECUTION_CONTRACT_ADDR: [u8; 32] = [3u8; 32];

    #[test]
    fn encode() {
        let target_addr = ink_address_to_target_address(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Ink(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let new_target_addr =
            ink_address_to_target_address(NEW_TARGET_CONTRACT_ADDR);
        let new_target_system = TargetSystem::ContractAddress(new_target_addr);
        let new_resource_id = ResourceId::new(new_target_system, target_chain);
        let proposal = ResourceIdUpdateProposal::new(
            header,
            new_resource_id,
            HANDLER_ADDR,
            EXECUTION_CONTRACT_ADDR,
        );
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e563060000000004000000000000000100000000000088386fc84ba6bc95484008f6362f93160ef3e5630600000000040000000001000000000000000000b312bec018884c2d66667c67a90508214bd8bafc06000000000402020202020202020202020202020202020202020202020202020202020202020303030303030303030303030303030303030303030303030303030303030303"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        // do the reverse of encode
        let bytes = hex_literal::hex!(
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e563060000000004000000000000000100000000000088386fc84ba6bc95484008f6362f93160ef3e5630600000000040000000001000000000000000000b312bec018884c2d66667c67a90508214bd8bafc06000000000402020202020202020202020202020202020202020202020202020202020202020303030303030303030303030303030303030303030303030303030303030303"
        );
        let proposal =
            ResourceIdUpdateProposal::try_from(bytes.to_vec()).unwrap();
        let target_addr = ink_address_to_target_address(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Ink(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let new_target_addr =
            ink_address_to_target_address(NEW_TARGET_CONTRACT_ADDR);
        let new_target_system = TargetSystem::ContractAddress(new_target_addr);
        let new_resource_id = ResourceId::new(new_target_system, target_chain);
        let expected_proposal = ResourceIdUpdateProposal::new(
            header,
            new_resource_id,
            HANDLER_ADDR,
            EXECUTION_CONTRACT_ADDR,
        );
        assert_eq!(proposal, expected_proposal);
    }
}
