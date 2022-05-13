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
    src_chain: TypedChainId,
    latest_leaf_index: u32,
    merkle_root: [u8; 32],
    target: [u8; 32],
}

impl AnchorUpdateProposal {
    /// Length of the proposal in bytes.
    pub const LENGTH: usize = ProposalHeader::LENGTH
        + TypedChainId::LENGTH
        + core::mem::size_of::<u32>() // latest_leaf_index
        + 32 // merkle_root
        + 32; // target

    /// Creates a new anchor update proposal.
    #[must_use]
    pub const fn new(
        header: ProposalHeader,
        src_chain: TypedChainId,
        latest_leaf_index: u32,
        merkle_root: [u8; 32],
        target: [u8; 32],
    ) -> Self {
        Self {
            header,
            src_chain,
            latest_leaf_index,
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
    pub const fn src_chain(&self) -> TypedChainId {
        self.src_chain
    }

    /// Get the latest leaf index.
    #[must_use]
    pub const fn latest_leaf_index(&self) -> u32 {
        self.latest_leaf_index
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
        let t = t + TypedChainId::LENGTH;
        bytes[f..t].copy_from_slice(&self.src_chain.to_bytes());
        let f = t;
        let t = t + core::mem::size_of::<u32>();
        bytes[f..t].copy_from_slice(&self.latest_leaf_index.to_be_bytes());
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
        let t = t + TypedChainId::LENGTH;
        let mut src_chain_bytes = [0u8; TypedChainId::LENGTH];
        src_chain_bytes.copy_from_slice(&bytes[f..t]);
        let src_chain = TypedChainId::from(src_chain_bytes);
        let f = t;
        let t = t + core::mem::size_of::<u32>();
        let mut latest_leaf_index_bytes = [0u8; core::mem::size_of::<u32>()];
        latest_leaf_index_bytes.copy_from_slice(&bytes[f..t]);
        let latest_leaf_index = u32::from_be_bytes(latest_leaf_index_bytes);
        let f = t;
        let t = t + 32;
        let mut merkle_root = [0u8; 32];
        merkle_root.copy_from_slice(&bytes[f..t]);
        let f = t;
        let t = t + 32;
        let mut target = [0u8; 32];
        target.copy_from_slice(&bytes[f..t]);
        Self::new(header, src_chain, latest_leaf_index, merkle_root, target)
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
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let src_chain = TypedChainId::Evm(1);
        let latest_leaf_index = 0x0001;
        let merkle_root = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let proposal = AnchorUpdateProposal::new(
            header,
            src_chain,
            latest_leaf_index,
            merkle_root,
            target_system.into_fixed_bytes(),
        );
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe0000000101000000000100000001000102030405060708090a0b0c0d"
            "0e0f101112131415161718191a1b1c1d1e1f"
            "000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe0000000101000000000100000001000102030405060708090a0b0c0d"
            "0e0f101112131415161718191a1b1c1d1e1f"
            "000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
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
        let src_chain = TypedChainId::Evm(1);
        let latest_leaf_index = 0x0001;
        let merkle_root = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let expected = AnchorUpdateProposal::new(
            header,
            src_chain,
            latest_leaf_index,
            merkle_root,
            target_system.into_fixed_bytes(),
        );
        assert_eq!(proposal, expected);
    }
}
