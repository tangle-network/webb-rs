//! Minimum Withdrawal Limit Proposal.
use proposal_derive::Proposal;

use crate::ProposalHeader;

/// Minimum Withdrawal Limit Proposal.
///
/// The [`MinWithdrawalLimitProposal`] updates the minimum withdrawal amount
/// allowed on the variable anchor system.
///
/// The format of the proposal looks like:
/// ```text
/// ┌────────────────────┬────────────────────────┐
/// │                    │                        │
/// │ ProposalHeader 40B │ MinWithdrawalLimit 32B │
/// │                    │                        │
/// └────────────────────┴────────────────────────┘
/// ```
#[derive(Proposal, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[proposal(
    function_sig = "function configureMaximumDepositLimit(uint256 maximumDepositAmount, uint32 nonce)"
)]
pub struct MinWithdrawalLimitProposal {
    header: ProposalHeader,
    min_withdrawal_limit: [u8; 32],
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
        let min_withdrawal_limit = hex_literal::hex!(
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
        );
        let proposal =
            MinWithdrawalLimitProposal::new(header, min_withdrawal_limit);
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
            crate::from_slice::<MinWithdrawalLimitProposal>(&bytes).unwrap();
        let header = proposal.header();
        let resource_id = header.resource_id();
        let target_system = resource_id.target_system();
        let target_chain = resource_id.typed_chain_id();
        let function_signature = header.function_signature();
        let nonce = header.nonce();
        let min_withdrawal_limit = proposal.min_withdrawal_limit();
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
            min_withdrawal_limit,
            &hex_literal::hex!(
                "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
            )
        );
    }
}
