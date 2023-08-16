//! Wrapping Fee Update Proposal.
use proposal_derive::Proposal;

use crate::ProposalHeader;

/// Wrapping Fee Update Proposal.
///
/// The [`WrappingFeeUpdateProposal`] updates the wrapping fee percentage.
///
/// The format of the proposal looks like:
/// ```text
/// ┌────────────────────┬──────────────────┐
/// │                    │                  │
/// │ ProposalHeader 40B │ WrappingFee 2B   │
/// │                    │                  │
/// └────────────────────┴──────────────────┘
/// ```
///
/// ## Notes
/// The wrapping fee percentage is a number between 0 and 10000.
///
/// For example, a fee of `42.20%` is encoded as `4220`.
#[derive(Proposal, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[proposal(
    function_sig = "function setFee(uint16 _feePercentage, uint32 _nonce)"
)]
pub struct WrappingFeeUpdateProposal {
    header: ProposalHeader,
    wrapping_fee: u16,
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
        let wrapping_fee = 0x01;
        let proposal = WrappingFeeUpdateProposal::new(header, wrapping_fee);
        let bytes = crate::to_vec(&proposal).unwrap();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe000000010001"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe000000010001"
        );
        let proposal =
            crate::from_slice::<WrappingFeeUpdateProposal>(&bytes).unwrap();
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
        assert_eq!(proposal.wrapping_fee(), &0x01);
    }
}
