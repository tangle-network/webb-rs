use ethers::types::{
    Bytes, EIP1186ProofResponse, StorageProof, H256, U256, U64,
};
use trie_db::proof::verify_proof;

fn parse_eth_get_proof(proof: &str) -> EIP1186ProofResponse {
    serde_json::from_str(proof).unwrap()
}

fn verify_account_proof(state_root: H256, address_hash: H256, proof: &str) {
    let parsed_proof = parse_eth_get_proof(proof);
    verify_proof(state_root, parsed_proof, items);
}

fn verify_storage_proof(proof: &str) {
    let parsed_proof = parse_eth_get_proof(proof);
    verify_proof(parsed_proof.storage_hash, parsed_proof, items);
}

#[cfg(test)]
mod tests {
    use super::{
        parse_eth_get_proof, verify_account_proof, verify_storage_proof,
    };
    use ethers::types::EIP1186ProofResponse;

    #[test]
    fn test_verify_eth_get_proof() {
        // Parse hardcoded Merkle trie proof
        let hardcoded_proof = r#"{"address":"0xbfce6b877ebff977bb6e80b24fbbb7bc4ebca4df","balance":"0x0","codeHash":"0x887da3cea6169f31edeb72a08a8ddb87ac755b6d39dbd9fe692e64db48d01d39","nonce":"0x1","storageHash":"0x19cbdcf7cb8bea9507a1f96a7611d588335446f8ca27802fba36ab7c198fcb44","accountProof":["0xf901f1a0b6bf0093a89fc7d11249b8f3d53a9842056cb2217da061bb418cb1a47964b603a0c530dbaa948dfccae8cd27d79e9fd1d71102af7c96a014a32513c3da8b08d0aea0bed1ed1fc5339c7f7959ef025dcede0c99fda88c996486db7647fe49a0f1b519a0b1de04907155ac9102090f82ae04b5716d1cf2bd4d8e2424917d90ee415532bca0de26cb1b4fd99c4d3ed75d4a67931e3c252605c7d68e0148d5327f341bfd5283a022d331fc7c624c391c29236882aad4c424cf90cfbd8e8beb941ddccb29f8106ba0196e0e1c4cd3799ba92dd2568734588a541e9b0d1f0c12bfe94732cd178e594da0c8f5daad4e5dc27440870850040efab5630eb2def2d789cd937c3dcf22192b6aa03dc477ff2f714970ac68cfb6689040748fc8b8e06e474f1a0b55f91dae1c13bfa0f896992a4638f44afcbc158006dee53f163c08f167dead4f4a54f80de6b82ae2a083bdd6e2b5e69cd159491c0927352b543f6d34903fcb1f3f57bed9e06a0a1c8e80a0cdd1b5c6429a44802664273998a18f0b8831aeb39a60a314a3bf926b341add93a0f08144a0299820b6ea6a478811cd96b1907650cf55989b70efa879e536b2a6eca0a9e6cc0d5192cb036c2454c7cf19ff53abf1861b50043a7b3713bc003a5a7d88a0e649a289933a420c1f201faaad3dccd15e13538b85f38e76b6d40b23da7a3e3180","0xf85180a0b86274a4b148d41ba047ae7aad1668f4507a417b5d4b582630d4023995b6f09580808080808080808080808080a00844a6824efe874b61c480f88d1b0639780aa16aefbad5647403fc7ea6c274b280","0xf869a020952e86c2ad89a0772acc940e2c5a0eec5c4d95a70affe1f0c5de2387f7c5c2b846f8440180a019cbdcf7cb8bea9507a1f96a7611d588335446f8ca27802fba36ab7c198fcb44a0887da3cea6169f31edeb72a08a8ddb87ac755b6d39dbd9fe692e64db48d01d39"],"storageProof":[{"key":"0xac33ff75c19e70fe83507db0d683fd3465c996598dc972688b7ace676c89077b","proof":["0xf901b1a097166a38c9e39f2d5b7eb7c3c32cc22396387dc550398c8db7721c3c1a478810a01575f87884ba38a75d8041f28fb756e5f59d1eb62c6d3558e172e69d3040ec4d80a0b6df00330786f17ea6907ff7761f9dd8d59535ce0f7ba33f8cafac8e94ec8e5280a043bb357d4ce407cfe9cc415600471aff852eeb3f7dd18f9956040fdfdf083f74a08c7ebea2ae63915497f3a74ea13f8bbe492ef31c148de6139d23f74d2e292513a024844e957318bcf4fd3da57118509a3588e450161832cafb65c0d5dd4594b09fa06a2f79212553bf199ef7b02ef82b3762f96de6b59e87fa0b88423a242b98d0f4a0b073cbccb9b39181a17281696cc63aba42f849d5e7b5765fa542fd02fd4b3584a0fb9f130854c2908748a4b208dae8187b1d52b822ec83167dd23b6948a872e247a0cfa71aea3f693d5484f10ec42357ac692ccfe22b42040d63be8851b73c4f7c53a046b189e5114bf8d2deb463354d67c679c6eb6209816daeafed0eee6ef1e59ba7a02e548024ba626cf86ebc7983bcf693eec58969056be784a97f1e2ff536bfa00ba00dda30a304520b446db65f7a9fe612407b856e9b7fd20ee00e47be69276e33db8080","0xf8b180a09dda8ee0506dd8da6c3a9968c053a400ff92ef28c89891ea0ac45a6d3cd56515a0a6926de29bb0247c685826ee452d16972b739a19801bffcc3397af59247c2c4ba05babcf173c1b76ed504e16622d047bedfff34083c7639b05ac1c8eb551801002808080a029bfe029aaef01d61d071f5789e808802233cc795b322cd46954662dd9fdae178080808080a05a10b0c631b24115764ba1389a66fb11e2bc0ca5ceea00c741da0a0de555f061808080","0xf843a0202944a272ac5bae96b5bd2f67b6c13276d541dc09eb1cf414d96b19a09e1c2fa1a023ab323453748129f2765f79615022f5bebd6f4096a796300aab049a60b0f187"],"value":"0x23ab323453748129f2765f79615022f5bebd6f4096a796300aab049a60b0f187"}]}"#;

        let address_hash_vec = hex::decode(
            "f1952e86c2ad89a0772acc940e2c5a0eec5c4d95a70affe1f0c5de2387f7c5c2",
        )
        .unwrap();
        let mut address_hash_bytes = [0u8; 32];
        address_hash_bytes.copy_from_slice(&address_hash_vec);
        let hardcoded_address_hash = address_hash_bytes.into();

        let state_root_vec = hex::decode(
            "4abc4de1c6dfac1f0b733be3797461bf1b0f7c2f2be11cc9f1137054fc4a1314",
        )
        .unwrap();
        let mut state_root_bytes = [0u8; 32];
        state_root_bytes.copy_from_slice(&state_root_vec);
        let hardcoded_state_root = state_root_bytes.into();

        let parsed_proof: EIP1186ProofResponse =
            parse_eth_get_proof(hardcoded_proof);

        verify_account_proof(
            hardcoded_state_root,
            hardcoded_address_hash,
            hardcoded_proof,
        );

        verify_storage_proof(hardcoded_proof);
    }
}
