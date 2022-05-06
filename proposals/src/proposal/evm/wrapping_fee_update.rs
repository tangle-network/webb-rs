//! Wrapping Fee Update Proposal.
use crate::ProposalHeader;

/// Wrapping Fee Update Proposal.
///
/// The [`WrappingFeeUpdateProposal`] updates the wrapping fee percentage.
///
/// The format of the proposal looks like:
/// ```text
/// ┌────────────────────┬──────────────────┐
/// │                    │                  │
/// │ ProposalHeader 40B │ WrappingFee 1B   │
/// │                    │                  │
/// └────────────────────┴──────────────────┘
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WrappingFeeUpdateProposal {
    header: ProposalHeader,
    wrapping_fee: u8,
}

impl WrappingFeeUpdateProposal {
    /// Length of the proposal in bytes.
    pub const LENGTH: usize = ProposalHeader::LENGTH + 1; // wrapping_fee

    /// Creates a new wrapping fee update proposal.
    ///
    /// Wrapping fee is in the range of 0 to 100.
    ///
    /// **Note:** in debug mode, this may panic if the fee is out of range.
    #[must_use]
    pub const fn new(header: ProposalHeader, wrapping_fee: u8) -> Self {
        debug_assert!(wrapping_fee <= 100);
        Self {
            header,
            wrapping_fee,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the wrapping fee.
    ///
    /// Wrapping fees are in the range [0, 100].
    ///
    /// *Note*: In debug builds, this will panic if the wrapping fee is out of
    /// range.
    #[must_use]
    pub const fn wrapping_fee(&self) -> u8 {
        debug_assert!(self.wrapping_fee <= 100);
        self.wrapping_fee
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        bytes[f..t].copy_from_slice(&self.header.to_bytes());
        bytes[t] = self.wrapping_fee;
        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<[u8; WrappingFeeUpdateProposal::LENGTH]>
    for WrappingFeeUpdateProposal
{
    fn from(bytes: [u8; WrappingFeeUpdateProposal::LENGTH]) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);
        let wrapping_fee = bytes[t];
        Self::new(header, wrapping_fee)
    }
}

impl From<WrappingFeeUpdateProposal>
    for [u8; WrappingFeeUpdateProposal::LENGTH]
{
    fn from(proposal: WrappingFeeUpdateProposal) -> Self {
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
        let wrapping_fee = 0x01;
        let proposal = WrappingFeeUpdateProposal::new(header, wrapping_fee);
        let bytes = proposal.to_bytes();
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
        let proposal = WrappingFeeUpdateProposal::from(bytes);
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
        assert_eq!(proposal.wrapping_fee(), 0x01);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn should_panic_if_wrapping_fee_out_of_range() {
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
        let wrapping_fee = 101;
        let _ = WrappingFeeUpdateProposal::new(header, wrapping_fee);
    }
}
