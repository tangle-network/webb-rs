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

        let target_details = match target_system {
            TargetSystem::Substrate(target) => target,
            _ => unreachable!("Unexpected target system for substrate"),
        };

        // add proposal header 40B
        out.extend_from_slice(&self.header.to_bytes());

        let call = ExecuteAddTokenToPoolShare {
            r_id: self.header.resource_id().to_bytes(),
            name: self.name.clone(),
            asset_id: self.asset_id,
        };
        // add pallet index
        out.push(target_details.pallet_index);
        // add call index
        out.push(target_details.call_index);
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
    r_id: [u8; 32],
    name: Vec<u8>,
    asset_id: u32,
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
            .call_index(1)
            .tree_id(2)
            .build();
        let target_system = TargetSystem::Substrate(target);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
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
            "0000000000000000000000000000000000000000230100000002020000000001cafebabe00000001", // header
            "23", // pallet index
            "01", // call index
            "0000000000000000000000000000000000000000230100000002020000000001", // resource id
            "1074657374", // name
            "01000000"    // asset id
        );
        assert_eq!(expected, hex::encode(bytes));
    }

    #[test]
    fn decode() {
        let proposal_bytes = hex_literal::hex!(
          "0000000000000000000000000000000000000000230100000002020000000001cafebabe00000001" // header
          "23" // pallet index
          "01" // call index
          "0000000000000000000000000000000000000000230100000002020000000001" // resource id
          "1074657374" // name
          "01000000"  // asset id
        );

        let proposal =
            TokenAddProposal::try_from(proposal_bytes.to_vec()).unwrap();
        let target = SubstrateTargetSystem::builder()
            .pallet_index(35)
            .call_index(1)
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
