//! Set Treasury Handler Proposal.
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
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SetTreasuryHandlerProposal {
    header: ProposalHeader,
    handler_address: [u8; 20],
}

impl SetTreasuryHandlerProposal {
    /// Length of the proposal in bytes.
    pub const LENGTH: usize = ProposalHeader::LENGTH + 20; // token_address

    /// Creates a new token add proposal.
    #[must_use]
    pub const fn new(header: ProposalHeader, address: [u8; 20]) -> Self {
        Self {
            header,
            handler_address: address,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the handler address.
    #[must_use]
    pub const fn handler_address(&self) -> [u8; 20] {
        self.handler_address
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
        bytes[f..t].copy_from_slice(&self.handler_address);
        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<[u8; SetTreasuryHandlerProposal::LENGTH]>
    for SetTreasuryHandlerProposal
{
    fn from(bytes: [u8; SetTreasuryHandlerProposal::LENGTH]) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);
        let f = t;
        let t = t + 20;
        let mut handler_address = [0u8; 20];
        handler_address.copy_from_slice(&bytes[f..t]);
        Self::new(header, handler_address)
    }
}

impl From<SetTreasuryHandlerProposal>
    for [u8; SetTreasuryHandlerProposal::LENGTH]
{
    fn from(proposal: SetTreasuryHandlerProposal) -> Self {
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
        let target_chain = TypedChainId::Cosmos(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let handler_address =
            hex_literal::hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
        let proposal = SetTreasuryHandlerProposal::new(header, handler_address);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa040000000004"
            "cafebabe00000001bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa040000000004"
            "cafebabe00000001bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        );
        let proposal = SetTreasuryHandlerProposal::from(bytes);
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
        assert_eq!(target_chain, TypedChainId::Cosmos(4));
        assert_eq!(
            function_signature,
            FunctionSignature::new(hex_literal::hex!("cafebabe"))
        );
        assert_eq!(nonce, Nonce::from(0x0001));
        assert_eq!(
            proposal.handler_address(),
            hex_literal::hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
        );
    }
}
