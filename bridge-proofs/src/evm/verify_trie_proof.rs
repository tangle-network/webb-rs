use ethereum::ReceiptV3;
use ethereum_types::H256;
use rlp::Rlp;

pub struct TrieProver;

impl TrieProver {
    pub fn near_keccak256(data: &[u8]) -> [u8; 32] {
        let mut buffer = [0u8; 32];
        let mut hasher = tiny_keccak::Keccak::v256();
        tiny_keccak::Hasher::update(&mut hasher, data);
        tiny_keccak::Hasher::finalize(hasher, &mut buffer);
        buffer
    }

    fn extract_nibbles(a: Vec<u8>) -> Vec<u8> {
        a.iter().flat_map(|b| vec![b >> 4, b & 0x0F]).collect()
    }

    /// Get element at position `pos` from rlp encoded data,
    /// and decode it as vector of bytes
    fn get_vec(data: &Rlp, pos: usize) -> Vec<u8> {
        data.at(pos).unwrap().as_val::<Vec<u8>>().unwrap()
    }

    fn concat_nibbles(a: Vec<u8>) -> Vec<u8> {
        a.iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .zip(a.iter().enumerate().filter(|(i, _)| i % 2 == 1))
            .map(|((_, x), (_, y))| (x << 4) | y)
            .collect()
    }

    fn verify_log_entry(
        log_index: u64,
        log_entry_data: Vec<u8>,
        receipt_index: u64,
        receipt_data: Vec<u8>,
        header_data: Vec<u8>,
        proof: Vec<Vec<u8>>,
    ) -> bool {
        let log_entry: ethereum::Log =
            rlp::decode(log_entry_data.as_slice()).unwrap();
        let receipt: ethereum::ReceiptV3 =
            rlp::decode(receipt_data.as_slice()).unwrap();
        let header: ethereum::Header =
            rlp::decode(header_data.as_slice()).unwrap();

        let r = match receipt {
            ReceiptV3::EIP1559(r) => {
                // Verify log_entry included in receipt
                if r.logs[log_index as usize] == log_entry {
                    return false;
                }

                r
            }
            ReceiptV3::EIP2930(r) => {
                // Verify log_entry included in receipt
                if r.logs[log_index as usize] == log_entry {
                    return false;
                }

                r
            }
            ReceiptV3::Legacy(r) => {
                // Verify log_entry included in receipt
                if r.logs[log_index as usize] == log_entry {
                    return false;
                }

                r
            }
        };

        // Verify receipt included into header
        let verification_result = Self::verify_trie_proof(
            header.receipts_root,
            rlp::encode(&receipt_index).to_vec(),
            proof,
        );

        return verification_result == receipt_data;
    }

    /// Verify the proof recursively traversing through the key.
    /// Return the value at the end of the key, in case the proof is valid.
    ///
    /// @param expected_root is the expected root of the current node.
    /// @param key is the key for which we are proving the value.
    /// @param proof contains relevant information to verify data is valid
    ///
    /// Patricia Trie: https://eth.wiki/en/fundamentals/patricia-tree
    /// Patricia Img:  https://ethereum.stackexchange.com/questions/268/ethereum-block-architecture/6413#6413
    ///
    /// Verification:  https://github.com/slockit/in3/wiki/Ethereum-Verification-and-MerkleProof#receipt-proof
    /// Article:       https://medium.com/@ouvrard.pierre.alain/merkle-proof-verification-for-ethereum-patricia-tree-48f29658eec
    /// Python impl:   https://gist.github.com/mfornet/0ff283274c0162f1cca45966bccf69ee
    ///
    pub fn verify_trie_proof(
        expected_root: H256,
        key: Vec<u8>,
        proof: Vec<Vec<u8>>,
    ) -> Vec<u8> {
        let mut actual_key = vec![];
        for el in key {
            actual_key.push(el / 16);
            actual_key.push(el % 16);
        }
        Self::_verify_trie_proof(
            (expected_root.0).into(),
            &actual_key,
            &proof,
            0,
            0,
        )
    }

    fn _verify_trie_proof(
        expected_root: Vec<u8>,
        key: &Vec<u8>,
        proof: &Vec<Vec<u8>>,
        key_index: usize,
        proof_index: usize,
    ) -> Vec<u8> {
        let node = &proof[proof_index];

        if key_index == 0 {
            // trie root is always a hash
            assert_eq!(Self::near_keccak256(node), expected_root.as_slice());
        } else if node.len() < 32 {
            // if rlp < 32 bytes, then it is not hashed
            assert_eq!(node.as_slice(), expected_root);
        } else {
            assert_eq!(Self::near_keccak256(node), expected_root.as_slice());
        }

        let node = Rlp::new(&node.as_slice());

        if node.iter().count() == 17 {
            // Branch node
            if key_index == key.len() {
                assert_eq!(proof_index + 1, proof.len());
                Self::get_vec(&node, 16)
            } else {
                let new_expected_root =
                    Self::get_vec(&node, key[key_index] as usize);
                Self::_verify_trie_proof(
                    new_expected_root,
                    key,
                    proof,
                    key_index + 1,
                    proof_index + 1,
                )
            }
        } else {
            // Leaf or extension node
            assert_eq!(node.iter().count(), 2);
            let path_u8 = Self::get_vec(&node, 0);
            // Extract first nibble
            let head = path_u8[0] / 16;
            // assert!(0 <= head); is implicit because of type limits
            assert!(head <= 3);

            // Extract path
            let mut path = vec![];
            if head % 2 == 1 {
                path.push(path_u8[0] % 16);
            }
            for val in path_u8.into_iter().skip(1) {
                path.push(val / 16);
                path.push(val % 16);
            }
            assert_eq!(
                path.as_slice(),
                &key[key_index..key_index + path.len()]
            );

            if head >= 2 {
                // Leaf node
                assert_eq!(proof_index + 1, proof.len());
                assert_eq!(key_index + path.len(), key.len());
                Self::get_vec(&node, 1)
            } else {
                // Extension node
                let new_expected_root = Self::get_vec(&node, 1);
                Self::_verify_trie_proof(
                    new_expected_root,
                    key,
                    proof,
                    key_index + path.len(),
                    proof_index + 1,
                )
            }
        }
    }
}
