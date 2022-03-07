//! The Proposal Header.

/// The Proposal Target System.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "scale", derive(scale_info::TypeInfo))]
pub enum TargetSystem {
    /// Ethereum Contract address (20 bytes).
    ContractAddress([u8; 20]),
    /// Webb Protocol Merkle `TreeId` (4 bytes).
    TreeId(u32),
}

/// Proposal Nonce (4 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "scale", derive(scale_info::TypeInfo))]
pub struct Nonce(u32);

/// Proposal Target Function Signature (4 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "scale", derive(scale_info::TypeInfo))]
pub struct FunctionSignature([u8; 4]);

/// Proposal Target `ResourceId` (32 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "scale", derive(scale_info::TypeInfo))]
pub struct ResourceId([u8; 32]);

/// Proposal Target `ChainType` (2 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[repr(u16)]
pub enum ChainType {
    /// Unknown chain type.
    ///
    /// This is used when the chain type is not known.
    Unknown = 0x0000,
    /// EVM Based Chain (Mainnet, Polygon, ...etc)
    Evm = 0x0100,
    /// Substrate Based Chain (Webb, Edgeware, ...etc)
    Substrate = 0x0200,
    /// Polkadot Relay Chain.
    PolkadotRelayChain = 0x0301,
    /// Kusama Relay Chain.
    KusamaRelayChain = 0x0302,
    /// CosmWasm Contract.
    Cosmos = 0x0400,
    /// Solana Program.
    Solana = 0x0500,
}

/// Proposal Target `ChainId` (4 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "scale", derive(scale_info::TypeInfo))]
pub struct ChainId(u32);

/// Proposal Header (40 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[allow(clippy::module_name_repetitions)]
pub struct ProposalHeader {
    resource_id: ResourceId,
    function_signature: FunctionSignature,
    nonce: Nonce,
}

impl ChainType {
    /// Length of the `ChainType` in bytes.
    pub const LENGTH: usize = core::mem::size_of::<u16>();

    /// Convert `ChainType` to `u16`.
    #[must_use]
    pub const fn to_u16(self) -> u16 {
        match self {
            ChainType::Evm => 0x0100,
            ChainType::Substrate => 0x0200,
            ChainType::PolkadotRelayChain => 0x0301,
            ChainType::KusamaRelayChain => 0x0302,
            ChainType::Cosmos => 0x0400,
            ChainType::Solana => 0x0500,
            ChainType::Unknown => 0x0000,
        }
    }

    /// Get the underlying bytes of `ChainType`.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; Self::LENGTH] {
        self.to_u16().to_be_bytes()
    }
    /// Get the underlying bytes of `ChainType`.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<ChainType> for [u8; ChainType::LENGTH] {
    fn from(chain_type: ChainType) -> Self {
        chain_type.to_bytes()
    }
}

impl From<[u8; ChainType::LENGTH]> for ChainType {
    fn from(bytes: [u8; ChainType::LENGTH]) -> Self {
        let v = u16::from_be_bytes(bytes);
        match v {
            0x0100 => ChainType::Evm,
            0x0200 => ChainType::Substrate,
            0x0301 => ChainType::PolkadotRelayChain,
            0x0302 => ChainType::KusamaRelayChain,
            0x0400 => ChainType::Cosmos,
            0x0500 => ChainType::Solana,
            _ => ChainType::Unknown,
        }
    }
}

#[cfg(feature = "scale")]
impl scale_codec::Encode for ChainType {
    fn size_hint(&self) -> usize {
        Self::LENGTH
    }

    fn encode_to<T: scale_codec::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.to_bytes());
    }
}

#[cfg(feature = "scale")]
impl scale_codec::Decode for ChainType {
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

impl ChainId {
    /// Length of the `ChainId` in bytes.
    pub const LENGTH: usize = core::mem::size_of::<u32>();

    /// Convert `ChainId` to `u32`.
    #[must_use]
    pub const fn to_u32(self) -> u32 {
        self.0
    }

    /// Get the underlying bytes of `ChainId`.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; Self::LENGTH] {
        self.0.to_be_bytes()
    }
    /// Get the underlying bytes of `ChainId`.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<ChainId> for [u8; ChainId::LENGTH] {
    fn from(chain_id: ChainId) -> Self {
        chain_id.to_bytes()
    }
}

impl From<[u8; ChainId::LENGTH]> for ChainId {
    fn from(bytes: [u8; ChainId::LENGTH]) -> Self {
        let v = u32::from_be_bytes(bytes);
        ChainId(v)
    }
}

impl From<u32> for ChainId {
    fn from(v: u32) -> Self {
        ChainId(v)
    }
}

impl From<ChainId> for u32 {
    fn from(chain_id: ChainId) -> Self {
        chain_id.0
    }
}

#[cfg(feature = "scale")]
impl scale_codec::Encode for ChainId {
    fn size_hint(&self) -> usize {
        Self::LENGTH
    }

    fn encode_to<T: scale_codec::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.to_bytes());
    }
}

#[cfg(feature = "scale")]
impl scale_codec::Decode for ChainId {
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

impl ResourceId {
    /// Length of the `ResourceId` (32 bytes).
    pub const LENGTH: usize = 32;
    /// Create a new `ResourceId`.
    #[must_use]
    pub fn new(
        target_system: TargetSystem,
        chain_type: ChainType,
        chain_id: ChainId,
    ) -> Self {
        let mut bytes = [0u8; Self::LENGTH];
        let target_system_bytes: [u8; TargetSystem::LENGTH] =
            target_system.into();
        let f = 0;
        let t = TargetSystem::LENGTH;
        bytes[f..t].copy_from_slice(&target_system_bytes);
        let f = t;
        let t = t + ChainType::LENGTH;
        bytes[f..t].copy_from_slice(&chain_type.into_bytes());
        let f = t;
        let t = t + ChainId::LENGTH;
        bytes[f..t].copy_from_slice(&chain_id.into_bytes());
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

    /// Get the [`ChainType`] from the `ResourceId`.
    ///
    /// The `ChainType` is the 27th & 28th bytes of the `ResourceId`.
    ///
    /// **Note**: This will return [`ChainType::Unknown`] if the `ChainType` is
    /// not known. which could be the case for a proposal specification
    /// changed in the future without updating this crate.
    #[must_use]
    pub fn chain_type(&self) -> ChainType {
        let mut bytes = [0u8; 2];
        let f = TargetSystem::LENGTH;
        let t = f + ChainType::LENGTH;
        bytes.copy_from_slice(&self.0[f..t]);
        ChainType::from(bytes)
    }

    /// Get the [`ChainId`] from the `ResourceId`.
    ///
    /// The `ChainId` is the last 4 bytes of the `ResourceId`.
    #[must_use]
    pub fn chain_id(&self) -> ChainId {
        let mut bytes = [0u8; 4];
        let f = TargetSystem::LENGTH + ChainType::LENGTH;
        let t = f + ChainId::LENGTH;
        bytes.copy_from_slice(&self.0[f..t]);
        ChainId::from(bytes)
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

#[cfg(feature = "scale")]
impl scale_codec::Encode for ResourceId {
    fn size_hint(&self) -> usize {
        Self::LENGTH
    }

    fn encode_to<T: scale_codec::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.to_bytes());
    }
}

#[cfg(feature = "scale")]
impl scale_codec::Decode for ResourceId {
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

#[cfg(feature = "scale")]
impl scale_codec::Encode for TargetSystem {
    fn size_hint(&self) -> usize {
        Self::LENGTH
    }

    fn encode_to<T: scale_codec::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.to_bytes());
    }
}

#[cfg(feature = "scale")]
impl scale_codec::Decode for TargetSystem {
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

#[cfg(feature = "scale")]
impl scale_codec::Encode for FunctionSignature {
    fn size_hint(&self) -> usize {
        Self::LENGTH
    }

    fn encode_to<T: scale_codec::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.to_bytes());
    }
}

#[cfg(feature = "scale")]
impl scale_codec::Decode for FunctionSignature {
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

#[cfg(feature = "scale")]
impl scale_codec::Encode for Nonce {
    fn size_hint(&self) -> usize {
        Self::LENGTH
    }

    fn encode_to<T: scale_codec::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(&self.to_bytes());
    }
}

#[cfg(feature = "scale")]
impl scale_codec::Decode for Nonce {
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
        let target_chain = ChainType::Evm;
        let target_chain_id = ChainId::from(4);
        let resource_id =
            ResourceId::new(target_system, target_chain, target_chain_id);
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
        assert_eq!(resource_id.chain_type(), ChainType::Evm);
        assert_eq!(resource_id.chain_id(), ChainId::from(4));
    }

    #[test]
    fn encode_header() {
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain = ChainType::Evm;
        let target_chain_id = ChainId::from(4);
        let resource_id =
            ResourceId::new(target_system, target_chain, target_chain_id);
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
                ChainType::Evm,
                ChainId::from(4)
            )
        );
        assert_eq!(
            header.function_signature(),
            FunctionSignature::new(hex_literal::hex!("f00dbabe"))
        );
        assert_eq!(header.nonce(), Nonce::from(0x0001));
    }
}
