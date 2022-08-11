//! The Proposal Header.

use crate::nonce::Nonce;

/// The Proposal Target System.
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
    /// Webb Protocol Merkle `TreeId` (4 bytes).
    TreeId(u32),
}

impl TargetSystem {
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
            TargetSystem::TreeId(tree_id) => encode(&tree_id.to_be_bytes()),
        }
    }
}

/// Proposal Target Function Signature (4 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "scale",
    derive(
        scale_info::TypeInfo,
        scale_codec::Encode,
        scale_codec::Decode,
        scale_codec::MaxEncodedLen
    )
)]
pub struct FunctionSignature(pub [u8; 4]);

/// Proposal Target `ResourceId` (32 bytes).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "scale",
    derive(
        scale_info::TypeInfo,
        scale_codec::Encode,
        scale_codec::Decode,
        scale_codec::MaxEncodedLen
    )
)]
pub struct ResourceId(pub [u8; 32]);

/// Proposal Target Chain and its type (6 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "scale",
    derive(
        scale_info::TypeInfo,
        scale_codec::Encode,
        scale_codec::Decode,
        scale_codec::MaxEncodedLen
    )
)]
pub enum TypedChainId {
    /// None chain type.
    ///
    /// This is used when the chain type is not known.
    None,
    /// EVM Based Chain (Mainnet, Polygon, ...etc)
    Evm(u32),
    /// Standalone Substrate Based Chain (Webb, Edgeware, ...etc)
    Substrate(u32),
    /// Polkadot Parachains.
    PolkadotParachain(u32),
    /// Kusama Parachains.
    KusamaParachain(u32),
    /// Rococo Parachains.
    RococoParachain(u32),
    /// Cosmos / CosmWasm Chains.
    Cosmos(u32),
    /// Solana Program.
    Solana(u32),
    /// Ink Based Chains
    Ink(u32),
}

impl Default for TypedChainId {
    fn default() -> Self {
        TypedChainId::None
    }
}

/// Proposal Header (40 bytes).
/// ┌────────────────────┬─────────────────┬───────────────┐
/// │                    │                 │               │
/// │   Resource ID 32B  │ Function Sig 4B │    Nonce 4B   │
/// │                    │                 │               │
/// └────────────────────┴─────────────────┴───────────────┘
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[allow(clippy::module_name_repetitions)]
pub struct ProposalHeader {
    /// Resource ID of the execution context
    pub resource_id: ResourceId,
    /// Function signature / identifier
    pub function_signature: FunctionSignature,
    /// Nonce for proposal execution
    pub nonce: Nonce,
}

impl TypedChainId {
    /// Length of the [`TypedChainId`] in bytes.
    pub const LENGTH: usize = 6;

    /// Get the chain id as a `u64`. This represents
    /// the typed chain ID that should be used to differentiate
    /// between differently typed chains with the same underlying
    /// chain id.
    #[must_use]
    pub fn chain_id(&self) -> u64 {
        let mut buf: [u8; 8] = [0u8; 8];
        buf[2..8].copy_from_slice(&self.to_bytes());
        u64::from_be_bytes(buf)
    }

    /// Get the chain id as a `u32`. This represents
    /// the un-typed underlying chain ID for the chain.
    #[must_use]
    pub const fn underlying_chain_id(&self) -> u32 {
        match self {
            TypedChainId::Evm(id)
            | TypedChainId::Substrate(id)
            | TypedChainId::PolkadotParachain(id)
            | TypedChainId::KusamaParachain(id)
            | TypedChainId::RococoParachain(id)
            | TypedChainId::Cosmos(id)
            | TypedChainId::Solana(id)
            | TypedChainId::Ink(id) => *id,
            Self::None => 0,
        }
    }

    /// Get the underlying bytes of `ChainType`.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];
        match self {
            TypedChainId::Evm(id) => {
                bytes[0..2].copy_from_slice(&(0x0100u16).to_be_bytes());
                bytes[2..6].copy_from_slice(&id.to_be_bytes());
            }
            TypedChainId::Substrate(id) => {
                bytes[0..2].copy_from_slice(&(0x0200u16).to_be_bytes());
                bytes[2..6].copy_from_slice(&id.to_be_bytes());
            }
            TypedChainId::PolkadotParachain(id) => {
                bytes[0..2].copy_from_slice(&(0x0301u16).to_be_bytes());
                bytes[2..6].copy_from_slice(&id.to_be_bytes());
            }
            TypedChainId::KusamaParachain(id) => {
                bytes[0..2].copy_from_slice(&(0x0302u16).to_be_bytes());
                bytes[2..6].copy_from_slice(&id.to_be_bytes());
            }
            TypedChainId::RococoParachain(id) => {
                bytes[0..2].copy_from_slice(&(0x0303u16).to_be_bytes());
                bytes[2..6].copy_from_slice(&id.to_be_bytes());
            }
            TypedChainId::Cosmos(id) => {
                bytes[0..2].copy_from_slice(&(0x0400u16).to_be_bytes());
                bytes[2..6].copy_from_slice(&id.to_be_bytes());
            }
            TypedChainId::Solana(id) => {
                bytes[0..2].copy_from_slice(&(0x0500u16).to_be_bytes());
                bytes[2..6].copy_from_slice(&id.to_be_bytes());
            }
            TypedChainId::Ink(id) => {
                bytes[0..2].copy_from_slice(&(0x0600u16).to_be_bytes());
                bytes[2..6].copy_from_slice(&id.to_be_bytes());
            }
            TypedChainId::None => {
                bytes[0..2].copy_from_slice(&(0x0000u16).to_be_bytes());
                bytes[2..6].copy_from_slice(&0u32.to_be_bytes());
            }
        }
        bytes
    }
    /// Get the underlying bytes of `ChainType`.
    #[must_use]
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<TypedChainId> for [u8; TypedChainId::LENGTH] {
    fn from(v: TypedChainId) -> Self {
        v.into_bytes()
    }
}

impl From<[u8; Self::LENGTH]> for TypedChainId {
    fn from(bytes: [u8; Self::LENGTH]) -> Self {
        let ty = [bytes[0], bytes[1]];
        let ty = u16::from_be_bytes(ty);
        let id = u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]);
        match ty {
            0x0100 => TypedChainId::Evm(id),
            0x0200 => TypedChainId::Substrate(id),
            0x0301 => TypedChainId::PolkadotParachain(id),
            0x0302 => TypedChainId::KusamaParachain(id),
            0x0303 => TypedChainId::RococoParachain(id),
            0x0400 => TypedChainId::Cosmos(id),
            0x0500 => TypedChainId::Solana(id),
            0x0600 => TypedChainId::Ink(id),
            _ => Self::None,
        }
    }
}

impl From<[u8; Self::LENGTH + 2]> for TypedChainId {
    fn from(bytes: [u8; Self::LENGTH + 2]) -> Self {
        let ty = [bytes[2], bytes[3]];
        let ty = u16::from_be_bytes(ty);
        let id = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        match ty {
            0x0100 => TypedChainId::Evm(id),
            0x0200 => TypedChainId::Substrate(id),
            0x0301 => TypedChainId::PolkadotParachain(id),
            0x0302 => TypedChainId::KusamaParachain(id),
            0x0303 => TypedChainId::RococoParachain(id),
            0x0400 => TypedChainId::Cosmos(id),
            0x0500 => TypedChainId::Solana(id),
            0x0600 => TypedChainId::Ink(id),
            _ => Self::None,
        }
    }
}

impl From<u64> for TypedChainId {
    fn from(val: u64) -> Self {
        TypedChainId::from(val.to_be_bytes())
    }
}

impl ResourceId {
    /// Length of the `ResourceId` (32 bytes).
    pub const LENGTH: usize = TargetSystem::LENGTH + TypedChainId::LENGTH;
    /// Create a new `ResourceId`.
    #[must_use]
    pub fn new(
        target_system: TargetSystem,
        typed_chain_id: TypedChainId,
    ) -> Self {
        let mut bytes = [0u8; Self::LENGTH];
        let target_system_bytes: [u8; TargetSystem::LENGTH] =
            target_system.into();
        let f = 0;
        let t = TargetSystem::LENGTH;
        bytes[f..t].copy_from_slice(&target_system_bytes);
        let f = t;
        let t = t + TypedChainId::LENGTH;
        bytes[f..t].copy_from_slice(&typed_chain_id.into_bytes());
        Self(bytes)
    }

    /// Get the [`TargetSystem`] from the `ResourceId`.
    ///
    /// The `TargetSystem` is the first 26 bytes of the `ResourceId`.
    #[must_use]
    pub fn target_system(&self) -> TargetSystem {
        let mut bytes = [0u8; TargetSystem::LENGTH];
        bytes.copy_from_slice(&self.0[0..TargetSystem::LENGTH]);
        TargetSystem::from(bytes)
    }

    /// Get the [`TypedChainId`] from the `ResourceId`.
    ///
    /// The `ChainType` is the 27th & 28th bytes of the `ResourceId`.
    /// Then Followed by the `ChainId`.
    #[must_use]
    pub fn typed_chain_id(&self) -> TypedChainId {
        let mut bytes = [0u8; TypedChainId::LENGTH];
        let f = TargetSystem::LENGTH;
        let t = f + TypedChainId::LENGTH;
        bytes.copy_from_slice(&self.0[f..t]);
        TypedChainId::from(bytes)
    }

    /// Get the underlying bytes of the `ResourceId`.
    #[must_use]
    pub const fn to_bytes(&self) -> [u8; Self::LENGTH] {
        self.0
    }

    /// Get the underlying bytes of the `ResourceId` without copying.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.0
    }
}

impl From<[u8; ResourceId::LENGTH]> for ResourceId {
    fn from(bytes: [u8; ResourceId::LENGTH]) -> Self {
        Self(bytes)
    }
}

impl From<ResourceId> for [u8; ResourceId::LENGTH] {
    fn from(resource_id: ResourceId) -> Self {
        resource_id.0
    }
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
    /// Create a new `TargetSystem` as a `TreeId`.
    #[must_use]
    pub fn new_tree_id(tree_id: u32) -> Self {
        Self::TreeId(tree_id)
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
            TargetSystem::TreeId(tree_id) => {
                let mut bytes = [0u8; TargetSystem::LENGTH];
                let f = 22usize;
                let t = f + core::mem::size_of::<u32>();
                bytes[f..t].copy_from_slice(&tree_id.to_be_bytes());
                bytes
            }
        }
    }

    /// Get the underlying bytes of the `TargetSystem` without copying.
    #[must_use]
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<[u8; TargetSystem::LENGTH]> for TargetSystem {
    fn from(bytes: [u8; TargetSystem::LENGTH]) -> Self {
        // check the first 22 bytes are zeros.
        // if not, it is a contract address.
        if bytes[0..22].iter().all(|&x| x == 0) {
            let mut tree_id_bytes = [0u8; 4];
            let f = 22usize;
            let t = f + core::mem::size_of::<u32>();
            tree_id_bytes.copy_from_slice(&bytes[f..t]);
            TargetSystem::TreeId(u32::from_be_bytes(tree_id_bytes))
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
        TargetSystem::TreeId(0)
    }
}

impl FunctionSignature {
    /// Length of the `FunctionSignature` (4 bytes).
    pub const LENGTH: usize = 4;
    /// Create a new `FunctionSignature`.
    #[must_use]
    pub const fn new(b: [u8; Self::LENGTH]) -> Self {
        Self(b)
    }

    /// Get the underlying bytes of the `FunctionSignature`.
    #[must_use]
    pub const fn to_bytes(&self) -> [u8; Self::LENGTH] {
        self.0
    }
    /// Get the underlying bytes of the `FunctionSignature` without copying.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.0
    }
}

impl From<[u8; FunctionSignature::LENGTH]> for FunctionSignature {
    fn from(bytes: [u8; FunctionSignature::LENGTH]) -> Self {
        Self(bytes)
    }
}

impl From<FunctionSignature> for [u8; FunctionSignature::LENGTH] {
    fn from(signature: FunctionSignature) -> Self {
        signature.0
    }
}

impl Nonce {
    /// Length of the `Nonce` (4 bytes).
    pub const LENGTH: usize = 4;
    /// Create a new `Nonce`.
    #[must_use]
    pub const fn new(n: u32) -> Self {
        Self(n)
    }

    /// Get the nonce as a `u32`.
    #[must_use]
    pub const fn to_u32(&self) -> u32 {
        self.0
    }

    /// Get the underlying value of the `Nonce`.
    #[must_use]
    pub const fn to_bytes(&self) -> [u8; Self::LENGTH] {
        self.0.to_be_bytes()
    }
    /// Get the underlying value of the `Nonce` without copying.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.0.to_be_bytes()
    }
}

impl From<u32> for Nonce {
    fn from(nonce: u32) -> Self {
        Self(nonce)
    }
}

impl From<Nonce> for u32 {
    fn from(nonce: Nonce) -> Self {
        nonce.0
    }
}

impl From<[u8; Nonce::LENGTH]> for Nonce {
    fn from(bytes: [u8; Nonce::LENGTH]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }
}

impl From<Nonce> for [u8; Nonce::LENGTH] {
    fn from(nonce: Nonce) -> Self {
        nonce.0.to_be_bytes()
    }
}

impl ProposalHeader {
    /// Length of the `ProposalHeader`.
    pub const LENGTH: usize =
        ResourceId::LENGTH + FunctionSignature::LENGTH + Nonce::LENGTH;
    /// Create a new `ProposalHeader`.
    #[must_use]
    pub const fn new(
        resource_id: ResourceId,
        function_signature: FunctionSignature,
        nonce: Nonce,
    ) -> Self {
        Self {
            resource_id,
            function_signature,
            nonce,
        }
    }
    /// Get the `ResourceId` from the `ProposalHeader`.
    #[must_use]
    pub fn resource_id(&self) -> ResourceId {
        self.resource_id
    }

    /// Get the `FunctionSignature` from the `ProposalHeader`.
    #[must_use]
    pub fn function_signature(&self) -> FunctionSignature {
        self.function_signature
    }

    /// Get the `Nonce` from the `ProposalHeader`.
    #[must_use]
    pub fn nonce(&self) -> Nonce {
        self.nonce
    }

    /// Get the underlying bytes of the `ProposalHeader`.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];
        let f = 0usize;
        let t = ResourceId::LENGTH;
        bytes[f..t].copy_from_slice(&self.resource_id.to_bytes());
        let f = t;
        let t = f + FunctionSignature::LENGTH;
        bytes[f..t].copy_from_slice(&self.function_signature.to_bytes());
        let f = t;
        let t = f + Nonce::LENGTH;
        bytes[f..t].copy_from_slice(&self.nonce.to_bytes());
        bytes
    }

    /// Get the underlying bytes of the `ProposalHeader` without copying.
    #[must_use]
    pub fn into_bytes(self) -> [u8; 40] {
        self.to_bytes()
    }
}

impl From<[u8; ProposalHeader::LENGTH]> for ProposalHeader {
    fn from(bytes: [u8; ProposalHeader::LENGTH]) -> Self {
        let mut resource_id_bytes = [0u8; ResourceId::LENGTH];
        let f = 0usize;
        let t = ResourceId::LENGTH;
        resource_id_bytes.copy_from_slice(&bytes[f..t]);
        let resource_id = ResourceId::from(resource_id_bytes);
        let mut function_signature_bytes = [0u8; FunctionSignature::LENGTH];
        let f = t;
        let t = f + FunctionSignature::LENGTH;
        function_signature_bytes.copy_from_slice(&bytes[f..t]);
        let function_signature =
            FunctionSignature::from(function_signature_bytes);
        let mut nonce_bytes = [0u8; Nonce::LENGTH];
        let f = t;
        let t = f + Nonce::LENGTH;
        nonce_bytes.copy_from_slice(&bytes[f..t]);
        let nonce = Nonce::from(nonce_bytes);
        Self {
            resource_id,
            function_signature,
            nonce,
        }
    }
}

impl From<ProposalHeader> for [u8; ProposalHeader::LENGTH] {
    fn from(header: ProposalHeader) -> Self {
        header.into_bytes()
    }
}

#[cfg(feature = "scale")]
impl scale_codec::Encode for ProposalHeader {
    fn size_hint(&self) -> usize {
        Self::LENGTH
    }

    fn encode_to<T: scale_codec::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.to_bytes());
    }
}

#[cfg(feature = "scale")]
impl scale_codec::Decode for ProposalHeader {
    fn decode<I: scale_codec::Input>(
        input: &mut I,
    ) -> Result<Self, scale_codec::Error> {
        let mut bytes = [0u8; Self::LENGTH];
        input.read(&mut bytes)?;
        Ok(Self::from(bytes))
    }

    fn encoded_fixed_size() -> Option<usize> {
        Some(Self::LENGTH)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_resource_id() {
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        assert_eq!(
            resource_id.to_bytes(),
            // first 6 bytes are zeros.
            // next is the target_system contract address.
            // then two bytes of the chain type.
            // lastly is the 4 bytes of the chain id.
            hex_literal::hex!(
                "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            )
        );
    }

    #[test]
    fn decode_resource_id() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
        );
        let resource_id = ResourceId::from(bytes);
        assert_eq!(
            resource_id.target_system(),
            TargetSystem::new_contract_address(hex_literal::hex!(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            ))
        );
        assert_eq!(resource_id.typed_chain_id(), TypedChainId::Evm(4));
        assert_eq!(resource_id.typed_chain_id().underlying_chain_id(), 4);
        assert_eq!(resource_id.typed_chain_id().chain_id(), 1_099_511_627_780);
    }

    #[test]
    fn from_bytes_typed_chain() {
        let typed_chain_id = TypedChainId::Evm(4);
        let bytes = typed_chain_id.to_bytes();
        let mut larger_bytes = [0u8; TypedChainId::LENGTH + 2];
        larger_bytes[2..].copy_from_slice(&bytes);
        let typed_chain_from_larger_bytes = TypedChainId::from(larger_bytes);
        let typed_chain_from_bytes = TypedChainId::from(bytes);
        let typed_chain_from_u64 =
            TypedChainId::from(typed_chain_id.chain_id());
        assert_eq!(typed_chain_from_bytes, typed_chain_from_larger_bytes);
        assert_eq!(typed_chain_from_bytes, typed_chain_id);
        assert_eq!(typed_chain_from_u64, typed_chain_id);
    }

    #[test]
    fn encode_header() {
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("f00dbabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);
        assert_eq!(
            header.to_bytes(),
            hex_literal::hex!(
                "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004f00dbabe00000001"
            )
        );
    }

    #[test]
    fn decode_header() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004f00dbabe00000001"
        );
        let header = ProposalHeader::from(bytes);
        assert_eq!(
            header.resource_id(),
            ResourceId::new(
                TargetSystem::new_contract_address(hex_literal::hex!(
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                )),
                TypedChainId::Evm(4)
            )
        );
        assert_eq!(
            header.function_signature(),
            FunctionSignature::new(hex_literal::hex!("f00dbabe"))
        );
        assert_eq!(header.nonce(), Nonce::from(0x0001));
    }
}
