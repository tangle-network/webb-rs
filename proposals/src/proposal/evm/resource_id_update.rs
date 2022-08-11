//! Resource Id Update Proposal.
use crate::{ProposalHeader, ResourceId};

/// Resource Id Update Proposal.
///
/// The [`ResourceIdUpdateProposal`] maps a new resource Id to a handler
/// address.
/// The format of the proposal looks like:
/// ```text
/// ┌────────────────────┬───────────────────┬────────────────────┐
/// │                    │                   │                    │
/// │ ProposalHeader 40B │ NewResourceId 32B │ HandlerAddress 20B │
/// │                    │                   │                    │
/// └────────────────────┴───────────────────┴────────────────────┘
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ResourceIdUpdateProposal {
    header: ProposalHeader,
    new_resource_id: ResourceId,
    handler_address: [u8; 20],
}

impl ResourceIdUpdateProposal {
    /// Length of the proposal in bytes.
    pub const LENGTH: usize = ProposalHeader::LENGTH
        + ResourceId::LENGTH // new_resource_id
        + 20; // handler_address

    /// Creates a new resource id update proposal.
    #[must_use]
    pub const fn new(
        header: ProposalHeader,
        new_resource_id: ResourceId,
        handler_address: [u8; 20],
    ) -> Self {
        Self {
            header,
            new_resource_id,
            handler_address,
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
    pub const fn handler_address(&self) -> [u8; 20] {
        self.handler_address
    }

    /// Get the execution address.
    #[must_use]
    pub fn execution_address(&self) -> [u8; 20] {
        self.new_resource_id().target_system().into_evm_address()
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        bytes[f..t].copy_from_slice(&self.header.to_bytes());
        let f = t;
        let t = f + ResourceId::LENGTH;
        bytes[f..t].copy_from_slice(&self.new_resource_id.to_bytes());
        let f = t;
        let t = f + 20;
        bytes[f..t].copy_from_slice(&self.handler_address);
        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<[u8; ResourceIdUpdateProposal::LENGTH]> for ResourceIdUpdateProposal {
    fn from(bytes: [u8; ResourceIdUpdateProposal::LENGTH]) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);
        let f = t;
        let t = f + ResourceId::LENGTH;
        let mut new_resource_id = [0u8; 32];
        new_resource_id.copy_from_slice(&bytes[f..t]);
        let new_resource_id = ResourceId::from(new_resource_id);
        let f = t;
        let t = f + 20;
        let mut handler_address = [0u8; 20];
        handler_address.copy_from_slice(&bytes[f..t]);
        Self {
            header,
            new_resource_id,
            handler_address,
        }
    }
}

impl From<ResourceIdUpdateProposal> for [u8; ResourceIdUpdateProposal::LENGTH] {
    fn from(proposal: ResourceIdUpdateProposal) -> Self {
        proposal.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        FunctionSignature, Nonce, ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    #[test]
    fn encode() {
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let new_target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"),
        );
        let new_resource_id = ResourceId::new(new_target_system, target_chain);
        let handler_address =
            hex_literal::hex!("cccccccccccccccccccccccccccccccccccccccc");
        let proposal = ResourceIdUpdateProposal::new(
            header,
            new_resource_id,
            handler_address,
        );
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
        "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004" // resource_id
        "cafebabe00000001" // function_signature + nonce
        "000000000000bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb010000000004" // new_resource_id
        "cccccccccccccccccccccccccccccccccccccccc" // handler_address
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        // do the reverse of encode
        let bytes = hex_literal::hex!(
        "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004" // resource_id
        "cafebabe00000001" // function_signature + nonce
        "000000000000bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb010000000004" // new_resource_id
        "cccccccccccccccccccccccccccccccccccccccc" // handler_address
        );
        let proposal = ResourceIdUpdateProposal::from(bytes);
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let new_target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"),
        );
        let new_resource_id = ResourceId::new(new_target_system, target_chain);
        let handler_address =
            hex_literal::hex!("cccccccccccccccccccccccccccccccccccccccc");
        let expected_proposal = ResourceIdUpdateProposal::new(
            header,
            new_resource_id,
            handler_address,
        );
        assert_eq!(proposal, expected_proposal);
    }
}
