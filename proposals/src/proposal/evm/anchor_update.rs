//! Anchor Update Proposal.
use crate::{ProposalHeader, TypedChainId};

/// Anchor Update Proposal.
///
/// The [`AnchorUpdateProposal`] updates the target Anchor's knowledge of the
/// source Anchor's Merkle roots. This knowledge is necessary to prove
/// membership in the source Anchor's Merkle tree on the target chain.
///
/// The format of the proposal is:
/// ```text
/// ┌────────────────────┬─────────────────┬───────────────┬────────────────────┬────────────────┬───────────────┐
/// │                    │                 │               │                    │                │               │
/// │ ProposalHeader 40B │ SrcChainType 2B │ SrcChainId 4B │ LatestLeafIndex 4B │ MerkleRoot 32B │ Target ID 32B │
/// │                    │                 │               │                    │                │               │
/// └────────────────────┴─────────────────┴───────────────┴────────────────────┴────────────────┴───────────────┘
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AnchorUpdateProposal {
    header: ProposalHeader,
    merkle_root: [u8; 32],
    target: [u8; 32],
}

impl AnchorUpdateProposal {
    /// Length of the proposal in bytes.
    pub const LENGTH: usize = ProposalHeader::LENGTH
        + 32    // merkle_root
        + 32; // src resource id (target of the proposal)

    /// Creates a new anchor update proposal.
    #[must_use]
    pub const fn new(
        header: ProposalHeader,
        merkle_root: [u8; 32],
        target: [u8; 32],
    ) -> Self {
        Self {
            header,
            merkle_root,
            target,
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
        let mut buf = [0u8; 6];
        buf.copy_from_slice(self.target[26..32].to_vec().as_slice());
        TypedChainId::from(buf)
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

    /// Get the target identifier.
    #[must_use]
    pub const fn target(&self) -> &[u8; 32] {
        &self.target
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        bytes[f..t].copy_from_slice(&self.header.to_bytes());
        let f = t;
        let t = t + 32;
        bytes[f..t].copy_from_slice(&self.merkle_root);
        let f = t;
        let t = t + 32;
        bytes[f..t].copy_from_slice(&self.target);
        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<[u8; AnchorUpdateProposal::LENGTH]> for AnchorUpdateProposal {
    fn from(bytes: [u8; AnchorUpdateProposal::LENGTH]) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);
        let f = t;
        let t = t + 32;
        let mut merkle_root = [0u8; 32];
        merkle_root.copy_from_slice(&bytes[f..t]);
        let f = t;
        let t = t + 32;
        let mut target = [0u8; 32];
        target.copy_from_slice(&bytes[f..t]);
        Self::new(header, merkle_root, target)
    }
}

impl From<AnchorUpdateProposal> for [u8; AnchorUpdateProposal::LENGTH] {
    fn from(proposal: AnchorUpdateProposal) -> Self {
        proposal.to_bytes()
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
            AnchorUpdateProposal::new(header, merkle_root, src_resource_id.0);
        let bytes = proposal.to_bytes();
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
        let proposal = AnchorUpdateProposal::from(bytes);
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
            AnchorUpdateProposal::new(header, merkle_root, src_resource_id.0);
        assert_eq!(proposal, expected);
    }
}
