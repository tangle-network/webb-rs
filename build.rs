use std::error::Error;

use std::io::Write;

use ethers::contract::Abigen;
use tempfile::NamedTempFile;

fn parse_and_write_abigen(
    path: &str,
    out: &str,
    contract_name: &str,
) -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed={}", path);
    println!("cargo:rerun-if-changed={}", out);

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
        "contracts/tornado/Anchor.json",
        "src/evm/contract/tornado/anchor.rs",
        "AnchorContract",
    )
}

fn build_darkwebb_anchor() -> Result<(), Box<dyn Error>> {
    parse_and_write_abigen(
        "contracts/darkwebb/Anchor2.json",
        "src/evm/contract/darkwebb/anchor2.rs",
        "Anchor2Contract",
    )
}

fn build_darkwebb_webb_anchor() -> Result<(), Box<dyn Error>> {
    parse_and_write_abigen(
        "contracts/darkwebb/WEBBAnchor2.json",
        "src/evm/contract/darkwebb/webb_anchor2.rs",
        "WEBBAnchor2Contract",
    )
}

fn build_darkwebb_bridge() -> Result<(), Box<dyn Error>> {
    parse_and_write_abigen(
        "contracts/darkwebb/Bridge.json",
        "src/evm/contract/darkwebb/bridge.rs",
        "BridgeContract",
    )
}

fn build_gov_bravo() -> Result<(), Box<dyn Error>> {
    parse_and_write_abigen(
        "contracts/darkwebb/GovernanceBravoDelegate.json",
        "src/evm/contract/darkwebb/governance_bravo_delegate.rs",
        "GovernanceBravoDelegateContract",
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    build_tornado_anchor()?;
    build_darkwebb_anchor()?;
    build_darkwebb_webb_anchor()?;
    build_darkwebb_bridge()?;
    build_gov_bravo()?;
    Ok(())
}
