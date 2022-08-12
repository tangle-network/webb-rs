use tiny_keccak::{Hasher, Keccak};

/// Convert the ink account address(32 bytes)
/// and contract address(32 bytes) to Webb proposal target system address(20
/// bytes).
///
/// Example:
///   Contract/account addr:  juno16e3t7td2wu0wmggnxa3xnyu5whljyed69ptvkp
///   Keccak256 hash:
/// 7164f72199667ca6301626c67e4dba0a5a2e4cd1703af65d04e3e566845a4f7c
///   Target addr(hex):       7e4dba0a5a2e4cd1703af65d04e3e566845a4f7c
pub fn ink_address_to_target_address(ink_address: [u8; 32]) -> [u8; 20] {
    let mut keccak = Keccak::v256();
    keccak.update(&ink_address);

    let mut output = [0u8; 32];
    keccak.finalize(&mut output);

    let mut target_addr: [u8; 20] = [0u8; 20];
    target_addr.copy_from_slice(&output[12..]);

    target_addr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proposal::ink::util::ink_address_to_target_address;

    #[test]
    fn ink_addr_conversion_works() {
        let ink_addr = [0u8; 32];
        let expected = "88386fc84ba6bc95484008f6362f93160ef3e563".to_string();

        let decoded = hex::decode(expected.clone()).unwrap();

        let target_addr = ink_address_to_target_address(ink_addr);

        assert_eq!(decoded, target_addr.to_vec());
    }
}
