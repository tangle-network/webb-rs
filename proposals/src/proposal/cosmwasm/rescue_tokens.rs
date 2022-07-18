//! Rescue Tokens Proposal.
use crate::ProposalHeader;
use cosmwasm_std::{from_slice, to_binary, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Rescue Tokens Proposal.
///
/// The `RescueTokensProposal` rescues tokens from the treasury to a specified
/// `to` address.
///
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RescueTokensProposal {
    header: ProposalHeader,
    token_address: String,
    recipient: String,
    amount: [u8; 32],
}

impl RescueTokensProposal {
    /// Creates a new resource id update proposal.
    #[must_use]
    pub const fn new(
        header: ProposalHeader,
        token_address: String,
        recipient: String,
        amount: [u8; 32],
    ) -> Self {
        Self {
            header,
            token_address,
            recipient,
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
    pub fn token_address(&self) -> String {
        self.token_address.clone()
    }

    /// Get the to token address.
    #[must_use]
    pub fn recipient(&self) -> String {
        self.recipient.clone()
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());

        let mut rescue_amt_bytes = [0u8; 16];
        rescue_amt_bytes.copy_from_slice(self.amount.split_at(16).1);
        let message = to_binary(&RescueTokens {
            token_address: self.token_address.clone(),
            to: self.recipient.clone(),
            amount_to_rescue: Uint128::from(u128::from_be_bytes(
                rescue_amt_bytes,
            )),
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

impl From<Vec<u8>> for RescueTokensProposal {
    fn from(bytes: Vec<u8>) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);

        let f = t;
        let decoded_message: RescueTokens = from_slice(&bytes[f..]).unwrap();
        let token_address = decoded_message.token_address;
        let recipient = decoded_message.to;
        let mut amount = [0u8; 32];
        amount[16..].copy_from_slice(
            &decoded_message.amount_to_rescue.u128().to_be_bytes(),
        );

        Self {
            header,
            token_address,
            recipient,
            amount,
        }
    }
}

impl From<RescueTokensProposal> for Vec<u8> {
    fn from(proposal: RescueTokensProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
struct RescueTokens {
    token_address: String,
    to: String,
    amount_to_rescue: Uint128,
    nonce: u32,
}

#[cfg(test)]
mod tests {
    use crate::{
        cosmwasm::cosmos_address_to_target_address, FunctionSignature, Nonce,
        ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    const TARGET_CONTRACT_ADDR: &str =
        "juno1hset4pny4h8xm4s4lek57msq7j4zwfqwjf7zxqjt4npxyv0lrgnsp8qy9j";
    const TOKEN_ADDR: &str =
        "juno1u235cpgju5vvlzp4w53vu0z5x3etytdpeh78ffekctfcmfc8ezhs9p248h";
    const RECIPIENT: &str =
        "juno1afxj87jjd4usd80gsprtq76uykv02egaydwvj62ldhngzj2zdamqxn9an3";

    #[test]
    fn encode() {
        let target_addr =
            cosmos_address_to_target_address(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let token_address = TOKEN_ADDR.to_string();
        let recipient = RECIPIENT.to_string();
        let amount = hex_literal::hex!(
            "000000000000000000000000000000000000000000000000000000000000000f"
        );
        let proposal =
            RescueTokensProposal::new(header, token_address, recipient, amount);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f01000000000400000000000000017b22746f6b656e5f61646472657373223a226a756e6f31753233356370676a753576766c7a70347735337675307a357833657479746470656837386666656b637466636d666338657a6873397032343868222c22746f223a226a756e6f316166786a38376a6a64347573643830677370727471373675796b763032656761796477766a36326c64686e677a6a327a64616d71786e39616e33222c22616d6f756e745f746f5f726573637565223a223135222c226e6f6e6365223a317d"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        // the reverse of encode
        let bytes = hex_literal::hex!(
            "000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f01000000000400000000000000017b22746f6b656e5f61646472657373223a226a756e6f31753233356370676a753576766c7a70347735337675307a357833657479746470656837386666656b637466636d666338657a6873397032343868222c22746f223a226a756e6f316166786a38376a6a64347573643830677370727471373675796b763032656761796477766a36326c64686e677a6a327a64616d71786e39616e33222c22616d6f756e745f746f5f726573637565223a223135222c226e6f6e6365223a317d"
        );
        let expected_proposal = RescueTokensProposal::from(bytes.to_vec());
        let target_addr =
            cosmos_address_to_target_address(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let token_address = TOKEN_ADDR.to_string();
        let recipient = RECIPIENT.to_string();
        let amount = hex_literal::hex!(
            "000000000000000000000000000000000000000000000000000000000000000f"
        );
        let proposal =
            RescueTokensProposal::new(header, token_address, recipient, amount);
        assert_eq!(proposal, expected_proposal);
    }
}
