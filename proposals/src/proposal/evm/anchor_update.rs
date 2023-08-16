//! Anchor Update Proposal.
use crate::{ProposalHeader, ResourceId, TypedChainId};
use proposal_derive::Proposal;

/// Anchor Update Proposal.
///
/// The [`AnchorUpdateProposal`] updates the target Anchor's knowledge of the
/// source Anchor's Merkle roots. This knowledge is necessary to prove
/// membership in the source Anchor's Merkle tree on the target chain.
///
/// The format of the proposal is:
/// ```text
/// ┌────────────────────┬─────────────────┬─────────────────────┐
/// │                    │                 │                     │
/// │ ProposalHeader 40B │  MerkleRoot 32B │ Src Resource ID 32B │
/// │                    │                 │                     │
/// └────────────────────┴─────────────────┴─────────────────────┘
/// ```
#[derive(Proposal, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[proposal(
    function_sig = "function updateEdge(uint256 root, uint32 latestLeafIndex, bytes32 srcResourceID)"
)]
pub struct AnchorUpdateProposal {
    header: ProposalHeader,
    merkle_root: [u8; 32],
    src_resource_id: ResourceId,
}

impl AnchorUpdateProposal {
    /// Get the source chain.
    #[must_use]
    pub fn src_chain(&self) -> TypedChainId {
        self.src_resource_id.typed_chain_id()
    }

    /// Get the latest leaf index.
    #[must_use]
    pub const fn latest_leaf_index(&self) -> u32 {
        self.header.nonce.to_u32()
    }
}

#[cfg(test)]
mod tests {
    use crate::{FunctionSignature, Nonce, ResourceId, TargetSystem};

    use super::*;

    #[test]
    fn encode() {
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let latest_leaf_index = 0x0001;
        let header = ProposalHeader::new(
            resource_id,
            function_signature,
            Nonce::from(latest_leaf_index),
        );
        let merkle_root = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let src_chain_id = TypedChainId::Evm(1);
        let src_resource_id = ResourceId::new(target_system, src_chain_id);
        let proposal =
            AnchorUpdateProposal::new(header, merkle_root, src_resource_id);
        let bytes = crate::to_vec(&proposal).unwrap();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe00000001000102030405060708090a0b0c0d"
            "0e0f101112131415161718191a1b1c1d1e1f"
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000001"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe00000001000102030405060708090a0b0c0d"
            "0e0f101112131415161718191a1b1c1d1e1f"
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000001"
        );
        let proposal =
            crate::from_slice::<AnchorUpdateProposal>(&bytes).unwrap();
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let merkle_root = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let src_chain = TypedChainId::Evm(1);
        let src_resource_id = ResourceId::new(target_system, src_chain);
        let expected =
            AnchorUpdateProposal::new(header, merkle_root, src_resource_id);
        assert_eq!(proposal, expected);
    }
}
