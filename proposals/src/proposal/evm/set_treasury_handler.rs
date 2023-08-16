//! Set Treasury Handler Proposal.
use proposal_derive::Proposal;

use crate::ProposalHeader;

/// Set Treasury Handler Proposal.
///
/// The [`SetTreasuryHandlerProposal`] Proposal sets the treasury handler
/// address.
///
/// The format of the proposal looks like:
/// ```text
/// ┌────────────────────┬────────────────────────────┐
/// │                    │                            │
/// │ ProposalHeader 40B │ TreasuryHandlerAddress 20B │
/// │                    │                            │
/// └────────────────────┴────────────────────────────┘
/// ```
#[derive(Proposal, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[proposal(
    function_sig = "function setHandler(address _newHandler, uint32 _nonce)"
)]
pub struct SetTreasuryHandlerProposal {
    header: ProposalHeader,
    handler_address: [u8; 20],
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
        let handler_address =
            hex_literal::hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
        let proposal = SetTreasuryHandlerProposal::new(header, handler_address);
        let bytes = crate::to_vec(&proposal).unwrap();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe00000001bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe00000001bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        );
        let proposal =
            crate::from_slice::<SetTreasuryHandlerProposal>(&bytes).unwrap();
        let header = proposal.header();
        let resource_id = header.resource_id();
        let target_system = resource_id.target_system();
        let target_chain = resource_id.typed_chain_id();
        let function_signature = header.function_signature();
        let nonce = header.nonce();
        assert_eq!(
            target_system,
            TargetSystem::new_contract_address(hex_literal::hex!(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            ))
        );
        assert_eq!(target_chain, TypedChainId::Evm(4));
        assert_eq!(
            function_signature,
            FunctionSignature::new(hex_literal::hex!("cafebabe"))
        );
        assert_eq!(nonce, Nonce::from(0x0001));
        assert_eq!(
            proposal.handler_address(),
            &hex_literal::hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
        );
    }
}
