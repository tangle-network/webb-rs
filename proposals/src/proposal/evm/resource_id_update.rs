//! Resource Id Update Proposal.
use proposal_derive::Proposal;

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
#[derive(
    Proposal,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[proposal(function_sig = "function adminSetResourceWithSignature(
		bytes32 resourceID,
		bytes4 functionSig,
		uint32 nonce,
		bytes32 newResourceID,
		address handlerAddress,
		bytes memory sig
	)")]
pub struct ResourceIdUpdateProposal {
    header: ProposalHeader,
    new_resource_id: ResourceId,
    handler_address: [u8; 20],
}

impl ResourceIdUpdateProposal {
    /// Get the execution address.
    #[must_use]
    pub fn execution_address(&self) -> [u8; 20] {
        self.new_resource_id()
            .target_system()
            .into_contract_address_or_default()
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
        let bytes = crate::to_vec(&proposal).unwrap();
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
        let proposal =
            crate::from_slice::<ResourceIdUpdateProposal>(&bytes).unwrap();
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
