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

        Abigen::new(contract_name, path)?
            .add_derive("serde::Serialize")?
            .add_derive("serde::Deserialize")?
            .format(false) // don't use rustfmt for now.
            .generate()?
            .write_to_file(out)?;
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

    pub fn build_protocol_solidity_masp() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/MultiAssetVAnchorBatchTree.json",
            "src/evm/contract/protocol_solidity/masp.rs",
            "MaspContract",
        )
    }

    pub fn build_protocol_solidity_masp_proxy() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/MultiAssetVAnchorProxy.json",
            "src/evm/contract/protocol_solidity/masp_proxy.rs",
            "MaspProxyContract",
        )
    }

    pub fn build_protocol_solidity_open_vanchor() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/OpenVAnchor.json",
            "src/evm/contract/protocol_solidity/open_variable_anchor.rs",
            "OpenVAnchorContract",
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
    use std::io::Read;

    use super::*;
    use frame_metadata::RuntimeMetadataPrefixed;
    use scale::Decode;
    use subxt_codegen::CratePath;

    fn parse_and_generate_runtime(
        path: &str,
        out: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("cargo:rerun-if-changed=./{}", path);
        let mut file = std::fs::File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        let metadata =
            <RuntimeMetadataPrefixed as Decode>::decode(&mut &bytes[..])?;
        // Module under which the API is generated.
        let item_mod = syn::parse_quote!(
            pub mod api {}
        );
        // Default type substitutes.
        let substs = TypeSubstitutes::new(&CratePath::default());
        // Generate the Runtime API.
        let generator = subxt_codegen::RuntimeGenerator::new(metadata);
        let mut generated_type_derives =
            subxt_codegen::DerivesRegistry::new(&CratePath::default());
        generated_type_derives.extend_for_all(
            vec![
                syn::parse_quote!(Eq),
                syn::parse_quote!(PartialEq),
                syn::parse_quote!(Clone),
            ]
            .into_iter(),
        );
        let runtime_api = generator.generate_runtime(
            item_mod,
            generated_type_derives,
            substs,
            CratePath::default(),
        );
        std::fs::write(out, runtime_api.to_string())?;
        Ok(())
    }

    pub fn generate_dkg_runtime() -> Result<(), Box<dyn Error>> {
        parse_and_generate_runtime(
            "metadata/dkg-runtime.scale",
            "src/substrate/dkg_runtime.rs",
        )
    }

    pub fn generate_protocol_substrate_runtime() -> Result<(), Box<dyn Error>> {
        parse_and_generate_runtime(
            "metadata/protocol-substrate-runtime.scale",
            "src/substrate/protocol_substrate_runtime.rs",
        )
    }

    pub fn generate_tangle_runtime() -> Result<(), Box<dyn Error>> {
        parse_and_generate_runtime(
            "metadata/tangle-runtime.scale",
            "src/substrate/tangle_runtime.rs",
        )
    }
}

#[cfg(any(feature = "generate-substrate", feature = "generate-contracts"))]
fn run_cargo_fmt() -> Result<(), Box<dyn Error>> {
    // Run rustfmt on all files in the `src` directory.
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("fmt").status()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "generate-contracts")]
    {
        evm::build_protocol_solidity_masp()?;
        evm::build_protocol_solidity_masp_proxy()?;
        evm::build_protocol_solidity_vanchor_base()?;
        evm::build_protocol_solidity_vanchor()?;
        evm::build_protocol_solidity_open_vanchor()?;
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
        run_cargo_fmt()?;
    }
    #[cfg(feature = "generate-substrate")]
    {
        substrate::generate_dkg_runtime()?;
        substrate::generate_protocol_substrate_runtime()?;
        substrate::generate_tangle_runtime()?;
        run_cargo_fmt()?;
    }
    Ok(())
}
