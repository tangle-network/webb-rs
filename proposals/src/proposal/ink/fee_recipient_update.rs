//! Fee Recipient Update Proposal.
use crate::ProposalHeader;

/// Fee Recipient Update Proposal.
///
/// The [`FeeRecipientUpdateProposal`] updates the recipient of the wrapping
/// fees.
///
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct FeeRecipientUpdateProposal {
    header: ProposalHeader,
    fee_recipient_address: [u8; 32],
}

impl FeeRecipientUpdateProposal {
    /// Creates a new fee recipient update proposal.
    #[must_use]
    pub const fn new(header: ProposalHeader, address: [u8; 32]) -> Self {
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
    pub fn fee_recipient_address(&self) -> [u8; 32] {
        self.fee_recipient_address.clone()
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
            fee_percentage: None,
            fee_recipient: Some(self.fee_recipient_address()),
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

impl TryFrom<Vec<u8>> for FeeRecipientUpdateProposal {
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

        // parse encoded proposal call
        let decoded_msg: UpdateConfigMsg =
            scale_codec::Decode::decode(&mut &bytes[40..])?;

        Ok(Self::new(header, decoded_msg.fee_recipient.unwrap()))
    }
}

impl From<FeeRecipientUpdateProposal> for Vec<u8> {
    fn from(proposal: FeeRecipientUpdateProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct UpdateConfigMsg {
    pub governor: Option<[u8; 32]>,
    pub is_native_allowed: Option<bool>,
    pub wrapping_limit: Option<u128>,
    pub fee_percentage: Option<u8>,
    pub fee_recipient: Option<[u8; 32]>,
}

#[cfg(test)]
mod tests {
    use crate::{
        ink::ink_address_to_target_address, FunctionSignature, Nonce,
        ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    const TARGET_CONTRACT_ADDR: [u8; 32] = [0u8; 32];
    const NEW_FEE_RECP_ADDR: [u8; 32] = [0u8; 32];

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
        let proposal =
            FeeRecipientUpdateProposal::new(header, NEW_FEE_RECP_ADDR);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e563060000000004000000000000000100000000010000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!("00000000000088386fc84ba6bc95484008f6362f93160ef3e563060000000004000000000000000100000000010000000000000000000000000000000000000000000000000000000000000000");
        let proposal =
            FeeRecipientUpdateProposal::try_from(bytes.to_vec()).unwrap();
        let header = proposal.header();
        let resource_id = header.resource_id();
        let target_system = resource_id.target_system();
        let target_chain = resource_id.typed_chain_id();
        let function_signature = header.function_signature();
        let nonce = header.nonce();
        assert_eq!(
            target_system,
            TargetSystem::ContractAddress(ink_address_to_target_address(
                TARGET_CONTRACT_ADDR
            ))
        );
        assert_eq!(target_chain, TypedChainId::Ink(4));
        assert_eq!(
            function_signature,
            FunctionSignature::new(hex_literal::hex!("00000000"))
        );
        assert_eq!(nonce, Nonce::from(0x0001));
        assert_eq!(proposal.fee_recipient_address(), NEW_FEE_RECP_ADDR,);
    }
}
