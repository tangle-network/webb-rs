//! Wrapping Fee Update Proposal.
use crate::ResourceId;

/// Wrapping Fee Update Proposal.
///
/// The [`WrappingFeeUpdateProposal`] updates the wrapping fee percentage for a specific pool share.
#[allow(clippy::module_name_repetitions)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
pub struct WrappingFeeUpdateProposal {
    #[builder(default = 35)]
    pallet_index: u8,
    #[builder(default = 0)]
    call_index: u8,
    resource_id: ResourceId,
    #[builder(setter(transform = |v: u128| check_and_validate_wrapping_fee(v)))]
    wrapping_fee_percent: u128,
    into_pool_share_id: u32,
}

impl WrappingFeeUpdateProposal {
    /// Get the resource id.
    #[must_use]
    pub const fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    /// Get the wrapping fee percent.
    #[must_use]
    pub const fn wrapping_fee_percent(&self) -> u128 {
        debug_assert!(
            self.wrapping_fee_percent <= 100,
            "wrapping fee percent is too large"
        );
        self.wrapping_fee_percent
    }

    /// Get the pool share id.
    /// The pool share id is the id of the pool share that the wrapping fee will be applied to.
    #[must_use]
    pub const fn pool_share_id(&self) -> u32 {
        self.into_pool_share_id
    }

    /// Convert the proposal to a vector of bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(40);
        let call = ExecuteWrappingFeeUpdate {
            r_id: self.resource_id.to_bytes(),
            wrapping_fee_percent: self.wrapping_fee_percent,
            into_pool_share_id: self.into_pool_share_id,
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

impl From<WrappingFeeUpdateProposal> for Vec<u8> {
    fn from(proposal: WrappingFeeUpdateProposal) -> Self {
        proposal.into_bytes()
    }
}

impl TryFrom<Vec<u8>> for WrappingFeeUpdateProposal {
    type Error = scale_codec::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let pallet_index = value.get(0).copied().ok_or_else(|| {
            scale_codec::Error::from("invalid proposal: missing pallet index")
        })?;

        let call_index = value.get(1).copied().ok_or_else(|| {
            scale_codec::Error::from("invalid proposal: missing call index")
        })?;

        let call: ExecuteWrappingFeeUpdate =
            scale_codec::Decode::decode(&mut &value[2..])?;

        let resource_id = ResourceId::from(call.r_id);
        let wrapping_fee_percent = call.wrapping_fee_percent;
        let into_pool_share_id = call.into_pool_share_id;
        let proposal = WrappingFeeUpdateProposal {
            pallet_index,
            call_index,
            resource_id,
            wrapping_fee_percent,
            into_pool_share_id,
        };
        Ok(proposal)
    }
}

#[derive(scale_codec::Encode, scale_codec::Decode)]
struct ExecuteWrappingFeeUpdate {
    r_id: [u8; 32],
    wrapping_fee_percent: u128,
    into_pool_share_id: u32,
}

fn check_and_validate_wrapping_fee(wrapping_fee_percent: u128) -> u128 {
    debug_assert!(
        wrapping_fee_percent <= 100,
        "wrapping fee percent is too large"
    );
    wrapping_fee_percent
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
        let proposal = WrappingFeeUpdateProposal::builder()
            .resource_id(resource_id)
            .wrapping_fee_percent(5)
            .into_pool_share_id(1)
            .build();
        let bytes = proposal.to_bytes();
        let expected = concat!(
            "23", // pallet index
            "00", // call index
            "0000000000000000000000000000000000000000000000000002020000000001", // resource id
            "05000000000000000000000000000000", // wrapping fee percent
            "01000000"                          // pool share id
        );
        assert_eq!(expected, hex::encode(bytes));
    }

    #[test]
    fn decode() {
        let proposal_bytes = hex_literal::hex!(
          "23" // pallet index
          "00" // call index
          "0000000000000000000000000000000000000000000000000002020000000001" // resource id
          "05000000000000000000000000000000" // wrapping fee percent
          "01000000"  // pool share id
        );

        let proposal =
            WrappingFeeUpdateProposal::try_from(proposal_bytes.to_vec())
                .unwrap();
        assert_eq!(
            proposal.resource_id(),
            ResourceId::new(
                TargetSystem::new_tree_id(2),
                TypedChainId::Substrate(1)
            )
        );
        assert_eq!(proposal.wrapping_fee_percent(), 5);
        assert_eq!(proposal.pool_share_id(), 1);
    }

    #[cfg(debug_assertions)]
    #[should_panic(expected = "wrapping fee percent is too large")]
    #[test]
    fn should_check_wrapping_fee_value() {
        let target_system = TargetSystem::new_tree_id(2);
        let target_chain = TypedChainId::Substrate(1);
        let resource_id = ResourceId::new(target_system, target_chain);
        let _ = WrappingFeeUpdateProposal::builder()
            .resource_id(resource_id)
            .wrapping_fee_percent(101)
            .into_pool_share_id(1)
            .build();
    }
}
