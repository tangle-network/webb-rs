//! Anchor Update Proposal.
use cosmwasm_std::{from_slice, to_binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ProposalHeader, ResourceId, TypedChainId};

/// Anchor Update Proposal.
///
/// The [`AnchorUpdateProposal`] updates the target Anchor's knowledge of the
/// source Anchor's Merkle roots. This knowledge is necessary to prove
/// membership in the source Anchor's Merkle tree on the target chain.
///
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AnchorUpdateProposal {
    header: ProposalHeader,
    merkle_root: [u8; 32],
    src_resource_id: ResourceId,
}

impl AnchorUpdateProposal {
    /// Creates a new anchor update proposal.
    #[must_use]
    pub const fn new(
        header: ProposalHeader,
        merkle_root: [u8; 32],
        src_resource_id: ResourceId,
    ) -> Self {
        Self {
            header,
            merkle_root,
            src_resource_id,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the source chain.
    #[must_use]
    pub fn src_chain(&self) -> TypedChainId {
        self.src_resource_id.typed_chain_id()
    }

    /// Get the src_resource_id identifier.
    #[must_use]
    pub const fn src_resource_id(&self) -> ResourceId {
        self.src_resource_id
    }

    /// Get the latest leaf index.
    #[must_use]
    pub const fn latest_leaf_index(&self) -> u32 {
        self.header.nonce.to_u32()
    }

    /// Get the merkle root.
    #[must_use]
    pub const fn merkle_root(&self) -> &[u8; 32] {
        &self.merkle_root
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend_from_slice(&self.header.to_bytes());

        let message = to_binary(&UpdateEdge {
            root: self.merkle_root,
            src_resource_id: self.src_resource_id.to_bytes(),
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

impl From<Vec<u8>> for AnchorUpdateProposal {
    fn from(bytes: Vec<u8>) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);

        let f = t;
        let msg_bytes = bytes[f..].to_vec();
        let decoded_edge_data: UpdateEdge = from_slice(&msg_bytes).unwrap();

        Self::new(
            header,
            decoded_edge_data.root,
            ResourceId(decoded_edge_data.src_resource_id),
        )
    }
}

impl From<AnchorUpdateProposal> for Vec<u8> {
    fn from(proposal: AnchorUpdateProposal) -> Self {
        proposal.to_bytes()
    }
}

// if we have EVM available, we can convert the EVM proposal to a cosmwasm proposal
#[cfg(feature = "evm")]
impl From<crate::evm::AnchorUpdateProposal> for AnchorUpdateProposal {
    fn from(proposal: crate::evm::AnchorUpdateProposal) -> Self {
        AnchorUpdateProposal::new(
            proposal.header(),
            *proposal.merkle_root(),
            proposal.src_resource_id(),
        )
    }
}

// if we have Substrate available, we can convert the EVM proposal to a cosmwasm proposal
#[cfg(feature = "substrate")]
impl From<crate::substrate::AnchorUpdateProposal> for AnchorUpdateProposal {
    fn from(proposal: crate::substrate::AnchorUpdateProposal) -> Self {
        AnchorUpdateProposal::new(
            proposal.header(),
            *proposal.merkle_root(),
            proposal.src_resource_id(),
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
struct UpdateEdge {
    root: [u8; 32],
    src_resource_id: [u8; 32],
}

#[cfg(test)]
mod tests {
    use crate::{
        cosmwasm::cosmos_address_to_target_address, FunctionSignature, Nonce,
        ResourceId, TargetSystem,
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
        let latest_leaf_index = 0x0001;
        let nonce = Nonce::from(latest_leaf_index);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let src_chain = TypedChainId::Cosmos(1);
        let src_resource_id = ResourceId::new(target_system, src_chain);
        let merkle_root = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let proposal =
            AnchorUpdateProposal::new(header, merkle_root, src_resource_id);
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f04000000000400000000000000017b22726f6f74223a5b302c312c322c332c342c352c362c372c382c392c31302c31312c31322c31332c31342c31352c31362c31372c31382c31392c32302c32312c32322c32332c32342c32352c32362c32372c32382c32392c33302c33315d2c227372635f7265736f757263655f6964223a5b302c302c302c302c302c302c3137392c3131352c3133312c3136322c3137332c34352c3233332c3233302c3134312c3136372c39352c38382c36322c3132352c31342c3234322c3233342c3232352c32342c37392c342c302c302c302c302c315d7d"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!("000000000000b37383a2ad2de9e68da75f583e7d0ef2eae1184f04000000000400000000000000017b22726f6f74223a5b302c312c322c332c342c352c362c372c382c392c31302c31312c31322c31332c31342c31352c31362c31372c31382c31392c32302c32312c32322c32332c32342c32352c32362c32372c32382c32392c33302c33315d2c227372635f7265736f757263655f6964223a5b302c302c302c302c302c302c3137392c3131352c3133312c3136322c3137332c34352c3233332c3233302c3134312c3136372c39352c38382c36322c3132352c31342c3234322c3233342c3232352c32342c37392c342c302c302c302c302c315d7d");
        let proposal = AnchorUpdateProposal::from(bytes.to_vec());
        let target_addr =
            cosmos_address_to_target_address(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Cosmos(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let latest_leaf_index = 0x0001;
        let nonce = Nonce::from(latest_leaf_index);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let src_chain_id = TypedChainId::Cosmos(1);
        let src_resource_id = ResourceId::new(target_system, src_chain_id);
        let merkle_root = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let expected =
            AnchorUpdateProposal::new(header, merkle_root, src_resource_id);
        assert_eq!(proposal, expected);
    }
}
