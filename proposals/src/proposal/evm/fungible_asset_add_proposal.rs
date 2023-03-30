//! Fungible Token Add Proposal.
use crate::ProposalHeader;

/// Fungible Asset Add Proposal.
///
/// The [`FungibleAssetAddProposal`] allows a new wrapped fungible token to be transacted into the
/// masp pool
///
/// The format of the proposal looks like:
/// ```text
/// ┌────────────────────┬──────────────────┬────────────┬──────────┬────────────┬
/// │                    │                  │            │          │            │
/// │ ProposalHeader 40B │ TokenHandler 20B │ AssetId 4B │ Name 32B │ Symbol 32B │
/// │                    │                  │            │          │            │
/// └────────────────────┴──────────────────┴────────────┴──────────┴────────────┴
/// ```
#[allow(clippy::module_name_repetitions)]
// #[derive(
//     Debug, Copy, Clone, PartialEq, Eq, Hash, typed_builder::TypedBuilder,
// )]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FungibleAssetAddProposal {
    header: ProposalHeader,
    token_handler: [u8; 20],
    asset_id: [u8; 4],
    name: [u8; 32],
    symbol: [u8; 32],
}

impl FungibleAssetAddProposal {
    /// Length of the proposal in bytes.
    pub const LENGTH: usize = ProposalHeader::LENGTH + 20 + 4 + 32 + 32; // token_address

    /// Creates a new fungible asset add proposal.
    #[must_use]
    pub const fn new(
        header: ProposalHeader,
        token_handler: [u8; 20],
        asset_id: [u8; 4],
        name: [u8; 32],
        symbol: [u8; 32],
    ) -> Self {
        Self {
            header,
            token_handler,
            asset_id,
            name,
            symbol,
        }
    }

    /// Get the proposal header.
    #[must_use]
    pub const fn header(&self) -> ProposalHeader {
        self.header
    }

    /// Get the token handler.
    #[must_use]
    pub const fn token_handler(&self) -> [u8; 20] {
        self.token_handler
    }

    /// Get the asset id.
    #[must_use]
    pub const fn asset_id(&self) -> [u8; 4] {
        self.asset_id
    }

    /// Get the token name.
    #[must_use]
    pub const fn name(&self) -> [u8; 32] {
        self.name
    }

    /// Get the token symbol.
    #[must_use]
    pub const fn symbol(&self) -> [u8; 32] {
        self.symbol
    }

    /// Get the proposal as a bytes
    #[must_use]
    pub fn to_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        bytes[f..t].copy_from_slice(&self.header.to_bytes());
        let f = t;
        let t = t + 20;
        bytes[f..t].copy_from_slice(&self.token_handler);
        let f = t;
        let t = t + 4;
        bytes[f..t].copy_from_slice(&self.asset_id);
        let f = t;
        let t = t + 32;
        bytes[f..t].copy_from_slice(&self.name);
        let f = t;
        let t = t + 32;
        bytes[f..t].copy_from_slice(&self.symbol);
        bytes
    }

    /// Get the proposal as a bytes without copying.
    #[must_use]
    pub fn into_bytes(self) -> [u8; Self::LENGTH] {
        self.to_bytes()
    }
}

impl From<[u8; FungibleAssetAddProposal::LENGTH]> for FungibleAssetAddProposal {
    fn from(bytes: [u8; FungibleAssetAddProposal::LENGTH]) -> Self {
        let f = 0usize;
        let t = ProposalHeader::LENGTH;
        let mut header_bytes = [0u8; ProposalHeader::LENGTH];
        header_bytes.copy_from_slice(&bytes[f..t]);
        let header = ProposalHeader::from(header_bytes);
        let f = t;
        let t = t + 20;
        let mut token_handler = [0u8; 20];
        token_handler.copy_from_slice(&bytes[f..t]);
        let f = t;
        let t = t + 4;
        let mut asset_id = [0u8; 4];
        asset_id.copy_from_slice(&bytes[f..t]);
        let f = t;
        let t = t + 32;
        let mut name = [0u8; 32];
        name.copy_from_slice(&bytes[f..t]);
        let f = t;
        let t = t + 32;
        let mut symbol = [0u8; 32];
        symbol.copy_from_slice(&bytes[f..t]);

        Self::new(header, token_handler, asset_id, name, symbol)
    }
}

impl From<FungibleAssetAddProposal> for [u8; FungibleAssetAddProposal::LENGTH] {
    fn from(proposal: FungibleAssetAddProposal) -> Self {
        proposal.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        FunctionSignature, Nonce, ResourceId, TargetSystem, TypedChainId,
    };

    use super::*;

    #[test]
    fn encode() {
        let target_system = TargetSystem::new_contract_address(
            hex_literal::hex!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        );
        let target_chain = TypedChainId::Evm(4);
        let resource_id = ResourceId::new(target_system, target_chain);
        let function_signature =
            FunctionSignature::new(hex_literal::hex!("cafebabe"));
        let nonce = Nonce::from(0x0001);
        let header =
            ProposalHeader::new(resource_id, function_signature, nonce);

        let token_handler =
            hex_literal::hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
        let asset_id = hex_literal::hex!("00000000");
        let name = hex_literal::hex!(
            "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
        );
        let symbol = hex_literal::hex!(
            "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
        );
        let proposal = FungibleAssetAddProposal::new(
            header,
            token_handler,
            asset_id,
            name,
            symbol,
        );
        let bytes = proposal.to_bytes();
        let expected = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe00000001bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb00000000"
            "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
            "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
        );
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decode() {
        let bytes = hex_literal::hex!(
            "000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa010000000004"
            "cafebabe00000001bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb00000000"
            "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
            "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
        );
        let proposal = FungibleAssetAddProposal::from(bytes);

        let header = proposal.header();
        let resource_id = header.resource_id();
        let target_system = resource_id.target_system();
        let target_chain = resource_id.typed_chain_id();
        let function_signature = header.function_signature();
        let nonce = header.nonce();
        assert_eq!(
            target_system,
            TargetSystem::new_contract_address(hex_literal::hex!(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            ))
        );
        assert_eq!(target_chain, TypedChainId::Evm(4));
        assert_eq!(
            function_signature,
            FunctionSignature::new(hex_literal::hex!("cafebabe"))
        );
        assert_eq!(nonce, Nonce::from(0x0001));
        assert_eq!(
            proposal.token_handler(),
            hex_literal::hex!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
        );
        assert_eq!(proposal.asset_id(), hex_literal::hex!("00000000"));
        assert_eq!(
            proposal.name(),
            hex_literal::hex!("cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc")
        );
        assert_eq!(
            proposal.symbol(),
            hex_literal::hex!("dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd")
        );
    }
}
