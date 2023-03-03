// //! EVM Bridge Proofs
// #![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(feature = "std")]
// use serde::{Deserialize, Serialize};

// use ethereum_types::{Address, H256, U256, U64};
// use scale_codec::{Decode, Encode};

// #[cfg(test)]
// mod tests;

pub mod verify_trie_proof;

// mod types;

// use webb_proposals::{
//     evm::AnchorUpdateProposal, FunctionSignature, Nonce, ProposalHeader,
//     ResourceId, TargetSystem, TypedChainId,
// };

// #[derive(
//     Debug, Default, Clone, PartialEq, Eq, Encode, Decode,
// scale_info::TypeInfo, )]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// pub struct StorageProof {
//     pub key: H256,
//     pub proof: Vec<Vec<u8>>,
//     pub value: U256,
// }

// #[derive(Debug, Default, Clone, PartialEq, Eq, Encode, Decode)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// pub struct EIP1186Proof {
//     pub address: Address,
//     pub balance: U256,
//     pub code_hash: H256,
//     pub nonce: U64,
//     pub storage_hash: H256,
//     pub account_proof: Vec<Vec<u8>>,
//     pub storage_proof: Vec<StorageProof>,
// }

// pub fn get_anchor_update_proposal(
//     linked_resource_ids: Vec<ResourceId>,
//     log_index: u64,
//     log_entry_data: Vec<u8>,
//     receipt_index: u64,
//     receipt_data: Vec<u8>,
//     header_data: Vec<u8>,
//     proof: Vec<Vec<u8>>,
// ) -> AnchorUpdateProposal {
//     let src_chain = TypedChainId::Evm(1);
//     let src_target_system = TargetSystem::ContractAddress([0u8; 20]);
//     let src_resource_id = ResourceId::new(src_target_system, src_chain);
//     let header = ProposalHeader::new(
//         src_resource_id,
//         FunctionSignature::new([0u8; 4]),
//         Nonce(1u32),
//     );
//     let merkle_root = [0u8; 32];
//     AnchorUpdateProposal::new(header, merkle_root, src_resource_id)
// }
