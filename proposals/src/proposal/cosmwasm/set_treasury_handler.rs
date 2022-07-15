//! Set Treasury Handler Proposal.
use crate::ProposalHeader;
use cosmwasm_std::{from_slice, to_binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Set Treasury Handler Proposal.
///
/// The [`SetTreasuryHandlerProposal`] Proposal sets the treasury handler
/// address.
///
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetTreasuryHandlerProposal {
    header: ProposalHeader,
    handler_address: String,
}

impl SetTreasuryHandlerProposal {
    /// Creates a new token add proposal.
    #[must_use]
    pub const fn new(header: ProposalHeader, address: String) -> Self {
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
    pub fn handler_address(&self) -> String {
        self.handler_address.clone()
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());

        let message = to_binary(&SetHandler {
            handler: self.handler_address.clone(),
            nonce: self.header.nonce().0,
        })
        .unwrap();
        bytes.extend_from_slice(&message.as_slice());

        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl From<Vec<u8>> for SetTreasuryHandlerProposal {
    fn from(bytes: Vec<u8>) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);

        let f = t;
        let decoded_message: SetHandler = from_slice(&bytes[f..]).unwrap();

        Self::new(header, decoded_message.handler)
    }
}

impl From<SetTreasuryHandlerProposal> for Vec<u8> {
    fn from(proposal: SetTreasuryHandlerProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
struct SetHandler {
    handler: String,
    nonce: u32,
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
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let handler_address =
            "juno1afxj87jjd4usd80gsprtq76uykv02egaydwvj62ldhngzj2zdamqxn9an3"
                .to_string();
        let proposal = SetTreasuryHandlerProposal::new(header, handler_address);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa01000000000400000000000000017b2268616e646c6572223a226a756e6f316166786a38376a6a64347573643830677370727471373675796b763032656761796477766a36326c64686e677a6a327a64616d71786e39616e33222c226e6f6e6365223a317d"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa01000000000400000000000000017b2268616e646c6572223a226a756e6f316166786a38376a6a64347573643830677370727471373675796b763032656761796477766a36326c64686e677a6a327a64616d71786e39616e33222c226e6f6e6365223a317d"
        );
        let proposal = SetTreasuryHandlerProposal::from(bytes.to_vec());
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
            FunctionSignature::new(hex_literal::hex!("00000000"))
        );
        assert_eq!(nonce, Nonce::from(0x0001));
        assert_eq!(
            proposal.handler_address(),
            "juno1afxj87jjd4usd80gsprtq76uykv02egaydwvj62ldhngzj2zdamqxn9an3"
                .to_string()
        );
    }
}
