pub use open_v_anchor_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod open_v_anchor_contract {
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
    #[doc = "OpenVAnchorContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static OPENVANCHORCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"_levels\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"contract IHasher\",\"name\":\"_hasher\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EdgeAddition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EdgeUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint32\",\"name\":\"leafIndex\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Insertion\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCommitment\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"nullifier\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewNullifier\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"key\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PublicKey\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EVM_CHAIN_ID_TYPE\",\"outputs\":[{\"internalType\":\"bytes2\",\"name\":\"\",\"type\":\"bytes2\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FIELD_SIZE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_EXT_AMOUNT\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_FEE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ROOT_HISTORY_SIZE\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ZERO_VALUE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"depositAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"_executeWrapping\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureMaximumDepositLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureMinimalWithdrawalLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentNeighborRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint48\",\"name\":\"destinationChainId\",\"type\":\"uint48\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"depositAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"delegatedCalldata\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blinding\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeExistsForChain\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeList\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"srcResourceID\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledSubtrees\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainIdType\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestNeighborEdges\",\"outputs\":[{\"internalType\":\"struct OpenLinkableAnchor.Edge[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"srcResourceID\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestNeighborRoots\",\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProposalNonce\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"handler\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_chainID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasEdge\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IHasher\",\"name\":\"_hasher\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_left\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_right\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hashLeftRight\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasher\",\"outputs\":[{\"internalType\":\"contract IHasher\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_neighborChainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isKnownNeighborRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isKnownRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_nullifierHashes\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpentArray\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"\",\"type\":\"bool[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_roots\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidRoots\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"levels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maximumDepositAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minimalWithdrawalAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"neighborRoots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nextIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nullifierHashes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_resourceId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseChainIdFromResourceId\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafindex\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setHandler\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrapIntoNative\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrapIntoToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_leafIndex\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_srcResourceID\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"updateEdge\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"withdrawAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"delegatedCalldata\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blinding\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"merkleProof\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"commitmentIndex\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"withdrawAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"delegatedCalldata\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blinding\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"merkleProof\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"commitmentIndex\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"withdrawAndUnwrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint48\",\"name\":\"destinationChainId\",\"type\":\"uint48\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"depositAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"delegatedCalldata\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blinding\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrapAndDeposit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrapNative\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"wrapToken\",\"outputs\":[]}]") . expect ("invalid abi")
    });
    pub struct OpenVAnchorContract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for OpenVAnchorContract<M> {
        fn clone(&self) -> Self {
            OpenVAnchorContract(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for OpenVAnchorContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for OpenVAnchorContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(OpenVAnchorContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> OpenVAnchorContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                OPENVANCHORCONTRACT_ABI.clone(),
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
        #[doc = "Calls the contract's `_executeWrapping` (0x94eee060) function"]
        pub fn execute_wrapping(
            &self,
            token_address: ethers::core::types::Address,
            deposit_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [148, 238, 224, 96],
                    (token_address, deposit_amount),
                )
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
        #[doc = "Calls the contract's `deposit` (0xe918fdaf) function"]
        pub fn deposit(
            &self,
            destination_chain_id: u64,
            deposit_amount: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            delegated_calldata: ethers::core::types::Bytes,
            blinding: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [233, 24, 253, 175],
                    (
                        destination_chain_id,
                        deposit_amount,
                        recipient,
                        delegated_calldata,
                        blinding,
                    ),
                )
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
        #[doc = "Calls the contract's `withdraw` (0xc35230c5) function"]
        pub fn withdraw(
            &self,
            withdraw_amount: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            delegated_calldata: ethers::core::types::Bytes,
            blinding: ethers::core::types::U256,
            merkle_proof: ::std::vec::Vec<[u8; 32]>,
            commitment_index: u32,
            root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [195, 82, 48, 197],
                    (
                        withdraw_amount,
                        recipient,
                        delegated_calldata,
                        blinding,
                        merkle_proof,
                        commitment_index,
                        root,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAndUnwrap` (0x54c41a1b) function"]
        pub fn withdraw_and_unwrap(
            &self,
            withdraw_amount: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            delegated_calldata: ethers::core::types::Bytes,
            blinding: ethers::core::types::U256,
            merkle_proof: ::std::vec::Vec<[u8; 32]>,
            commitment_index: u32,
            root: [u8; 32],
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [84, 196, 26, 27],
                    (
                        withdraw_amount,
                        recipient,
                        delegated_calldata,
                        blinding,
                        merkle_proof,
                        commitment_index,
                        root,
                        token_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapAndDeposit` (0x30e415a3) function"]
        pub fn wrap_and_deposit(
            &self,
            destination_chain_id: u64,
            deposit_amount: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            delegated_calldata: ethers::core::types::Bytes,
            blinding: ethers::core::types::U256,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [48, 228, 21, 163],
                    (
                        destination_chain_id,
                        deposit_amount,
                        recipient,
                        delegated_calldata,
                        blinding,
                        token_address,
                    ),
                )
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
        ) -> ethers::contract::builders::Event<M, OpenVAnchorContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for OpenVAnchorContract<M>
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
    pub enum OpenVAnchorContractEvents {
        EdgeAdditionFilter(EdgeAdditionFilter),
        EdgeUpdateFilter(EdgeUpdateFilter),
        InitializedFilter(InitializedFilter),
        InsertionFilter(InsertionFilter),
        NewCommitmentFilter(NewCommitmentFilter),
        NewNullifierFilter(NewNullifierFilter),
        PublicKeyFilter(PublicKeyFilter),
    }
    impl ethers::contract::EthLogDecode for OpenVAnchorContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = EdgeAdditionFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::EdgeAdditionFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EdgeUpdateFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::EdgeUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::InitializedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InsertionFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::InsertionFilter(decoded));
            }
            if let Ok(decoded) = NewCommitmentFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::NewCommitmentFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewNullifierFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::NewNullifierFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PublicKeyFilter::decode_log(log) {
                return Ok(OpenVAnchorContractEvents::PublicKeyFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for OpenVAnchorContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OpenVAnchorContractEvents::EdgeAdditionFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractEvents::EdgeUpdateFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractEvents::InitializedFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractEvents::InsertionFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractEvents::NewCommitmentFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractEvents::NewNullifierFilter(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractEvents::PublicKeyFilter(element) => {
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
    #[doc = "Container type for all input parameters for the `_executeWrapping` function with signature `_executeWrapping(address,uint256)` and selector `[148, 238, 224, 96]`"]
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
        name = "_executeWrapping",
        abi = "_executeWrapping(address,uint256)"
    )]
    pub struct ExecuteWrappingCall {
        pub token_address: ethers::core::types::Address,
        pub deposit_amount: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `deposit` function with signature `deposit(uint48,uint256,address,bytes,uint256)` and selector `[233, 24, 253, 175]`"]
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
        name = "deposit",
        abi = "deposit(uint48,uint256,address,bytes,uint256)"
    )]
    pub struct DepositCall {
        pub destination_chain_id: u64,
        pub deposit_amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub delegated_calldata: ethers::core::types::Bytes,
        pub blinding: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `token` function with signature `token()` and selector `[252, 12, 84, 106]`"]
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
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    #[doc = "Container type for all input parameters for the `unwrapIntoNative` function with signature `unwrapIntoNative(address,uint256)` and selector `[159, 248, 0, 99]`"]
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
        name = "unwrapIntoNative",
        abi = "unwrapIntoNative(address,uint256)"
    )]
    pub struct UnwrapIntoNativeCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unwrapIntoToken` function with signature `unwrapIntoToken(address,uint256)` and selector `[79, 64, 18, 65]`"]
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
        name = "unwrapIntoToken",
        abi = "unwrapIntoToken(address,uint256)"
    )]
    pub struct UnwrapIntoTokenCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,address,bytes,uint256,bytes32[],uint32,bytes32)` and selector `[195, 82, 48, 197]`"]
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
        name = "withdraw",
        abi = "withdraw(uint256,address,bytes,uint256,bytes32[],uint32,bytes32)"
    )]
    pub struct WithdrawCall {
        pub withdraw_amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub delegated_calldata: ethers::core::types::Bytes,
        pub blinding: ethers::core::types::U256,
        pub merkle_proof: ::std::vec::Vec<[u8; 32]>,
        pub commitment_index: u32,
        pub root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `withdrawAndUnwrap` function with signature `withdrawAndUnwrap(uint256,address,bytes,uint256,bytes32[],uint32,bytes32,address)` and selector `[84, 196, 26, 27]`"]
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
        name = "withdrawAndUnwrap",
        abi = "withdrawAndUnwrap(uint256,address,bytes,uint256,bytes32[],uint32,bytes32,address)"
    )]
    pub struct WithdrawAndUnwrapCall {
        pub withdraw_amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub delegated_calldata: ethers::core::types::Bytes,
        pub blinding: ethers::core::types::U256,
        pub merkle_proof: ::std::vec::Vec<[u8; 32]>,
        pub commitment_index: u32,
        pub root: [u8; 32],
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `wrapAndDeposit` function with signature `wrapAndDeposit(uint48,uint256,address,bytes,uint256,address)` and selector `[48, 228, 21, 163]`"]
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
        name = "wrapAndDeposit",
        abi = "wrapAndDeposit(uint48,uint256,address,bytes,uint256,address)"
    )]
    pub struct WrapAndDepositCall {
        pub destination_chain_id: u64,
        pub deposit_amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub delegated_calldata: ethers::core::types::Bytes,
        pub blinding: ethers::core::types::U256,
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `wrapNative` function with signature `wrapNative()` and selector `[106, 212, 129, 243]`"]
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
    #[ethcall(name = "wrapNative", abi = "wrapNative()")]
    pub struct WrapNativeCall;
    #[doc = "Container type for all input parameters for the `wrapToken` function with signature `wrapToken(address,uint256)` and selector `[70, 11, 83, 227]`"]
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
    #[ethcall(name = "wrapToken", abi = "wrapToken(address,uint256)")]
    pub struct WrapTokenCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    pub enum OpenVAnchorContractCalls {
        EvmChainIdType(EvmChainIdTypeCall),
        FieldSize(FieldSizeCall),
        MaxExtAmount(MaxExtAmountCall),
        MaxFee(MaxFeeCall),
        RootHistorySize(RootHistorySizeCall),
        ZeroValue(ZeroValueCall),
        ExecuteWrapping(ExecuteWrappingCall),
        Commitments(CommitmentsCall),
        ConfigureMaximumDepositLimit(ConfigureMaximumDepositLimitCall),
        ConfigureMinimalWithdrawalLimit(ConfigureMinimalWithdrawalLimitCall),
        CurrentNeighborRootIndex(CurrentNeighborRootIndexCall),
        CurrentRootIndex(CurrentRootIndexCall),
        Deposit(DepositCall),
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
        Token(TokenCall),
        UnwrapIntoNative(UnwrapIntoNativeCall),
        UnwrapIntoToken(UnwrapIntoTokenCall),
        UpdateEdge(UpdateEdgeCall),
        Withdraw(WithdrawCall),
        WithdrawAndUnwrap(WithdrawAndUnwrapCall),
        WrapAndDeposit(WrapAndDepositCall),
        WrapNative(WrapNativeCall),
        WrapToken(WrapTokenCall),
    }
    impl ethers::core::abi::AbiDecode for OpenVAnchorContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <EvmChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::EvmChainIdType(decoded));
            }
            if let Ok(decoded) =
                <FieldSizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::FieldSize(decoded));
            }
            if let Ok(decoded) =
                <MaxExtAmountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::MaxExtAmount(decoded));
            }
            if let Ok(decoded) =
                <MaxFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::MaxFee(decoded));
            }
            if let Ok(decoded) =
                <RootHistorySizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::RootHistorySize(decoded));
            }
            if let Ok(decoded) =
                <ZeroValueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::ZeroValue(decoded));
            }
            if let Ok(decoded) =
                <ExecuteWrappingCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::ExecuteWrapping(decoded));
            }
            if let Ok(decoded) =
                <CommitmentsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Commitments(decoded));
            }
            if let Ok (decoded) = < ConfigureMaximumDepositLimitCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: ConfigureMaximumDepositLimit (decoded)) }
            if let Ok (decoded) = < ConfigureMinimalWithdrawalLimitCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: ConfigureMinimalWithdrawalLimit (decoded)) }
            if let Ok (decoded) = < CurrentNeighborRootIndexCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: CurrentNeighborRootIndex (decoded)) }
            if let Ok(decoded) =
                <CurrentRootIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::CurrentRootIndex(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <EdgeExistsForChainCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::EdgeExistsForChain(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <EdgeIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::EdgeIndex(decoded));
            }
            if let Ok(decoded) =
                <EdgeListCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::EdgeList(decoded));
            }
            if let Ok(decoded) =
                <FilledSubtreesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::FilledSubtrees(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetChainIdType(decoded));
            }
            if let Ok(decoded) =
                <GetLastRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetLastRoot(decoded));
            }
            if let Ok (decoded) = < GetLatestNeighborEdgesCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: GetLatestNeighborEdges (decoded)) }
            if let Ok (decoded) = < GetLatestNeighborRootsCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: GetLatestNeighborRoots (decoded)) }
            if let Ok(decoded) =
                <GetProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::GetProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <HandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::HasEdge(decoded));
            }
            if let Ok(decoded) =
                <HashLeftRightCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::HashLeftRight(decoded));
            }
            if let Ok(decoded) =
                <HasherCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Hasher(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Initialize(decoded));
            }
            if let Ok (decoded) = < IsKnownNeighborRootCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: IsKnownNeighborRoot (decoded)) }
            if let Ok(decoded) =
                <IsKnownRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::IsKnownRoot(decoded));
            }
            if let Ok(decoded) =
                <IsSpentCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::IsSpent(decoded));
            }
            if let Ok(decoded) =
                <IsSpentArrayCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::IsSpentArray(decoded));
            }
            if let Ok(decoded) =
                <IsValidRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::IsValidRoots(decoded));
            }
            if let Ok(decoded) =
                <LastBalanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::LastBalance(decoded));
            }
            if let Ok(decoded) =
                <LevelsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Levels(decoded));
            }
            if let Ok (decoded) = < MaximumDepositAmountCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: MaximumDepositAmount (decoded)) }
            if let Ok (decoded) = < MinimalWithdrawalAmountCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: MinimalWithdrawalAmount (decoded)) }
            if let Ok(decoded) =
                <NeighborRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::NeighborRoots(decoded));
            }
            if let Ok(decoded) =
                <NextIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::NextIndex(decoded));
            }
            if let Ok(decoded) =
                <NullifierHashesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::NullifierHashes(decoded));
            }
            if let Ok (decoded) = < ParseChainIdFromResourceIdCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (OpenVAnchorContractCalls :: ParseChainIdFromResourceId (decoded)) }
            if let Ok(decoded) =
                <RootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Roots(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::SetHandler(decoded));
            }
            if let Ok(decoded) =
                <TokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Token(decoded));
            }
            if let Ok(decoded) =
                <UnwrapIntoNativeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::UnwrapIntoNative(decoded));
            }
            if let Ok(decoded) =
                <UnwrapIntoTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::UnwrapIntoToken(decoded));
            }
            if let Ok(decoded) =
                <UpdateEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::UpdateEdge(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAndUnwrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::WithdrawAndUnwrap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WrapAndDepositCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::WrapAndDeposit(decoded));
            }
            if let Ok(decoded) =
                <WrapNativeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::WrapNative(decoded));
            }
            if let Ok(decoded) =
                <WrapTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(OpenVAnchorContractCalls::WrapToken(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for OpenVAnchorContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                OpenVAnchorContractCalls::EvmChainIdType(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::FieldSize(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::MaxExtAmount(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::MaxFee(element) => element.encode(),
                OpenVAnchorContractCalls::RootHistorySize(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::ZeroValue(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::ExecuteWrapping(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Commitments(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::ConfigureMaximumDepositLimit(
                    element,
                ) => element.encode(),
                OpenVAnchorContractCalls::ConfigureMinimalWithdrawalLimit(
                    element,
                ) => element.encode(),
                OpenVAnchorContractCalls::CurrentNeighborRootIndex(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::CurrentRootIndex(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Deposit(element) => element.encode(),
                OpenVAnchorContractCalls::EdgeExistsForChain(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::EdgeIndex(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::EdgeList(element) => element.encode(),
                OpenVAnchorContractCalls::FilledSubtrees(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetChainId(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetChainIdType(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetLastRoot(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetLatestNeighborEdges(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetLatestNeighborRoots(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::GetProposalNonce(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Handler(element) => element.encode(),
                OpenVAnchorContractCalls::HasEdge(element) => element.encode(),
                OpenVAnchorContractCalls::HashLeftRight(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Hasher(element) => element.encode(),
                OpenVAnchorContractCalls::Initialize(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::IsKnownNeighborRoot(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::IsKnownRoot(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::IsSpent(element) => element.encode(),
                OpenVAnchorContractCalls::IsSpentArray(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::IsValidRoots(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::LastBalance(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Levels(element) => element.encode(),
                OpenVAnchorContractCalls::MaximumDepositAmount(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::MinimalWithdrawalAmount(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::NeighborRoots(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::NextIndex(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::NullifierHashes(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::ParseChainIdFromResourceId(
                    element,
                ) => element.encode(),
                OpenVAnchorContractCalls::Roots(element) => element.encode(),
                OpenVAnchorContractCalls::SetHandler(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Token(element) => element.encode(),
                OpenVAnchorContractCalls::UnwrapIntoNative(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::UnwrapIntoToken(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::UpdateEdge(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::Withdraw(element) => element.encode(),
                OpenVAnchorContractCalls::WithdrawAndUnwrap(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::WrapAndDeposit(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::WrapNative(element) => {
                    element.encode()
                }
                OpenVAnchorContractCalls::WrapToken(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for OpenVAnchorContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                OpenVAnchorContractCalls::EvmChainIdType(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::FieldSize(element) => element.fmt(f),
                OpenVAnchorContractCalls::MaxExtAmount(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::MaxFee(element) => element.fmt(f),
                OpenVAnchorContractCalls::RootHistorySize(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::ZeroValue(element) => element.fmt(f),
                OpenVAnchorContractCalls::ExecuteWrapping(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::Commitments(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::ConfigureMaximumDepositLimit(
                    element,
                ) => element.fmt(f),
                OpenVAnchorContractCalls::ConfigureMinimalWithdrawalLimit(
                    element,
                ) => element.fmt(f),
                OpenVAnchorContractCalls::CurrentNeighborRootIndex(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::CurrentRootIndex(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::Deposit(element) => element.fmt(f),
                OpenVAnchorContractCalls::EdgeExistsForChain(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::EdgeIndex(element) => element.fmt(f),
                OpenVAnchorContractCalls::EdgeList(element) => element.fmt(f),
                OpenVAnchorContractCalls::FilledSubtrees(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetChainId(element) => element.fmt(f),
                OpenVAnchorContractCalls::GetChainIdType(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetLastRoot(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetLatestNeighborEdges(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetLatestNeighborRoots(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::GetProposalNonce(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::Handler(element) => element.fmt(f),
                OpenVAnchorContractCalls::HasEdge(element) => element.fmt(f),
                OpenVAnchorContractCalls::HashLeftRight(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::Hasher(element) => element.fmt(f),
                OpenVAnchorContractCalls::Initialize(element) => element.fmt(f),
                OpenVAnchorContractCalls::IsKnownNeighborRoot(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::IsKnownRoot(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::IsSpent(element) => element.fmt(f),
                OpenVAnchorContractCalls::IsSpentArray(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::IsValidRoots(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::LastBalance(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::Levels(element) => element.fmt(f),
                OpenVAnchorContractCalls::MaximumDepositAmount(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::MinimalWithdrawalAmount(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::NeighborRoots(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::NextIndex(element) => element.fmt(f),
                OpenVAnchorContractCalls::NullifierHashes(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::ParseChainIdFromResourceId(
                    element,
                ) => element.fmt(f),
                OpenVAnchorContractCalls::Roots(element) => element.fmt(f),
                OpenVAnchorContractCalls::SetHandler(element) => element.fmt(f),
                OpenVAnchorContractCalls::Token(element) => element.fmt(f),
                OpenVAnchorContractCalls::UnwrapIntoNative(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::UnwrapIntoToken(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::UpdateEdge(element) => element.fmt(f),
                OpenVAnchorContractCalls::Withdraw(element) => element.fmt(f),
                OpenVAnchorContractCalls::WithdrawAndUnwrap(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::WrapAndDeposit(element) => {
                    element.fmt(f)
                }
                OpenVAnchorContractCalls::WrapNative(element) => element.fmt(f),
                OpenVAnchorContractCalls::WrapToken(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<EvmChainIdTypeCall> for OpenVAnchorContractCalls {
        fn from(var: EvmChainIdTypeCall) -> Self {
            OpenVAnchorContractCalls::EvmChainIdType(var)
        }
    }
    impl ::std::convert::From<FieldSizeCall> for OpenVAnchorContractCalls {
        fn from(var: FieldSizeCall) -> Self {
            OpenVAnchorContractCalls::FieldSize(var)
        }
    }
    impl ::std::convert::From<MaxExtAmountCall> for OpenVAnchorContractCalls {
        fn from(var: MaxExtAmountCall) -> Self {
            OpenVAnchorContractCalls::MaxExtAmount(var)
        }
    }
    impl ::std::convert::From<MaxFeeCall> for OpenVAnchorContractCalls {
        fn from(var: MaxFeeCall) -> Self {
            OpenVAnchorContractCalls::MaxFee(var)
        }
    }
    impl ::std::convert::From<RootHistorySizeCall> for OpenVAnchorContractCalls {
        fn from(var: RootHistorySizeCall) -> Self {
            OpenVAnchorContractCalls::RootHistorySize(var)
        }
    }
    impl ::std::convert::From<ZeroValueCall> for OpenVAnchorContractCalls {
        fn from(var: ZeroValueCall) -> Self {
            OpenVAnchorContractCalls::ZeroValue(var)
        }
    }
    impl ::std::convert::From<ExecuteWrappingCall> for OpenVAnchorContractCalls {
        fn from(var: ExecuteWrappingCall) -> Self {
            OpenVAnchorContractCalls::ExecuteWrapping(var)
        }
    }
    impl ::std::convert::From<CommitmentsCall> for OpenVAnchorContractCalls {
        fn from(var: CommitmentsCall) -> Self {
            OpenVAnchorContractCalls::Commitments(var)
        }
    }
    impl ::std::convert::From<ConfigureMaximumDepositLimitCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: ConfigureMaximumDepositLimitCall) -> Self {
            OpenVAnchorContractCalls::ConfigureMaximumDepositLimit(var)
        }
    }
    impl ::std::convert::From<ConfigureMinimalWithdrawalLimitCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: ConfigureMinimalWithdrawalLimitCall) -> Self {
            OpenVAnchorContractCalls::ConfigureMinimalWithdrawalLimit(var)
        }
    }
    impl ::std::convert::From<CurrentNeighborRootIndexCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: CurrentNeighborRootIndexCall) -> Self {
            OpenVAnchorContractCalls::CurrentNeighborRootIndex(var)
        }
    }
    impl ::std::convert::From<CurrentRootIndexCall> for OpenVAnchorContractCalls {
        fn from(var: CurrentRootIndexCall) -> Self {
            OpenVAnchorContractCalls::CurrentRootIndex(var)
        }
    }
    impl ::std::convert::From<DepositCall> for OpenVAnchorContractCalls {
        fn from(var: DepositCall) -> Self {
            OpenVAnchorContractCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<EdgeExistsForChainCall> for OpenVAnchorContractCalls {
        fn from(var: EdgeExistsForChainCall) -> Self {
            OpenVAnchorContractCalls::EdgeExistsForChain(var)
        }
    }
    impl ::std::convert::From<EdgeIndexCall> for OpenVAnchorContractCalls {
        fn from(var: EdgeIndexCall) -> Self {
            OpenVAnchorContractCalls::EdgeIndex(var)
        }
    }
    impl ::std::convert::From<EdgeListCall> for OpenVAnchorContractCalls {
        fn from(var: EdgeListCall) -> Self {
            OpenVAnchorContractCalls::EdgeList(var)
        }
    }
    impl ::std::convert::From<FilledSubtreesCall> for OpenVAnchorContractCalls {
        fn from(var: FilledSubtreesCall) -> Self {
            OpenVAnchorContractCalls::FilledSubtrees(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for OpenVAnchorContractCalls {
        fn from(var: GetChainIdCall) -> Self {
            OpenVAnchorContractCalls::GetChainId(var)
        }
    }
    impl ::std::convert::From<GetChainIdTypeCall> for OpenVAnchorContractCalls {
        fn from(var: GetChainIdTypeCall) -> Self {
            OpenVAnchorContractCalls::GetChainIdType(var)
        }
    }
    impl ::std::convert::From<GetLastRootCall> for OpenVAnchorContractCalls {
        fn from(var: GetLastRootCall) -> Self {
            OpenVAnchorContractCalls::GetLastRoot(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborEdgesCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: GetLatestNeighborEdgesCall) -> Self {
            OpenVAnchorContractCalls::GetLatestNeighborEdges(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborRootsCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: GetLatestNeighborRootsCall) -> Self {
            OpenVAnchorContractCalls::GetLatestNeighborRoots(var)
        }
    }
    impl ::std::convert::From<GetProposalNonceCall> for OpenVAnchorContractCalls {
        fn from(var: GetProposalNonceCall) -> Self {
            OpenVAnchorContractCalls::GetProposalNonce(var)
        }
    }
    impl ::std::convert::From<HandlerCall> for OpenVAnchorContractCalls {
        fn from(var: HandlerCall) -> Self {
            OpenVAnchorContractCalls::Handler(var)
        }
    }
    impl ::std::convert::From<HasEdgeCall> for OpenVAnchorContractCalls {
        fn from(var: HasEdgeCall) -> Self {
            OpenVAnchorContractCalls::HasEdge(var)
        }
    }
    impl ::std::convert::From<HashLeftRightCall> for OpenVAnchorContractCalls {
        fn from(var: HashLeftRightCall) -> Self {
            OpenVAnchorContractCalls::HashLeftRight(var)
        }
    }
    impl ::std::convert::From<HasherCall> for OpenVAnchorContractCalls {
        fn from(var: HasherCall) -> Self {
            OpenVAnchorContractCalls::Hasher(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for OpenVAnchorContractCalls {
        fn from(var: InitializeCall) -> Self {
            OpenVAnchorContractCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsKnownNeighborRootCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: IsKnownNeighborRootCall) -> Self {
            OpenVAnchorContractCalls::IsKnownNeighborRoot(var)
        }
    }
    impl ::std::convert::From<IsKnownRootCall> for OpenVAnchorContractCalls {
        fn from(var: IsKnownRootCall) -> Self {
            OpenVAnchorContractCalls::IsKnownRoot(var)
        }
    }
    impl ::std::convert::From<IsSpentCall> for OpenVAnchorContractCalls {
        fn from(var: IsSpentCall) -> Self {
            OpenVAnchorContractCalls::IsSpent(var)
        }
    }
    impl ::std::convert::From<IsSpentArrayCall> for OpenVAnchorContractCalls {
        fn from(var: IsSpentArrayCall) -> Self {
            OpenVAnchorContractCalls::IsSpentArray(var)
        }
    }
    impl ::std::convert::From<IsValidRootsCall> for OpenVAnchorContractCalls {
        fn from(var: IsValidRootsCall) -> Self {
            OpenVAnchorContractCalls::IsValidRoots(var)
        }
    }
    impl ::std::convert::From<LastBalanceCall> for OpenVAnchorContractCalls {
        fn from(var: LastBalanceCall) -> Self {
            OpenVAnchorContractCalls::LastBalance(var)
        }
    }
    impl ::std::convert::From<LevelsCall> for OpenVAnchorContractCalls {
        fn from(var: LevelsCall) -> Self {
            OpenVAnchorContractCalls::Levels(var)
        }
    }
    impl ::std::convert::From<MaximumDepositAmountCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: MaximumDepositAmountCall) -> Self {
            OpenVAnchorContractCalls::MaximumDepositAmount(var)
        }
    }
    impl ::std::convert::From<MinimalWithdrawalAmountCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: MinimalWithdrawalAmountCall) -> Self {
            OpenVAnchorContractCalls::MinimalWithdrawalAmount(var)
        }
    }
    impl ::std::convert::From<NeighborRootsCall> for OpenVAnchorContractCalls {
        fn from(var: NeighborRootsCall) -> Self {
            OpenVAnchorContractCalls::NeighborRoots(var)
        }
    }
    impl ::std::convert::From<NextIndexCall> for OpenVAnchorContractCalls {
        fn from(var: NextIndexCall) -> Self {
            OpenVAnchorContractCalls::NextIndex(var)
        }
    }
    impl ::std::convert::From<NullifierHashesCall> for OpenVAnchorContractCalls {
        fn from(var: NullifierHashesCall) -> Self {
            OpenVAnchorContractCalls::NullifierHashes(var)
        }
    }
    impl ::std::convert::From<ParseChainIdFromResourceIdCall>
        for OpenVAnchorContractCalls
    {
        fn from(var: ParseChainIdFromResourceIdCall) -> Self {
            OpenVAnchorContractCalls::ParseChainIdFromResourceId(var)
        }
    }
    impl ::std::convert::From<RootsCall> for OpenVAnchorContractCalls {
        fn from(var: RootsCall) -> Self {
            OpenVAnchorContractCalls::Roots(var)
        }
    }
    impl ::std::convert::From<SetHandlerCall> for OpenVAnchorContractCalls {
        fn from(var: SetHandlerCall) -> Self {
            OpenVAnchorContractCalls::SetHandler(var)
        }
    }
    impl ::std::convert::From<TokenCall> for OpenVAnchorContractCalls {
        fn from(var: TokenCall) -> Self {
            OpenVAnchorContractCalls::Token(var)
        }
    }
    impl ::std::convert::From<UnwrapIntoNativeCall> for OpenVAnchorContractCalls {
        fn from(var: UnwrapIntoNativeCall) -> Self {
            OpenVAnchorContractCalls::UnwrapIntoNative(var)
        }
    }
    impl ::std::convert::From<UnwrapIntoTokenCall> for OpenVAnchorContractCalls {
        fn from(var: UnwrapIntoTokenCall) -> Self {
            OpenVAnchorContractCalls::UnwrapIntoToken(var)
        }
    }
    impl ::std::convert::From<UpdateEdgeCall> for OpenVAnchorContractCalls {
        fn from(var: UpdateEdgeCall) -> Self {
            OpenVAnchorContractCalls::UpdateEdge(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for OpenVAnchorContractCalls {
        fn from(var: WithdrawCall) -> Self {
            OpenVAnchorContractCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawAndUnwrapCall> for OpenVAnchorContractCalls {
        fn from(var: WithdrawAndUnwrapCall) -> Self {
            OpenVAnchorContractCalls::WithdrawAndUnwrap(var)
        }
    }
    impl ::std::convert::From<WrapAndDepositCall> for OpenVAnchorContractCalls {
        fn from(var: WrapAndDepositCall) -> Self {
            OpenVAnchorContractCalls::WrapAndDeposit(var)
        }
    }
    impl ::std::convert::From<WrapNativeCall> for OpenVAnchorContractCalls {
        fn from(var: WrapNativeCall) -> Self {
            OpenVAnchorContractCalls::WrapNative(var)
        }
    }
    impl ::std::convert::From<WrapTokenCall> for OpenVAnchorContractCalls {
        fn from(var: WrapTokenCall) -> Self {
            OpenVAnchorContractCalls::WrapToken(var)
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
    #[doc = "Container type for all return fields from the `token` function with signature `token()` and selector `[252, 12, 84, 106]`"]
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
    pub struct TokenReturn(pub ethers::core::types::Address);
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
