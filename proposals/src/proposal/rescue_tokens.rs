//! Rescue Tokens Proposal.
use crate::ProposalHeader;

/// Rescue Tokens Proposal.
///
/// the format of the proposal looks like this:
/// ```text
/// ┌────────────────────┬───────────────────┬────────────────────┬────────────┐
/// │                    │                   │                    │            │
/// │ ProposalHeader 40B │ TokenAddress 20B  │ ToTokenAddress 20B │ Amount 32B │
/// │                    │                   │                    │            │
/// └────────────────────┴───────────────────┴────────────────────┴────────────┘
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RescueTokensProposal {
    header: ProposalHeader,
    token_address: [u8; 20],
    to_token_address: [u8; 20],
    amount: [u8; 32],
}

impl RescueTokensProposal {
    /// Length of the proposal in bytes.
    pub const LENGTH: usize = ProposalHeader::LENGTH
        + 20 // token_address
        + 20 // to_token_address
        + 32; // amount

    /// Creates a new resource id update proposal.
    #[must_use]
    pub const fn new(
        header: ProposalHeader,
        token_address: [u8; 20],
        to_token_address: [u8; 20],
        amount: [u8; 32],
    ) -> Self {
        Self {
            header,
            token_address,
            to_token_address,
            amount,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the amount.
    #[must_use]
    pub const fn amount(&self) -> [u8; 32] {
        self.amount
    }

    /// Get the token address.
    #[must_use]
    pub const fn token_address(&self) -> [u8; 20] {
        self.token_address
    }

    /// Get the to token address.
    #[must_use]
    pub const fn to_token_address(&self) -> [u8; 20] {
        self.to_token_address
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        bytes[f..t].copy_from_slice(&self.header.to_bytes());
        let f = t;
        let t = f + 20;
        bytes[f..t].copy_from_slice(&self.token_address);
        let f = t;
        let t = f + 20;
        bytes[f..t].copy_from_slice(&self.to_token_address);
        let f = t;
        let t = f + 32;
        bytes[f..t].copy_from_slice(&self.amount);
        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<[u8; RescueTokensProposal::LENGTH]> for RescueTokensProposal {
    fn from(bytes: [u8; RescueTokensProposal::LENGTH]) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);
        let f = t;
        let t = f + 20;
        let mut token_address = [0u8; 20];
        token_address.copy_from_slice(&bytes[f..t]);
        let f = t;
        let t = f + 20;
        let mut to_token_address = [0u8; 20];
        to_token_address.copy_from_slice(&bytes[f..t]);
        let f = t;
        let t = f + 32;
        let mut amount = [0u8; 32];
        amount.copy_from_slice(&bytes[f..t]);
        Self {
            header,
            token_address,
            to_token_address,
            amount,
        }
    }
}

impl From<RescueTokensProposal> for [u8; RescueTokensProposal::LENGTH] {
    fn from(proposal: RescueTokensProposal) -> Self {
        proposal.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ChainId, ChainType, FunctionSignature, Nonce, ResourceId, TargetSystem,
    };

    use super::*;

    #[test]
    fn encode() {
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain_type = ChainType::Evm;
        let target_chain_id = ChainId::from(4);
        let resource_id =
            ResourceId::new(target_system, target_chain_type, target_chain_id);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let token_address =
            hex_literal::hex!("cccccccccccccccccccccccccccccccccccccccc");
        let to_token_address =
            hex_literal::hex!("dddddddddddddddddddddddddddddddddddddddd");
        let amount = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let proposal = RescueTokensProposal::new(
            header,
            token_address,
            to_token_address,
            amount,
        );
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
        "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004" // resource_id
        "cafebabe00000001" // function_signature + nonce
        "cccccccccccccccccccccccccccccccccccccccc" // token_address
        "dddddddddddddddddddddddddddddddddddddddd" // to_token_address
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
        let expected_proposal = RescueTokensProposal::from(bytes);
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain_type = ChainType::Evm;
        let target_chain_id = ChainId::from(4);
        let resource_id =
            ResourceId::new(target_system, target_chain_type, target_chain_id);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let token_address =
            hex_literal::hex!("cccccccccccccccccccccccccccccccccccccccc");
        let to_token_address =
            hex_literal::hex!("dddddddddddddddddddddddddddddddddddddddd");
        let amount = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let proposal = RescueTokensProposal::new(
            header,
            token_address,
            to_token_address,
            amount,
        );
        assert_eq!(proposal, expected_proposal);
    }
}
