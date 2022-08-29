use std::error::Error;

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
            .add_event_derive("serde::Serialize")
            .add_event_derive("serde::Deserialize")
            .rustfmt(false) // don't use rustfmt for now.
            .generate()?
            .write_to_file(out)?;
        Ok(())
    }

    pub fn build_protocol_solidity_anchor_base() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/AnchorBase.json",
            "src/evm/contract/protocol_solidity/anchor_base.rs",
            "AnchorBaseContract",
        )
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

    pub fn build_protocol_solidity_governed_token_wrapper(
    ) -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/GovernedTokenWrapper.json",
            "src/evm/contract/protocol_solidity/governed_token_wrapper.rs",
            "GovernedTokenWrapperContract",
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
}

#[cfg(feature = "generate-substrate")]
mod substrate {
    use std::io::Read;

    use frame_metadata::RuntimeMetadataPrefixed;
    use scale::Decode;

    use super::*;

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
        let generator = subxt_codegen::RuntimeGenerator::new(metadata);
        let item_mod = syn::parse_quote!(
            pub mod api {}
        );
        let mut generated_type_derives =
            subxt_codegen::DerivesRegistry::default();
        generated_type_derives.extend_for_all(
            vec![
                syn::parse_quote!(Eq),
                syn::parse_quote!(PartialEq),
                syn::parse_quote!(Clone),
            ]
            .into_iter(),
        );
        let runtime_api =
            generator.generate_runtime(item_mod, generated_type_derives);
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

    pub fn generate_egg_runtime() -> Result<(), Box<dyn Error>> {
        parse_and_generate_runtime(
            "metadata/egg-runtime.scale",
            "src/substrate/egg_runtime.rs",
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
        evm::build_protocol_solidity_anchor_base()?;
        evm::build_protocol_solidity_vanchor_base()?;
        evm::build_protocol_solidity_vanchor()?;
        evm::build_protocol_solidity_anchor_handler()?;
        evm::build_protocol_solidity_signature_bridge()?;
        evm::build_protocol_solidity_token_wrapper()?;
        evm::build_protocol_solidity_governed_token_wrapper()?;
        evm::build_protocol_solidity_token_wrapper_handler()?;
        evm::build_protocol_solidity_treasury()?;
        evm::build_protocol_solidity_treasury_handler()?;
        run_cargo_fmt()?;
    }
    #[cfg(feature = "generate-substrate")]
    {
        substrate::generate_dkg_runtime()?;
        substrate::generate_protocol_substrate_runtime()?;
        substrate::generate_egg_runtime()?;
        run_cargo_fmt()?;
    }
    Ok(())
}
