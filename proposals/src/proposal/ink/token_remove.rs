//! Token Remove Proposal.
use crate::ProposalHeader;

/// Token Remove Proposal.
///
/// The [`TokenRemoveProposal`] allows the token specified by the `TokenAddress` to
/// be wrapped into the WEBB token.
///
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct TokenRemoveProposal {
    header: ProposalHeader,
    token_address: [u8; 32],
}

impl TokenRemoveProposal {
    /// Creates a new token add proposal.
    #[must_use]
    pub const fn new(header: ProposalHeader, address: [u8; 32]) -> Self {
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
    pub fn token_address(&self) -> [u8; 32] {
        self.token_address.clone()
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());

        let message = RemovePsp22TokenAddr {
            token: self.token_address.clone(),
            nonce: self.header.nonce().0 as u64,
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

impl TryFrom<Vec<u8>> for TokenRemoveProposal {
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

        let decoded_msg: RemovePsp22TokenAddr =
            scale_codec::Decode::decode(&mut &bytes[40..])?;

        Ok(Self::new(header, decoded_msg.token))
    }
}

impl From<TokenRemoveProposal> for Vec<u8> {
    fn from(proposal: TokenRemoveProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct RemovePsp22TokenAddr {
    token: [u8; 32],
    nonce: u64,
}

#[cfg(test)]
mod tests {
    use crate::ink::ink_address_to_target_address;
    use crate::{
        FunctionSignature, Nonce,
        ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    const TARGET_CONTRACT_ADDR: [u8; 32] = [0; 32];
    const TOKEN_ADDR: [u8; 32] = [1; 32];

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
        let proposal = TokenRemoveProposal::new(header, TOKEN_ADDR);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e563060000000004000000000000000101010101010101010101010101010101010101010101010101010101010101010100000000000000"
        );

        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e563060000000004000000000000000101010101010101010101010101010101010101010101010101010101010101010100000000000000"
        );
        let proposal = TokenRemoveProposal::try_from(bytes.to_vec()).unwrap();
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
            )),
        );
        assert_eq!(target_chain, TypedChainId::Ink(4));
        assert_eq!(
            function_signature,
            FunctionSignature::new(hex_literal::hex!("00000000"))
        );
        assert_eq!(nonce, Nonce::from(0x0001));
        assert_eq!(proposal.token_address(), TOKEN_ADDR);
    }
}
