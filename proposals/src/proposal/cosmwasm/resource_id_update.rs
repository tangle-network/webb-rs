//! Resource Id Update Proposal.
use crate::{ProposalHeader, ResourceId};
use cosmwasm_std::{from_slice, to_binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Resource Id Update Proposal.
///
/// The [`ResourceIdUpdateProposal`] maps a new resource Id to a handler
/// address.
///
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceIdUpdateProposal {
    header: ProposalHeader,
    new_resource_id: ResourceId,
    handler_address: String,
    execution_address: String,
}

impl ResourceIdUpdateProposal {
    /// Creates a new resource id update proposal.
    #[must_use]
    pub const fn new(
        header: ProposalHeader,
        new_resource_id: ResourceId,
        handler_address: String,
        execution_address: String,
    ) -> Self {
        Self {
            header,
            new_resource_id,
            handler_address,
            execution_address,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the new resource id.
    #[must_use]
    pub const fn new_resource_id(&self) -> ResourceId {
        self.new_resource_id
    }

    /// Get the handler address.
    #[must_use]
    pub fn handler_address(&self) -> String {
        self.handler_address.clone()
    }

    /// Get the execution address.
    #[must_use]
    pub fn execution_address(&self) -> String {
        self.execution_address.clone()
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.header.to_bytes());

        let message = to_binary(&ResourceIdUpdateData {
            resource_id: self.header.resource_id().to_bytes(),
            function_sig: self.header.function_signature().to_bytes(),
            nonce: self.header.nonce().0,
            new_resource_id: self.new_resource_id.to_bytes(),
            handler_addr: self.handler_address.clone(),
            execution_context_addr: self.execution_address.clone(),
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

impl From<Vec<u8>> for ResourceIdUpdateProposal {
    fn from(bytes: Vec<u8>) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);

        let f = t;
        let decoded_msg: ResourceIdUpdateData =
            from_slice(&bytes[f..]).unwrap();

        Self {
            header,
            new_resource_id: decoded_msg.new_resource_id.into(),
            handler_address: decoded_msg.handler_addr,
            execution_address: decoded_msg.execution_context_addr,
        }
    }
}

impl From<ResourceIdUpdateProposal> for Vec<u8> {
    fn from(proposal: ResourceIdUpdateProposal) -> Self {
        proposal.to_bytes()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
struct ResourceIdUpdateData {
    pub resource_id: [u8; 32],
    pub function_sig: [u8; 4],
    pub nonce: u32,
    pub new_resource_id: [u8; 32],
    pub handler_addr: String,
    pub execution_context_addr: String,
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
    const NEW_TARGET_CONTRACT_ADDR: &str =
        "juno18edfd7nquv9zfgprt5lpzcykczw0llwqcc40f5lkxq2vdw7nnp7qr7a22a";
    const HANDLER_ADDR: &str =
        "juno1u235cpgju5vvlzp4w53vu0z5x3etytdpeh78ffekctfcmfc8ezhs9p248h";
    const EXECUTION_CONTRACT_ADDR: &str =
        "juno1afxj87jjd4usd80gsprtq76uykv02egaydwvj62ldhngzj2zdamqxn9an3";

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
        let new_target_addr =
            cosmos_addr_2_target_addr(NEW_TARGET_CONTRACT_ADDR);
        let new_target_system = TargetSystem::ContractAddress(new_target_addr);
        let new_resource_id = ResourceId::new(new_target_system, target_chain);
        let handler_address = HANDLER_ADDR.to_string();
        let execution_address = EXECUTION_CONTRACT_ADDR.to_string();
        let proposal = ResourceIdUpdateProposal::new(
            header,
            new_resource_id,
            handler_address,
            execution_address,
        );
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f04000000000400000000000000017b227265736f757263655f6964223a5b302c302c302c302c302c302c3137392c3131352c3133312c3136322c3137332c34352c3233332c3233302c3134312c3136372c39352c38382c36322c3132352c31342c3234322c3233342c3232352c32342c37392c342c302c302c302c302c345d2c2266756e6374696f6e5f736967223a5b302c302c302c305d2c226e6f6e6365223a312c226e65775f7265736f757263655f6964223a5b302c302c302c302c302c302c39382c3139332c3130352c3130362c3134392c3138312c33392c3134362c32332c3133392c3233352c3233382c34312c33312c3132372c39372c3135322c31372c39372c3136332c342c302c302c302c302c345d2c2268616e646c65725f61646472223a226a756e6f31753233356370676a753576766c7a70347735337675307a357833657479746470656837386666656b637466636d666338657a6873397032343868222c22657865637574696f6e5f636f6e746578745f61646472223a226a756e6f316166786a38376a6a64347573643830677370727471373675796b763032656761796477766a36326c64686e677a6a327a64616d71786e39616e33227d"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        // do the reverse of encode
        let bytes = hex_literal::hex!(
            "000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f04000000000400000000000000017b227265736f757263655f6964223a5b302c302c302c302c302c302c3137392c3131352c3133312c3136322c3137332c34352c3233332c3233302c3134312c3136372c39352c38382c36322c3132352c31342c3234322c3233342c3232352c32342c37392c342c302c302c302c302c345d2c2266756e6374696f6e5f736967223a5b302c302c302c305d2c226e6f6e6365223a312c226e65775f7265736f757263655f6964223a5b302c302c302c302c302c302c39382c3139332c3130352c3130362c3134392c3138312c33392c3134362c32332c3133392c3233352c3233382c34312c33312c3132372c39372c3135322c31372c39372c3136332c342c302c302c302c302c345d2c2268616e646c65725f61646472223a226a756e6f31753233356370676a753576766c7a70347735337675307a357833657479746470656837386666656b637466636d666338657a6873397032343868222c22657865637574696f6e5f636f6e746578745f61646472223a226a756e6f316166786a38376a6a64347573643830677370727471373675796b763032656761796477766a36326c64686e677a6a327a64616d71786e39616e33227d"
        );
        let proposal = ResourceIdUpdateProposal::from(bytes.to_vec());
        let target_addr = cosmos_addr_2_target_addr(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Cosmos(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let new_target_addr =
            cosmos_addr_2_target_addr(NEW_TARGET_CONTRACT_ADDR);
        let new_target_system = TargetSystem::ContractAddress(new_target_addr);
        let new_resource_id = ResourceId::new(new_target_system, target_chain);
        let handler_address = HANDLER_ADDR.to_string();
        let execution_address = EXECUTION_CONTRACT_ADDR.to_string();
        let expected_proposal = ResourceIdUpdateProposal::new(
            header,
            new_resource_id,
            handler_address,
            execution_address,
        );
        assert_eq!(proposal, expected_proposal);
    }
}
