#![allow(clippy::exhaustive_enums)]
//! Rescue Tokens Proposal.
use crate::ProposalHeader;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
/// Rescue Tokens Proposal.
///
/// The `RescueTokensProposal` rescues tokens from the treasury to a specified
/// `to` address.
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct RescueTokensProposal {
    header: ProposalHeader,
    token_address: [u8; 32],
    recipient: [u8; 32],
    amount: [u8; 32],
}

impl RescueTokensProposal {
    /// Creates a new resource id update proposal.
    #[must_use]
    pub const fn new(
        header: ProposalHeader,
        token_address: [u8; 32],
        recipient: [u8; 32],
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
    pub fn token_address(&self) -> [u8; 32] {
        self.token_address
    }

    /// Get the to token address.
    #[must_use]
    pub fn recipient(&self) -> [u8; 32] {
        self.recipient
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(ProposalHeader::LENGTH + 104);
        bytes.extend_from_slice(&self.header.to_bytes());

        let mut rescue_amt_bytes = [0u8; 16];
        rescue_amt_bytes.copy_from_slice(self.amount.split_at(16).1);
        let message = RescueTokens {
            token_address: self.token_address,
            to: self.recipient,
            amount_to_rescue: u128::from_be_bytes(rescue_amt_bytes),
            nonce: self.header.nonce().0,
        };
        scale_codec::Encode::encode_to(&message, &mut bytes);

        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl TryFrom<Vec<u8>> for RescueTokensProposal {
    type Error = scale_codec::Error;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        let parsed_header =
            bytes.get(0..ProposalHeader::LENGTH).ok_or_else(|| {
                scale_codec::Error::from(
                    "invaid proposal: invalid length of proposal",
                )
            })?;

        header_bytes.copy_from_slice(parsed_header);
        let header = ProposalHeader::from(header_bytes);

        let decoded_message: RescueTokens =
            scale_codec::Decode::decode(&mut &bytes[40..])?;
        let token_address = decoded_message.token_address;
        let recipient = decoded_message.to;
        let mut amount = [0u8; 32];
        amount[16..]
            .copy_from_slice(&decoded_message.amount_to_rescue.to_be_bytes());

        Ok(Self {
            header,
            token_address,
            recipient,
            amount,
        })
    }
}

impl From<RescueTokensProposal> for Vec<u8> {
    fn from(proposal: RescueTokensProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct RescueTokens {
    token_address: [u8; 32],
    to: [u8; 32],
    amount_to_rescue: u128,
    nonce: u32,
}

#[cfg(test)]
mod tests {
    use crate::ink::ink_address_to_target_address;
    use crate::{
        FunctionSignature, Nonce, ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    const TARGET_CONTRACT_ADDR: [u8; 32] = [0u8; 32];
    const TOKEN_ADDR: [u8; 32] = [1u8; 32];
    const RECIPIENT: [u8; 32] = [2u8; 32];

    #[test]
    fn encode() {
        let target_addr = ink_address_to_target_address(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let amount = hex_literal::hex!(
            "000000000000000000000000000000000000000000000000000000000000000f"
        );
        let proposal =
            RescueTokensProposal::new(header, TOKEN_ADDR, RECIPIENT, amount);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e5630100000000040000000000000001010101010101010101010101010101010101010101010101010101010101010102020202020202020202020202020202020202020202020202020202020202020f00000000000000000000000000000001000000"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        // the reverse of encode
        let bytes = hex_literal::hex!(
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e5630100000000040000000000000001010101010101010101010101010101010101010101010101010101010101010102020202020202020202020202020202020202020202020202020202020202020f00000000000000000000000000000001000000"
        );
        let expected_proposal =
            RescueTokensProposal::try_from(bytes.to_vec()).unwrap();
        let target_addr = ink_address_to_target_address(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let amount = hex_literal::hex!(
            "000000000000000000000000000000000000000000000000000000000000000f"
        );
        let proposal =
            RescueTokensProposal::new(header, TOKEN_ADDR, RECIPIENT, amount);
        assert_eq!(proposal, expected_proposal);
    }
}
