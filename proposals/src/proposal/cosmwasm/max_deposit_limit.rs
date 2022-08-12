//! Maximum Deposit Limit Proposal.
use crate::ProposalHeader;
use cosmwasm_std::{from_slice, to_binary, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Maximum Deposit Limit Proposal.
///
/// The [`MaxDepositLimitProposal`] updates the maximum deposit amount allowed
/// on the variable anchor system.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MaxDepositLimitProposal {
    header: ProposalHeader,
    max_deposit_limit: [u8; 32],
}

impl MaxDepositLimitProposal {
    /// Creates a new max deposit limit proposal.
    #[must_use]
    pub const fn new(header: ProposalHeader, max_limit: [u8; 32]) -> Self {
        Self {
            header,
            max_deposit_limit: max_limit,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the min withdrawal limit.
    #[must_use]
    pub const fn max_deposit_limit(&self) -> [u8; 32] {
        self.max_deposit_limit
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());

        let mut deposit_limit_bytes = [0u8; 16];
        deposit_limit_bytes
            .copy_from_slice(self.max_deposit_limit.split_at(16).1);
        let message = to_binary(&ConfigureMaximumDepositLimit {
            maximum_deposit_amount: Uint128::from(u128::from_be_bytes(
                deposit_limit_bytes,
            )),
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

impl From<Vec<u8>> for MaxDepositLimitProposal {
    fn from(bytes: Vec<u8>) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);

        let mut max_deposit_limit = [0u8; 32];
        let f = t;
        let msg_bytes = &bytes[f..];
        let decoded_msg: ConfigureMaximumDepositLimit =
            from_slice(&msg_bytes).unwrap();
        max_deposit_limit[16..].copy_from_slice(
            &decoded_msg.maximum_deposit_amount.u128().to_be_bytes(),
        );

        Self::new(header, max_deposit_limit)
    }
}

impl From<MaxDepositLimitProposal> for Vec<u8> {
    fn from(proposal: MaxDepositLimitProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
struct ConfigureMaximumDepositLimit {
    maximum_deposit_amount: Uint128,
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
        let max_deposit_limit = hex_literal::hex!(
            "00000000000000000000000000000000101112131415161718191a1b1c1d1e1f"
        );
        let proposal = MaxDepositLimitProposal::new(header, max_deposit_limit);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f04000000000400000000000000017b226d6178696d756d5f6465706f7369745f616d6f756e74223a223231333536323833353734303736383931343933393438393639393739363835343435313531227d"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!("000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f04000000000400000000000000017b226d6178696d756d5f6465706f7369745f616d6f756e74223a223231333536323833353734303736383931343933393438393639393739363835343435313531227d");
        let proposal = MaxDepositLimitProposal::from(bytes.to_vec());
        let header = proposal.header();
        let resource_id = header.resource_id();
        let target_system = resource_id.target_system();
        let target_chain = resource_id.typed_chain_id();
        let function_signature = header.function_signature();
        let nonce = header.nonce();
        let max_deposit_limit = proposal.max_deposit_limit();
        assert_eq!(
            target_system,
            TargetSystem::ContractAddress(cosmos_address_to_target_address(
                TARGET_CONTRACT_ADDR
            ))
        );
        assert_eq!(target_chain, TypedChainId::Cosmos(4));
        assert_eq!(
            function_signature,
            FunctionSignature::new(hex_literal::hex!("00000000"))
        );
        assert_eq!(nonce, Nonce::from(0x0001));
        assert_eq!(
            max_deposit_limit,
            hex_literal::hex!(
                "00000000000000000000000000000000101112131415161718191a1b1c1d1e1f"
            )
        );
    }
}
