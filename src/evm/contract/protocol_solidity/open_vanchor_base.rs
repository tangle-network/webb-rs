pub use open_v_anchor_base_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod open_v_anchor_base_contract {
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
    #[doc = "OpenVAnchorBaseContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static OPENVANCHORBASECONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EdgeAddition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EdgeUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint32\",\"name\":\"leafIndex\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Insertion\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCommitment\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"nullifier\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewNullifier\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"key\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PublicKey\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EVM_CHAIN_ID_TYPE\",\"outputs\":[{\"internalType\":\"bytes2\",\"name\":\"\",\"type\":\"bytes2\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FIELD_SIZE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_EXT_AMOUNT\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_FEE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ROOT_HISTORY_SIZE\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ZERO_VALUE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureMaximumDepositLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureMinimalWithdrawalLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentNeighborRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeExistsForChain\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeList\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"srcResourceID\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledSubtrees\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainIdType\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestNeighborEdges\",\"outputs\":[{\"internalType\":\"struct OpenLinkableAnchor.Edge[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"srcResourceID\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestNeighborRoots\",\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProposalNonce\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"handler\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_chainID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasEdge\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IHasher\",\"name\":\"_hasher\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_left\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_right\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hashLeftRight\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasher\",\"outputs\":[{\"internalType\":\"contract IHasher\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_neighborChainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isKnownNeighborRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isKnownRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_nullifierHashes\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpentArray\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"\",\"type\":\"bool[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_roots\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidRoots\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"levels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maximumDepositAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minimalWithdrawalAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"neighborRoots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nextIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nullifierHashes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_resourceId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseChainIdFromResourceId\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafindex\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setHandler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_leafIndex\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_srcResourceID\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"updateEdge\",\"outputs\":[]}]") . expect ("invalid abi")
    });
    pub struct OpenVAnchorBaseContract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for OpenVAnchorBaseContract<M> {
        fn clone(&self) -> Self {
            OpenVAnchorBaseContract(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for OpenVAnchorBaseContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for OpenVAnchorBaseContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(OpenVAnchorBaseContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> OpenVAnchorBaseContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                OPENVANCHORBASECONTRACT_ABI.clone(),
                client,
            )
            .into()
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
        #[doc = "Calls the contract's `MAX_EXT_AMOUNT` (0x7fe24ffe) function"]
        pub fn max_ext_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, I256> {
            self.0
                .method_hash([127, 226, 79, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_FEE` (0xbc063e1a) function"]
        pub fn max_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([188, 6, 62, 26], ())
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
        #[doc = "Calls the contract's `configureMaximumDepositLimit` (0x8c832b13) function"]
        pub fn configure_maximum_deposit_limit(
            &self,
            maximum_deposit_amount: ethers::core::types::U256,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [140, 131, 43, 19],
                    (maximum_deposit_amount, nonce),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `configureMinimalWithdrawalLimit` (0x1f7f99f7) function"]
        pub fn configure_minimal_withdrawal_limit(
            &self,
            minimal_withdrawal_amount: ethers::core::types::U256,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [31, 127, 153, 247],
                    (minimal_withdrawal_amount, nonce),
                )
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
        #[doc = "Calls the contract's `initialize` (0xe4a30116) function"]
        pub fn initialize(
            &self,
            minimal_withdrawal_amount: ethers::core::types::U256,
            maximum_deposit_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [228, 163, 1, 22],
                    (minimal_withdrawal_amount, maximum_deposit_amount),
                )
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
        #[doc = "Calls the contract's `lastBalance` (0x8f1c56bd) function"]
        pub fn last_balance(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([143, 28, 86, 189], ())
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
        #[doc = "Calls the contract's `maximumDepositAmount` (0x78abb49b) function"]
        pub fn maximum_deposit_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([120, 171, 180, 155], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minimalWithdrawalAmount` (0x840b2791) function"]
        pub fn minimal_withdrawal_amount(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([132, 11, 39, 145], ())
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
        #[doc = "Calls the contract's `parseChainIdFromResourceId` (0xc2230d6e) function"]
        pub fn parse_chain_id_from_resource_id(
            &self,
            resource_id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([194, 35, 13, 110], resource_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roots` (0xc2b40ae4) function"]
        pub fn roots(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ([u8; 32], ethers::core::types::U256),
        > {
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
        #[doc = "Calls the contract's `updateEdge` (0xa07b7e1d) function"]
        pub fn update_edge(
            &self,
            root: [u8; 32],
            leaf_index: u32,
            src_resource_id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [160, 123, 126, 29],
                    (root, leaf_index, src_resource_id),
                )
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
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Insertion` event"]
        pub fn insertion_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InsertionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewCommitment` event"]
        pub fn new_commitment_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewCommitmentFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewNullifier` event"]
        pub fn new_nullifier_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NewNullifierFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PublicKey` event"]
        pub fn public_key_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PublicKeyFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, OpenVAnchorBaseContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for OpenVAnchorBaseContract<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
    #[ethevent(name = "NewCommitment", abi = "NewCommitment(bytes32,uint256)")]
    pub struct NewCommitmentFilter {
        pub commitment: [u8; 32],
        pub index: ethers::core::types::U256,
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
    #[ethevent(name = "NewNullifier", abi = "NewNullifier(bytes32)")]
    pub struct NewNullifierFilter {
        pub nullifier: [u8; 32],
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
    #[ethevent(name = "PublicKey", abi = "PublicKey(address,bytes)")]
    pub struct PublicKeyFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        pub key: ethers::core::types::Bytes,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum OpenVAnchorBaseContractEvents {
        EdgeAdditionFilter(EdgeAdditionFilter),
        EdgeUpdateFilter(EdgeUpdateFilter),
        InitializedFilter(InitializedFilter),
        InsertionFilter(InsertionFilter),
        NewCommitmentFilter(NewCommitmentFilter),
        NewNullifierFilter(NewNullifierFilter),
        PublicKeyFilter(PublicKeyFilter),
    }
    impl ethers::contract::EthLogDecode for OpenVAnchorBaseContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = EdgeAdditionFilter::decode_log(log) {
                return Ok(OpenVAnchorBaseContractEvents::EdgeAdditionFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EdgeUpdateFilter::decode_log(log) {
                return Ok(OpenVAnchorBaseContractEvents::EdgeUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(OpenVAnchorBaseContractEvents::InitializedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InsertionFilter::decode_log(log) {
                return Ok(OpenVAnchorBaseContractEvents::InsertionFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewCommitmentFilter::decode_log(log) {
                return Ok(OpenVAnchorBaseContractEvents::NewCommitmentFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewNullifierFilter::decode_log(log) {
                return Ok(OpenVAnchorBaseContractEvents::NewNullifierFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PublicKeyFilter::decode_log(log) {
                return Ok(OpenVAnchorBaseContractEvents::PublicKeyFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for OpenVAnchorBaseContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OpenVAnchorBaseContractEvents::EdgeAdditionFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorBaseContractEvents::EdgeUpdateFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorBaseContractEvents::InitializedFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorBaseContractEvents::InsertionFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorBaseContractEvents::NewCommitmentFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorBaseContractEvents::NewNullifierFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorBaseContractEvents::PublicKeyFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `EVM_CHAIN_ID_TYPE` function with signature `EVM_CHAIN_ID_TYPE()` and selector `[139, 126, 135, 130]`"]
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
    #[doc = "Container type for all input parameters for the `FIELD_SIZE` function with signature `FIELD_SIZE()` and selector `[65, 74, 55, 186]`"]
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
    #[doc = "Container type for all input parameters for the `MAX_EXT_AMOUNT` function with signature `MAX_EXT_AMOUNT()` and selector `[127, 226, 79, 254]`"]
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
    #[ethcall(name = "MAX_EXT_AMOUNT", abi = "MAX_EXT_AMOUNT()")]
    pub struct MaxExtAmountCall;
    #[doc = "Container type for all input parameters for the `MAX_FEE` function with signature `MAX_FEE()` and selector `[188, 6, 62, 26]`"]
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
    #[ethcall(name = "MAX_FEE", abi = "MAX_FEE()")]
    pub struct MaxFeeCall;
    #[doc = "Container type for all input parameters for the `ROOT_HISTORY_SIZE` function with signature `ROOT_HISTORY_SIZE()` and selector `[205, 135, 163, 180]`"]
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
    #[doc = "Container type for all input parameters for the `ZERO_VALUE` function with signature `ZERO_VALUE()` and selector `[236, 115, 41, 89]`"]
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
    #[doc = "Container type for all input parameters for the `commitments` function with signature `commitments(bytes32)` and selector `[131, 157, 249, 69]`"]
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
    #[doc = "Container type for all input parameters for the `configureMaximumDepositLimit` function with signature `configureMaximumDepositLimit(uint256,uint32)` and selector `[140, 131, 43, 19]`"]
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
        name = "configureMaximumDepositLimit",
        abi = "configureMaximumDepositLimit(uint256,uint32)"
    )]
    pub struct ConfigureMaximumDepositLimitCall {
        pub maximum_deposit_amount: ethers::core::types::U256,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `configureMinimalWithdrawalLimit` function with signature `configureMinimalWithdrawalLimit(uint256,uint32)` and selector `[31, 127, 153, 247]`"]
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
        name = "configureMinimalWithdrawalLimit",
        abi = "configureMinimalWithdrawalLimit(uint256,uint32)"
    )]
    pub struct ConfigureMinimalWithdrawalLimitCall {
        pub minimal_withdrawal_amount: ethers::core::types::U256,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `currentNeighborRootIndex` function with signature `currentNeighborRootIndex(uint256)` and selector `[93, 45, 118, 108]`"]
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
    #[doc = "Container type for all input parameters for the `currentRootIndex` function with signature `currentRootIndex()` and selector `[144, 238, 176, 43]`"]
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
    #[doc = "Container type for all input parameters for the `edgeExistsForChain` function with signature `edgeExistsForChain(uint256)` and selector `[250, 115, 22, 135]`"]
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
    #[doc = "Container type for all input parameters for the `edgeIndex` function with signature `edgeIndex(uint256)` and selector `[231, 14, 168, 124]`"]
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
    #[doc = "Container type for all input parameters for the `edgeList` function with signature `edgeList(uint256)` and selector `[219, 201, 22, 184]`"]
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
    #[doc = "Container type for all input parameters for the `filledSubtrees` function with signature `filledSubtrees(uint256)` and selector `[241, 120, 228, 124]`"]
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
    #[doc = "Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
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
    #[doc = "Container type for all input parameters for the `getChainIdType` function with signature `getChainIdType()` and selector `[76, 131, 12, 189]`"]
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
    #[doc = "Container type for all input parameters for the `getLastRoot` function with signature `getLastRoot()` and selector `[186, 112, 247, 87]`"]
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
    #[doc = "Container type for all input parameters for the `getLatestNeighborEdges` function with signature `getLatestNeighborEdges()` and selector `[140, 13, 52, 216]`"]
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
    #[doc = "Container type for all input parameters for the `getLatestNeighborRoots` function with signature `getLatestNeighborRoots()` and selector `[30, 98, 118, 23]`"]
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
    #[doc = "Container type for all input parameters for the `getProposalNonce` function with signature `getProposalNonce()` and selector `[11, 39, 251, 154]`"]
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
    #[doc = "Container type for all input parameters for the `handler` function with signature `handler()` and selector `[200, 9, 22, 212]`"]
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
    #[doc = "Container type for all input parameters for the `hasEdge` function with signature `hasEdge(uint256)` and selector `[146, 21, 99, 17]`"]
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
    #[doc = "Container type for all input parameters for the `hashLeftRight` function with signature `hashLeftRight(address,bytes32,bytes32)` and selector `[142, 163, 9, 158]`"]
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
    #[doc = "Container type for all input parameters for the `hasher` function with signature `hasher()` and selector `[237, 51, 99, 159]`"]
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
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(uint256,uint256)` and selector `[228, 163, 1, 22]`"]
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
    #[ethcall(name = "initialize", abi = "initialize(uint256,uint256)")]
    pub struct InitializeCall {
        pub minimal_withdrawal_amount: ethers::core::types::U256,
        pub maximum_deposit_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isKnownNeighborRoot` function with signature `isKnownNeighborRoot(uint256,bytes32)` and selector `[17, 228, 220, 185]`"]
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
    #[doc = "Container type for all input parameters for the `isKnownRoot` function with signature `isKnownRoot(bytes32)` and selector `[109, 152, 51, 227]`"]
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
    #[doc = "Container type for all input parameters for the `isSpent` function with signature `isSpent(bytes32)` and selector `[229, 40, 93, 204]`"]
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
    #[doc = "Container type for all input parameters for the `isSpentArray` function with signature `isSpentArray(bytes32[])` and selector `[159, 161, 45, 11]`"]
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
    #[doc = "Container type for all input parameters for the `isValidRoots` function with signature `isValidRoots(bytes32[])` and selector `[97, 110, 9, 87]`"]
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
    #[doc = "Container type for all input parameters for the `lastBalance` function with signature `lastBalance()` and selector `[143, 28, 86, 189]`"]
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
    #[ethcall(name = "lastBalance", abi = "lastBalance()")]
    pub struct LastBalanceCall;
    #[doc = "Container type for all input parameters for the `levels` function with signature `levels()` and selector `[78, 207, 81, 139]`"]
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
    #[doc = "Container type for all input parameters for the `maximumDepositAmount` function with signature `maximumDepositAmount()` and selector `[120, 171, 180, 155]`"]
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
    #[ethcall(name = "maximumDepositAmount", abi = "maximumDepositAmount()")]
    pub struct MaximumDepositAmountCall;
    #[doc = "Container type for all input parameters for the `minimalWithdrawalAmount` function with signature `minimalWithdrawalAmount()` and selector `[132, 11, 39, 145]`"]
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
        name = "minimalWithdrawalAmount",
        abi = "minimalWithdrawalAmount()"
    )]
    pub struct MinimalWithdrawalAmountCall;
    #[doc = "Container type for all input parameters for the `neighborRoots` function with signature `neighborRoots(uint256,uint32)` and selector `[67, 231, 17, 159]`"]
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
    #[doc = "Container type for all input parameters for the `nextIndex` function with signature `nextIndex()` and selector `[252, 126, 156, 111]`"]
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
    #[doc = "Container type for all input parameters for the `nullifierHashes` function with signature `nullifierHashes(bytes32)` and selector `[23, 204, 145, 92]`"]
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
    #[doc = "Container type for all input parameters for the `parseChainIdFromResourceId` function with signature `parseChainIdFromResourceId(bytes32)` and selector `[194, 35, 13, 110]`"]
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
        name = "parseChainIdFromResourceId",
        abi = "parseChainIdFromResourceId(bytes32)"
    )]
    pub struct ParseChainIdFromResourceIdCall {
        pub resource_id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `roots` function with signature `roots(uint256)` and selector `[194, 180, 10, 228]`"]
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
    #[doc = "Container type for all input parameters for the `setHandler` function with signature `setHandler(address,uint32)` and selector `[114, 193, 173, 3]`"]
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
    #[doc = "Container type for all input parameters for the `updateEdge` function with signature `updateEdge(bytes32,uint32,bytes32)` and selector `[160, 123, 126, 29]`"]
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
    #[ethcall(name = "updateEdge", abi = "updateEdge(bytes32,uint32,bytes32)")]
    pub struct UpdateEdgeCall {
        pub root: [u8; 32],
        pub leaf_index: u32,
        pub src_resource_id: [u8; 32],
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum OpenVAnchorBaseContractCalls {
        EvmChainIdType(EvmChainIdTypeCall),
        FieldSize(FieldSizeCall),
        MaxExtAmount(MaxExtAmountCall),
        MaxFee(MaxFeeCall),
        RootHistorySize(RootHistorySizeCall),
        ZeroValue(ZeroValueCall),
        Commitments(CommitmentsCall),
        ConfigureMaximumDepositLimit(ConfigureMaximumDepositLimitCall),
        ConfigureMinimalWithdrawalLimit(ConfigureMinimalWithdrawalLimitCall),
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
        Initialize(InitializeCall),
        IsKnownNeighborRoot(IsKnownNeighborRootCall),
        IsKnownRoot(IsKnownRootCall),
        IsSpent(IsSpentCall),
        IsSpentArray(IsSpentArrayCall),
        IsValidRoots(IsValidRootsCall),
        LastBalance(LastBalanceCall),
        Levels(LevelsCall),
        MaximumDepositAmount(MaximumDepositAmountCall),
        MinimalWithdrawalAmount(MinimalWithdrawalAmountCall),
        NeighborRoots(NeighborRootsCall),
        NextIndex(NextIndexCall),
        NullifierHashes(NullifierHashesCall),
        ParseChainIdFromResourceId(ParseChainIdFromResourceIdCall),
        Roots(RootsCall),
        SetHandler(SetHandlerCall),
        UpdateEdge(UpdateEdgeCall),
    }
    impl ethers::core::abi::AbiDecode for OpenVAnchorBaseContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <EvmChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::EvmChainIdType(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <FieldSizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::FieldSize(decoded));
            }
            if let Ok(decoded) =
                <MaxExtAmountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::MaxExtAmount(decoded));
            }
            if let Ok(decoded) =
                <MaxFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::MaxFee(decoded));
            }
            if let Ok(decoded) =
                <RootHistorySizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::RootHistorySize(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ZeroValueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::ZeroValue(decoded));
            }
            if let Ok(decoded) =
                <CommitmentsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::Commitments(decoded));
            }
            if let Ok (decoded) = < ConfigureMaximumDepositLimitCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorBaseContractCalls :: ConfigureMaximumDepositLimit (decoded)) }
            if let Ok (decoded) = < ConfigureMinimalWithdrawalLimitCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorBaseContractCalls :: ConfigureMinimalWithdrawalLimit (decoded)) }
            if let Ok (decoded) = < CurrentNeighborRootIndexCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorBaseContractCalls :: CurrentNeighborRootIndex (decoded)) }
            if let Ok(decoded) =
                <CurrentRootIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::CurrentRootIndex(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <EdgeExistsForChainCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::EdgeExistsForChain(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <EdgeIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::EdgeIndex(decoded));
            }
            if let Ok(decoded) =
                <EdgeListCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::EdgeList(decoded));
            }
            if let Ok(decoded) =
                <FilledSubtreesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::FilledSubtrees(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::GetChainIdType(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetLastRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::GetLastRoot(decoded));
            }
            if let Ok (decoded) = < GetLatestNeighborEdgesCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorBaseContractCalls :: GetLatestNeighborEdges (decoded)) }
            if let Ok (decoded) = < GetLatestNeighborRootsCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorBaseContractCalls :: GetLatestNeighborRoots (decoded)) }
            if let Ok(decoded) =
                <GetProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::GetProposalNonce(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <HandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::HasEdge(decoded));
            }
            if let Ok(decoded) =
                <HashLeftRightCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::HashLeftRight(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <HasherCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::Hasher(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::Initialize(decoded));
            }
            if let Ok (decoded) = < IsKnownNeighborRootCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorBaseContractCalls :: IsKnownNeighborRoot (decoded)) }
            if let Ok(decoded) =
                <IsKnownRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::IsKnownRoot(decoded));
            }
            if let Ok(decoded) =
                <IsSpentCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::IsSpent(decoded));
            }
            if let Ok(decoded) =
                <IsSpentArrayCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::IsSpentArray(decoded));
            }
            if let Ok(decoded) =
                <IsValidRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::IsValidRoots(decoded));
            }
            if let Ok(decoded) =
                <LastBalanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::LastBalance(decoded));
            }
            if let Ok(decoded) =
                <LevelsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::Levels(decoded));
            }
            if let Ok (decoded) = < MaximumDepositAmountCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorBaseContractCalls :: MaximumDepositAmount (decoded)) }
            if let Ok (decoded) = < MinimalWithdrawalAmountCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorBaseContractCalls :: MinimalWithdrawalAmount (decoded)) }
            if let Ok(decoded) =
                <NeighborRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::NeighborRoots(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <NextIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::NextIndex(decoded));
            }
            if let Ok(decoded) =
                <NullifierHashesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::NullifierHashes(
                    decoded,
                ));
            }
            if let Ok (decoded) = < ParseChainIdFromResourceIdCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorBaseContractCalls :: ParseChainIdFromResourceId (decoded)) }
            if let Ok(decoded) =
                <RootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::Roots(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::SetHandler(decoded));
            }
            if let Ok(decoded) =
                <UpdateEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorBaseContractCalls::UpdateEdge(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for OpenVAnchorBaseContractCalls {
        fn encode(self) -> Vec<u8> {
            match self { OpenVAnchorBaseContractCalls :: EvmChainIdType (element) => element . encode () , OpenVAnchorBaseContractCalls :: FieldSize (element) => element . encode () , OpenVAnchorBaseContractCalls :: MaxExtAmount (element) => element . encode () , OpenVAnchorBaseContractCalls :: MaxFee (element) => element . encode () , OpenVAnchorBaseContractCalls :: RootHistorySize (element) => element . encode () , OpenVAnchorBaseContractCalls :: ZeroValue (element) => element . encode () , OpenVAnchorBaseContractCalls :: Commitments (element) => element . encode () , OpenVAnchorBaseContractCalls :: ConfigureMaximumDepositLimit (element) => element . encode () , OpenVAnchorBaseContractCalls :: ConfigureMinimalWithdrawalLimit (element) => element . encode () , OpenVAnchorBaseContractCalls :: CurrentNeighborRootIndex (element) => element . encode () , OpenVAnchorBaseContractCalls :: CurrentRootIndex (element) => element . encode () , OpenVAnchorBaseContractCalls :: EdgeExistsForChain (element) => element . encode () , OpenVAnchorBaseContractCalls :: EdgeIndex (element) => element . encode () , OpenVAnchorBaseContractCalls :: EdgeList (element) => element . encode () , OpenVAnchorBaseContractCalls :: FilledSubtrees (element) => element . encode () , OpenVAnchorBaseContractCalls :: GetChainId (element) => element . encode () , OpenVAnchorBaseContractCalls :: GetChainIdType (element) => element . encode () , OpenVAnchorBaseContractCalls :: GetLastRoot (element) => element . encode () , OpenVAnchorBaseContractCalls :: GetLatestNeighborEdges (element) => element . encode () , OpenVAnchorBaseContractCalls :: GetLatestNeighborRoots (element) => element . encode () , OpenVAnchorBaseContractCalls :: GetProposalNonce (element) => element . encode () , OpenVAnchorBaseContractCalls :: Handler (element) => element . encode () , OpenVAnchorBaseContractCalls :: HasEdge (element) => element . encode () , OpenVAnchorBaseContractCalls :: HashLeftRight (element) => element . encode () , OpenVAnchorBaseContractCalls :: Hasher (element) => element . encode () , OpenVAnchorBaseContractCalls :: Initialize (element) => element . encode () , OpenVAnchorBaseContractCalls :: IsKnownNeighborRoot (element) => element . encode () , OpenVAnchorBaseContractCalls :: IsKnownRoot (element) => element . encode () , OpenVAnchorBaseContractCalls :: IsSpent (element) => element . encode () , OpenVAnchorBaseContractCalls :: IsSpentArray (element) => element . encode () , OpenVAnchorBaseContractCalls :: IsValidRoots (element) => element . encode () , OpenVAnchorBaseContractCalls :: LastBalance (element) => element . encode () , OpenVAnchorBaseContractCalls :: Levels (element) => element . encode () , OpenVAnchorBaseContractCalls :: MaximumDepositAmount (element) => element . encode () , OpenVAnchorBaseContractCalls :: MinimalWithdrawalAmount (element) => element . encode () , OpenVAnchorBaseContractCalls :: NeighborRoots (element) => element . encode () , OpenVAnchorBaseContractCalls :: NextIndex (element) => element . encode () , OpenVAnchorBaseContractCalls :: NullifierHashes (element) => element . encode () , OpenVAnchorBaseContractCalls :: ParseChainIdFromResourceId (element) => element . encode () , OpenVAnchorBaseContractCalls :: Roots (element) => element . encode () , OpenVAnchorBaseContractCalls :: SetHandler (element) => element . encode () , OpenVAnchorBaseContractCalls :: UpdateEdge (element) => element . encode () }
        }
    }
    impl ::std::fmt::Display for OpenVAnchorBaseContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self { OpenVAnchorBaseContractCalls :: EvmChainIdType (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: FieldSize (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: MaxExtAmount (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: MaxFee (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: RootHistorySize (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: ZeroValue (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: Commitments (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: ConfigureMaximumDepositLimit (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: ConfigureMinimalWithdrawalLimit (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: CurrentNeighborRootIndex (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: CurrentRootIndex (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: EdgeExistsForChain (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: EdgeIndex (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: EdgeList (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: FilledSubtrees (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: GetChainId (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: GetChainIdType (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: GetLastRoot (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: GetLatestNeighborEdges (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: GetLatestNeighborRoots (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: GetProposalNonce (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: Handler (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: HasEdge (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: HashLeftRight (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: Hasher (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: Initialize (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: IsKnownNeighborRoot (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: IsKnownRoot (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: IsSpent (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: IsSpentArray (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: IsValidRoots (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: LastBalance (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: Levels (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: MaximumDepositAmount (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: MinimalWithdrawalAmount (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: NeighborRoots (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: NextIndex (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: NullifierHashes (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: ParseChainIdFromResourceId (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: Roots (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: SetHandler (element) => element . fmt (f) , OpenVAnchorBaseContractCalls :: UpdateEdge (element) => element . fmt (f) }
        }
    }
    impl ::std::convert::From<EvmChainIdTypeCall> for OpenVAnchorBaseContractCalls {
        fn from(var: EvmChainIdTypeCall) -> Self {
            OpenVAnchorBaseContractCalls::EvmChainIdType(var)
        }
    }
    impl ::std::convert::From<FieldSizeCall> for OpenVAnchorBaseContractCalls {
        fn from(var: FieldSizeCall) -> Self {
            OpenVAnchorBaseContractCalls::FieldSize(var)
        }
    }
    impl ::std::convert::From<MaxExtAmountCall> for OpenVAnchorBaseContractCalls {
        fn from(var: MaxExtAmountCall) -> Self {
            OpenVAnchorBaseContractCalls::MaxExtAmount(var)
        }
    }
    impl ::std::convert::From<MaxFeeCall> for OpenVAnchorBaseContractCalls {
        fn from(var: MaxFeeCall) -> Self {
            OpenVAnchorBaseContractCalls::MaxFee(var)
        }
    }
    impl ::std::convert::From<RootHistorySizeCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: RootHistorySizeCall) -> Self {
            OpenVAnchorBaseContractCalls::RootHistorySize(var)
        }
    }
    impl ::std::convert::From<ZeroValueCall> for OpenVAnchorBaseContractCalls {
        fn from(var: ZeroValueCall) -> Self {
            OpenVAnchorBaseContractCalls::ZeroValue(var)
        }
    }
    impl ::std::convert::From<CommitmentsCall> for OpenVAnchorBaseContractCalls {
        fn from(var: CommitmentsCall) -> Self {
            OpenVAnchorBaseContractCalls::Commitments(var)
        }
    }
    impl ::std::convert::From<ConfigureMaximumDepositLimitCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: ConfigureMaximumDepositLimitCall) -> Self {
            OpenVAnchorBaseContractCalls::ConfigureMaximumDepositLimit(var)
        }
    }
    impl ::std::convert::From<ConfigureMinimalWithdrawalLimitCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: ConfigureMinimalWithdrawalLimitCall) -> Self {
            OpenVAnchorBaseContractCalls::ConfigureMinimalWithdrawalLimit(var)
        }
    }
    impl ::std::convert::From<CurrentNeighborRootIndexCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: CurrentNeighborRootIndexCall) -> Self {
            OpenVAnchorBaseContractCalls::CurrentNeighborRootIndex(var)
        }
    }
    impl ::std::convert::From<CurrentRootIndexCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: CurrentRootIndexCall) -> Self {
            OpenVAnchorBaseContractCalls::CurrentRootIndex(var)
        }
    }
    impl ::std::convert::From<EdgeExistsForChainCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: EdgeExistsForChainCall) -> Self {
            OpenVAnchorBaseContractCalls::EdgeExistsForChain(var)
        }
    }
    impl ::std::convert::From<EdgeIndexCall> for OpenVAnchorBaseContractCalls {
        fn from(var: EdgeIndexCall) -> Self {
            OpenVAnchorBaseContractCalls::EdgeIndex(var)
        }
    }
    impl ::std::convert::From<EdgeListCall> for OpenVAnchorBaseContractCalls {
        fn from(var: EdgeListCall) -> Self {
            OpenVAnchorBaseContractCalls::EdgeList(var)
        }
    }
    impl ::std::convert::From<FilledSubtreesCall> for OpenVAnchorBaseContractCalls {
        fn from(var: FilledSubtreesCall) -> Self {
            OpenVAnchorBaseContractCalls::FilledSubtrees(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for OpenVAnchorBaseContractCalls {
        fn from(var: GetChainIdCall) -> Self {
            OpenVAnchorBaseContractCalls::GetChainId(var)
        }
    }
    impl ::std::convert::From<GetChainIdTypeCall> for OpenVAnchorBaseContractCalls {
        fn from(var: GetChainIdTypeCall) -> Self {
            OpenVAnchorBaseContractCalls::GetChainIdType(var)
        }
    }
    impl ::std::convert::From<GetLastRootCall> for OpenVAnchorBaseContractCalls {
        fn from(var: GetLastRootCall) -> Self {
            OpenVAnchorBaseContractCalls::GetLastRoot(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborEdgesCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: GetLatestNeighborEdgesCall) -> Self {
            OpenVAnchorBaseContractCalls::GetLatestNeighborEdges(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborRootsCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: GetLatestNeighborRootsCall) -> Self {
            OpenVAnchorBaseContractCalls::GetLatestNeighborRoots(var)
        }
    }
    impl ::std::convert::From<GetProposalNonceCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: GetProposalNonceCall) -> Self {
            OpenVAnchorBaseContractCalls::GetProposalNonce(var)
        }
    }
    impl ::std::convert::From<HandlerCall> for OpenVAnchorBaseContractCalls {
        fn from(var: HandlerCall) -> Self {
            OpenVAnchorBaseContractCalls::Handler(var)
        }
    }
    impl ::std::convert::From<HasEdgeCall> for OpenVAnchorBaseContractCalls {
        fn from(var: HasEdgeCall) -> Self {
            OpenVAnchorBaseContractCalls::HasEdge(var)
        }
    }
    impl ::std::convert::From<HashLeftRightCall> for OpenVAnchorBaseContractCalls {
        fn from(var: HashLeftRightCall) -> Self {
            OpenVAnchorBaseContractCalls::HashLeftRight(var)
        }
    }
    impl ::std::convert::From<HasherCall> for OpenVAnchorBaseContractCalls {
        fn from(var: HasherCall) -> Self {
            OpenVAnchorBaseContractCalls::Hasher(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for OpenVAnchorBaseContractCalls {
        fn from(var: InitializeCall) -> Self {
            OpenVAnchorBaseContractCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsKnownNeighborRootCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: IsKnownNeighborRootCall) -> Self {
            OpenVAnchorBaseContractCalls::IsKnownNeighborRoot(var)
        }
    }
    impl ::std::convert::From<IsKnownRootCall> for OpenVAnchorBaseContractCalls {
        fn from(var: IsKnownRootCall) -> Self {
            OpenVAnchorBaseContractCalls::IsKnownRoot(var)
        }
    }
    impl ::std::convert::From<IsSpentCall> for OpenVAnchorBaseContractCalls {
        fn from(var: IsSpentCall) -> Self {
            OpenVAnchorBaseContractCalls::IsSpent(var)
        }
    }
    impl ::std::convert::From<IsSpentArrayCall> for OpenVAnchorBaseContractCalls {
        fn from(var: IsSpentArrayCall) -> Self {
            OpenVAnchorBaseContractCalls::IsSpentArray(var)
        }
    }
    impl ::std::convert::From<IsValidRootsCall> for OpenVAnchorBaseContractCalls {
        fn from(var: IsValidRootsCall) -> Self {
            OpenVAnchorBaseContractCalls::IsValidRoots(var)
        }
    }
    impl ::std::convert::From<LastBalanceCall> for OpenVAnchorBaseContractCalls {
        fn from(var: LastBalanceCall) -> Self {
            OpenVAnchorBaseContractCalls::LastBalance(var)
        }
    }
    impl ::std::convert::From<LevelsCall> for OpenVAnchorBaseContractCalls {
        fn from(var: LevelsCall) -> Self {
            OpenVAnchorBaseContractCalls::Levels(var)
        }
    }
    impl ::std::convert::From<MaximumDepositAmountCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: MaximumDepositAmountCall) -> Self {
            OpenVAnchorBaseContractCalls::MaximumDepositAmount(var)
        }
    }
    impl ::std::convert::From<MinimalWithdrawalAmountCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: MinimalWithdrawalAmountCall) -> Self {
            OpenVAnchorBaseContractCalls::MinimalWithdrawalAmount(var)
        }
    }
    impl ::std::convert::From<NeighborRootsCall> for OpenVAnchorBaseContractCalls {
        fn from(var: NeighborRootsCall) -> Self {
            OpenVAnchorBaseContractCalls::NeighborRoots(var)
        }
    }
    impl ::std::convert::From<NextIndexCall> for OpenVAnchorBaseContractCalls {
        fn from(var: NextIndexCall) -> Self {
            OpenVAnchorBaseContractCalls::NextIndex(var)
        }
    }
    impl ::std::convert::From<NullifierHashesCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: NullifierHashesCall) -> Self {
            OpenVAnchorBaseContractCalls::NullifierHashes(var)
        }
    }
    impl ::std::convert::From<ParseChainIdFromResourceIdCall>
        for OpenVAnchorBaseContractCalls
    {
        fn from(var: ParseChainIdFromResourceIdCall) -> Self {
            OpenVAnchorBaseContractCalls::ParseChainIdFromResourceId(var)
        }
    }
    impl ::std::convert::From<RootsCall> for OpenVAnchorBaseContractCalls {
        fn from(var: RootsCall) -> Self {
            OpenVAnchorBaseContractCalls::Roots(var)
        }
    }
    impl ::std::convert::From<SetHandlerCall> for OpenVAnchorBaseContractCalls {
        fn from(var: SetHandlerCall) -> Self {
            OpenVAnchorBaseContractCalls::SetHandler(var)
        }
    }
    impl ::std::convert::From<UpdateEdgeCall> for OpenVAnchorBaseContractCalls {
        fn from(var: UpdateEdgeCall) -> Self {
            OpenVAnchorBaseContractCalls::UpdateEdge(var)
        }
    }
    #[doc = "Container type for all return fields from the `EVM_CHAIN_ID_TYPE` function with signature `EVM_CHAIN_ID_TYPE()` and selector `[139, 126, 135, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct EvmChainIdTypeReturn(pub [u8; 2]);
    #[doc = "Container type for all return fields from the `FIELD_SIZE` function with signature `FIELD_SIZE()` and selector `[65, 74, 55, 186]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct FieldSizeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MAX_EXT_AMOUNT` function with signature `MAX_EXT_AMOUNT()` and selector `[127, 226, 79, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct MaxExtAmountReturn(pub I256);
    #[doc = "Container type for all return fields from the `MAX_FEE` function with signature `MAX_FEE()` and selector `[188, 6, 62, 26]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct MaxFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `ROOT_HISTORY_SIZE` function with signature `ROOT_HISTORY_SIZE()` and selector `[205, 135, 163, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct RootHistorySizeReturn(pub u32);
    #[doc = "Container type for all return fields from the `ZERO_VALUE` function with signature `ZERO_VALUE()` and selector `[236, 115, 41, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct ZeroValueReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `commitments` function with signature `commitments(bytes32)` and selector `[131, 157, 249, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct CommitmentsReturn(pub bool);
    #[doc = "Container type for all return fields from the `currentNeighborRootIndex` function with signature `currentNeighborRootIndex(uint256)` and selector `[93, 45, 118, 108]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct CurrentNeighborRootIndexReturn(pub u32);
    #[doc = "Container type for all return fields from the `currentRootIndex` function with signature `currentRootIndex()` and selector `[144, 238, 176, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct CurrentRootIndexReturn(pub u32);
    #[doc = "Container type for all return fields from the `edgeExistsForChain` function with signature `edgeExistsForChain(uint256)` and selector `[250, 115, 22, 135]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct EdgeExistsForChainReturn(pub bool);
    #[doc = "Container type for all return fields from the `edgeIndex` function with signature `edgeIndex(uint256)` and selector `[231, 14, 168, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct EdgeIndexReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `edgeList` function with signature `edgeList(uint256)` and selector `[219, 201, 22, 184]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct EdgeListReturn {
        pub chain_id: ethers::core::types::U256,
        pub root: [u8; 32],
        pub latest_leaf_index: ethers::core::types::U256,
        pub src_resource_id: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `filledSubtrees` function with signature `filledSubtrees(uint256)` and selector `[241, 120, 228, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct FilledSubtreesReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetChainIdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getChainIdType` function with signature `getChainIdType()` and selector `[76, 131, 12, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetChainIdTypeReturn(pub u64);
    #[doc = "Container type for all return fields from the `getLastRoot` function with signature `getLastRoot()` and selector `[186, 112, 247, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetLastRootReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getLatestNeighborEdges` function with signature `getLatestNeighborEdges()` and selector `[140, 13, 52, 216]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetLatestNeighborEdgesReturn(
        pub  ::std::vec::Vec<(
            ethers::core::types::U256,
            [u8; 32],
            ethers::core::types::U256,
            [u8; 32],
        )>,
    );
    #[doc = "Container type for all return fields from the `getLatestNeighborRoots` function with signature `getLatestNeighborRoots()` and selector `[30, 98, 118, 23]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetLatestNeighborRootsReturn(pub ::std::vec::Vec<[u8; 32]>);
    #[doc = "Container type for all return fields from the `getProposalNonce` function with signature `getProposalNonce()` and selector `[11, 39, 251, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct GetProposalNonceReturn(pub u32);
    #[doc = "Container type for all return fields from the `handler` function with signature `handler()` and selector `[200, 9, 22, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct HandlerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `hasEdge` function with signature `hasEdge(uint256)` and selector `[146, 21, 99, 17]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct HasEdgeReturn(pub bool);
    #[doc = "Container type for all return fields from the `hashLeftRight` function with signature `hashLeftRight(address,bytes32,bytes32)` and selector `[142, 163, 9, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct HashLeftRightReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `hasher` function with signature `hasher()` and selector `[237, 51, 99, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct HasherReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isKnownNeighborRoot` function with signature `isKnownNeighborRoot(uint256,bytes32)` and selector `[17, 228, 220, 185]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct IsKnownNeighborRootReturn(pub bool);
    #[doc = "Container type for all return fields from the `isKnownRoot` function with signature `isKnownRoot(bytes32)` and selector `[109, 152, 51, 227]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct IsKnownRootReturn(pub bool);
    #[doc = "Container type for all return fields from the `isSpent` function with signature `isSpent(bytes32)` and selector `[229, 40, 93, 204]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct IsSpentReturn(pub bool);
    #[doc = "Container type for all return fields from the `isSpentArray` function with signature `isSpentArray(bytes32[])` and selector `[159, 161, 45, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct IsSpentArrayReturn(pub ::std::vec::Vec<bool>);
    #[doc = "Container type for all return fields from the `isValidRoots` function with signature `isValidRoots(bytes32[])` and selector `[97, 110, 9, 87]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct IsValidRootsReturn(pub bool);
    #[doc = "Container type for all return fields from the `lastBalance` function with signature `lastBalance()` and selector `[143, 28, 86, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct LastBalanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `levels` function with signature `levels()` and selector `[78, 207, 81, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct LevelsReturn(pub u32);
    #[doc = "Container type for all return fields from the `maximumDepositAmount` function with signature `maximumDepositAmount()` and selector `[120, 171, 180, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct MaximumDepositAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `minimalWithdrawalAmount` function with signature `minimalWithdrawalAmount()` and selector `[132, 11, 39, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct MinimalWithdrawalAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `neighborRoots` function with signature `neighborRoots(uint256,uint32)` and selector `[67, 231, 17, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct NeighborRootsReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `nextIndex` function with signature `nextIndex()` and selector `[252, 126, 156, 111]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct NextIndexReturn(pub u32);
    #[doc = "Container type for all return fields from the `nullifierHashes` function with signature `nullifierHashes(bytes32)` and selector `[23, 204, 145, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct NullifierHashesReturn(pub bool);
    #[doc = "Container type for all return fields from the `parseChainIdFromResourceId` function with signature `parseChainIdFromResourceId(bytes32)` and selector `[194, 35, 13, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct ParseChainIdFromResourceIdReturn(pub u64);
    #[doc = "Container type for all return fields from the `roots` function with signature `roots(uint256)` and selector `[194, 180, 10, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct RootsReturn {
        pub root: [u8; 32],
        pub latest_leafindex: ethers::core::types::U256,
    }
    #[doc = "`Edge(uint256,bytes32,uint256,bytes32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct Edge {
        pub chain_id: ethers::core::types::U256,
        pub root: [u8; 32],
        pub latest_leaf_index: ethers::core::types::U256,
        pub src_resource_id: [u8; 32],
    }
}
