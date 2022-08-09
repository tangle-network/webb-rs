/// The Proposal Target System.
/// Target system format for Substrate
/// ┌─────────────────────┬───────┬───────┬─────────────┐
/// │                     │       │       │             │
/// │           Zeros     │ Pallet│ Call  │   Tree ID   │
/// │           (20B)     │  Idx  │  Idx  │    (4B)     │
/// │                     │       │       │             │
/// └─────────────────────┴───────┴───────┴─────────────┘
///                       ▲       ▲       ▲             ▲
///                       │   20  │   21  │ 22 23 24 25 │
///                       │       │       │             │
///
/// Target system format for Evm
/// ┌────────────────┬──────────────────────────────────┐
/// │                │                                  │
/// │      Zeros     │     Contract Address             │
/// │      (6B)      │           (20B)                  │
/// │                │                                  │
/// └────────────────┘──────────────────────────────────┘
///                  ▲                                  ▲
///                  │ 6                             25 │
///                  │                                  │
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
pub enum TargetSystem {
    /// Ethereum Contract address (20 bytes).
    ContractAddress([u8; 20]),
    /// Webb Protocol-Substrate 6 bytes (pallet_index, call_index, tree_id ).
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
pub struct SubstrateTargetSystem {
    /// Pallet index of proposal handler pallet
    pub pallet_index: u8,
    /// Call index of proposal handler pallet
    pub call_index: u8,
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
            TargetSystem::Substrate(target_system) => {
                let mut bytes = [0u8; TargetSystem::LENGTH];
                let f = 22usize;
                let t = f + core::mem::size_of::<u32>();
                bytes[f - 1] = target_system.call_index;
                bytes[f - 2] = target_system.pallet_index;
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

    /// Get susbtrate TargetSystem details
    #[must_use]
    pub fn get_substrate_target_system(self) -> Option<SubstrateTargetSystem> {
        match self {
            TargetSystem::Substrate(target_system) => Some(target_system),
            _ => None,
        }
    }

    /// Turns `self` into a 32 byte array.
    #[must_use]
    pub fn into_fixed_bytes(self) -> [u8; 32] {
        let encode = |elt: &[u8]| {
            let mut buf = [0u8; 32];
            buf.iter_mut()
                .rev()
                .zip(elt.iter().rev())
                .for_each(|(a, b)| *a = *b);
            buf
        };
        match self {
            TargetSystem::ContractAddress(address) => encode(&address),
            TargetSystem::Substrate(target) => {
                let mut bytes = Vec::with_capacity(6);
                // add pallet index
                bytes.push(target.pallet_index);
                // add call index
                bytes.push(target.call_index);
                // add tree id
                bytes.extend_from_slice(&target.tree_id.to_be_bytes());
                encode(&bytes.as_slice())
            }
        }
    }
}

impl From<[u8; TargetSystem::LENGTH]> for TargetSystem {
    fn from(bytes: [u8; TargetSystem::LENGTH]) -> Self {
        // check the first 20 bytes are zeros.
        // if not, it is a contract address.
        if bytes[0..20].iter().all(|&x| x == 0) {
            let mut tree_id_bytes = [0u8; 4];
            let f = 22usize;
            let t = f + core::mem::size_of::<u32>();
            tree_id_bytes.copy_from_slice(&bytes[f..t]);
            let tree_id = u32::from_be_bytes(tree_id_bytes);
            let target = SubstrateTargetSystem::builder()
                .pallet_index(bytes[f - 2])
                .call_index(bytes[f - 1])
                .tree_id(tree_id)
                .build();
            TargetSystem::Substrate(target)
        } else {
            let mut address_bytes = [0u8; 20];
            let f = 6usize;
            let t = f + 20;
            address_bytes.copy_from_slice(&bytes[f..t]);
            TargetSystem::ContractAddress(address_bytes)
        }
    }
}

impl From<TargetSystem> for [u8; TargetSystem::LENGTH] {
    fn from(target_system: TargetSystem) -> Self {
        target_system.into_bytes()
    }
}

impl Default for TargetSystem {
    fn default() -> Self {
        let target = SubstrateTargetSystem::builder()
            .pallet_index(0)
            .call_index(0)
            .tree_id(0)
            .build();
        TargetSystem::Substrate(target)
    }
}
