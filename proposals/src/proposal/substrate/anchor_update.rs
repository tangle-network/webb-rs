//! Anchor Update Proposal.
use crate::{ProposalHeader, TypedChainId};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Anchor Update Proposal.
///
/// The [`AnchorUpdateProposal`] updates the target Anchor's knowledge of the
/// source Anchor's Merkle roots. This knowledge is necessary to prove
/// membership in the source Anchor's Merkle tree on the target chain.
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct AnchorUpdateProposal {
    header: ProposalHeader,
    #[builder(default = 50)]
    pallet_index: u8,
    #[builder(default = 1)]
    call_index: u8,
    merkle_root: [u8; 32],
    target: [u8; 32],
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

    /// Get the target.
    #[must_use]
    pub const fn target(&self) -> &[u8; 32] {
        &self.target
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(120);
        // add proposal header 40B
        out.extend_from_slice(&self.header.to_bytes());

        let mut src_chain_id_bytes = [0u8; 6];
        src_chain_id_bytes
            .copy_from_slice(self.target[26..32].to_vec().as_slice());
        let src_chain_id = TypedChainId::from(src_chain_id_bytes);
        let call = ExecuteAnchorUpdateProposal {
            r_id: self.header().resource_id().to_bytes(),
            anchor_metadata: EdgeMetadata {
                src_chain_id: src_chain_id.chain_id(),
                root: Element(self.merkle_root),
                latest_leaf_index: self.header().nonce().to_u32(),
                target: Element(self.target),
            },
        };

        // add pallet index
        out.push(self.pallet_index);
        // add call index
        out.push(self.call_index);
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

        // parse pallet index
        let pallet_index = value.get(40).copied().ok_or_else(|| {
            scale_codec::Error::from("invalid proposal: missing pallet index")
        })?;
        // parse call index
        let call_index = value.get(41).copied().ok_or_else(|| {
            scale_codec::Error::from("invalid proposal: missing call index")
        })?;
        // parse encoded proposal call
        let call: ExecuteAnchorUpdateProposal =
            scale_codec::Decode::decode(&mut &value[42..])?;
        let src_chain = TypedChainId::from(call.anchor_metadata.src_chain_id);
        let merkle_root = call.anchor_metadata.root.0;
        let latest_leaf_index = call.anchor_metadata.latest_leaf_index;
        let target = call.anchor_metadata.target.0;
        let proposal = AnchorUpdateProposal {
            header,
            pallet_index,
            call_index,
            merkle_root,
            target,
        };
        Ok(proposal)
    }
}

// if we have EVM available, we can convert the EVM proposal to a substrate proposal
#[cfg(feature = "evm")]
impl From<crate::evm::AnchorUpdateProposal> for AnchorUpdateProposal {
    fn from(proposal: crate::evm::AnchorUpdateProposal) -> Self {
        AnchorUpdateProposal::builder()
            .header(proposal.header())
            .merkle_root(*proposal.merkle_root())
            .target(*proposal.target())
            .build()
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct Element(pub [u8; 32]);

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct EdgeMetadata {
    src_chain_id: u64,
    root: Element,
    latest_leaf_index: u32,
    target: Element,
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteAnchorUpdateProposal {
    r_id: [u8; 32],
    anchor_metadata: EdgeMetadata,
}

#[cfg(test)]
mod tests {
    use crate::{FunctionSignature, Nonce, ResourceId, TargetSystem};

    use super::*;

    #[test]
    fn encode() {
        let target_system = TargetSystem::new_tree_id(2);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let src_chain = TypedChainId::Substrate(2);
        let latest_leaf_index = 0x0001;
        let merkle_root = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
            0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let target = [0x11u8; 32];
        let proposal = AnchorUpdateProposal::builder()
            .header(header)
            .merkle_root(merkle_root)
            .target(target)
            .build();
        let bytes = proposal.to_bytes();
        let expected = concat!(
          "0000000000000000000000000000000000000000000000000002020000000001cafebabe00000001", // header
          "3201", // pallet index, call index
          "0000000000000000000000000000000000000000000000000002020000000001", // resource id
          "0200000000020000000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f01000000", // metadata
          "1111111111111111111111111111111111111111111111111111111111111111" // target
        );
        let bytes_hex = hex::encode(bytes);
        assert_eq!(bytes_hex, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "0000000000000000000000000000000000000000000000000002020000000001cafebabe00000001" //header
            "3201" // pallet index, call index
            "0000000000000000000000000000000000000000000000000002020000000001" // resource id
            "0200000000020000000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f01000000" // metadata
            "1111111111111111111111111111111111111111111111111111020000000002" // target
        );

        let proposal = AnchorUpdateProposal::try_from(bytes.to_vec()).unwrap();
        assert_eq!(proposal.pallet_index, 0x32);
        assert_eq!(proposal.call_index, 0x01);
        assert_eq!(
            proposal.header.resource_id().target_system(),
            TargetSystem::new_tree_id(2)
        );
        assert_eq!(
            proposal.header.resource_id().typed_chain_id(),
            TypedChainId::Substrate(1)
        );
        let target_chain_bytes = proposal.target[26..].to_vec();
        assert_eq!(target_chain_bytes, TypedChainId::Substrate(2).to_bytes());
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
        assert_eq!(proposal.target, [0x11u8; 32]);
    }
}
