#![allow(clippy::exhaustive_enums)]
//! The Proposal Target System.

//! Target system format for Substrate
//! ┌─────────────────────┬───────┬───────┬─────────────┐
//! │                     │       │       │             │
//! │           Zeros     │ Pallet│ Call  │   Tree ID   │
//! │           (20B)     │  Idx  │  Idx  │    (4B)     │
//! │                     │       │       │             │
//! └─────────────────────┴───────┴───────┴─────────────┘
//!                       ▲       ▲       ▲             ▲
//!                       │   20  │   21  │ 22 23 24 25 │
//!                       │       │       │             │
//!
//! Target system format for Evm
//! ┌────────────────┬──────────────────────────────────┐
//! │                │                                  │
//! │      Zeros     │     Contract Address             │
//! │      (6B)      │           (20B)                  │
//! │                │                                  │
//! └────────────────┘──────────────────────────────────┘
//!                  ▲                                  ▲
//!                  │ 6                             25 │
//!                  │                                  │

#[cfg(all(not(feature = "std"), feature = "substrate"))]
use alloc::vec::Vec;
/// `TargetSystem` (26 Bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "scale",
    derive(
        scale_info::TypeInfo,
        scale_codec::Encode,
        scale_codec::Decode,
        scale_codec::MaxEncodedLen
    )
)]
#[non_exhaustive]
pub enum TargetSystem {
    /// Ethereum Contract address (20 bytes).
    ContractAddress([u8; 20]),
    /// Webb Protocol-Substrate 6 bytes (pallet_index, call_index, tree_id ).
    #[cfg(any(feature = "substrate", feature = "ink"))]
    Substrate(SubstrateTargetSystem),
}

/// Substrate Target System
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
)]
#[cfg_attr(
    feature = "scale",
    derive(
        scale_info::TypeInfo,
        scale_codec::Encode,
        scale_codec::Decode,
        scale_codec::MaxEncodedLen
    )
)]
#[cfg(any(feature = "substrate", feature = "ink"))]
#[allow(clippy::module_name_repetitions)]
pub struct SubstrateTargetSystem {
    /// Pallet index of proposal handler pallet
    pub pallet_index: u8,
    /// Webb Protocol Merkle `TreeId` (4 bytes).
    pub tree_id: u32,
}

impl TargetSystem {
    /// Length of the `TargetSystem` (26 bytes).
    pub const LENGTH: usize = 26;
    /// Create a new `TargetSystem` as a `ContractAddress`.
    #[must_use]
    pub fn new_contract_address<T: Into<[u8; 20]>>(address: T) -> Self {
        let bytes = address.into();
        Self::ContractAddress(bytes)
    }

    /// Get the underlying bytes of the `TargetSystem`.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        match self {
            TargetSystem::ContractAddress(address) => {
                let mut bytes = [0u8; TargetSystem::LENGTH];
                let f = 6usize;
                let t = f + 20;
                bytes[f..t].copy_from_slice(address);
                bytes
            }
            #[cfg(any(feature = "substrate", feature = "ink"))]
            TargetSystem::Substrate(target_system) => {
                let mut bytes = [0u8; TargetSystem::LENGTH];
                let f = 22usize;
                let t = f + core::mem::size_of::<u32>();
                bytes[f - 1] = target_system.pallet_index;
                bytes[f..t]
                    .copy_from_slice(&target_system.tree_id.to_be_bytes());
                bytes
            }
        }
    }

    /// Get the underlying bytes of the `TargetSystem` without copying.
    #[must_use]
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }

    /// Get substrate `TargetSystem` details
    #[cfg(any(feature = "substrate", feature = "ink"))]
    #[must_use]
    pub fn get_substrate_target_system(self) -> Option<SubstrateTargetSystem> {
        match self {
            TargetSystem::Substrate(target_system) => Some(target_system),
            _ => None,
        }
    }

    /// Get smart contract address from the target system.
    /// Returns `[0; 20]` if the target system is not a contract address.
    #[must_use]
    pub fn into_contract_address_or_default(self) -> [u8; 20] {
        match self {
            TargetSystem::ContractAddress(address) => address,
            #[cfg(any(feature = "substrate", feature = "ink"))]
            _ => [0; 20],
        }
    }
}

impl From<[u8; TargetSystem::LENGTH]> for TargetSystem {
    fn from(bytes: [u8; TargetSystem::LENGTH]) -> Self {
        // check the first 20 bytes are zeros.
        // if so, it is a substrate target system.
        let substrate_based_system =
            cfg!(feature = "substrate") || cfg!(feature = "ink");
        if bytes[0..20] == [0u8; 20] && substrate_based_system {
            #[cfg(any(feature = "substrate", feature = "ink"))]
            {
                let mut tree_id_bytes = [0u8; 4];
                let f = 22usize;
                let t = f + core::mem::size_of::<u32>();
                tree_id_bytes.copy_from_slice(&bytes[f..t]);
                let tree_id = u32::from_be_bytes(tree_id_bytes);
                let target = SubstrateTargetSystem::builder()
                    .pallet_index(bytes[f - 1])
                    .tree_id(tree_id)
                    .build();
                TargetSystem::Substrate(target)
            }
            #[cfg(not(any(feature = "substrate", feature = "ink")))]
            {
                unreachable!("This should not happen, since substrate_based_system is false at compile time");
            }
        } else {
            let mut address = [0u8; 20];
            let f = 6usize;
            let t = f + 20;
            address.copy_from_slice(&bytes[f..t]);
            TargetSystem::ContractAddress(address)
        }
    }
}

impl From<TargetSystem> for [u8; TargetSystem::LENGTH] {
    fn from(target_system: TargetSystem) -> Self {
        target_system.into_bytes()
    }
}
#[cfg(feature = "substrate")]
impl Default for TargetSystem {
    fn default() -> Self {
        let target = SubstrateTargetSystem::builder()
            .pallet_index(0)
            .tree_id(0)
            .build();
        TargetSystem::Substrate(target)
    }
}
