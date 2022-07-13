//! Token Add Proposal.
use crate::ProposalHeader;
use cosmwasm_std::{from_slice, to_binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Token Add Proposal.
///
/// The [`TokenAddProposal`] allows the token specified by the `TokenAddress` to
/// be wrapped into the WEBB token.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenAddProposal {
    header: ProposalHeader,
    token_address: String,
}

impl TokenAddProposal {
    /// Creates a new token add proposal.
    #[must_use]
    pub const fn new(header: ProposalHeader, address: String) -> Self {
        Self {
            header,
            token_address: address,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the token address.
    #[must_use]
    pub fn token_address(&self) -> String {
        self.token_address.clone()
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());

        let message = to_binary(&AddCw20TokenAddr {
            token: self.token_address.clone(),
            nonce: self.header.nonce().0 as u64,
        })
        .unwrap();
        bytes.extend_from_slice(message.as_slice());
        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl From<Vec<u8>> for TokenAddProposal {
    fn from(bytes: Vec<u8>) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);

        let f = t;
        let decoded_msg: AddCw20TokenAddr = from_slice(&bytes[f..]).unwrap();

        Self::new(header, decoded_msg.token)
    }
}

impl From<TokenAddProposal> for Vec<u8> {
    fn from(proposal: TokenAddProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
struct AddCw20TokenAddr {
    token: String,
    nonce: u64,
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
        let token_address =
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string();
        let proposal = TokenAddProposal::new(header, token_address);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa040000000004cafebabe000000017b22746f6b656e223a2262626262626262626262626262626262626262626262626262626262626262626262626262626262222c226e6f6e6365223a317d"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa040000000004cafebabe000000017b22746f6b656e223a2262626262626262626262626262626262626262626262626262626262626262626262626262626262222c226e6f6e6365223a317d"
        );
        let proposal = TokenAddProposal::from(bytes.to_vec());
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
            proposal.token_address(),
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string()
        );
    }
}
