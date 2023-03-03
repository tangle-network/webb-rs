// use crate::evm::verify_trie_proof::TrieProver;
// use ethereum_types::{Address, Bloom, H160, H256, H64, U256, U64};
// use rlp::{Rlp, RlpStream};
// use rlp_derive::{
//     RlpDecodable as RlpDecodableDerive, RlpEncodable as RlpEncodableDerive,
// };
// use scale_codec::{Decode, Encode};

// use hex::{FromHex, ToHex};
// use serde::{Deserialize, Deserializer, Serializer};

// #[derive(
//     Debug, Clone, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Deserialize,
// )]
// pub struct BlockHeader {
//     pub parent_hash: H256,
//     pub sha_3_uncles: H256,
//     pub miner: Address,
//     pub state_root: H256,
//     pub transactions_root: H256,
//     pub receipts_root: H256,
//     pub logs_bloom: Bloom,
//     pub difficulty: U256,
//     pub number: U256,
//     pub gas_limit: U64,
//     pub gas_used: U64,
//     pub timestamp: U64,
//     pub mix_hash: H256,
//     pub nonce: H64,
// }

// impl BlockHeader {
//     pub fn hash(&self) -> H256 {
//         let mut stream = RlpStream::new();
//         self.stream_rlp(&mut stream, false);
//         let data = stream.out();
//         TrieProver::near_keccak256(&data).into()
//     }

//     pub fn seal_hash(&self) -> H256 {
//         let mut stream = RlpStream::new();
//         self.stream_rlp(&mut stream, true);
//         let data = stream.out();
//         TrieProver::near_keccak256(&data).into()
//     }

//     fn stream_rlp(&self, stream: &mut RlpStream, partial: bool) {
//         stream.begin_list(13 + if !partial { 2 } else { 0 });
//         stream.append(&self.parent_hash);
//         stream.append(&self.sha_3_uncles);
//         stream.append(&self.miner);
//         stream.append(&self.state_root);
//         stream.append(&self.transactions_root);
//         stream.append(&self.receipts_root);
//         stream.append(&self.logs_bloom);
//         stream.append(&self.difficulty);
//         stream.append(&self.number);
//         stream.append(&self.gas_limit);
//         stream.append(&self.gas_used);
//         stream.append(&self.timestamp);

//         if !partial {
//             stream.append(&self.mix_hash);
//             stream.append(&self.nonce);
//         }
//     }
// }

// impl rlp::Encodable for BlockHeader {
//     fn rlp_append(&self, s: &mut RlpStream) {
//         self.stream_rlp(s, false);
//     }
// }

// impl rlp::Decodable for BlockHeader {
//     fn decode(rlp: &Rlp) -> Result<Self, rlp::DecoderError> {
//         Ok(Self {
//             parent_hash: rlp.val_at(0)?,
//             sha_3_uncles: rlp.val_at(1)?,
//             miner: rlp.val_at(2)?,
//             state_root: rlp.val_at(3)?,
//             transactions_root: rlp.val_at(4)?,
//             receipts_root: rlp.val_at(5)?,
//             logs_bloom: rlp.val_at(6)?,
//             difficulty: rlp.val_at(7)?,
//             number: rlp.val_at(8)?,
//             gas_limit: rlp.val_at(9)?,
//             gas_used: rlp.val_at(10)?,
//             timestamp: rlp.val_at(11)?,
//             mix_hash: rlp.val_at(12)?,
//             nonce: rlp.val_at(13)?,
//         })
//     }
// }

// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Eq,
//     RlpEncodableDerive,
//     RlpDecodableDerive,
//     Deserialize,
// )]
// pub struct Receipt {
//     pub status: bool,
//     pub gas_used: U256,
//     pub logs_bloom: Bloom,
//     pub logs: Vec<LogEntry>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
// pub struct LogEntry {
//     pub address: H160,
//     pub topics: Vec<H256>,
//     pub data: Vec<u8>,
// }

// impl rlp::Decodable for LogEntry {
//     fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
//         let result = LogEntry {
//             address: rlp.val_at(0usize)?,
//             topics: rlp.list_at(1usize)?,
//             data: rlp.val_at(2usize)?,
//         };
//         Ok(result)
//     }
// }

// impl rlp::Encodable for LogEntry {
//     fn rlp_append(&self, stream: &mut rlp::RlpStream) {
//         stream.begin_list(3usize);
//         stream.append(&self.address);
//         stream.append_list::<H256, _>(&self.topics);
//         stream.append(&self.data);
//     }
// }
