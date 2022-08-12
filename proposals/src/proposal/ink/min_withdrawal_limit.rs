//! Minimum Withdrawal Limit Proposal.
use crate::ProposalHeader;

/// Minimum Withdrawal Limit Proposal.
///
/// The [`MinWithdrawalLimitProposal`] updates the minimum withdrawal amount
/// allowed on the variable anchor system.
///
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct MinWithdrawalLimitProposal {
    header: ProposalHeader,
    min_withdrawal_limit: [u8; 32],
}

impl MinWithdrawalLimitProposal {
    /// Creates a new min withdrawal limit proposal.
    #[must_use]
    pub const fn new(header: ProposalHeader, min_limit: [u8; 32]) -> Self {
        Self {
            header,
            min_withdrawal_limit: min_limit,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the min withdrawal limit.
    #[must_use]
    pub const fn min_withdrawal_limit(&self) -> [u8; 32] {
        self.min_withdrawal_limit
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());

        let mut deposit_limit_bytes = [0u8; 16];
        deposit_limit_bytes
            .copy_from_slice(self.min_withdrawal_limit.split_at(16).1);
        let message = ConfigureMinimalWithdrawalLimit {
            minimal_withdrawal_amount: u128::from_be_bytes(deposit_limit_bytes),
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

impl TryFrom<Vec<u8>> for MinWithdrawalLimitProposal {
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

        let mut max_deposit_limit = [0u8; 32];

        let decoded_msg: ConfigureMinimalWithdrawalLimit =
            scale_codec::Decode::decode(&mut &bytes[40..])?;

        max_deposit_limit[16..].copy_from_slice(
            &decoded_msg.minimal_withdrawal_amount.to_be_bytes(),
        );

        Ok(Self::new(header, max_deposit_limit))
    }
}

impl From<MinWithdrawalLimitProposal> for Vec<u8> {
    fn from(proposal: MinWithdrawalLimitProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ConfigureMinimalWithdrawalLimit {
    minimal_withdrawal_amount: u128,
}

#[cfg(test)]
mod tests {
    use crate::ink::ink_address_to_target_address;
    use crate::{
        FunctionSignature, Nonce,
        ResourceId, TargetSystem, TypedChainId,
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
        let min_withdrawal_limit = hex_literal::hex!(
            "00000000000000000000000000000000101112131415161718191a1b1c1d1e1f"
        );
        let proposal =
            MinWithdrawalLimitProposal::new(header, min_withdrawal_limit);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e56306000000000400000000000000011f1e1d1c1b1a19181716151413121110"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e56306000000000400000000000000011f1e1d1c1b1a19181716151413121110"
        );
        let proposal =
            MinWithdrawalLimitProposal::try_from(bytes.to_vec()).unwrap();
        let header = proposal.header();
        let resource_id = header.resource_id();
        let target_system = resource_id.target_system();
        let target_chain = resource_id.typed_chain_id();
        let function_signature = header.function_signature();
        let nonce = header.nonce();
        let min_withdrawal_limit = proposal.min_withdrawal_limit();
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
        assert_eq!(
            min_withdrawal_limit,
            hex_literal::hex!(
                "00000000000000000000000000000000101112131415161718191a1b1c1d1e1f"
            )
        );
    }
}
