#![allow(clippy::exhaustive_enums)]
//! Anchor Update Proposal.
use crate::target_system::TargetSystem;
use crate::{ProposalHeader, ResourceId, TypedChainId};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Anchor Update Proposal.
///
/// The [`AnchorUpdateProposal`] updates a target Anchor's knowledge of the
/// source Anchor's Merkle roots. This knowledge is necessary to prove
/// membership in the source Anchor's Merkle tree on the `src_resource_id`
/// chain.
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct AnchorUpdateProposal {
    header: ProposalHeader,
    merkle_root: [u8; 32],
    src_resource_id: ResourceId,
}

impl AnchorUpdateProposal {
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

    /// Get the `src_resource_id` identifier.
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

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(120);
        let target_system = self.header().resource_id().target_system();

        let TargetSystem::Substrate(target_details) = target_system else { unreachable!("Unexpected target system for substrate") };

        // add proposal header 40B
        out.extend_from_slice(&self.header.to_bytes());

        let call = ExecuteAnchorUpdateProposal {
            r_id: self.header().resource_id().to_bytes(),
            merkle_root: self.merkle_root,
            src_resource_id: self.src_resource_id().to_bytes(),
            nonce: self.header().nonce().to_u32(),
        };

        // add pallet index
        out.push(target_details.pallet_index);
        // add call index, it is big-endian encoded from a u32 (4-bytes)
        // the last byte should contain the u8 call index
        out.push(self.header().function_signature.0[3]);
        scale_codec::Encode::encode_to(&call, &mut out);
        out
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl From<AnchorUpdateProposal> for Vec<u8> {
    fn from(proposal: AnchorUpdateProposal) -> Self {
        proposal.into_bytes()
    }
}

impl TryFrom<Vec<u8>> for AnchorUpdateProposal {
    type Error = scale_codec::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        // parse header bytes
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        let parsed_header =
            value.get(0..ProposalHeader::LENGTH).ok_or_else(|| {
                scale_codec::Error::from(
                    "invaid proposal: invalid length of proposal",
                )
            })?;
        header_bytes.copy_from_slice(parsed_header);
        let header = ProposalHeader::from(header_bytes);

        // parse encoded proposal call
        let call: ExecuteAnchorUpdateProposal =
            scale_codec::Decode::decode(&mut &value[42..])?;
        let merkle_root = call.merkle_root;
        let src_resource_id = call.src_resource_id;
        let proposal = AnchorUpdateProposal {
            header,
            merkle_root,
            src_resource_id: ResourceId(src_resource_id),
        };
        Ok(proposal)
    }
}

// if we have EVM available, we can convert the EVM proposal to a substrate
// proposal
#[cfg(feature = "evm")]
impl From<crate::evm::AnchorUpdateProposal> for AnchorUpdateProposal {
    fn from(proposal: crate::evm::AnchorUpdateProposal) -> Self {
        AnchorUpdateProposal::builder()
            .header(proposal.header())
            .merkle_root(*proposal.merkle_root())
            .src_resource_id(proposal.src_resource_id())
            .build()
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteAnchorUpdateProposal {
    r_id: [u8; 32],
    merkle_root: [u8; 32],
    src_resource_id: [u8; 32],
    nonce: u32,
}

#[cfg(test)]
mod tests {
    use crate::{
        FunctionSignature, Nonce, ResourceId, SubstrateTargetSystem,
        TargetSystem,
    };

    use super::*;

    #[test]
    fn encode() {
        let target = SubstrateTargetSystem::builder()
            .pallet_index(50)
            .tree_id(2)
            .build();
        let target_system = TargetSystem::Substrate(target);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature = FunctionSignature::new([0, 0, 0, 1]);
        let latest_leaf_index = 0x0001;
        let nonce = Nonce::from(latest_leaf_index);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let src_chain_id = TypedChainId::Substrate(2);

        let merkle_root = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let src_system = SubstrateTargetSystem::builder()
            .pallet_index(50)
            .tree_id(3)
            .build();
        let src_target_system = TargetSystem::Substrate(src_system);
        let src_resource_id = ResourceId::new(src_target_system, src_chain_id);
        let proposal = AnchorUpdateProposal::builder()
            .header(header)
            .merkle_root(merkle_root)
            .src_resource_id(src_resource_id)
            .build();
        let bytes = proposal.to_bytes();
        let expected = concat!(
          "00000000000000000000000000000000000000000032000000020200000000010000000100000001", // header
          "32",                                                               // pallet index
          "01",                                                               // call index
          "0000000000000000000000000000000000000000003200000002020000000001", // r_id
          "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f", // merkle root"
          "0000000000000000000000000000000000000000003200000003020000000002", // src resource id
          "01000000",                                                         // nonce
        );
        let bytes_hex = hex::encode(bytes);
        assert_eq!(bytes_hex, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "00000000000000000000000000000000000000000032000000020200000000010000000100000001"  // header
            "32"                                                                // pallet index
            "01"                                                                // call index
            "0000000000000000000000000000000000000000003200000002020000000001" // r_id
            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"  // merkle root"
            "0000000000000000000000000000000000000000003200000003020000000002"  // src resource id
            "01000000"                                                          // nonce
        );

        let proposal = AnchorUpdateProposal::try_from(bytes.to_vec()).unwrap();
        let target = SubstrateTargetSystem::builder()
            .pallet_index(50)
            .tree_id(2)
            .build();
        assert_eq!(
            proposal.header.resource_id().target_system(),
            TargetSystem::Substrate(target)
        );
        assert_eq!(
            proposal.header.resource_id().typed_chain_id(),
            TypedChainId::Substrate(1)
        );
        let src_resource_id_chain_bytes =
            proposal.src_resource_id().typed_chain_id().to_bytes();
        assert_eq!(
            src_resource_id_chain_bytes,
            TypedChainId::Substrate(2).to_bytes()
        );
        assert_eq!(
            proposal.merkle_root,
            [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
                0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
                0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
                0x1e, 0x1f,
            ]
        );
        assert_eq!(proposal.header().nonce().to_u32(), 0x0001);
        assert_eq!(proposal.src_resource_id().to_bytes(), hex_literal::hex!("0000000000000000000000000000000000000000003200000003020000000002"));
    }
}
