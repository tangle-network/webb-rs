//! Wrapping Fee Update Proposal.
use crate::ProposalHeader;

/// Wrapping Fee Update Proposal.
///
/// The [`WrappingFeeUpdateProposal`] updates the wrapping fee percentage.
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct WrappingFeeUpdateProposal {
    header: ProposalHeader,
    wrapping_fee: u16,
}

impl WrappingFeeUpdateProposal {
    /// Creates a new wrapping fee update proposal.
    ///
    /// Wrapping fee is in the range of 0 to 10_000.
    ///
    /// **Note:** in debug mode, this may panic if the fee is out of range.
    #[must_use]
    pub const fn new(header: ProposalHeader, wrapping_fee: u16) -> Self {
        debug_assert!(wrapping_fee <= 10_000);
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
    /// Wrapping fees are in the range [0, 10_000].
    ///
    /// *Note*: In debug builds, this will panic if the wrapping fee is out of
    /// range.
    #[must_use]
    pub const fn wrapping_fee(&self) -> u16 {
        debug_assert!(self.wrapping_fee <= 10_000);
        self.wrapping_fee
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());

        let message = UpdateConfigMsg {
            governor: None,
            is_native_allowed: None,
            wrapping_limit: None,
            fee_percentage: Some(self.wrapping_fee),
            fee_recipient: None,
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

impl TryFrom<Vec<u8>> for WrappingFeeUpdateProposal {
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

        let message: UpdateConfigMsg =
            scale_codec::Decode::decode(&mut &bytes[40..])?;

        Ok(Self::new(header, message.fee_percentage.unwrap()))
    }
}

impl From<WrappingFeeUpdateProposal> for Vec<u8> {
    fn from(proposal: WrappingFeeUpdateProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct UpdateConfigMsg {
    pub governor: Option<[u8; 32]>,
    pub is_native_allowed: Option<bool>,
    pub wrapping_limit: Option<u128>,
    pub fee_percentage: Option<u16>,
    pub fee_recipient: Option<[u8; 32]>,
}

#[cfg(test)]
mod tests {
    use crate::ink::ink_address_to_target_address;
    use crate::{
        FunctionSignature, Nonce, ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    const TARGET_CONTRACT_ADDR: [u8; 32] = [0u8; 32];

    #[test]
    fn encode() {
        let target_addr = ink_address_to_target_address(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Ink(4);
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
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e5630600000000040000000000000001000000010100"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e5630600000000040000000000000001000000010100"
        );
        let proposal =
            WrappingFeeUpdateProposal::try_from(bytes.to_vec()).unwrap();
        let header = proposal.header();
        let target_system = TargetSystem::ContractAddress(
            ink_address_to_target_address(TARGET_CONTRACT_ADDR),
        );
        let target_chain = TypedChainId::Ink(4);
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
            ink_address_to_target_address(TARGET_CONTRACT_ADDR),
        );
        let target_chain = TypedChainId::Ink(4);
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
