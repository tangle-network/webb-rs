use tiny_keccak::{Hasher, Keccak};

/// Convert the cosmos-sdk chain(like Juno, Terra) account address(43 ~ 45 bytes)
/// and contract address(63 ~ 65 bytes) to Webb proposal target system address(20 bytes).
///
/// Example:
///   Contract/account addr:  juno16e3t7td2wu0wmggnxa3xnyu5whljyed69ptvkp
///   Keccak256 hash:         7164f72199667ca6301626c67e4dba0a5a2e4cd1703af65d04e3e566845a4f7c
///   Target addr(hex):       7e4dba0a5a2e4cd1703af65d04e3e566845a4f7c
///
pub fn cosmos_address_to_target_address(bech32_addr_string: &str) -> [u8; 20] {
    let mut keccak = Keccak::v256();
    keccak.update(bech32_addr_string.as_bytes());

    let mut output = [0u8; 32];
    keccak.finalize(&mut output);

    let mut target_addr: [u8; 20] = [0u8; 20];
    target_addr.copy_from_slice(&output[12..]);

    target_addr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosmos_addr_conversion_works() {
        let cosmos_addr =
            "juno16e3t7td2wu0wmggnxa3xnyu5whljyed69ptvkp".to_string();
        let expected = "7e4dba0a5a2e4cd1703af65d04e3e566845a4f7c".to_string();
        let decoded = hex::decode(expected).unwrap();

        let target_addr = cosmos_address_to_target_address(&cosmos_addr);

        assert_eq!(decoded, target_addr.to_vec());
    }
}
