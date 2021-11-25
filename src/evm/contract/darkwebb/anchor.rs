pub use anchorcontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod anchorcontract_mod {
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
    #[doc = "AnchorContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ANCHORCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IVerifier\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"contract IPoseidonT3\",\"name\":\"_hasher\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_denomination\",\"type\":\"uint256\"},{\"internalType\":\"uint32\",\"name\":\"_merkleTreeHeight\",\"type\":\"uint32\"},{\"internalType\":\"contract ITokenWrapper\",\"name\":\"_token\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_bridge\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_admin\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\"},{\"internalType\":\"uint8\",\"name\":\"_maxEdges\",\"type\":\"uint8\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint32\",\"name\":\"leafIndex\",\"type\":\"uint32\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\"}],\"name\":\"Deposit\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\"}],\"name\":\"EdgeAddition\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\"}],\"name\":\"EdgeUpdate\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"nullifierHash\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint32\",\"name\":\"insertedIndex\",\"type\":\"uint32\"}],\"name\":\"Refresh\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"nullifierHash\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\"}],\"name\":\"Withdrawal\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"FIELD_SIZE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ROOT_HISTORY_SIZE\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ZERO_VALUE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"sourceChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"leafIndex\",\"type\":\"uint256\"}],\"name\":\"addEdge\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"bridge\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"currentNeighborRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"currentRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"denomination\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_commitment\",\"type\":\"bytes32\"}],\"name\":\"deposit\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeExistsForChain\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeList\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"filledSubtrees\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getDenomination\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getLastRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getLatestNeighborRoots\",\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"roots\",\"type\":\"bytes32[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"handler\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_chainID\",\"type\":\"uint256\"}],\"name\":\"hasEdge\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract IPoseidonT3\",\"name\":\"_hasher\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"_left\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_right\",\"type\":\"bytes32\"}],\"name\":\"hashLeftRight\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"hasher\",\"outputs\":[{\"internalType\":\"contract IPoseidonT3\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"neighborChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"}],\"name\":\"isKnownNeighborRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"}],\"name\":\"isKnownRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\"}],\"name\":\"isSpent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_nullifierHashes\",\"type\":\"bytes32[]\"}],\"name\":\"isSpentArray\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"spent\",\"type\":\"bool[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"roots\",\"type\":\"bytes32[]\"}],\"name\":\"isValidRoots\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"levels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"maxEdges\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"name\":\"neighborRoots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"nextIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"nullifierHashes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"roots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_bridge\",\"type\":\"address\"}],\"name\":\"setBridge\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\"}],\"name\":\"setHandler\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256[8]\",\"name\":\"_proof\",\"type\":\"uint256[8]\"}],\"name\":\"unpackProof\",\"outputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\"},{\"internalType\":\"uint256[2][2]\",\"name\":\"\",\"type\":\"uint256[2][2]\"},{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"unwrapIntoNative\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"unwrapIntoToken\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"sourceChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"leafIndex\",\"type\":\"uint256\"}],\"name\":\"updateEdge\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"verifier\",\"outputs\":[{\"internalType\":\"contract IVerifier\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_proof\",\"type\":\"bytes\"},{\"components\":[{\"internalType\":\"bytes\",\"name\":\"_roots\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_refreshCommitment\",\"type\":\"bytes32\"},{\"internalType\":\"address payable\",\"name\":\"_recipient\",\"type\":\"address\"},{\"internalType\":\"address payable\",\"name\":\"_relayer\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_fee\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_refund\",\"type\":\"uint256\"}],\"internalType\":\"struct IAnchor.PublicInputs\",\"name\":\"_publicInputs\",\"type\":\"tuple\"}],\"name\":\"withdraw\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_proof\",\"type\":\"bytes\"},{\"components\":[{\"internalType\":\"bytes\",\"name\":\"_roots\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_refreshCommitment\",\"type\":\"bytes32\"},{\"internalType\":\"address payable\",\"name\":\"_recipient\",\"type\":\"address\"},{\"internalType\":\"address payable\",\"name\":\"_relayer\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_fee\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_refund\",\"type\":\"uint256\"}],\"internalType\":\"struct IAnchor.PublicInputs\",\"name\":\"_publicInputs\",\"type\":\"tuple\"},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"}],\"name\":\"withdrawAndUnwrap\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"_commitment\",\"type\":\"bytes32\"}],\"name\":\"wrapAndDeposit\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"wrapNative\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"wrapToken\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"}],\"name\":\"zeros\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct AnchorContract<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AnchorContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AnchorContract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AnchorContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AnchorContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                ANCHORCONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
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
        #[doc = "Calls the contract's `addEdge` (0x3b9e44de) function"]
        pub fn add_edge(
            &self,
            source_chain_id: ethers::core::types::U256,
            root: [u8; 32],
            leaf_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [59, 158, 68, 222],
                    (source_chain_id, root, leaf_index),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bridge` (0xe78cea92) function"]
        pub fn bridge(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([231, 140, 234, 146], ())
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
        #[doc = "Calls the contract's `denomination` (0x8bca6d16) function"]
        pub fn denomination(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([139, 202, 109, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xb214faa5) function"]
        pub fn deposit(
            &self,
            commitment: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 20, 250, 165], commitment)
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
        #[doc = "Calls the contract's `getDenomination` (0x1fc601c9) function"]
        pub fn get_denomination(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([31, 198, 1, 201], ())
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
        #[doc = "Calls the contract's `getLatestNeighborRoots` (0x1e627617) function"]
        pub fn get_latest_neighbor_roots(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, Vec<[u8; 32]>>
        {
            self.0
                .method_hash([30, 98, 118, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getToken` (0x21df0da7) function"]
        pub fn get_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([33, 223, 13, 167], ())
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
        ) -> ethers::contract::builders::ContractCall<M, Vec<bool>> {
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
        #[doc = "Calls the contract's `setBridge` (0x8dd14802) function"]
        pub fn set_bridge(
            &self,
            bridge: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 209, 72, 2], bridge)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHandler` (0xbac426d0) function"]
        pub fn set_handler(
            &self,
            handler: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 196, 38, 208], handler)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token` (0xfc0c546a) function"]
        pub fn token(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpackProof` (0xf5ab0dd6) function"]
        pub fn unpack_proof(
            &self,
            proof: [ethers::core::types::U256; 8usize],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                [ethers::core::types::U256; 2],
                [[ethers::core::types::U256; 2]; 2],
                [ethers::core::types::U256; 2],
            ),
        > {
            self.0
                .method_hash([245, 171, 13, 214], proof)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapIntoNative` (0x9ff80063) function"]
        pub fn unwrap_into_native(
            &self,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 248, 0, 99], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapIntoToken` (0x4f401241) function"]
        pub fn unwrap_into_token(
            &self,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 64, 18, 65], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateEdge` (0x44347ba9) function"]
        pub fn update_edge(
            &self,
            source_chain_id: ethers::core::types::U256,
            root: [u8; 32],
            leaf_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [68, 52, 123, 169],
                    (source_chain_id, root, leaf_index),
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
        #[doc = "Calls the contract's `withdraw` (0xd0e8d34a) function"]
        pub fn withdraw(
            &self,
            proof: Vec<u8>,
            public_inputs: PublicInputs,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 232, 211, 74], (proof, public_inputs))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAndUnwrap` (0xdf203aa7) function"]
        pub fn withdraw_and_unwrap(
            &self,
            proof: Vec<u8>,
            public_inputs: PublicInputs,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [223, 32, 58, 167],
                    (proof, public_inputs, token_address),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapAndDeposit` (0xcd3a9550) function"]
        pub fn wrap_and_deposit(
            &self,
            token_address: ethers::core::types::Address,
            commitment: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 58, 149, 80], (token_address, commitment))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapNative` (0x6ad481f3) function"]
        pub fn wrap_native(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 212, 129, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapToken` (0x460b53e3) function"]
        pub fn wrap_token(
            &self,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 11, 83, 227], (token_address, amount))
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
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
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
        #[doc = "Gets the contract's `Refresh` event"]
        pub fn refresh_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RefreshFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdrawal` event"]
        pub fn withdrawal_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, WithdrawalFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, AnchorContractEvents>
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
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(bytes32,uint32,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub commitment: [u8; 32],
        pub leaf_index: u32,
        pub timestamp: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
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
    )]
    #[ethevent(name = "Refresh", abi = "Refresh(bytes32,bytes32,uint32)")]
    pub struct RefreshFilter {
        #[ethevent(indexed)]
        pub commitment: [u8; 32],
        pub nullifier_hash: [u8; 32],
        pub inserted_index: u32,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "Withdrawal",
        abi = "Withdrawal(address,bytes32,address,uint256)"
    )]
    pub struct WithdrawalFilter {
        pub to: ethers::core::types::Address,
        pub nullifier_hash: [u8; 32],
        #[ethevent(indexed)]
        pub relayer: ethers::core::types::Address,
        pub fee: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AnchorContractEvents {
        DepositFilter(DepositFilter),
        EdgeAdditionFilter(EdgeAdditionFilter),
        EdgeUpdateFilter(EdgeUpdateFilter),
        RefreshFilter(RefreshFilter),
        WithdrawalFilter(WithdrawalFilter),
    }
    impl ethers::contract::EthLogDecode for AnchorContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(AnchorContractEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = EdgeAdditionFilter::decode_log(log) {
                return Ok(AnchorContractEvents::EdgeAdditionFilter(decoded));
            }
            if let Ok(decoded) = EdgeUpdateFilter::decode_log(log) {
                return Ok(AnchorContractEvents::EdgeUpdateFilter(decoded));
            }
            if let Ok(decoded) = RefreshFilter::decode_log(log) {
                return Ok(AnchorContractEvents::RefreshFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalFilter::decode_log(log) {
                return Ok(AnchorContractEvents::WithdrawalFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AnchorContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AnchorContractEvents::DepositFilter(element) => element.fmt(f),
                AnchorContractEvents::EdgeAdditionFilter(element) => {
                    element.fmt(f)
                }
                AnchorContractEvents::EdgeUpdateFilter(element) => {
                    element.fmt(f)
                }
                AnchorContractEvents::RefreshFilter(element) => element.fmt(f),
                AnchorContractEvents::WithdrawalFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `FIELD_SIZE`function with signature `FIELD_SIZE()` and selector `[65, 74, 55, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
    )]
    #[ethcall(name = "ZERO_VALUE", abi = "ZERO_VALUE()")]
    pub struct ZeroValueCall;
    #[doc = "Container type for all input parameters for the `addEdge`function with signature `addEdge(uint256,bytes32,uint256)` and selector `[59, 158, 68, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addEdge", abi = "addEdge(uint256,bytes32,uint256)")]
    pub struct AddEdgeCall {
        pub source_chain_id: ethers::core::types::U256,
        pub root: [u8; 32],
        pub leaf_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `admin`function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    #[doc = "Container type for all input parameters for the `bridge`function with signature `bridge()` and selector `[231, 140, 234, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "bridge", abi = "bridge()")]
    pub struct BridgeCall;
    #[doc = "Container type for all input parameters for the `commitments`function with signature `commitments(bytes32)` and selector `[131, 157, 249, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
    )]
    #[ethcall(name = "currentRootIndex", abi = "currentRootIndex()")]
    pub struct CurrentRootIndexCall;
    #[doc = "Container type for all input parameters for the `denomination`function with signature `denomination()` and selector `[139, 202, 109, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "denomination", abi = "denomination()")]
    pub struct DenominationCall;
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(bytes32)` and selector `[178, 20, 250, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deposit", abi = "deposit(bytes32)")]
    pub struct DepositCall {
        pub commitment: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `edgeExistsForChain`function with signature `edgeExistsForChain(uint256)` and selector `[250, 115, 22, 135]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
    )]
    #[ethcall(name = "filledSubtrees", abi = "filledSubtrees(uint256)")]
    pub struct FilledSubtreesCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `getDenomination`function with signature `getDenomination()` and selector `[31, 198, 1, 201]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getDenomination", abi = "getDenomination()")]
    pub struct GetDenominationCall;
    #[doc = "Container type for all input parameters for the `getLastRoot`function with signature `getLastRoot()` and selector `[186, 112, 247, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getLastRoot", abi = "getLastRoot()")]
    pub struct GetLastRootCall;
    #[doc = "Container type for all input parameters for the `getLatestNeighborRoots`function with signature `getLatestNeighborRoots()` and selector `[30, 98, 118, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "getLatestNeighborRoots",
        abi = "getLatestNeighborRoots()"
    )]
    pub struct GetLatestNeighborRootsCall;
    #[doc = "Container type for all input parameters for the `getToken`function with signature `getToken()` and selector `[33, 223, 13, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getToken", abi = "getToken()")]
    pub struct GetTokenCall;
    #[doc = "Container type for all input parameters for the `handler`function with signature `handler()` and selector `[200, 9, 22, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
    )]
    #[ethcall(name = "roots", abi = "roots(uint256)")]
    pub struct RootsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `setBridge`function with signature `setBridge(address)` and selector `[141, 209, 72, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setBridge", abi = "setBridge(address)")]
    pub struct SetBridgeCall {
        pub bridge: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setHandler`function with signature `setHandler(address)` and selector `[186, 196, 38, 208]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setHandler", abi = "setHandler(address)")]
    pub struct SetHandlerCall {
        pub handler: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `token`function with signature `token()` and selector `[252, 12, 84, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    #[doc = "Container type for all input parameters for the `unpackProof`function with signature `unpackProof(uint256[8])` and selector `[245, 171, 13, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "unpackProof", abi = "unpackProof(uint256[8])")]
    pub struct UnpackProofCall {
        pub proof: [ethers::core::types::U256; 8usize],
    }
    #[doc = "Container type for all input parameters for the `unwrapIntoNative`function with signature `unwrapIntoNative(address,uint256)` and selector `[159, 248, 0, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "unwrapIntoNative",
        abi = "unwrapIntoNative(address,uint256)"
    )]
    pub struct UnwrapIntoNativeCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unwrapIntoToken`function with signature `unwrapIntoToken(address,uint256)` and selector `[79, 64, 18, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "unwrapIntoToken",
        abi = "unwrapIntoToken(address,uint256)"
    )]
    pub struct UnwrapIntoTokenCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateEdge`function with signature `updateEdge(uint256,bytes32,uint256)` and selector `[68, 52, 123, 169]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "updateEdge", abi = "updateEdge(uint256,bytes32,uint256)")]
    pub struct UpdateEdgeCall {
        pub source_chain_id: ethers::core::types::U256,
        pub root: [u8; 32],
        pub leaf_index: ethers::core::types::U256,
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
    )]
    #[ethcall(name = "verifier", abi = "verifier()")]
    pub struct VerifierCall;
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(bytes,(bytes,bytes32,bytes32,address,address,uint256,uint256))` and selector `[208, 232, 211, 74]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "withdraw",
        abi = "withdraw(bytes,(bytes,bytes32,bytes32,address,address,uint256,uint256))"
    )]
    pub struct WithdrawCall {
        pub proof: Vec<u8>,
        pub public_inputs: PublicInputs,
    }
    #[doc = "Container type for all input parameters for the `withdrawAndUnwrap`function with signature `withdrawAndUnwrap(bytes,(bytes,bytes32,bytes32,address,address,uint256,uint256),address)` and selector `[223, 32, 58, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "withdrawAndUnwrap",
        abi = "withdrawAndUnwrap(bytes,(bytes,bytes32,bytes32,address,address,uint256,uint256),address)"
    )]
    pub struct WithdrawAndUnwrapCall {
        pub proof: Vec<u8>,
        pub public_inputs: PublicInputs,
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `wrapAndDeposit`function with signature `wrapAndDeposit(address,bytes32)` and selector `[205, 58, 149, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wrapAndDeposit", abi = "wrapAndDeposit(address,bytes32)")]
    pub struct WrapAndDepositCall {
        pub token_address: ethers::core::types::Address,
        pub commitment: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `wrapNative`function with signature `wrapNative()` and selector `[106, 212, 129, 243]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wrapNative", abi = "wrapNative()")]
    pub struct WrapNativeCall;
    #[doc = "Container type for all input parameters for the `wrapToken`function with signature `wrapToken(address,uint256)` and selector `[70, 11, 83, 227]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "wrapToken", abi = "wrapToken(address,uint256)")]
    pub struct WrapTokenCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `zeros`function with signature `zeros(uint256)` and selector `[232, 41, 85, 136]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "zeros", abi = "zeros(uint256)")]
    pub struct ZerosCall {
        pub i: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AnchorContractCalls {
        FieldSize(FieldSizeCall),
        RootHistorySize(RootHistorySizeCall),
        ZeroValue(ZeroValueCall),
        AddEdge(AddEdgeCall),
        Admin(AdminCall),
        Bridge(BridgeCall),
        Commitments(CommitmentsCall),
        CurrentNeighborRootIndex(CurrentNeighborRootIndexCall),
        CurrentRootIndex(CurrentRootIndexCall),
        Denomination(DenominationCall),
        Deposit(DepositCall),
        EdgeExistsForChain(EdgeExistsForChainCall),
        EdgeIndex(EdgeIndexCall),
        EdgeList(EdgeListCall),
        FilledSubtrees(FilledSubtreesCall),
        GetDenomination(GetDenominationCall),
        GetLastRoot(GetLastRootCall),
        GetLatestNeighborRoots(GetLatestNeighborRootsCall),
        GetToken(GetTokenCall),
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
        SetBridge(SetBridgeCall),
        SetHandler(SetHandlerCall),
        Token(TokenCall),
        UnpackProof(UnpackProofCall),
        UnwrapIntoNative(UnwrapIntoNativeCall),
        UnwrapIntoToken(UnwrapIntoTokenCall),
        UpdateEdge(UpdateEdgeCall),
        Verifier(VerifierCall),
        Withdraw(WithdrawCall),
        WithdrawAndUnwrap(WithdrawAndUnwrapCall),
        WrapAndDeposit(WrapAndDepositCall),
        WrapNative(WrapNativeCall),
        WrapToken(WrapTokenCall),
        Zeros(ZerosCall),
    }
    impl ethers::core::abi::AbiDecode for AnchorContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FieldSizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::FieldSize(decoded));
            }
            if let Ok(decoded) =
                <RootHistorySizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::RootHistorySize(decoded));
            }
            if let Ok(decoded) =
                <ZeroValueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::ZeroValue(decoded));
            }
            if let Ok(decoded) =
                <AddEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::AddEdge(decoded));
            }
            if let Ok(decoded) =
                <AdminCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <BridgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Bridge(decoded));
            }
            if let Ok(decoded) =
                <CommitmentsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Commitments(decoded));
            }
            if let Ok (decoded) = < CurrentNeighborRootIndexCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (AnchorContractCalls :: CurrentNeighborRootIndex (decoded)) }
            if let Ok(decoded) =
                <CurrentRootIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::CurrentRootIndex(decoded));
            }
            if let Ok(decoded) =
                <DenominationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Denomination(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <EdgeExistsForChainCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::EdgeExistsForChain(decoded));
            }
            if let Ok(decoded) =
                <EdgeIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::EdgeIndex(decoded));
            }
            if let Ok(decoded) =
                <EdgeListCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::EdgeList(decoded));
            }
            if let Ok(decoded) =
                <FilledSubtreesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::FilledSubtrees(decoded));
            }
            if let Ok(decoded) =
                <GetDenominationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::GetDenomination(decoded));
            }
            if let Ok(decoded) =
                <GetLastRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::GetLastRoot(decoded));
            }
            if let Ok (decoded) = < GetLatestNeighborRootsCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (AnchorContractCalls :: GetLatestNeighborRoots (decoded)) }
            if let Ok(decoded) =
                <GetTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::GetToken(decoded));
            }
            if let Ok(decoded) =
                <HandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::HasEdge(decoded));
            }
            if let Ok(decoded) =
                <HashLeftRightCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::HashLeftRight(decoded));
            }
            if let Ok(decoded) =
                <HasherCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Hasher(decoded));
            }
            if let Ok (decoded) = < IsKnownNeighborRootCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (AnchorContractCalls :: IsKnownNeighborRoot (decoded)) }
            if let Ok(decoded) =
                <IsKnownRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::IsKnownRoot(decoded));
            }
            if let Ok(decoded) =
                <IsSpentCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::IsSpent(decoded));
            }
            if let Ok(decoded) =
                <IsSpentArrayCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::IsSpentArray(decoded));
            }
            if let Ok(decoded) =
                <IsValidRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::IsValidRoots(decoded));
            }
            if let Ok(decoded) =
                <LevelsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Levels(decoded));
            }
            if let Ok(decoded) =
                <MaxEdgesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::MaxEdges(decoded));
            }
            if let Ok(decoded) =
                <NeighborRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::NeighborRoots(decoded));
            }
            if let Ok(decoded) =
                <NextIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::NextIndex(decoded));
            }
            if let Ok(decoded) =
                <NullifierHashesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::NullifierHashes(decoded));
            }
            if let Ok(decoded) =
                <RootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Roots(decoded));
            }
            if let Ok(decoded) =
                <SetBridgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::SetBridge(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::SetHandler(decoded));
            }
            if let Ok(decoded) =
                <TokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Token(decoded));
            }
            if let Ok(decoded) =
                <UnpackProofCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::UnpackProof(decoded));
            }
            if let Ok(decoded) =
                <UnwrapIntoNativeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::UnwrapIntoNative(decoded));
            }
            if let Ok(decoded) =
                <UnwrapIntoTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::UnwrapIntoToken(decoded));
            }
            if let Ok(decoded) =
                <UpdateEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::UpdateEdge(decoded));
            }
            if let Ok(decoded) =
                <VerifierCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Verifier(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAndUnwrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::WithdrawAndUnwrap(decoded));
            }
            if let Ok(decoded) =
                <WrapAndDepositCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::WrapAndDeposit(decoded));
            }
            if let Ok(decoded) =
                <WrapNativeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::WrapNative(decoded));
            }
            if let Ok(decoded) =
                <WrapTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::WrapToken(decoded));
            }
            if let Ok(decoded) =
                <ZerosCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorContractCalls::Zeros(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AnchorContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AnchorContractCalls::FieldSize(element) => element.encode(),
                AnchorContractCalls::RootHistorySize(element) => {
                    element.encode()
                }
                AnchorContractCalls::ZeroValue(element) => element.encode(),
                AnchorContractCalls::AddEdge(element) => element.encode(),
                AnchorContractCalls::Admin(element) => element.encode(),
                AnchorContractCalls::Bridge(element) => element.encode(),
                AnchorContractCalls::Commitments(element) => element.encode(),
                AnchorContractCalls::CurrentNeighborRootIndex(element) => {
                    element.encode()
                }
                AnchorContractCalls::CurrentRootIndex(element) => {
                    element.encode()
                }
                AnchorContractCalls::Denomination(element) => element.encode(),
                AnchorContractCalls::Deposit(element) => element.encode(),
                AnchorContractCalls::EdgeExistsForChain(element) => {
                    element.encode()
                }
                AnchorContractCalls::EdgeIndex(element) => element.encode(),
                AnchorContractCalls::EdgeList(element) => element.encode(),
                AnchorContractCalls::FilledSubtrees(element) => {
                    element.encode()
                }
                AnchorContractCalls::GetDenomination(element) => {
                    element.encode()
                }
                AnchorContractCalls::GetLastRoot(element) => element.encode(),
                AnchorContractCalls::GetLatestNeighborRoots(element) => {
                    element.encode()
                }
                AnchorContractCalls::GetToken(element) => element.encode(),
                AnchorContractCalls::Handler(element) => element.encode(),
                AnchorContractCalls::HasEdge(element) => element.encode(),
                AnchorContractCalls::HashLeftRight(element) => element.encode(),
                AnchorContractCalls::Hasher(element) => element.encode(),
                AnchorContractCalls::IsKnownNeighborRoot(element) => {
                    element.encode()
                }
                AnchorContractCalls::IsKnownRoot(element) => element.encode(),
                AnchorContractCalls::IsSpent(element) => element.encode(),
                AnchorContractCalls::IsSpentArray(element) => element.encode(),
                AnchorContractCalls::IsValidRoots(element) => element.encode(),
                AnchorContractCalls::Levels(element) => element.encode(),
                AnchorContractCalls::MaxEdges(element) => element.encode(),
                AnchorContractCalls::NeighborRoots(element) => element.encode(),
                AnchorContractCalls::NextIndex(element) => element.encode(),
                AnchorContractCalls::NullifierHashes(element) => {
                    element.encode()
                }
                AnchorContractCalls::Roots(element) => element.encode(),
                AnchorContractCalls::SetBridge(element) => element.encode(),
                AnchorContractCalls::SetHandler(element) => element.encode(),
                AnchorContractCalls::Token(element) => element.encode(),
                AnchorContractCalls::UnpackProof(element) => element.encode(),
                AnchorContractCalls::UnwrapIntoNative(element) => {
                    element.encode()
                }
                AnchorContractCalls::UnwrapIntoToken(element) => {
                    element.encode()
                }
                AnchorContractCalls::UpdateEdge(element) => element.encode(),
                AnchorContractCalls::Verifier(element) => element.encode(),
                AnchorContractCalls::Withdraw(element) => element.encode(),
                AnchorContractCalls::WithdrawAndUnwrap(element) => {
                    element.encode()
                }
                AnchorContractCalls::WrapAndDeposit(element) => {
                    element.encode()
                }
                AnchorContractCalls::WrapNative(element) => element.encode(),
                AnchorContractCalls::WrapToken(element) => element.encode(),
                AnchorContractCalls::Zeros(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AnchorContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AnchorContractCalls::FieldSize(element) => element.fmt(f),
                AnchorContractCalls::RootHistorySize(element) => element.fmt(f),
                AnchorContractCalls::ZeroValue(element) => element.fmt(f),
                AnchorContractCalls::AddEdge(element) => element.fmt(f),
                AnchorContractCalls::Admin(element) => element.fmt(f),
                AnchorContractCalls::Bridge(element) => element.fmt(f),
                AnchorContractCalls::Commitments(element) => element.fmt(f),
                AnchorContractCalls::CurrentNeighborRootIndex(element) => {
                    element.fmt(f)
                }
                AnchorContractCalls::CurrentRootIndex(element) => {
                    element.fmt(f)
                }
                AnchorContractCalls::Denomination(element) => element.fmt(f),
                AnchorContractCalls::Deposit(element) => element.fmt(f),
                AnchorContractCalls::EdgeExistsForChain(element) => {
                    element.fmt(f)
                }
                AnchorContractCalls::EdgeIndex(element) => element.fmt(f),
                AnchorContractCalls::EdgeList(element) => element.fmt(f),
                AnchorContractCalls::FilledSubtrees(element) => element.fmt(f),
                AnchorContractCalls::GetDenomination(element) => element.fmt(f),
                AnchorContractCalls::GetLastRoot(element) => element.fmt(f),
                AnchorContractCalls::GetLatestNeighborRoots(element) => {
                    element.fmt(f)
                }
                AnchorContractCalls::GetToken(element) => element.fmt(f),
                AnchorContractCalls::Handler(element) => element.fmt(f),
                AnchorContractCalls::HasEdge(element) => element.fmt(f),
                AnchorContractCalls::HashLeftRight(element) => element.fmt(f),
                AnchorContractCalls::Hasher(element) => element.fmt(f),
                AnchorContractCalls::IsKnownNeighborRoot(element) => {
                    element.fmt(f)
                }
                AnchorContractCalls::IsKnownRoot(element) => element.fmt(f),
                AnchorContractCalls::IsSpent(element) => element.fmt(f),
                AnchorContractCalls::IsSpentArray(element) => element.fmt(f),
                AnchorContractCalls::IsValidRoots(element) => element.fmt(f),
                AnchorContractCalls::Levels(element) => element.fmt(f),
                AnchorContractCalls::MaxEdges(element) => element.fmt(f),
                AnchorContractCalls::NeighborRoots(element) => element.fmt(f),
                AnchorContractCalls::NextIndex(element) => element.fmt(f),
                AnchorContractCalls::NullifierHashes(element) => element.fmt(f),
                AnchorContractCalls::Roots(element) => element.fmt(f),
                AnchorContractCalls::SetBridge(element) => element.fmt(f),
                AnchorContractCalls::SetHandler(element) => element.fmt(f),
                AnchorContractCalls::Token(element) => element.fmt(f),
                AnchorContractCalls::UnpackProof(element) => element.fmt(f),
                AnchorContractCalls::UnwrapIntoNative(element) => {
                    element.fmt(f)
                }
                AnchorContractCalls::UnwrapIntoToken(element) => element.fmt(f),
                AnchorContractCalls::UpdateEdge(element) => element.fmt(f),
                AnchorContractCalls::Verifier(element) => element.fmt(f),
                AnchorContractCalls::Withdraw(element) => element.fmt(f),
                AnchorContractCalls::WithdrawAndUnwrap(element) => {
                    element.fmt(f)
                }
                AnchorContractCalls::WrapAndDeposit(element) => element.fmt(f),
                AnchorContractCalls::WrapNative(element) => element.fmt(f),
                AnchorContractCalls::WrapToken(element) => element.fmt(f),
                AnchorContractCalls::Zeros(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FieldSizeCall> for AnchorContractCalls {
        fn from(var: FieldSizeCall) -> Self {
            AnchorContractCalls::FieldSize(var)
        }
    }
    impl ::std::convert::From<RootHistorySizeCall> for AnchorContractCalls {
        fn from(var: RootHistorySizeCall) -> Self {
            AnchorContractCalls::RootHistorySize(var)
        }
    }
    impl ::std::convert::From<ZeroValueCall> for AnchorContractCalls {
        fn from(var: ZeroValueCall) -> Self {
            AnchorContractCalls::ZeroValue(var)
        }
    }
    impl ::std::convert::From<AddEdgeCall> for AnchorContractCalls {
        fn from(var: AddEdgeCall) -> Self {
            AnchorContractCalls::AddEdge(var)
        }
    }
    impl ::std::convert::From<AdminCall> for AnchorContractCalls {
        fn from(var: AdminCall) -> Self {
            AnchorContractCalls::Admin(var)
        }
    }
    impl ::std::convert::From<BridgeCall> for AnchorContractCalls {
        fn from(var: BridgeCall) -> Self {
            AnchorContractCalls::Bridge(var)
        }
    }
    impl ::std::convert::From<CommitmentsCall> for AnchorContractCalls {
        fn from(var: CommitmentsCall) -> Self {
            AnchorContractCalls::Commitments(var)
        }
    }
    impl ::std::convert::From<CurrentNeighborRootIndexCall>
        for AnchorContractCalls
    {
        fn from(var: CurrentNeighborRootIndexCall) -> Self {
            AnchorContractCalls::CurrentNeighborRootIndex(var)
        }
    }
    impl ::std::convert::From<CurrentRootIndexCall> for AnchorContractCalls {
        fn from(var: CurrentRootIndexCall) -> Self {
            AnchorContractCalls::CurrentRootIndex(var)
        }
    }
    impl ::std::convert::From<DenominationCall> for AnchorContractCalls {
        fn from(var: DenominationCall) -> Self {
            AnchorContractCalls::Denomination(var)
        }
    }
    impl ::std::convert::From<DepositCall> for AnchorContractCalls {
        fn from(var: DepositCall) -> Self {
            AnchorContractCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<EdgeExistsForChainCall> for AnchorContractCalls {
        fn from(var: EdgeExistsForChainCall) -> Self {
            AnchorContractCalls::EdgeExistsForChain(var)
        }
    }
    impl ::std::convert::From<EdgeIndexCall> for AnchorContractCalls {
        fn from(var: EdgeIndexCall) -> Self {
            AnchorContractCalls::EdgeIndex(var)
        }
    }
    impl ::std::convert::From<EdgeListCall> for AnchorContractCalls {
        fn from(var: EdgeListCall) -> Self {
            AnchorContractCalls::EdgeList(var)
        }
    }
    impl ::std::convert::From<FilledSubtreesCall> for AnchorContractCalls {
        fn from(var: FilledSubtreesCall) -> Self {
            AnchorContractCalls::FilledSubtrees(var)
        }
    }
    impl ::std::convert::From<GetDenominationCall> for AnchorContractCalls {
        fn from(var: GetDenominationCall) -> Self {
            AnchorContractCalls::GetDenomination(var)
        }
    }
    impl ::std::convert::From<GetLastRootCall> for AnchorContractCalls {
        fn from(var: GetLastRootCall) -> Self {
            AnchorContractCalls::GetLastRoot(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborRootsCall> for AnchorContractCalls {
        fn from(var: GetLatestNeighborRootsCall) -> Self {
            AnchorContractCalls::GetLatestNeighborRoots(var)
        }
    }
    impl ::std::convert::From<GetTokenCall> for AnchorContractCalls {
        fn from(var: GetTokenCall) -> Self {
            AnchorContractCalls::GetToken(var)
        }
    }
    impl ::std::convert::From<HandlerCall> for AnchorContractCalls {
        fn from(var: HandlerCall) -> Self {
            AnchorContractCalls::Handler(var)
        }
    }
    impl ::std::convert::From<HasEdgeCall> for AnchorContractCalls {
        fn from(var: HasEdgeCall) -> Self {
            AnchorContractCalls::HasEdge(var)
        }
    }
    impl ::std::convert::From<HashLeftRightCall> for AnchorContractCalls {
        fn from(var: HashLeftRightCall) -> Self {
            AnchorContractCalls::HashLeftRight(var)
        }
    }
    impl ::std::convert::From<HasherCall> for AnchorContractCalls {
        fn from(var: HasherCall) -> Self {
            AnchorContractCalls::Hasher(var)
        }
    }
    impl ::std::convert::From<IsKnownNeighborRootCall> for AnchorContractCalls {
        fn from(var: IsKnownNeighborRootCall) -> Self {
            AnchorContractCalls::IsKnownNeighborRoot(var)
        }
    }
    impl ::std::convert::From<IsKnownRootCall> for AnchorContractCalls {
        fn from(var: IsKnownRootCall) -> Self {
            AnchorContractCalls::IsKnownRoot(var)
        }
    }
    impl ::std::convert::From<IsSpentCall> for AnchorContractCalls {
        fn from(var: IsSpentCall) -> Self {
            AnchorContractCalls::IsSpent(var)
        }
    }
    impl ::std::convert::From<IsSpentArrayCall> for AnchorContractCalls {
        fn from(var: IsSpentArrayCall) -> Self {
            AnchorContractCalls::IsSpentArray(var)
        }
    }
    impl ::std::convert::From<IsValidRootsCall> for AnchorContractCalls {
        fn from(var: IsValidRootsCall) -> Self {
            AnchorContractCalls::IsValidRoots(var)
        }
    }
    impl ::std::convert::From<LevelsCall> for AnchorContractCalls {
        fn from(var: LevelsCall) -> Self {
            AnchorContractCalls::Levels(var)
        }
    }
    impl ::std::convert::From<MaxEdgesCall> for AnchorContractCalls {
        fn from(var: MaxEdgesCall) -> Self {
            AnchorContractCalls::MaxEdges(var)
        }
    }
    impl ::std::convert::From<NeighborRootsCall> for AnchorContractCalls {
        fn from(var: NeighborRootsCall) -> Self {
            AnchorContractCalls::NeighborRoots(var)
        }
    }
    impl ::std::convert::From<NextIndexCall> for AnchorContractCalls {
        fn from(var: NextIndexCall) -> Self {
            AnchorContractCalls::NextIndex(var)
        }
    }
    impl ::std::convert::From<NullifierHashesCall> for AnchorContractCalls {
        fn from(var: NullifierHashesCall) -> Self {
            AnchorContractCalls::NullifierHashes(var)
        }
    }
    impl ::std::convert::From<RootsCall> for AnchorContractCalls {
        fn from(var: RootsCall) -> Self {
            AnchorContractCalls::Roots(var)
        }
    }
    impl ::std::convert::From<SetBridgeCall> for AnchorContractCalls {
        fn from(var: SetBridgeCall) -> Self {
            AnchorContractCalls::SetBridge(var)
        }
    }
    impl ::std::convert::From<SetHandlerCall> for AnchorContractCalls {
        fn from(var: SetHandlerCall) -> Self {
            AnchorContractCalls::SetHandler(var)
        }
    }
    impl ::std::convert::From<TokenCall> for AnchorContractCalls {
        fn from(var: TokenCall) -> Self {
            AnchorContractCalls::Token(var)
        }
    }
    impl ::std::convert::From<UnpackProofCall> for AnchorContractCalls {
        fn from(var: UnpackProofCall) -> Self {
            AnchorContractCalls::UnpackProof(var)
        }
    }
    impl ::std::convert::From<UnwrapIntoNativeCall> for AnchorContractCalls {
        fn from(var: UnwrapIntoNativeCall) -> Self {
            AnchorContractCalls::UnwrapIntoNative(var)
        }
    }
    impl ::std::convert::From<UnwrapIntoTokenCall> for AnchorContractCalls {
        fn from(var: UnwrapIntoTokenCall) -> Self {
            AnchorContractCalls::UnwrapIntoToken(var)
        }
    }
    impl ::std::convert::From<UpdateEdgeCall> for AnchorContractCalls {
        fn from(var: UpdateEdgeCall) -> Self {
            AnchorContractCalls::UpdateEdge(var)
        }
    }
    impl ::std::convert::From<VerifierCall> for AnchorContractCalls {
        fn from(var: VerifierCall) -> Self {
            AnchorContractCalls::Verifier(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for AnchorContractCalls {
        fn from(var: WithdrawCall) -> Self {
            AnchorContractCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawAndUnwrapCall> for AnchorContractCalls {
        fn from(var: WithdrawAndUnwrapCall) -> Self {
            AnchorContractCalls::WithdrawAndUnwrap(var)
        }
    }
    impl ::std::convert::From<WrapAndDepositCall> for AnchorContractCalls {
        fn from(var: WrapAndDepositCall) -> Self {
            AnchorContractCalls::WrapAndDeposit(var)
        }
    }
    impl ::std::convert::From<WrapNativeCall> for AnchorContractCalls {
        fn from(var: WrapNativeCall) -> Self {
            AnchorContractCalls::WrapNative(var)
        }
    }
    impl ::std::convert::From<WrapTokenCall> for AnchorContractCalls {
        fn from(var: WrapTokenCall) -> Self {
            AnchorContractCalls::WrapToken(var)
        }
    }
    impl ::std::convert::From<ZerosCall> for AnchorContractCalls {
        fn from(var: ZerosCall) -> Self {
            AnchorContractCalls::Zeros(var)
        }
    }
    #[doc = "`PublicInputs(bytes,bytes32,bytes32,address,address,uint256,uint256)`"]
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthAbiType,
    )]
    pub struct PublicInputs {
        pub roots: Vec<u8>,
        pub nullifier_hash: [u8; 32],
        pub refresh_commitment: [u8; 32],
        pub recipient: ethers::core::types::Address,
        pub relayer: ethers::core::types::Address,
        pub fee: ethers::core::types::U256,
        pub refund: ethers::core::types::U256,
    }
}
