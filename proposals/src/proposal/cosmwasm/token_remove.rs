//! Token Remove Proposal.
use crate::ProposalHeader;
use cosmwasm_std::{from_slice, to_binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Token Remove Proposal.
///
/// Suppose a token has been allowed to be wrapped into the WEBB token by the
/// token add proposal. The [`TokenRemoveProposal`] can disallow this added
/// token from being wrapped into the WEBB token.
///
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenRemoveProposal {
    header: ProposalHeader,
    token_address: String,
}

impl TokenRemoveProposal {
    /// Length of the proposal in bytes.
    pub const LENGTH: usize = ProposalHeader::LENGTH + 20; // token_address

    /// Creates a new token remove proposal.
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

        let message = to_binary(&RemoveCw20TokenAddr {
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

impl From<Vec<u8>> for TokenRemoveProposal {
    fn from(bytes: Vec<u8>) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);

        let f = t;
        let decoded_msg: RemoveCw20TokenAddr = from_slice(&bytes[f..]).unwrap();
        Self::new(header, decoded_msg.token)
    }
}

impl From<TokenRemoveProposal> for Vec<u8> {
    fn from(proposal: TokenRemoveProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
struct RemoveCw20TokenAddr {
    token: String,
    nonce: u64,
}

#[cfg(test)]
mod tests {
    use crate::{
        cosmwasm::cosmos_addr_2_target_addr, FunctionSignature, Nonce,
        ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    const TARGET_CONTRACT_ADDR: &str =
        "juno1hset4pny4h8xm4s4lek57msq7j4zwfqwjf7zxqjt4npxyv0lrgnsp8qy9j";
    const TOKEN_ADDR: &str =
        "juno1u235cpgju5vvlzp4w53vu0z5x3etytdpeh78ffekctfcmfc8ezhs9p248h";

    #[test]
    fn encode() {
        let target_addr = cosmos_addr_2_target_addr(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Cosmos(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let token_address = TOKEN_ADDR.to_string();
        let proposal = TokenRemoveProposal::new(header, token_address);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f04000000000400000000000000017b22746f6b656e223a226a756e6f31753233356370676a753576766c7a70347735337675307a357833657479746470656837386666656b637466636d666338657a6873397032343868222c226e6f6e6365223a317d"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f04000000000400000000000000017b22746f6b656e223a226a756e6f31753233356370676a753576766c7a70347735337675307a357833657479746470656837386666656b637466636d666338657a6873397032343868222c226e6f6e6365223a317d"
        );
        let proposal = TokenRemoveProposal::from(bytes.to_vec());
        let header = proposal.header();
        let resource_id = header.resource_id();
        let target_system = resource_id.target_system();
        let target_chain = resource_id.typed_chain_id();
        let function_signature = header.function_signature();
        let nonce = header.nonce();
        assert_eq!(
            target_system,
            TargetSystem::ContractAddress(cosmos_addr_2_target_addr(
                TARGET_CONTRACT_ADDR
            ))
        );
        assert_eq!(target_chain, TypedChainId::Cosmos(4));
        assert_eq!(
            function_signature,
            FunctionSignature::new(hex_literal::hex!("00000000"))
        );
        assert_eq!(nonce, Nonce::from(0x0001));
        assert_eq!(proposal.token_address(), TOKEN_ADDR.to_string(),);
    }
}
