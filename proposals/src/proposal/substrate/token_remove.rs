//! Token Remove Proposal.
use crate::ResourceId;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

/// Token Remove Proposal.
///
/// The [`TokenRemoveProposal`] allows the token specified by the `AssetId` to
/// be removed from a token pool.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder)]
pub struct TokenRemoveProposal {
    #[builder(default = 35)]
    pallet_index: u8,
    #[builder(default = 2)]
    call_index: u8,
    resource_id: ResourceId,
    #[builder(setter(transform = |v: String| v.into_bytes()))]
    name: Vec<u8>,
    asset_id: u32,
}

impl TokenRemoveProposal {
    /// Get the resource id.
    #[must_use]
    pub const fn resource_id(&self) -> ResourceId {
        self.resource_id
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
        let mut out = Vec::with_capacity(40 + self.name.len());
        let call = ExecuteRemoveTokenFromPoolShare {
            r_id: self.resource_id.to_bytes(),
            name: self.name.clone(),
            asset_id: self.asset_id,
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

impl From<TokenRemoveProposal> for Vec<u8> {
    fn from(proposal: TokenRemoveProposal) -> Self {
        proposal.into_bytes()
    }
}

impl TryFrom<Vec<u8>> for TokenRemoveProposal {
    type Error = scale_codec::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let pallet_index = value.get(0).copied().ok_or_else(|| {
            scale_codec::Error::from("invalid proposal: missing pallet index")
        })?;

        let call_index = value.get(1).copied().ok_or_else(|| {
            scale_codec::Error::from("invalid proposal: missing call index")
        })?;

        let call: ExecuteRemoveTokenFromPoolShare =
            scale_codec::Decode::decode(&mut &value[2..])?;

        let resource_id = ResourceId::from(call.r_id);
        let name = call.name;
        let asset_id = call.asset_id;
        let proposal = TokenRemoveProposal {
            pallet_index,
            call_index,
            resource_id,
            name,
            asset_id,
        };
        Ok(proposal)
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteRemoveTokenFromPoolShare {
    r_id: [u8; 32],
    name: Vec<u8>,
    asset_id: u32,
}

#[cfg(test)]
mod tests {
    use crate::{TargetSystem, TypedChainId};

    use super::*;

    #[test]
    fn encode() {
        let target_system = TargetSystem::new_tree_id(2);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let proposal = TokenRemoveProposal::builder()
            .resource_id(resource_id)
            .name("test".to_string())
            .asset_id(1)
            .build();
        let bytes = proposal.to_bytes();
        let expected = concat!(
            "23", // pallet index
            "02", // call index
            "0000000000000000000000000000000000000000000000000002020000000001", // resource id
            "1074657374", // name
            "01000000"    // asset id
        );
        assert_eq!(expected, hex::encode(bytes));
    }

    #[test]
    fn decode() {
        let proposal_bytes = hex_literal::hex!(
          "23" // pallet index
          "02" // call index
          "0000000000000000000000000000000000000000000000000002020000000001" // resource id
          "1074657374" // name
          "01000000"  // asset id
        );

        let proposal =
            TokenRemoveProposal::try_from(proposal_bytes.to_vec()).unwrap();
        assert_eq!(
            proposal.resource_id(),
            ResourceId::new(
                TargetSystem::new_tree_id(2),
                TypedChainId::Substrate(1)
            )
        );
        assert_eq!(proposal.name(), "test");
        assert_eq!(proposal.asset_id(), 1);
    }
}
