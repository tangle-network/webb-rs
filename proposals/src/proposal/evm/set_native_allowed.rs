//! Set Native Allowed Proposal.
use proposal_derive::Proposal;

use crate::ProposalHeader;

/// Set Native Allowed Proposal.
///
/// The [`SetNativeAllowedProposal`] updates the Funguible Token Wrapper
/// to whether or not allow native tokens to be wrapped.
///
/// The format of the proposal looks like:
/// ```text
/// ┌────────────────────┬──────────────────┐
/// │                    │                  │
/// │ ProposalHeader 40B │ NativeAllowed 1B │
/// │                    │                  │
/// └────────────────────┴──────────────────┘
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
#[proposal(
    function_sig = "function setNativeAllowed(bool _nativeAllowed, uint32 _nonce)"
)]
pub struct SetNativeAllowedProposal {
    header: ProposalHeader,
    native_allowed: bool,
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
        let native_allowed = true;
        let proposal = SetNativeAllowedProposal::new(header, native_allowed);
        let bytes = crate::to_vec(&proposal).unwrap();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe0000000101"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe0000000101"
        );
        let proposal =
            crate::from_slice::<SetNativeAllowedProposal>(&bytes).unwrap();
        let header = proposal.header();
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let expected_header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        assert_eq!(header, expected_header);
        assert_eq!(proposal.native_allowed(), &true);
    }
}
