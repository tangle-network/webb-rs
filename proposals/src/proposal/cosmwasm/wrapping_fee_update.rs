//! Wrapping Fee Update Proposal.
use crate::ProposalHeader;
use cosmwasm_std::{from_slice, to_binary, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Wrapping Fee Update Proposal.
///
/// The [`WrappingFeeUpdateProposal`] updates the wrapping fee percentage.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WrappingFeeUpdateProposal {
    header: ProposalHeader,
    wrapping_fee: u16,
}

impl WrappingFeeUpdateProposal {
    /// Creates a new wrapping fee update proposal.
    ///
    /// Wrapping fee is in the range of 0 to 10000.
    ///
    /// **Note:** in debug mode, this may panic if the fee is out of range.
    #[must_use]
    pub const fn new(header: ProposalHeader, wrapping_fee: u16) -> Self {
        debug_assert!(wrapping_fee <= 10000);
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
    /// Wrapping fees are in the range [0, 10000].
    ///
    /// *Note*: In debug builds, this will panic if the wrapping fee is out of
    /// range.
    #[must_use]
    pub const fn wrapping_fee(&self) -> u16 {
        debug_assert!(self.wrapping_fee <= 10000);
        self.wrapping_fee
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());

        let msg = to_binary(&UpdateConfigMsg {
            governor: None,
            is_native_allowed: None,
            wrapping_limit: None,
            fee_percentage: Some(self.wrapping_fee),
            fee_recipient: None,
        })
        .unwrap();
        bytes.extend_from_slice(msg.as_slice());

        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl From<Vec<u8>> for WrappingFeeUpdateProposal {
    fn from(bytes: Vec<u8>) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);

        let f = t;
        let message: UpdateConfigMsg = from_slice(&bytes[f..]).unwrap();

        Self::new(header, message.fee_percentage.unwrap())
    }
}

impl From<WrappingFeeUpdateProposal> for Vec<u8> {
    fn from(proposal: WrappingFeeUpdateProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
struct UpdateConfigMsg {
    pub governor: Option<String>,
    pub is_native_allowed: Option<bool>,
    pub wrapping_limit: Option<Uint128>,
    pub fee_percentage: Option<u16>,
    pub fee_recipient: Option<String>,
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

    #[test]
    fn encode() {
        let target_addr =
            cosmos_address_to_target_address(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Cosmos(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let wrapping_fee = 0x01;
        let proposal = WrappingFeeUpdateProposal::new(header, wrapping_fee);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f04000000000400000000000000017b22676f7665726e6f72223a6e756c6c2c2269735f6e61746976655f616c6c6f776564223a6e756c6c2c227772617070696e675f6c696d6974223a6e756c6c2c226665655f70657263656e74616765223a312c226665655f726563697069656e74223a6e756c6c7d"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f04000000000400000000000000017b22676f7665726e6f72223a6e756c6c2c2269735f6e61746976655f616c6c6f776564223a6e756c6c2c227772617070696e675f6c696d6974223a6e756c6c2c226665655f70657263656e74616765223a312c226665655f726563697069656e74223a6e756c6c7d"
        );
        let proposal = WrappingFeeUpdateProposal::from(bytes.to_vec());
        let header = proposal.header();
        let target_system = TargetSystem::ContractAddress(
            cosmos_address_to_target_address(TARGET_CONTRACT_ADDR),
        );
        let target_chain = TypedChainId::Cosmos(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
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
        let target_system = TargetSystem::ContractAddress(
            cosmos_address_to_target_address(TARGET_CONTRACT_ADDR),
        );
        let target_chain = TypedChainId::Cosmos(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let wrapping_fee = 101;
        let _ = WrappingFeeUpdateProposal::new(header, wrapping_fee);
    }
}
