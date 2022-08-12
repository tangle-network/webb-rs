//! Anchor Update Proposal.

use crate::{ProposalHeader, ResourceId, TypedChainId};

/// Anchor Update Proposal.
///
/// The [`AnchorUpdateProposal`] updates the target Anchor's knowledge of the
/// source Anchor's Merkle roots. This knowledge is necessary to prove
/// membership in the source Anchor's Merkle tree on the target chain.
///
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

        let message = UpdateEdge {
            root: self.merkle_root,
            src_resource_id: self.src_resource_id.to_bytes(),
        };

        scale_codec::Encode::encode_to(&message, &mut bytes);

        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl TryFrom<Vec<u8>> for AnchorUpdateProposal {
    type Error = scale_codec::Error;
    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        let parsed_header =
            bytes.get(0..ProposalHeader::LENGTH).ok_or_else(|| {
                scale_codec::Error::from(
                    "invaid proposal: invalid length of proposal",
                )
            })?;

        header_bytes.copy_from_slice(parsed_header);
        let header = ProposalHeader::from(header_bytes);

        let decoded_edge_data: UpdateEdge = scale_codec::Decode::decode(&mut &bytes[40..])?;

        Ok(Self::new(
            header,
            decoded_edge_data.root,
            ResourceId(decoded_edge_data.src_resource_id),
        ))
    }
}

impl From<AnchorUpdateProposal> for Vec<u8> {
    fn from(proposal: AnchorUpdateProposal) -> Self {
        proposal.to_bytes()
    }
}

// if we have EVM available, we can convert the EVM proposal to a ink proposal
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

// if we have Substrate available, we can convert the Substrate proposal to a ink proposal
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

// if we have Cosmwasm available, we can convert the Cosmwasm proposal to a ink proposal
#[cfg(feature = "cosmwasm")]
impl From<crate::cosmwasm::AnchorUpdateProposal> for AnchorUpdateProposal {
    fn from(proposal: crate::cosmwasm::AnchorUpdateProposal) -> Self {
        AnchorUpdateProposal::new(
            proposal.header(),
            *proposal.merkle_root(),
            proposal.src_resource_id(),
        )
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct UpdateEdge {
    root: [u8; 32],
    src_resource_id: [u8; 32],
}

#[cfg(test)]
mod tests {
    use crate::{
        FunctionSignature, Nonce,
        ResourceId, TargetSystem,
    };
    use crate::ink::ink_address_to_target_address;

    use super::*;

    const TARGET_CONTRACT_ADDR: [u8; 32] = [0u8; 32];

    #[test]
    fn encode() {
        let target_addr =
            ink_address_to_target_address(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Ink(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let latest_leaf_index = 0x0001;
        let nonce = Nonce::from(latest_leaf_index);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let src_chain = TypedChainId::Ink(1);
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
            "00000000000088386fc84ba6bc95484008f6362f93160ef3e5630600000000040000000000000001000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f00000000000088386fc84ba6bc95484008f6362f93160ef3e563060000000001"
        );
        let decoded = hex::encode([0, 0, 0, 0, 0, 0, 136, 56, 111, 200, 75, 166, 188, 149, 72, 64, 8, 246, 54, 47, 147, 22, 14, 243, 229, 99, 6, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 0, 0, 0, 0, 0, 136, 56, 111, 200, 75, 166, 188, 149, 72, 64, 8, 246, 54, 47, 147, 22, 14, 243, 229, 99, 6, 0, 0, 0, 0, 1]);
        dbg!(decoded);
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!("00000000000088386fc84ba6bc95484008f6362f93160ef3e5630600000000040000000000000001000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f00000000000088386fc84ba6bc95484008f6362f93160ef3e563060000000001");
        let proposal = AnchorUpdateProposal::try_from(bytes.to_vec()).unwrap();
        let target_addr =
            ink_address_to_target_address(TARGET_CONTRACT_ADDR);
        let target_system = TargetSystem::ContractAddress(target_addr);
        let target_chain = TypedChainId::Ink(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("00000000"));
        let latest_leaf_index = 0x0001;
        let nonce = Nonce::from(latest_leaf_index);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let src_chain_id = TypedChainId::Ink(1);
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
