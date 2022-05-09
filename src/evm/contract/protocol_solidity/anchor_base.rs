pub use anchorbasecontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod anchorbasecontract_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "AnchorBaseContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ANCHORBASECONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\"}],\"name\":\"EdgeAddition\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\"}],\"name\":\"EdgeUpdate\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint32\",\"name\":\"leafIndex\",\"type\":\"uint32\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\"}],\"name\":\"Insertion\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"EVM_CHAIN_ID_TYPE\",\"outputs\":[{\"internalType\":\"bytes2\",\"name\":\"\",\"type\":\"bytes2\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"FIELD_SIZE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ROOT_HISTORY_SIZE\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ZERO_VALUE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"currentNeighborRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"currentRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeExistsForChain\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeList\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"target\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"filledSubtrees\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getChainIdType\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getLastRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getLatestNeighborEdges\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"target\",\"type\":\"bytes32\"}],\"internalType\":\"struct LinkableTree.Edge[]\",\"name\":\"\",\"type\":\"tuple[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getLatestNeighborRoots\",\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"\",\"type\":\"bytes32[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getProposalNonce\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"handler\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_chainID\",\"type\":\"uint256\"}],\"name\":\"hasEdge\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract IPoseidonT3\",\"name\":\"_hasher\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"_left\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_right\",\"type\":\"bytes32\"}],\"name\":\"hashLeftRight\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"hasher\",\"outputs\":[{\"internalType\":\"contract IPoseidonT3\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_neighborChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"}],\"name\":\"isKnownNeighborRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"}],\"name\":\"isKnownRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\"}],\"name\":\"isSpent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_nullifierHashes\",\"type\":\"bytes32[]\"}],\"name\":\"isSpentArray\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"\",\"type\":\"bool[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_roots\",\"type\":\"bytes32[]\"}],\"name\":\"isValidRoots\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"levels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"maxEdges\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"name\":\"neighborRoots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"nextIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"nullifierHashes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"roots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\"}],\"name\":\"setHandler\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\"}],\"name\":\"setVerifier\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256[8]\",\"name\":\"_proof\",\"type\":\"uint256[8]\"}],\"name\":\"unpackProof\",\"outputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\"},{\"internalType\":\"uint256[2][2]\",\"name\":\"\",\"type\":\"uint256[2][2]\"},{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_sourceChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"_leafIndex\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_target\",\"type\":\"bytes32\"}],\"name\":\"updateEdge\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"verifier\",\"outputs\":[{\"internalType\":\"contract IAnchorVerifier\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"}],\"name\":\"zeros\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct AnchorBaseContract<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AnchorBaseContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for AnchorBaseContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AnchorBaseContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AnchorBaseContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                ANCHORBASECONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `EVM_CHAIN_ID_TYPE` (0x8b7e8782) function"]
        pub fn evm_chain_id_type(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 2]> {
            self.0
                .method_hash([139, 126, 135, 130], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `FIELD_SIZE` (0x414a37ba) function"]
        pub fn field_size(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([65, 74, 55, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ROOT_HISTORY_SIZE` (0xcd87a3b4) function"]
        pub fn root_history_size(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([205, 135, 163, 180], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ZERO_VALUE` (0xec732959) function"]
        pub fn zero_value(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([236, 115, 41, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `commitments` (0x839df945) function"]
        pub fn commitments(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([131, 157, 249, 69], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentNeighborRootIndex` (0x5d2d766c) function"]
        pub fn current_neighbor_root_index(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([93, 45, 118, 108], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentRootIndex` (0x90eeb02b) function"]
        pub fn current_root_index(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([144, 238, 176, 43], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `edgeExistsForChain` (0xfa731687) function"]
        pub fn edge_exists_for_chain(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 115, 22, 135], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `edgeIndex` (0xe70ea87c) function"]
        pub fn edge_index(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([231, 14, 168, 124], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `edgeList` (0xdbc916b8) function"]
        pub fn edge_list(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                [u8; 32],
                ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash([219, 201, 22, 184], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledSubtrees` (0xf178e47c) function"]
        pub fn filled_subtrees(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([241, 120, 228, 124], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChainId` (0x3408e470) function"]
        pub fn get_chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChainIdType` (0x4c830cbd) function"]
        pub fn get_chain_id_type(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([76, 131, 12, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastRoot` (0xba70f757) function"]
        pub fn get_last_root(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([186, 112, 247, 87], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLatestNeighborEdges` (0x8c0d34d8) function"]
        pub fn get_latest_neighbor_edges(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Edge>>
        {
            self.0
                .method_hash([140, 13, 52, 216], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLatestNeighborRoots` (0x1e627617) function"]
        pub fn get_latest_neighbor_roots(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<[u8; 32]>,
        > {
            self.0
                .method_hash([30, 98, 118, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getProposalNonce` (0x0b27fb9a) function"]
        pub fn get_proposal_nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([11, 39, 251, 154], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `handler` (0xc80916d4) function"]
        pub fn handler(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 9, 22, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasEdge` (0x92156311) function"]
        pub fn has_edge(
            &self,
            chain_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 21, 99, 17], chain_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hashLeftRight` (0x8ea3099e) function"]
        pub fn hash_left_right(
            &self,
            hasher: ethers::core::types::Address,
            left: [u8; 32],
            right: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([142, 163, 9, 158], (hasher, left, right))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasher` (0xed33639f) function"]
        pub fn hasher(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([237, 51, 99, 159], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKnownNeighborRoot` (0x11e4dcb9) function"]
        pub fn is_known_neighbor_root(
            &self,
            neighbor_chain_id: ethers::core::types::U256,
            root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([17, 228, 220, 185], (neighbor_chain_id, root))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKnownRoot` (0x6d9833e3) function"]
        pub fn is_known_root(
            &self,
            root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 152, 51, 227], root)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpent` (0xe5285dcc) function"]
        pub fn is_spent(
            &self,
            nullifier_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([229, 40, 93, 204], nullifier_hash)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpentArray` (0x9fa12d0b) function"]
        pub fn is_spent_array(
            &self,
            nullifier_hashes: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>>
        {
            self.0
                .method_hash([159, 161, 45, 11], nullifier_hashes)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isValidRoots` (0x616e0957) function"]
        pub fn is_valid_roots(
            &self,
            roots: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([97, 110, 9, 87], roots)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `levels` (0x4ecf518b) function"]
        pub fn levels(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([78, 207, 81, 139], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxEdges` (0x71523c32) function"]
        pub fn max_edges(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([113, 82, 60, 50], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `neighborRoots` (0x43e7119f) function"]
        pub fn neighbor_roots(
            &self,
            p0: ethers::core::types::U256,
            p1: u32,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([67, 231, 17, 159], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nextIndex` (0xfc7e9c6f) function"]
        pub fn next_index(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([252, 126, 156, 111], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nullifierHashes` (0x17cc915c) function"]
        pub fn nullifier_hashes(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([23, 204, 145, 92], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roots` (0xc2b40ae4) function"]
        pub fn roots(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([194, 180, 10, 228], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHandler` (0x72c1ad03) function"]
        pub fn set_handler(
            &self,
            handler: ethers::core::types::Address,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 193, 173, 3], (handler, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setVerifier` (0xa0d192f5) function"]
        pub fn set_verifier(
            &self,
            verifier: ethers::core::types::Address,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 209, 146, 245], (verifier, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpackProof` (0xf5ab0dd6) function"]
        pub fn unpack_proof(
            &self,
            proof: [ethers::core::types::U256; 8usize],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                [ethers::core::types::U256; 2usize],
                [[ethers::core::types::U256; 2usize]; 2usize],
                [ethers::core::types::U256; 2usize],
            ),
        > {
            self.0
                .method_hash([245, 171, 13, 214], proof)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateEdge` (0x8d09169d) function"]
        pub fn update_edge(
            &self,
            source_chain_id: ethers::core::types::U256,
            root: [u8; 32],
            leaf_index: ethers::core::types::U256,
            target: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [141, 9, 22, 157],
                    (source_chain_id, root, leaf_index, target),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifier` (0x2b7ac3f3) function"]
        pub fn verifier(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([43, 122, 195, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `zeros` (0xe8295588) function"]
        pub fn zeros(
            &self,
            i: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([232, 41, 85, 136], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `EdgeAddition` event"]
        pub fn edge_addition_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EdgeAdditionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EdgeUpdate` event"]
        pub fn edge_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EdgeUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Insertion` event"]
        pub fn insertion_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InsertionFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, AnchorBaseContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethevent(
        name = "EdgeAddition",
        abi = "EdgeAddition(uint256,uint256,bytes32)"
    )]
    pub struct EdgeAdditionFilter {
        pub chain_id: ethers::core::types::U256,
        pub latest_leaf_index: ethers::core::types::U256,
        pub merkle_root: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethevent(
        name = "EdgeUpdate",
        abi = "EdgeUpdate(uint256,uint256,bytes32)"
    )]
    pub struct EdgeUpdateFilter {
        pub chain_id: ethers::core::types::U256,
        pub latest_leaf_index: ethers::core::types::U256,
        pub merkle_root: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethevent(name = "Insertion", abi = "Insertion(bytes32,uint32,uint256)")]
    pub struct InsertionFilter {
        #[ethevent(indexed)]
        pub commitment: [u8; 32],
        pub leaf_index: u32,
        pub timestamp: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AnchorBaseContractEvents {
        EdgeAdditionFilter(EdgeAdditionFilter),
        EdgeUpdateFilter(EdgeUpdateFilter),
        InsertionFilter(InsertionFilter),
    }
    impl ethers::contract::EthLogDecode for AnchorBaseContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = EdgeAdditionFilter::decode_log(log) {
                return Ok(AnchorBaseContractEvents::EdgeAdditionFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EdgeUpdateFilter::decode_log(log) {
                return Ok(AnchorBaseContractEvents::EdgeUpdateFilter(decoded));
            }
            if let Ok(decoded) = InsertionFilter::decode_log(log) {
                return Ok(AnchorBaseContractEvents::InsertionFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AnchorBaseContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AnchorBaseContractEvents::EdgeAdditionFilter(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractEvents::EdgeUpdateFilter(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractEvents::InsertionFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `EVM_CHAIN_ID_TYPE`function with signature `EVM_CHAIN_ID_TYPE()` and selector `[139, 126, 135, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "EVM_CHAIN_ID_TYPE", abi = "EVM_CHAIN_ID_TYPE()")]
    pub struct EvmChainIdTypeCall;
    #[doc = "Container type for all input parameters for the `FIELD_SIZE`function with signature `FIELD_SIZE()` and selector `[65, 74, 55, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "FIELD_SIZE", abi = "FIELD_SIZE()")]
    pub struct FieldSizeCall;
    #[doc = "Container type for all input parameters for the `ROOT_HISTORY_SIZE`function with signature `ROOT_HISTORY_SIZE()` and selector `[205, 135, 163, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "ROOT_HISTORY_SIZE", abi = "ROOT_HISTORY_SIZE()")]
    pub struct RootHistorySizeCall;
    #[doc = "Container type for all input parameters for the `ZERO_VALUE`function with signature `ZERO_VALUE()` and selector `[236, 115, 41, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "ZERO_VALUE", abi = "ZERO_VALUE()")]
    pub struct ZeroValueCall;
    #[doc = "Container type for all input parameters for the `commitments`function with signature `commitments(bytes32)` and selector `[131, 157, 249, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "commitments", abi = "commitments(bytes32)")]
    pub struct CommitmentsCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `currentNeighborRootIndex`function with signature `currentNeighborRootIndex(uint256)` and selector `[93, 45, 118, 108]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "currentNeighborRootIndex",
        abi = "currentNeighborRootIndex(uint256)"
    )]
    pub struct CurrentNeighborRootIndexCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `currentRootIndex`function with signature `currentRootIndex()` and selector `[144, 238, 176, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "currentRootIndex", abi = "currentRootIndex()")]
    pub struct CurrentRootIndexCall;
    #[doc = "Container type for all input parameters for the `edgeExistsForChain`function with signature `edgeExistsForChain(uint256)` and selector `[250, 115, 22, 135]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "edgeExistsForChain", abi = "edgeExistsForChain(uint256)")]
    pub struct EdgeExistsForChainCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `edgeIndex`function with signature `edgeIndex(uint256)` and selector `[231, 14, 168, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "edgeIndex", abi = "edgeIndex(uint256)")]
    pub struct EdgeIndexCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `edgeList`function with signature `edgeList(uint256)` and selector `[219, 201, 22, 184]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "edgeList", abi = "edgeList(uint256)")]
    pub struct EdgeListCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `filledSubtrees`function with signature `filledSubtrees(uint256)` and selector `[241, 120, 228, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "filledSubtrees", abi = "filledSubtrees(uint256)")]
    pub struct FilledSubtreesCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `getChainId`function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    #[doc = "Container type for all input parameters for the `getChainIdType`function with signature `getChainIdType()` and selector `[76, 131, 12, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getChainIdType", abi = "getChainIdType()")]
    pub struct GetChainIdTypeCall;
    #[doc = "Container type for all input parameters for the `getLastRoot`function with signature `getLastRoot()` and selector `[186, 112, 247, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getLastRoot", abi = "getLastRoot()")]
    pub struct GetLastRootCall;
    #[doc = "Container type for all input parameters for the `getLatestNeighborEdges`function with signature `getLatestNeighborEdges()` and selector `[140, 13, 52, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "getLatestNeighborEdges",
        abi = "getLatestNeighborEdges()"
    )]
    pub struct GetLatestNeighborEdgesCall;
    #[doc = "Container type for all input parameters for the `getLatestNeighborRoots`function with signature `getLatestNeighborRoots()` and selector `[30, 98, 118, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "getLatestNeighborRoots",
        abi = "getLatestNeighborRoots()"
    )]
    pub struct GetLatestNeighborRootsCall;
    #[doc = "Container type for all input parameters for the `getProposalNonce`function with signature `getProposalNonce()` and selector `[11, 39, 251, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getProposalNonce", abi = "getProposalNonce()")]
    pub struct GetProposalNonceCall;
    #[doc = "Container type for all input parameters for the `handler`function with signature `handler()` and selector `[200, 9, 22, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "handler", abi = "handler()")]
    pub struct HandlerCall;
    #[doc = "Container type for all input parameters for the `hasEdge`function with signature `hasEdge(uint256)` and selector `[146, 21, 99, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "hasEdge", abi = "hasEdge(uint256)")]
    pub struct HasEdgeCall {
        pub chain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `hashLeftRight`function with signature `hashLeftRight(address,bytes32,bytes32)` and selector `[142, 163, 9, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "hashLeftRight",
        abi = "hashLeftRight(address,bytes32,bytes32)"
    )]
    pub struct HashLeftRightCall {
        pub hasher: ethers::core::types::Address,
        pub left: [u8; 32],
        pub right: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `hasher`function with signature `hasher()` and selector `[237, 51, 99, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "hasher", abi = "hasher()")]
    pub struct HasherCall;
    #[doc = "Container type for all input parameters for the `isKnownNeighborRoot`function with signature `isKnownNeighborRoot(uint256,bytes32)` and selector `[17, 228, 220, 185]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "isKnownNeighborRoot",
        abi = "isKnownNeighborRoot(uint256,bytes32)"
    )]
    pub struct IsKnownNeighborRootCall {
        pub neighbor_chain_id: ethers::core::types::U256,
        pub root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isKnownRoot`function with signature `isKnownRoot(bytes32)` and selector `[109, 152, 51, 227]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "isKnownRoot", abi = "isKnownRoot(bytes32)")]
    pub struct IsKnownRootCall {
        pub root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isSpent`function with signature `isSpent(bytes32)` and selector `[229, 40, 93, 204]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "isSpent", abi = "isSpent(bytes32)")]
    pub struct IsSpentCall {
        pub nullifier_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isSpentArray`function with signature `isSpentArray(bytes32[])` and selector `[159, 161, 45, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "isSpentArray", abi = "isSpentArray(bytes32[])")]
    pub struct IsSpentArrayCall {
        pub nullifier_hashes: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `isValidRoots`function with signature `isValidRoots(bytes32[])` and selector `[97, 110, 9, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "isValidRoots", abi = "isValidRoots(bytes32[])")]
    pub struct IsValidRootsCall {
        pub roots: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `levels`function with signature `levels()` and selector `[78, 207, 81, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "levels", abi = "levels()")]
    pub struct LevelsCall;
    #[doc = "Container type for all input parameters for the `maxEdges`function with signature `maxEdges()` and selector `[113, 82, 60, 50]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "maxEdges", abi = "maxEdges()")]
    pub struct MaxEdgesCall;
    #[doc = "Container type for all input parameters for the `neighborRoots`function with signature `neighborRoots(uint256,uint32)` and selector `[67, 231, 17, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "neighborRoots", abi = "neighborRoots(uint256,uint32)")]
    pub struct NeighborRootsCall(pub ethers::core::types::U256, pub u32);
    #[doc = "Container type for all input parameters for the `nextIndex`function with signature `nextIndex()` and selector `[252, 126, 156, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "nextIndex", abi = "nextIndex()")]
    pub struct NextIndexCall;
    #[doc = "Container type for all input parameters for the `nullifierHashes`function with signature `nullifierHashes(bytes32)` and selector `[23, 204, 145, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "nullifierHashes", abi = "nullifierHashes(bytes32)")]
    pub struct NullifierHashesCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `roots`function with signature `roots(uint256)` and selector `[194, 180, 10, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "roots", abi = "roots(uint256)")]
    pub struct RootsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `setHandler`function with signature `setHandler(address,uint32)` and selector `[114, 193, 173, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "setHandler", abi = "setHandler(address,uint32)")]
    pub struct SetHandlerCall {
        pub handler: ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `setVerifier`function with signature `setVerifier(address,uint32)` and selector `[160, 209, 146, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "setVerifier", abi = "setVerifier(address,uint32)")]
    pub struct SetVerifierCall {
        pub verifier: ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `unpackProof`function with signature `unpackProof(uint256[8])` and selector `[245, 171, 13, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "unpackProof", abi = "unpackProof(uint256[8])")]
    pub struct UnpackProofCall {
        pub proof: [ethers::core::types::U256; 8usize],
    }
    #[doc = "Container type for all input parameters for the `updateEdge`function with signature `updateEdge(uint256,bytes32,uint256,bytes32)` and selector `[141, 9, 22, 157]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(
        name = "updateEdge",
        abi = "updateEdge(uint256,bytes32,uint256,bytes32)"
    )]
    pub struct UpdateEdgeCall {
        pub source_chain_id: ethers::core::types::U256,
        pub root: [u8; 32],
        pub leaf_index: ethers::core::types::U256,
        pub target: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `verifier`function with signature `verifier()` and selector `[43, 122, 195, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "verifier", abi = "verifier()")]
    pub struct VerifierCall;
    #[doc = "Container type for all input parameters for the `zeros`function with signature `zeros(uint256)` and selector `[232, 41, 85, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "zeros", abi = "zeros(uint256)")]
    pub struct ZerosCall {
        pub i: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AnchorBaseContractCalls {
        EvmChainIdType(EvmChainIdTypeCall),
        FieldSize(FieldSizeCall),
        RootHistorySize(RootHistorySizeCall),
        ZeroValue(ZeroValueCall),
        Commitments(CommitmentsCall),
        CurrentNeighborRootIndex(CurrentNeighborRootIndexCall),
        CurrentRootIndex(CurrentRootIndexCall),
        EdgeExistsForChain(EdgeExistsForChainCall),
        EdgeIndex(EdgeIndexCall),
        EdgeList(EdgeListCall),
        FilledSubtrees(FilledSubtreesCall),
        GetChainId(GetChainIdCall),
        GetChainIdType(GetChainIdTypeCall),
        GetLastRoot(GetLastRootCall),
        GetLatestNeighborEdges(GetLatestNeighborEdgesCall),
        GetLatestNeighborRoots(GetLatestNeighborRootsCall),
        GetProposalNonce(GetProposalNonceCall),
        Handler(HandlerCall),
        HasEdge(HasEdgeCall),
        HashLeftRight(HashLeftRightCall),
        Hasher(HasherCall),
        IsKnownNeighborRoot(IsKnownNeighborRootCall),
        IsKnownRoot(IsKnownRootCall),
        IsSpent(IsSpentCall),
        IsSpentArray(IsSpentArrayCall),
        IsValidRoots(IsValidRootsCall),
        Levels(LevelsCall),
        MaxEdges(MaxEdgesCall),
        NeighborRoots(NeighborRootsCall),
        NextIndex(NextIndexCall),
        NullifierHashes(NullifierHashesCall),
        Roots(RootsCall),
        SetHandler(SetHandlerCall),
        SetVerifier(SetVerifierCall),
        UnpackProof(UnpackProofCall),
        UpdateEdge(UpdateEdgeCall),
        Verifier(VerifierCall),
        Zeros(ZerosCall),
    }
    impl ethers::core::abi::AbiDecode for AnchorBaseContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <EvmChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::EvmChainIdType(decoded));
            }
            if let Ok(decoded) =
                <FieldSizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::FieldSize(decoded));
            }
            if let Ok(decoded) =
                <RootHistorySizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::RootHistorySize(decoded));
            }
            if let Ok(decoded) =
                <ZeroValueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::ZeroValue(decoded));
            }
            if let Ok(decoded) =
                <CommitmentsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::Commitments(decoded));
            }
            if let Ok (decoded) = < CurrentNeighborRootIndexCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (AnchorBaseContractCalls :: CurrentNeighborRootIndex (decoded)) }
            if let Ok(decoded) =
                <CurrentRootIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::CurrentRootIndex(decoded));
            }
            if let Ok(decoded) =
                <EdgeExistsForChainCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::EdgeExistsForChain(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <EdgeIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::EdgeIndex(decoded));
            }
            if let Ok(decoded) =
                <EdgeListCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::EdgeList(decoded));
            }
            if let Ok(decoded) =
                <FilledSubtreesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::FilledSubtrees(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::GetChainIdType(decoded));
            }
            if let Ok(decoded) =
                <GetLastRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::GetLastRoot(decoded));
            }
            if let Ok (decoded) = < GetLatestNeighborEdgesCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (AnchorBaseContractCalls :: GetLatestNeighborEdges (decoded)) }
            if let Ok (decoded) = < GetLatestNeighborRootsCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (AnchorBaseContractCalls :: GetLatestNeighborRoots (decoded)) }
            if let Ok(decoded) =
                <GetProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::GetProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <HandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::HasEdge(decoded));
            }
            if let Ok(decoded) =
                <HashLeftRightCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::HashLeftRight(decoded));
            }
            if let Ok(decoded) =
                <HasherCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::Hasher(decoded));
            }
            if let Ok (decoded) = < IsKnownNeighborRootCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (AnchorBaseContractCalls :: IsKnownNeighborRoot (decoded)) }
            if let Ok(decoded) =
                <IsKnownRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::IsKnownRoot(decoded));
            }
            if let Ok(decoded) =
                <IsSpentCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::IsSpent(decoded));
            }
            if let Ok(decoded) =
                <IsSpentArrayCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::IsSpentArray(decoded));
            }
            if let Ok(decoded) =
                <IsValidRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::IsValidRoots(decoded));
            }
            if let Ok(decoded) =
                <LevelsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::Levels(decoded));
            }
            if let Ok(decoded) =
                <MaxEdgesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::MaxEdges(decoded));
            }
            if let Ok(decoded) =
                <NeighborRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::NeighborRoots(decoded));
            }
            if let Ok(decoded) =
                <NextIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::NextIndex(decoded));
            }
            if let Ok(decoded) =
                <NullifierHashesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::NullifierHashes(decoded));
            }
            if let Ok(decoded) =
                <RootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::Roots(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::SetHandler(decoded));
            }
            if let Ok(decoded) =
                <SetVerifierCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::SetVerifier(decoded));
            }
            if let Ok(decoded) =
                <UnpackProofCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::UnpackProof(decoded));
            }
            if let Ok(decoded) =
                <UpdateEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::UpdateEdge(decoded));
            }
            if let Ok(decoded) =
                <VerifierCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::Verifier(decoded));
            }
            if let Ok(decoded) =
                <ZerosCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorBaseContractCalls::Zeros(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AnchorBaseContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AnchorBaseContractCalls::EvmChainIdType(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::FieldSize(element) => element.encode(),
                AnchorBaseContractCalls::RootHistorySize(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::ZeroValue(element) => element.encode(),
                AnchorBaseContractCalls::Commitments(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::CurrentNeighborRootIndex(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::CurrentRootIndex(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::EdgeExistsForChain(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::EdgeIndex(element) => element.encode(),
                AnchorBaseContractCalls::EdgeList(element) => element.encode(),
                AnchorBaseContractCalls::FilledSubtrees(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::GetChainId(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::GetChainIdType(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::GetLastRoot(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::GetLatestNeighborEdges(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::GetLatestNeighborRoots(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::GetProposalNonce(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::Handler(element) => element.encode(),
                AnchorBaseContractCalls::HasEdge(element) => element.encode(),
                AnchorBaseContractCalls::HashLeftRight(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::Hasher(element) => element.encode(),
                AnchorBaseContractCalls::IsKnownNeighborRoot(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::IsKnownRoot(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::IsSpent(element) => element.encode(),
                AnchorBaseContractCalls::IsSpentArray(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::IsValidRoots(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::Levels(element) => element.encode(),
                AnchorBaseContractCalls::MaxEdges(element) => element.encode(),
                AnchorBaseContractCalls::NeighborRoots(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::NextIndex(element) => element.encode(),
                AnchorBaseContractCalls::NullifierHashes(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::Roots(element) => element.encode(),
                AnchorBaseContractCalls::SetHandler(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::SetVerifier(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::UnpackProof(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::UpdateEdge(element) => {
                    element.encode()
                }
                AnchorBaseContractCalls::Verifier(element) => element.encode(),
                AnchorBaseContractCalls::Zeros(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AnchorBaseContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AnchorBaseContractCalls::EvmChainIdType(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::FieldSize(element) => element.fmt(f),
                AnchorBaseContractCalls::RootHistorySize(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::ZeroValue(element) => element.fmt(f),
                AnchorBaseContractCalls::Commitments(element) => element.fmt(f),
                AnchorBaseContractCalls::CurrentNeighborRootIndex(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::CurrentRootIndex(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::EdgeExistsForChain(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::EdgeIndex(element) => element.fmt(f),
                AnchorBaseContractCalls::EdgeList(element) => element.fmt(f),
                AnchorBaseContractCalls::FilledSubtrees(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::GetChainId(element) => element.fmt(f),
                AnchorBaseContractCalls::GetChainIdType(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::GetLastRoot(element) => element.fmt(f),
                AnchorBaseContractCalls::GetLatestNeighborEdges(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::GetLatestNeighborRoots(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::GetProposalNonce(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::Handler(element) => element.fmt(f),
                AnchorBaseContractCalls::HasEdge(element) => element.fmt(f),
                AnchorBaseContractCalls::HashLeftRight(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::Hasher(element) => element.fmt(f),
                AnchorBaseContractCalls::IsKnownNeighborRoot(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::IsKnownRoot(element) => element.fmt(f),
                AnchorBaseContractCalls::IsSpent(element) => element.fmt(f),
                AnchorBaseContractCalls::IsSpentArray(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::IsValidRoots(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::Levels(element) => element.fmt(f),
                AnchorBaseContractCalls::MaxEdges(element) => element.fmt(f),
                AnchorBaseContractCalls::NeighborRoots(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::NextIndex(element) => element.fmt(f),
                AnchorBaseContractCalls::NullifierHashes(element) => {
                    element.fmt(f)
                }
                AnchorBaseContractCalls::Roots(element) => element.fmt(f),
                AnchorBaseContractCalls::SetHandler(element) => element.fmt(f),
                AnchorBaseContractCalls::SetVerifier(element) => element.fmt(f),
                AnchorBaseContractCalls::UnpackProof(element) => element.fmt(f),
                AnchorBaseContractCalls::UpdateEdge(element) => element.fmt(f),
                AnchorBaseContractCalls::Verifier(element) => element.fmt(f),
                AnchorBaseContractCalls::Zeros(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<EvmChainIdTypeCall> for AnchorBaseContractCalls {
        fn from(var: EvmChainIdTypeCall) -> Self {
            AnchorBaseContractCalls::EvmChainIdType(var)
        }
    }
    impl ::std::convert::From<FieldSizeCall> for AnchorBaseContractCalls {
        fn from(var: FieldSizeCall) -> Self {
            AnchorBaseContractCalls::FieldSize(var)
        }
    }
    impl ::std::convert::From<RootHistorySizeCall> for AnchorBaseContractCalls {
        fn from(var: RootHistorySizeCall) -> Self {
            AnchorBaseContractCalls::RootHistorySize(var)
        }
    }
    impl ::std::convert::From<ZeroValueCall> for AnchorBaseContractCalls {
        fn from(var: ZeroValueCall) -> Self {
            AnchorBaseContractCalls::ZeroValue(var)
        }
    }
    impl ::std::convert::From<CommitmentsCall> for AnchorBaseContractCalls {
        fn from(var: CommitmentsCall) -> Self {
            AnchorBaseContractCalls::Commitments(var)
        }
    }
    impl ::std::convert::From<CurrentNeighborRootIndexCall>
        for AnchorBaseContractCalls
    {
        fn from(var: CurrentNeighborRootIndexCall) -> Self {
            AnchorBaseContractCalls::CurrentNeighborRootIndex(var)
        }
    }
    impl ::std::convert::From<CurrentRootIndexCall> for AnchorBaseContractCalls {
        fn from(var: CurrentRootIndexCall) -> Self {
            AnchorBaseContractCalls::CurrentRootIndex(var)
        }
    }
    impl ::std::convert::From<EdgeExistsForChainCall> for AnchorBaseContractCalls {
        fn from(var: EdgeExistsForChainCall) -> Self {
            AnchorBaseContractCalls::EdgeExistsForChain(var)
        }
    }
    impl ::std::convert::From<EdgeIndexCall> for AnchorBaseContractCalls {
        fn from(var: EdgeIndexCall) -> Self {
            AnchorBaseContractCalls::EdgeIndex(var)
        }
    }
    impl ::std::convert::From<EdgeListCall> for AnchorBaseContractCalls {
        fn from(var: EdgeListCall) -> Self {
            AnchorBaseContractCalls::EdgeList(var)
        }
    }
    impl ::std::convert::From<FilledSubtreesCall> for AnchorBaseContractCalls {
        fn from(var: FilledSubtreesCall) -> Self {
            AnchorBaseContractCalls::FilledSubtrees(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for AnchorBaseContractCalls {
        fn from(var: GetChainIdCall) -> Self {
            AnchorBaseContractCalls::GetChainId(var)
        }
    }
    impl ::std::convert::From<GetChainIdTypeCall> for AnchorBaseContractCalls {
        fn from(var: GetChainIdTypeCall) -> Self {
            AnchorBaseContractCalls::GetChainIdType(var)
        }
    }
    impl ::std::convert::From<GetLastRootCall> for AnchorBaseContractCalls {
        fn from(var: GetLastRootCall) -> Self {
            AnchorBaseContractCalls::GetLastRoot(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborEdgesCall>
        for AnchorBaseContractCalls
    {
        fn from(var: GetLatestNeighborEdgesCall) -> Self {
            AnchorBaseContractCalls::GetLatestNeighborEdges(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborRootsCall>
        for AnchorBaseContractCalls
    {
        fn from(var: GetLatestNeighborRootsCall) -> Self {
            AnchorBaseContractCalls::GetLatestNeighborRoots(var)
        }
    }
    impl ::std::convert::From<GetProposalNonceCall> for AnchorBaseContractCalls {
        fn from(var: GetProposalNonceCall) -> Self {
            AnchorBaseContractCalls::GetProposalNonce(var)
        }
    }
    impl ::std::convert::From<HandlerCall> for AnchorBaseContractCalls {
        fn from(var: HandlerCall) -> Self {
            AnchorBaseContractCalls::Handler(var)
        }
    }
    impl ::std::convert::From<HasEdgeCall> for AnchorBaseContractCalls {
        fn from(var: HasEdgeCall) -> Self {
            AnchorBaseContractCalls::HasEdge(var)
        }
    }
    impl ::std::convert::From<HashLeftRightCall> for AnchorBaseContractCalls {
        fn from(var: HashLeftRightCall) -> Self {
            AnchorBaseContractCalls::HashLeftRight(var)
        }
    }
    impl ::std::convert::From<HasherCall> for AnchorBaseContractCalls {
        fn from(var: HasherCall) -> Self {
            AnchorBaseContractCalls::Hasher(var)
        }
    }
    impl ::std::convert::From<IsKnownNeighborRootCall> for AnchorBaseContractCalls {
        fn from(var: IsKnownNeighborRootCall) -> Self {
            AnchorBaseContractCalls::IsKnownNeighborRoot(var)
        }
    }
    impl ::std::convert::From<IsKnownRootCall> for AnchorBaseContractCalls {
        fn from(var: IsKnownRootCall) -> Self {
            AnchorBaseContractCalls::IsKnownRoot(var)
        }
    }
    impl ::std::convert::From<IsSpentCall> for AnchorBaseContractCalls {
        fn from(var: IsSpentCall) -> Self {
            AnchorBaseContractCalls::IsSpent(var)
        }
    }
    impl ::std::convert::From<IsSpentArrayCall> for AnchorBaseContractCalls {
        fn from(var: IsSpentArrayCall) -> Self {
            AnchorBaseContractCalls::IsSpentArray(var)
        }
    }
    impl ::std::convert::From<IsValidRootsCall> for AnchorBaseContractCalls {
        fn from(var: IsValidRootsCall) -> Self {
            AnchorBaseContractCalls::IsValidRoots(var)
        }
    }
    impl ::std::convert::From<LevelsCall> for AnchorBaseContractCalls {
        fn from(var: LevelsCall) -> Self {
            AnchorBaseContractCalls::Levels(var)
        }
    }
    impl ::std::convert::From<MaxEdgesCall> for AnchorBaseContractCalls {
        fn from(var: MaxEdgesCall) -> Self {
            AnchorBaseContractCalls::MaxEdges(var)
        }
    }
    impl ::std::convert::From<NeighborRootsCall> for AnchorBaseContractCalls {
        fn from(var: NeighborRootsCall) -> Self {
            AnchorBaseContractCalls::NeighborRoots(var)
        }
    }
    impl ::std::convert::From<NextIndexCall> for AnchorBaseContractCalls {
        fn from(var: NextIndexCall) -> Self {
            AnchorBaseContractCalls::NextIndex(var)
        }
    }
    impl ::std::convert::From<NullifierHashesCall> for AnchorBaseContractCalls {
        fn from(var: NullifierHashesCall) -> Self {
            AnchorBaseContractCalls::NullifierHashes(var)
        }
    }
    impl ::std::convert::From<RootsCall> for AnchorBaseContractCalls {
        fn from(var: RootsCall) -> Self {
            AnchorBaseContractCalls::Roots(var)
        }
    }
    impl ::std::convert::From<SetHandlerCall> for AnchorBaseContractCalls {
        fn from(var: SetHandlerCall) -> Self {
            AnchorBaseContractCalls::SetHandler(var)
        }
    }
    impl ::std::convert::From<SetVerifierCall> for AnchorBaseContractCalls {
        fn from(var: SetVerifierCall) -> Self {
            AnchorBaseContractCalls::SetVerifier(var)
        }
    }
    impl ::std::convert::From<UnpackProofCall> for AnchorBaseContractCalls {
        fn from(var: UnpackProofCall) -> Self {
            AnchorBaseContractCalls::UnpackProof(var)
        }
    }
    impl ::std::convert::From<UpdateEdgeCall> for AnchorBaseContractCalls {
        fn from(var: UpdateEdgeCall) -> Self {
            AnchorBaseContractCalls::UpdateEdge(var)
        }
    }
    impl ::std::convert::From<VerifierCall> for AnchorBaseContractCalls {
        fn from(var: VerifierCall) -> Self {
            AnchorBaseContractCalls::Verifier(var)
        }
    }
    impl ::std::convert::From<ZerosCall> for AnchorBaseContractCalls {
        fn from(var: ZerosCall) -> Self {
            AnchorBaseContractCalls::Zeros(var)
        }
    }
    #[doc = "`Edge(uint256,bytes32,uint256,bytes32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct Edge {
        pub chain_id: ethers::core::types::U256,
        pub root: [u8; 32],
        pub latest_leaf_index: ethers::core::types::U256,
        pub target: [u8; 32],
    }
}
