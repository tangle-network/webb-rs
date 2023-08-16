//! Maximum Deposit Limit Proposal.
use proposal_derive::Proposal;

use crate::ProposalHeader;

/// Maximum Deposit Limit Proposal.
///
/// The [`MaxDepositLimitProposal`] updates the maximum deposit amount allowed
/// on the variable anchor system.
///
/// The format of the proposal looks like:
/// ```text
/// ┌────────────────────┬──────────────────────┐
/// │                    │                      │
/// │ ProposalHeader 40B │ MaxDepositLimit 32B  │
/// │                    │                      │
/// └────────────────────┴──────────────────────┘
/// ```
#[derive(Proposal, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[proposal(
    function_sig = "function configureMaximumDepositLimit(uint256 maximumDepositAmount, uint32 nonce)"
)]
pub struct MaxDepositLimitProposal {
    header: ProposalHeader,
    max_deposit_limit: [u8; 32],
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
        let max_deposit_limit = hex_literal::hex!(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        );
        let proposal = MaxDepositLimitProposal::new(header, max_deposit_limit);
        let bytes = crate::to_vec(&proposal).unwrap();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe00000001000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe00000001000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        );
        let proposal =
            crate::from_slice::<MaxDepositLimitProposal>(&bytes).unwrap();
        let header = proposal.header();
        let resource_id = header.resource_id();
        let target_system = resource_id.target_system();
        let target_chain = resource_id.typed_chain_id();
        let function_signature = header.function_signature();
        let nonce = header.nonce();
        let max_deposit_limit = proposal.max_deposit_limit();
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
            max_deposit_limit,
            &hex_literal::hex!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
            )
        );
    }
}
