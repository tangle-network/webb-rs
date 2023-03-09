#![allow(clippy::exhaustive_enums)]
//! Token Add Proposal.
use crate::target_system::TargetSystem;
use crate::ProposalHeader;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};
/// Token Add Proposal.
///
/// The [`TokenAddProposal`] allows the token specified by the `AssetId` to
/// be added into a token pool.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder)]
pub struct TokenAddProposal {
    header: ProposalHeader,
    #[builder(setter(transform = |v: String| v.into_bytes()))]
    name: Vec<u8>,
    asset_id: u32,
}

impl TokenAddProposal {
    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the asset name.
    #[must_use]
    pub fn name(&self) -> String {
        String::from_utf8(self.name.clone()).expect("name is not valid utf8")
    }

    /// Get the latest leaf index.
    #[must_use]
    pub const fn asset_id(&self) -> u32 {
        self.asset_id
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(40 + 40 + self.name.len());
        let target_system = self.header().resource_id().target_system();

        let TargetSystem::Substrate(target_details) = target_system else { unreachable!("Unexpected target system for substrate") };

        // add proposal header 40B
        out.extend_from_slice(&self.header.to_bytes());

        let call = ExecuteAddTokenToPoolShare {
            name: self.name.clone(),
            asset_id: self.asset_id,
            nonce: self.header().nonce.to_u32(),
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

impl From<TokenAddProposal> for Vec<u8> {
    fn from(proposal: TokenAddProposal) -> Self {
        proposal.into_bytes()
    }
}

impl TryFrom<Vec<u8>> for TokenAddProposal {
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

        let call: ExecuteAddTokenToPoolShare =
            scale_codec::Decode::decode(&mut &value[42..])?;

        let name = call.name;
        let asset_id = call.asset_id;
        let proposal = TokenAddProposal {
            header,
            name,
            asset_id,
        };
        Ok(proposal)
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteAddTokenToPoolShare {
    name: Vec<u8>,
    asset_id: u32,
    nonce: u32,
}

#[cfg(test)]
mod tests {
    use crate::{
        FunctionSignature, Nonce, ResourceId, SubstrateTargetSystem,
        TargetSystem, TypedChainId,
    };

    use super::*;

    #[test]
    fn encode() {
        let target = SubstrateTargetSystem::builder()
            .pallet_index(35)
            .tree_id(2)
            .build();
        let target_system = TargetSystem::Substrate(target);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature = FunctionSignature::new([0, 0, 0, 1]);
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        let proposal = TokenAddProposal::builder()
            .header(header)
            .name("test".to_string())
            .asset_id(1)
            .build();
        let bytes = proposal.to_bytes();
        let expected = concat!(
        "00000000000000000000000000000000000000000023000000020200000000010000000100000001", // header
        "23",           // pallet index
        "01",           // call index
        "1074657374",   // name
        "01000000",     // asset id
        "01000000",     // nonce
        );
        assert_eq!(hex::encode(bytes), expected);
    }

    #[test]
    fn decode() {
        let proposal_bytes = hex_literal::hex!(
        "00000000000000000000000000000000000000000023000000020200000000010000000000000001" // header
        "23"              // pallet index
        "01"              // call index
        "1074657374"      // name
        "01000000"        // asset id
        "01000000"        // nonce
        );

        let proposal =
            TokenAddProposal::try_from(proposal_bytes.to_vec()).unwrap();
        let target = SubstrateTargetSystem::builder()
            .pallet_index(35)
            .tree_id(2)
            .build();
        assert_eq!(
            proposal.header.resource_id(),
            ResourceId::new(
                TargetSystem::Substrate(target),
                TypedChainId::Substrate(1)
            )
        );
        assert_eq!(proposal.name(), "test");
        assert_eq!(proposal.asset_id(), 1);
    }
}
