use std::error::Error;

#[cfg(feature = "generate-substrate")]
use subxt_codegen::TypeSubstitutes;

#[cfg(feature = "generate-contracts")]
mod evm {
    use super::*;
    use ethers::contract::Abigen;

    fn parse_and_write_abigen(
        path: &str,
        out: &str,
        contract_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("cargo:rerun-if-changed=./{}", path);
        println!("cargo:rerun-if-changed=./{}", out);

        let generated_tokens = Abigen::new(contract_name, path)?
            .add_derive("serde::Serialize")?
            .add_derive("serde::Deserialize")?
            .format(false) // don't use rustfmt for now.
            .generate()?
            .to_string();
        let syntax_tree = syn::parse_file(&generated_tokens).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        std::fs::write(out, formatted)?;
        Ok(())
    }

    pub fn build_protocol_solidity_vanchor_base() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/VAnchorBase.json",
            "src/evm/contract/protocol_solidity/vanchor_base.rs",
            "VAnchorBaseContract",
        )
    }

    pub fn build_protocol_solidity_vanchor() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/VAnchor.json",
            "src/evm/contract/protocol_solidity/variable_anchor.rs",
            "VAnchorContract",
        )
    }

    pub fn build_protocol_solidity_vanchor_tree() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/VAnchorTree.json",
            "src/evm/contract/protocol_solidity/variable_anchor_tree.rs",
            "VAnchorTreeContract",
        )
    }

    pub fn build_protocol_solidity_anchor_handler() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/AnchorHandler.json",
            "src/evm/contract/protocol_solidity/anchor_handler.rs",
            "AnchorHandlerContract",
        )
    }

    pub fn build_protocol_solidity_signature_bridge(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/SignatureBridge.json",
            "src/evm/contract/protocol_solidity/signature_bridge.rs",
            "SignatureBridgeContract",
        )
    }

    pub fn build_protocol_solidity_fungible_token_wrapper(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/FungibleTokenWrapper.json",
            "src/evm/contract/protocol_solidity/fungible_token_wrapper.rs",
            "FungibleTokenWrapperContract",
        )
    }

    pub fn build_protocol_solidity_token_wrapper() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/TokenWrapper.json",
            "src/evm/contract/protocol_solidity/token_wrapper.rs",
            "TokenWrapperContract",
        )
    }
    pub fn build_protocol_solidity_treasury() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/Treasury.json",
            "src/evm/contract/protocol_solidity/treasury.rs",
            "TreasuryContract",
        )
    }

    pub fn build_protocol_solidity_treasury_handler(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/TreasuryHandler.json",
            "src/evm/contract/protocol_solidity/treasury_handler.rs",
            "TreasuryHandlerContract",
        )
    }

    pub fn build_protocol_solidity_token_wrapper_handler(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/TokenWrapperHandler.json",
            "src/evm/contract/protocol_solidity/token_wrapper_handler.rs",
            "TokenWrapperHandlerContract",
        )
    }

    pub fn build_protocol_solidity_erc20_preset_minter_pauser(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/ERC20PresetMinterPauser.json",
            "src/evm/contract/protocol_solidity/erc20_preset_minter_pauser.rs",
            "ERC20PresetMinterPauserContract",
        )
    }

    pub fn build_protocol_solidity_poseidon_t3() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/PoseidonT3.json",
            "src/evm/contract/protocol_solidity/poseidon_t3.rs",
            "PoseidonT3Contract",
        )
    }

    pub fn build_protocol_solidity_poseidon_t4() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/PoseidonT4.json",
            "src/evm/contract/protocol_solidity/poseidon_t4.rs",
            "PoseidonT4Contract",
        )
    }

    pub fn build_protocol_solidity_poseidon_t6() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/PoseidonT6.json",
            "src/evm/contract/protocol_solidity/poseidon_t6.rs",
            "PoseidonT6Contract",
        )
    }

    pub fn build_protocol_solidity_poseidon_hasher(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/PoseidonHasher.json",
            "src/evm/contract/protocol_solidity/poseidon_hasher.rs",
            "PoseidonHasherContract",
        )
    }
}

#[cfg(feature = "generate-substrate")]
mod substrate {
    use super::*;
    use scale::Decode;
    use subxt_codegen::CratePath;

    fn parse_and_generate_runtime(
        path: &str,
        out: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("cargo:rerun-if-changed=./{}", path);
        let bytes = std::fs::read(path)?;

        let metadata =
            <subxt_metadata::Metadata as Decode>::decode(&mut &bytes[..])?;
        let crate_path = CratePath::default();
        // Module under which the API is generated.
        let item_mod = syn::parse_quote!(
            pub mod api {}
        );
        // Default type substitutes.
        let substs = TypeSubstitutes::with_default_substitutes(&crate_path);
        // Generate the Runtime API.
        let generator = subxt_codegen::RuntimeGenerator::new(metadata);
        let mut generated_type_derives =
            subxt_codegen::DerivesRegistry::with_default_derives(&crate_path);

        generated_type_derives.extend_for_all(
            [
                syn::parse_quote!(Eq),
                syn::parse_quote!(PartialEq),
                syn::parse_quote!(Clone),
            ]
            .into_iter(),
            [],
        );

        // Include metadata documentation in the Runtime API.
        let generate_docs = true;
        let runtime_api = generator.generate_runtime(
            item_mod,
            generated_type_derives,
            substs,
            crate_path,
            generate_docs,
        )?;
        let syntax_tree = syn::parse_file(&runtime_api.to_string()).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        std::fs::write(out, formatted)?;
        Ok(())
    }

    pub fn generate_tangle_runtime() -> Result<(), Box<dyn Error>> {
        parse_and_generate_runtime(
            "metadata/tangle-runtime.scale",
            "src/substrate/tangle_runtime.rs",
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "generate-contracts")]
    {
        evm::build_protocol_solidity_vanchor_base()?;
        evm::build_protocol_solidity_vanchor()?;
        evm::build_protocol_solidity_vanchor_tree()?;
        evm::build_protocol_solidity_anchor_handler()?;
        evm::build_protocol_solidity_signature_bridge()?;
        evm::build_protocol_solidity_token_wrapper()?;
        evm::build_protocol_solidity_fungible_token_wrapper()?;
        evm::build_protocol_solidity_token_wrapper_handler()?;
        evm::build_protocol_solidity_treasury()?;
        evm::build_protocol_solidity_treasury_handler()?;
        evm::build_protocol_solidity_erc20_preset_minter_pauser()?;
        evm::build_protocol_solidity_poseidon_t3()?;
        evm::build_protocol_solidity_poseidon_t4()?;
        evm::build_protocol_solidity_poseidon_t6()?;
        evm::build_protocol_solidity_poseidon_hasher()?;
    }
    #[cfg(feature = "generate-substrate")]
    {
        substrate::generate_tangle_runtime()?;
    }
    Ok(())
}
