//! Fee Recipient Update Proposal.
use crate::ProposalHeader;

/// Fee Recipient Update Proposal.
///
/// The [`FeeRecipientUpdateProposal`] updates the recipient of the wrapping fees. The format of the proposal looks like:
/// ```text
/// ┌────────────────────┬─────────────────────────┐
/// │                    │                         │
/// │ ProposalHeader 40B │ FeeRecipientAddress 20B │
/// │                    │                         │
/// └────────────────────┴─────────────────────────┘
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FeeRecipientUpdateProposal {
    header: ProposalHeader,
    fee_recipient_address: [u8; 20],
}

impl FeeRecipientUpdateProposal {
    /// Length of the proposal in bytes.
    pub const LENGTH: usize = ProposalHeader::LENGTH + 20; // fee_recipient_address

    /// Creates a new fee recipient update proposal.
    #[must_use]
    pub const fn new(header: ProposalHeader, address: [u8; 20]) -> Self {
        Self {
            header,
            fee_recipient_address: address,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the fee recipient address.
    #[must_use]
    pub const fn fee_recipient_address(&self) -> [u8; 20] {
        self.fee_recipient_address
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        bytes[f..t].copy_from_slice(&self.header.to_bytes());
        let f = t;
        let t = t + 20;
        bytes[f..t].copy_from_slice(&self.fee_recipient_address);
        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<[u8; FeeRecipientUpdateProposal::LENGTH]>
    for FeeRecipientUpdateProposal
{
    fn from(bytes: [u8; FeeRecipientUpdateProposal::LENGTH]) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);
        let f = t;
        let t = t + 20;
        let mut address = [0u8; 20];
        address.copy_from_slice(&bytes[f..t]);
        Self::new(header, address)
    }
}

impl From<FeeRecipientUpdateProposal>
    for [u8; FeeRecipientUpdateProposal::LENGTH]
{
    fn from(proposal: FeeRecipientUpdateProposal) -> Self {
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
        let new_fee_recipient_address =
            hex_literal::hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
        let proposal =
            FeeRecipientUpdateProposal::new(header, new_fee_recipient_address);
        let bytes = proposal.to_bytes();
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
        let proposal = FeeRecipientUpdateProposal::from(bytes);
        let header = proposal.header();
        let resource_id = header.resource_id();
        let target_system = resource_id.target_system();
        let target_chain_type = resource_id.chain_type();
        let target_chain_id = resource_id.chain_id();
        let function_signature = header.function_signature();
        let nonce = header.nonce();
        assert_eq!(
            target_system,
            TargetSystem::new_contract_address(hex_literal::hex!(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            ))
        );
        assert_eq!(target_chain_type, ChainType::Evm);
        assert_eq!(target_chain_id, ChainId::from(4));
        assert_eq!(
            function_signature,
            FunctionSignature::new(hex_literal::hex!("cafebabe"))
        );
        assert_eq!(nonce, Nonce::from(0x0001));
        assert_eq!(
            proposal.fee_recipient_address(),
            hex_literal::hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
        );
    }
}
