//! Anchor Update Proposal.
use crate::{ChainId, ChainType, ProposalHeader};

/// Anchor Update Proposal:
/// the format of the proposal looks like this:
/// ```text
/// ┌────────────────────┬─────────────────┬───────────────┬──────────────────┬────────────────┐
/// │                    │                 │               │                  │                │
/// │ ProposalHeader 40B │ SrcChainType 2B │ SrcChainId 4B │ LastLeafIndex 4B │ MerkleRoot 32B │
/// │                    │                 │               │                  │                │
/// └────────────────────┴─────────────────┴───────────────┴──────────────────┴────────────────┘
/// ```
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AnchorUpdateProposal {
    header: ProposalHeader,
    src_chain_type: ChainType,
    src_chain_id: ChainId,
    last_leaf_index: u32,
    merkle_root: [u8; 32],
}

impl AnchorUpdateProposal {
    /// Length of the proposal in bytes.
    pub const LENGTH: usize = ProposalHeader::LENGTH
        + ChainType::LENGTH
        + ChainId::LENGTH
        + core::mem::size_of::<u32>() // last_leaf_index
        + 32; // merkle_root

    /// Creates a new anchor update proposal.
    #[must_use]
    pub const fn new(
        header: ProposalHeader,
        src_chain_type: ChainType,
        src_chain_id: ChainId,
        last_leaf_index: u32,
        merkle_root: [u8; 32],
    ) -> Self {
        Self {
            header,
            src_chain_type,
            src_chain_id,
            last_leaf_index,
            merkle_root,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the source chain type.
    #[must_use]
    pub const fn src_chain_type(&self) -> ChainType {
        self.src_chain_type
    }

    /// Get the source chain id.
    #[must_use]
    pub const fn src_chain_id(&self) -> ChainId {
        self.src_chain_id
    }

    /// Get the last leaf index.
    #[must_use]
    pub const fn last_leaf_index(&self) -> u32 {
        self.last_leaf_index
    }

    /// Get the merkle root.
    #[must_use]
    pub const fn merkle_root(&self) -> &[u8; 32] {
        &self.merkle_root
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        bytes[f..t].copy_from_slice(&self.header.to_bytes());
        let f = t;
        let t = t + ChainType::LENGTH;
        bytes[f..t].copy_from_slice(&self.src_chain_type.to_bytes());
        let f = t;
        let t = t + ChainId::LENGTH;
        bytes[f..t].copy_from_slice(&self.src_chain_id.to_bytes());
        let f = t;
        let t = t + core::mem::size_of::<u32>();
        bytes[f..t].copy_from_slice(&self.last_leaf_index.to_be_bytes());
        let f = t;
        let t = t + 32;
        bytes[f..t].copy_from_slice(&self.merkle_root);
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
        let t = t + ChainType::LENGTH;
        let mut src_chain_type_bytes = [0u8; ChainType::LENGTH];
        src_chain_type_bytes.copy_from_slice(&bytes[f..t]);
        let src_chain_type = ChainType::from(src_chain_type_bytes);
        let f = t;
        let t = t + ChainId::LENGTH;
        let mut src_chain_id_bytes = [0u8; ChainId::LENGTH];
        src_chain_id_bytes.copy_from_slice(&bytes[f..t]);
        let src_chain_id = ChainId::from(src_chain_id_bytes);
        let f = t;
        let t = t + core::mem::size_of::<u32>();
        let mut last_leaf_index_bytes = [0u8; core::mem::size_of::<u32>()];
        last_leaf_index_bytes.copy_from_slice(&bytes[f..t]);
        let last_leaf_index = u32::from_be_bytes(last_leaf_index_bytes);
        let f = t;
        let t = t + 32;
        let mut merkle_root = [0u8; 32];
        merkle_root.copy_from_slice(&bytes[f..t]);
        Self::new(
            header,
            src_chain_type,
            src_chain_id,
            last_leaf_index,
            merkle_root,
        )
    }
}

impl From<AnchorUpdateProposal> for [u8; AnchorUpdateProposal::LENGTH] {
    fn from(proposal: AnchorUpdateProposal) -> Self {
        proposal.to_bytes()
    }
}
