use std::error::Error;

use std::io::Write;

use ethers::contract::Abigen;
use tempfile::NamedTempFile;

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

fn build_tornado_anchor() -> Result<(), Box<dyn Error>> {
    parse_and_write_abigen(
        "contracts/tornado/Tornado.json",
        "src/evm/contract/tornado/tornado.rs",
        "TornadoContract",
    )
}

fn build_darkwebb_anchor() -> Result<(), Box<dyn Error>> {
    parse_and_write_abigen(
        "contracts/darkwebb/Anchor.json",
        "src/evm/contract/darkwebb/anchor.rs",
        "AnchorContract",
    )
}

fn build_darkwebb_anchor_proxy() -> Result<(), Box<dyn Error>> {
    parse_and_write_abigen(
        "contracts/darkwebb/AnchorProxy.json",
        "src/evm/contract/darkwebb/anchor_proxy.rs",
        "AnchorProxyContract",
    )
}

fn build_darkwebb_bridge() -> Result<(), Box<dyn Error>> {
    parse_and_write_abigen(
        "contracts/darkwebb/Bridge.json",
        "src/evm/contract/darkwebb/bridge.rs",
        "BridgeContract",
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let generate_contracts_enabled =
        std::env::var_os("CARGO_FEATURE_GENERATE_CONTRACTS").is_some();
    if generate_contracts_enabled {
        build_tornado_anchor()?;
        build_darkwebb_anchor()?;
        build_darkwebb_anchor_proxy()?;
        build_darkwebb_bridge()?;
    }
    Ok(())
}
