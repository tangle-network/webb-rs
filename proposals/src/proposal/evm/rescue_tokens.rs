//! Rescue Tokens Proposal.
use proposal_derive::Proposal;

use crate::ProposalHeader;

/// Rescue Tokens Proposal.
///
/// The `RescueTokensProposal` rescues tokens from the treasury to a specified
/// `to` address.
/// The format of the proposal looks like this:
///
/// ```text
/// ┌────────────────────┬───────────────────┬───────────────┬────────────┐
/// │                    │                   │               │            │
/// │ ProposalHeader 40B │ TokenAddress 20B  │ Recipient 20B │ Amount 32B │
/// │                    │                   │               │            │
/// └────────────────────┴───────────────────┴───────────────┴────────────┘
/// ```
#[derive(Proposal, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[proposal(
    function_sig = "function rescueTokens(address _tokenAddress, address _to, uint256 _amountToRescue, uint32 _nonce)"
)]
pub struct RescueTokensProposal {
    header: ProposalHeader,
    token_address: [u8; 20],
    recipient: [u8; 20],
    amount: [u8; 32],
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
        let token_address =
            hex_literal::hex!("cccccccccccccccccccccccccccccccccccccccc");
        let recipient =
            hex_literal::hex!("dddddddddddddddddddddddddddddddddddddddd");
        let amount = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let proposal =
            RescueTokensProposal::new(header, token_address, recipient, amount);
        let bytes = crate::to_vec(&proposal).unwrap();
        let expected = hex_literal::hex!(
        "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004" // resource_id
        "cafebabe00000001" // function_signature + nonce
        "cccccccccccccccccccccccccccccccccccccccc" // token_address
        "dddddddddddddddddddddddddddddddddddddddd" // recipient
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f" // amount
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        // the reverse of encode
        let bytes = hex_literal::hex!(
        "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004" // resource_id
        "cafebabe00000001" // function_signature + nonce
        "cccccccccccccccccccccccccccccccccccccccc" // token_address
        "dddddddddddddddddddddddddddddddddddddddd" // new_token_address
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f" // amount
        );
        let expected_proposal =
            crate::from_slice::<RescueTokensProposal>(&bytes).unwrap();
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
        let token_address =
            hex_literal::hex!("cccccccccccccccccccccccccccccccccccccccc");
        let recipient =
            hex_literal::hex!("dddddddddddddddddddddddddddddddddddddddd");
        let amount = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let proposal =
            RescueTokensProposal::new(header, token_address, recipient, amount);
        assert_eq!(proposal, expected_proposal);
    }
}
