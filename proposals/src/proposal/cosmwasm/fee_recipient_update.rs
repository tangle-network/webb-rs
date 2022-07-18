//! Fee Recipient Update Proposal.
use crate::ProposalHeader;
use cosmwasm_std::{from_slice, to_binary, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Fee Recipient Update Proposal.
///
/// The [`FeeRecipientUpdateProposal`] updates the recipient of the wrapping
/// fees.
///
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeeRecipientUpdateProposal {
    header: ProposalHeader,
    fee_recipient_address: String,
}

impl FeeRecipientUpdateProposal {
    /// Creates a new fee recipient update proposal.
    #[must_use]
    pub const fn new(header: ProposalHeader, address: String) -> Self {
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
    pub fn fee_recipient_address(&self) -> String {
        self.fee_recipient_address.clone()
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes().to_vec();

        let message = to_binary(&UpdateConfigMsg {
            governor: None,
            is_native_allowed: None,
            wrapping_limit: None,
            fee_percentage: None,
            fee_recipient: Some(self.fee_recipient_address()),
        })
        .unwrap();
        bytes.extend_from_slice(&message.to_vec());

        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl From<Vec<u8>> for FeeRecipientUpdateProposal {
    fn from(bytes: Vec<u8>) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);

        let f = t;
        let msg_bytes = &bytes[f..];
        let decoded_msg: UpdateConfigMsg = from_slice(&msg_bytes).unwrap();

        Self::new(header, decoded_msg.fee_recipient.unwrap_or("".to_string()))
    }
}

impl From<FeeRecipientUpdateProposal> for Vec<u8> {
    fn from(proposal: FeeRecipientUpdateProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
struct UpdateConfigMsg {
    pub governor: Option<String>,
    pub is_native_allowed: Option<bool>,
    pub wrapping_limit: Option<Uint128>,
    pub fee_percentage: Option<u8>,
    pub fee_recipient: Option<String>,
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
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let new_fee_recipient_address =
            "juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8"
                .to_string();
        let proposal =
            FeeRecipientUpdateProposal::new(header, new_fee_recipient_address);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa04000000000400000000000000017b22676f7665726e6f72223a6e756c6c2c2269735f6e61746976655f616c6c6f776564223a6e756c6c2c227772617070696e675f6c696d6974223a6e756c6c2c226665655f70657263656e74616765223a6e756c6c2c226665655f726563697069656e74223a226a756e6f3134686a32746176713866706573647778786375343472747933686839307668756a7276636d73746c347a723374786d66767739736b6a75776738227d"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!("000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa04000000000400000000000000017b22676f7665726e6f72223a6e756c6c2c2269735f6e61746976655f616c6c6f776564223a6e756c6c2c227772617070696e675f6c696d6974223a6e756c6c2c226665655f70657263656e74616765223a6e756c6c2c226665655f726563697069656e74223a226a756e6f3134686a32746176713866706573647778786375343472747933686839307668756a7276636d73746c347a723374786d66767739736b6a75776738227d");
        let proposal = FeeRecipientUpdateProposal::from(bytes.to_vec());
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
            FunctionSignature::new(hex_literal::hex!("00000000"))
        );
        assert_eq!(nonce, Nonce::from(0x0001));
        assert_eq!(
            proposal.fee_recipient_address(),
            "juno14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9skjuwg8"
                .to_string(),
        );
    }
}
