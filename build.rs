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
            .rustfmt(false) // don't use rustfmt for now.
            .generate()?
            .write_to_file(out)?;
        Ok(())
    }

    pub fn build_tornado_contract() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/tornado/Tornado.json",
            "src/evm/contract/tornado/contract.rs",
            "TornadoContract",
        )
    }

    pub fn build_darkwebb_anchor() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/darkwebb/Anchor.json",
            "src/evm/contract/darkwebb/anchor.rs",
            "AnchorContract",
        )
    }

    pub fn build_darkwebb_anchor_proxy() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/darkwebb/AnchorProxy.json",
            "src/evm/contract/darkwebb/anchor_proxy.rs",
            "AnchorProxyContract",
        )
    }

    pub fn build_darkwebb_bridge() -> Result<(), Box<dyn Error>> {
        parse_and_write_abigen(
            "contracts/darkwebb/Bridge.json",
            "src/evm/contract/darkwebb/bridge.rs",
            "BridgeContract",
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
        let runtime_api =
            generator.generate_runtime(item_mod, Default::default());
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

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "generate-contracts")]
    {
        evm::build_tornado_contract()?;
        evm::build_darkwebb_anchor()?;
        evm::build_darkwebb_anchor_proxy()?;
        evm::build_darkwebb_bridge()?;
    }
    #[cfg(feature = "generate-substrate")]
    {
        substrate::generate_dkg_runtime()?;
        substrate::generate_protocol_substrate_runtime()?;
    }
    Ok(())
}
