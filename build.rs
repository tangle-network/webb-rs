use std::error::Error;

#[cfg(feature = "generate-contracts")]
mod evm {
    use std::io::Write;

    use ethers::contract::Abigen;
    use tempfile::NamedTempFile;

    use super::*;

    fn parse_and_write_abigen(
        path: &str,
        out: &str,
        contract_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("cargo:rerun-if-changed=./{}", path);
        println!("cargo:rerun-if-changed=./{}", out);

        let contract_file = std::fs::read_to_string(path)?;
        let raw: serde_json::Value = serde_json::from_str(&contract_file)?;
        let abi = serde_json::to_vec(&raw["abi"])?;
        let mut abi_file = NamedTempFile::new()?;
        abi_file.write_all(&abi)?;
        Abigen::new(contract_name, abi_file.path().to_string_lossy())?
            .add_event_derive("serde::Serialize")
            .add_event_derive("serde::Deserialize")
            .rustfmt(false) // don't use rustfmt for now.
            .generate()?
            .write_to_file(out)?;
        Ok(())
    }

    pub fn build_protocol_solidity_anchor() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/protocol-solidity/FixedDepositAnchor.json",
            "src/evm/contract/protocol_solidity/fixed_deposit_anchor.rs",
            "FixedDepositAnchorContract",
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

    pub fn build_protocol_solidity_anchor_proxy() -> Result<(), Box<dyn Error>>
    {
        parse_and_write_abigen(
            "contracts/protocol-solidity/AnchorProxy.json",
            "src/evm/contract/protocol_solidity/anchor_proxy.rs",
            "AnchorProxyContract",
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
            subxt_codegen::GeneratedTypeDerives::default();
        generated_type_derives.append(
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
        evm::build_protocol_solidity_anchor()?;
        evm::build_protocol_solidity_vanchor()?;
        evm::build_protocol_solidity_anchor_proxy()?;
        evm::build_protocol_solidity_anchor_handler()?;
        evm::build_protocol_solidity_signature_bridge()?;
        run_cargo_fmt()?;
    }
    #[cfg(feature = "generate-substrate")]
    {
        substrate::generate_dkg_runtime()?;
        substrate::generate_protocol_substrate_runtime()?;
        run_cargo_fmt()?;
    }
    Ok(())
}
