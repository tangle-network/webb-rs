use std::error::Error;
use std::io::Write;

use ethers::contract::Abigen;
use tempfile::NamedTempFile;

fn main() -> Result<(), Box<dyn Error>> {
    let anchor_contract_file =
        std::fs::read_to_string("contracts/Anchor.json")?;
    let anchor_raw: serde_json::Value =
        serde_json::from_str(&anchor_contract_file)?;
    let anchor_abi = serde_json::to_vec(&anchor_raw["abi"])?;
    let mut anchor_abi_file = NamedTempFile::new()?;
    anchor_abi_file.write_all(&anchor_abi)?;
    Abigen::new("AnchorContract", anchor_abi_file.path().to_string_lossy())?
        .rustfmt(true)
        .generate()?
        .write_to_file("src/evm/contract/anchor.rs")?;
    Ok(())
}
