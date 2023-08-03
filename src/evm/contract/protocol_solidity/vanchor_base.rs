pub use v_anchor_base_contract::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod v_anchor_base_contract {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"merkleRoot\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EdgeAddition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"merkleRoot\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EdgeUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint32\",\"name\":\"leafIndex\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newMerkleRoot\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Insertion\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"maximumDepositAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MaxDepositLimitUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"minimalWithdrawalAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MinWithdrawalLimitUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"subTreeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"leafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCommitment\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nullifier\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewNullifier\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"key\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PublicKey\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"handler\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetHandler\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EVM_CHAIN_ID_TYPE\",\"outputs\":[{\"internalType\":\"bytes2\",\"name\":\"\",\"type\":\"bytes2\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FIELD_SIZE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_EXT_AMOUNT\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_FEE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ROOT_HISTORY_SIZE\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ZERO_VALUE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_fromTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_toTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_extAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"_executeWrapping\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_fromTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_toTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_minusExtAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"_withdrawAndUnwrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"_extAmount\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_fee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"calculatePublicAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureMaximumDepositLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureMinimalWithdrawalLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentNeighborRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeExistsForChain\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeList\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"srcResourceID\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledSubtrees\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainIdType\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getHasher\",\"outputs\":[{\"internalType\":\"contract IHasher\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastRoot\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestNeighborEdges\",\"outputs\":[{\"internalType\":\"struct Edge[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"srcResourceID\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestNeighborRoots\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLevels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNextIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getZeroHash\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"handler\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_chainID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasEdge\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_left\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_right\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hashLeftRight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"initialized\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCorrectExecutionChain\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"resourceId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isCorrectExecutionContext\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_neighborChainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_root\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isKnownNeighborRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isKnownRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_nullifierHash\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_nullifierHashes\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpentArray\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"\",\"type\":\"bool[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_roots\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidRoots\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxEdges\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maximumDepositAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minimalWithdrawalAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"neighborRoots\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nullifierHashes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"outerLevels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_resourceId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseChainIdFromResourceId\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct VAnchorBase.Account\",\"name\":\"_account\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"keyData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"register\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roots\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"latestLeafindex\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setHandler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_leafIndex\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_srcResourceID\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"updateEdge\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static VANCHORBASECONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct VAnchorBaseContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for VAnchorBaseContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for VAnchorBaseContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for VAnchorBaseContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for VAnchorBaseContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(VAnchorBaseContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> VAnchorBaseContract<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                VANCHORBASECONTRACT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `EVM_CHAIN_ID_TYPE` (0x8b7e8782) function
        pub fn evm_chain_id_type(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 2]> {
            self.0
                .method_hash([139, 126, 135, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FIELD_SIZE` (0x414a37ba) function
        pub fn field_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([65, 74, 55, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_EXT_AMOUNT` (0x7fe24ffe) function
        pub fn max_ext_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::I256,
        > {
            self.0
                .method_hash([127, 226, 79, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_FEE` (0xbc063e1a) function
        pub fn max_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([188, 6, 62, 26], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ROOT_HISTORY_SIZE` (0xcd87a3b4) function
        pub fn root_history_size(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([205, 135, 163, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ZERO_VALUE` (0xec732959) function
        pub fn zero_value(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([236, 115, 41, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_executeWrapping` (0x6338bcbc) function
        pub fn execute_wrapping(
            &self,
            from_token_address: ::ethers::core::types::Address,
            to_token_address: ::ethers::core::types::Address,
            ext_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash(
                    [99, 56, 188, 188],
                    (from_token_address, to_token_address, ext_amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_withdrawAndUnwrap` (0x509cd41e) function
        pub fn withdraw_and_unwrap(
            &self,
            from_token_address: ::ethers::core::types::Address,
            to_token_address: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            minus_ext_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [80, 156, 212, 30],
                    (
                        from_token_address,
                        to_token_address,
                        recipient,
                        minus_ext_amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculatePublicAmount` (0x2570b7b4) function
        pub fn calculate_public_amount(
            &self,
            ext_amount: ::ethers::core::types::I256,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([37, 112, 183, 180], (ext_amount, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commitments` (0x49ce8997) function
        pub fn commitments(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 206, 137, 151], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configureMaximumDepositLimit` (0x8c832b13) function
        pub fn configure_maximum_deposit_limit(
            &self,
            maximum_deposit_amount: ::ethers::core::types::U256,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [140, 131, 43, 19],
                    (maximum_deposit_amount, nonce),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configureMinimalWithdrawalLimit` (0x1f7f99f7) function
        pub fn configure_minimal_withdrawal_limit(
            &self,
            minimal_withdrawal_amount: ::ethers::core::types::U256,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [31, 127, 153, 247],
                    (minimal_withdrawal_amount, nonce),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentNeighborRootIndex` (0x5d2d766c) function
        pub fn current_neighbor_root_index(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([93, 45, 118, 108], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `edgeExistsForChain` (0xfa731687) function
        pub fn edge_exists_for_chain(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 115, 22, 135], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `edgeIndex` (0xe70ea87c) function
        pub fn edge_index(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([231, 14, 168, 124], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `edgeList` (0xdbc916b8) function
        pub fn edge_list(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash([219, 201, 22, 184], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `filledSubtrees` (0xf178e47c) function
        pub fn filled_subtrees(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([241, 120, 228, 124], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainId` (0x3408e470) function
        pub fn get_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainIdType` (0x4c830cbd) function
        pub fn get_chain_id_type(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([76, 131, 12, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHasher` (0xea495db0) function
        pub fn get_hasher(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([234, 73, 93, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLastRoot` (0xba70f757) function
        pub fn get_last_root(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([186, 112, 247, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLatestNeighborEdges` (0x8c0d34d8) function
        pub fn get_latest_neighbor_edges(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Edge>>
        {
            self.0
                .method_hash([140, 13, 52, 216], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLatestNeighborRoots` (0x1e627617) function
        pub fn get_latest_neighbor_roots(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([30, 98, 118, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLevels` (0x0c394a60) function
        pub fn get_levels(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([12, 57, 74, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextIndex` (0x0eb7606f) function
        pub fn get_next_index(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([14, 183, 96, 111], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProposalNonce` (0x0b27fb9a) function
        pub fn get_proposal_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([11, 39, 251, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getZeroHash` (0x305e9eac) function
        pub fn get_zero_hash(
            &self,
            index: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([48, 94, 158, 172], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `handler` (0xc80916d4) function
        pub fn handler(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 9, 22, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasEdge` (0x92156311) function
        pub fn has_edge(
            &self,
            chain_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([146, 21, 99, 17], chain_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashLeftRight` (0x5bb93995) function
        pub fn hash_left_right(
            &self,
            left: ::ethers::core::types::U256,
            right: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([91, 185, 57, 149], (left, right))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xe4a30116) function
        pub fn initialize(
            &self,
            minimal_withdrawal_amount: ::ethers::core::types::U256,
            maximum_deposit_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [228, 163, 1, 22],
                    (minimal_withdrawal_amount, maximum_deposit_amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialized` (0x158ef93e) function
        pub fn initialized(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 142, 249, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCorrectExecutionChain` (0x830b2f57) function
        pub fn is_correct_execution_chain(
            &self,
            resource_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([131, 11, 47, 87], resource_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCorrectExecutionContext` (0xf5fc3d6b) function
        pub fn is_correct_execution_context(
            &self,
            resource_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([245, 252, 61, 107], resource_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isKnownNeighborRoot` (0x3bfa8d7a) function
        pub fn is_known_neighbor_root(
            &self,
            neighbor_chain_id: ::ethers::core::types::U256,
            root: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([59, 250, 141, 122], (neighbor_chain_id, root))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isKnownRoot` (0xa6232a93) function
        pub fn is_known_root(
            &self,
            root: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 35, 42, 147], root)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSpent` (0x5a129efe) function
        pub fn is_spent(
            &self,
            nullifier_hash: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 18, 158, 254], nullifier_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSpentArray` (0xea65ba49) function
        pub fn is_spent_array(
            &self,
            nullifier_hashes: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>>
        {
            self.0
                .method_hash([234, 101, 186, 73], nullifier_hashes)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidRoots` (0xb75e6798) function
        pub fn is_valid_roots(
            &self,
            roots: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([183, 94, 103, 152], roots)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastBalance` (0x8f1c56bd) function
        pub fn last_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([143, 28, 86, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxEdges` (0x71523c32) function
        pub fn max_edges(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([113, 82, 60, 50], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maximumDepositAmount` (0x78abb49b) function
        pub fn maximum_deposit_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([120, 171, 180, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minimalWithdrawalAmount` (0x840b2791) function
        pub fn minimal_withdrawal_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([132, 11, 39, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `neighborRoots` (0x43e7119f) function
        pub fn neighbor_roots(
            &self,
            p0: ::ethers::core::types::U256,
            p1: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([67, 231, 17, 159], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nullifierHashes` (0x1f79a1e9) function
        pub fn nullifier_hashes(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([31, 121, 161, 233], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `outerLevels` (0xbfbc0a39) function
        pub fn outer_levels(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([191, 188, 10, 57], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseChainIdFromResourceId` (0xc2230d6e) function
        pub fn parse_chain_id_from_resource_id(
            &self,
            resource_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([194, 35, 13, 110], resource_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalNonce` (0xcc3c74a1) function
        pub fn proposal_nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([204, 60, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `register` (0xb2bc6e0f) function
        pub fn register(
            &self,
            account: Account,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 188, 110, 15], (account,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `roots` (0xc2b40ae4) function
        pub fn roots(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, u32),
        > {
            self.0
                .method_hash([194, 180, 10, 228], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHandler` (0x72c1ad03) function
        pub fn set_handler(
            &self,
            handler: ::ethers::core::types::Address,
            nonce: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 193, 173, 3], (handler, nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateEdge` (0xc1922f9e) function
        pub fn update_edge(
            &self,
            root: ::ethers::core::types::U256,
            leaf_index: u32,
            src_resource_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [193, 146, 47, 158],
                    (root, leaf_index, src_resource_id),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EdgeAddition` event
        pub fn edge_addition_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EdgeAdditionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EdgeUpdate` event
        pub fn edge_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EdgeUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Insertion` event
        pub fn insertion_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InsertionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MaxDepositLimitUpdated` event
        pub fn max_deposit_limit_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MaxDepositLimitUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MinWithdrawalLimitUpdated` event
        pub fn min_withdrawal_limit_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinWithdrawalLimitUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewCommitment` event
        pub fn new_commitment_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewCommitmentFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewNullifier` event
        pub fn new_nullifier_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewNullifierFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PublicKey` event
        pub fn public_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PublicKeyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetHandler` event
        pub fn set_handler_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetHandlerFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VAnchorBaseContractEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for VAnchorBaseContract<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "EdgeAddition",
        abi = "EdgeAddition(uint256,uint256,uint256)"
    )]
    pub struct EdgeAdditionFilter {
        pub chain_id: ::ethers::core::types::U256,
        pub latest_leaf_index: ::ethers::core::types::U256,
        pub merkle_root: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "EdgeUpdate",
        abi = "EdgeUpdate(uint256,uint256,uint256)"
    )]
    pub struct EdgeUpdateFilter {
        pub chain_id: ::ethers::core::types::U256,
        pub latest_leaf_index: ::ethers::core::types::U256,
        pub merkle_root: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Insertion",
        abi = "Insertion(uint256,uint32,uint256,uint256)"
    )]
    pub struct InsertionFilter {
        #[ethevent(indexed)]
        pub commitment: ::ethers::core::types::U256,
        pub leaf_index: u32,
        pub timestamp: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub new_merkle_root: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "MaxDepositLimitUpdated",
        abi = "MaxDepositLimitUpdated(uint256,uint32)"
    )]
    pub struct MaxDepositLimitUpdatedFilter {
        pub maximum_deposit_amount: ::ethers::core::types::U256,
        pub nonce: u32,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "MinWithdrawalLimitUpdated",
        abi = "MinWithdrawalLimitUpdated(uint256,uint32)"
    )]
    pub struct MinWithdrawalLimitUpdatedFilter {
        pub minimal_withdrawal_amount: ::ethers::core::types::U256,
        pub nonce: u32,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "NewCommitment",
        abi = "NewCommitment(uint256,uint256,uint256,bytes)"
    )]
    pub struct NewCommitmentFilter {
        pub commitment: ::ethers::core::types::U256,
        pub sub_tree_index: ::ethers::core::types::U256,
        pub leaf_index: ::ethers::core::types::U256,
        pub encrypted_output: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "NewNullifier", abi = "NewNullifier(uint256)")]
    pub struct NewNullifierFilter {
        pub nullifier: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PublicKey", abi = "PublicKey(address,bytes)")]
    pub struct PublicKeyFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub key: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SetHandler", abi = "SetHandler(address,uint32)")]
    pub struct SetHandlerFilter {
        pub handler: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum VAnchorBaseContractEvents {
        EdgeAdditionFilter(EdgeAdditionFilter),
        EdgeUpdateFilter(EdgeUpdateFilter),
        InsertionFilter(InsertionFilter),
        MaxDepositLimitUpdatedFilter(MaxDepositLimitUpdatedFilter),
        MinWithdrawalLimitUpdatedFilter(MinWithdrawalLimitUpdatedFilter),
        NewCommitmentFilter(NewCommitmentFilter),
        NewNullifierFilter(NewNullifierFilter),
        PublicKeyFilter(PublicKeyFilter),
        SetHandlerFilter(SetHandlerFilter),
    }
    impl ::ethers::contract::EthLogDecode for VAnchorBaseContractEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EdgeAdditionFilter::decode_log(log) {
                return Ok(VAnchorBaseContractEvents::EdgeAdditionFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EdgeUpdateFilter::decode_log(log) {
                return Ok(VAnchorBaseContractEvents::EdgeUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InsertionFilter::decode_log(log) {
                return Ok(VAnchorBaseContractEvents::InsertionFilter(decoded));
            }
            if let Ok(decoded) = MaxDepositLimitUpdatedFilter::decode_log(log) {
                return Ok(
                    VAnchorBaseContractEvents::MaxDepositLimitUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                MinWithdrawalLimitUpdatedFilter::decode_log(log)
            {
                return Ok(
                    VAnchorBaseContractEvents::MinWithdrawalLimitUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = NewCommitmentFilter::decode_log(log) {
                return Ok(VAnchorBaseContractEvents::NewCommitmentFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewNullifierFilter::decode_log(log) {
                return Ok(VAnchorBaseContractEvents::NewNullifierFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PublicKeyFilter::decode_log(log) {
                return Ok(VAnchorBaseContractEvents::PublicKeyFilter(decoded));
            }
            if let Ok(decoded) = SetHandlerFilter::decode_log(log) {
                return Ok(VAnchorBaseContractEvents::SetHandlerFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for VAnchorBaseContractEvents {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::EdgeAdditionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EdgeUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsertionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxDepositLimitUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinWithdrawalLimitUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewCommitmentFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewNullifierFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PublicKeyFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetHandlerFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<EdgeAdditionFilter> for VAnchorBaseContractEvents {
        fn from(value: EdgeAdditionFilter) -> Self {
            Self::EdgeAdditionFilter(value)
        }
    }
    impl ::core::convert::From<EdgeUpdateFilter> for VAnchorBaseContractEvents {
        fn from(value: EdgeUpdateFilter) -> Self {
            Self::EdgeUpdateFilter(value)
        }
    }
    impl ::core::convert::From<InsertionFilter> for VAnchorBaseContractEvents {
        fn from(value: InsertionFilter) -> Self {
            Self::InsertionFilter(value)
        }
    }
    impl ::core::convert::From<MaxDepositLimitUpdatedFilter>
        for VAnchorBaseContractEvents
    {
        fn from(value: MaxDepositLimitUpdatedFilter) -> Self {
            Self::MaxDepositLimitUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<MinWithdrawalLimitUpdatedFilter>
        for VAnchorBaseContractEvents
    {
        fn from(value: MinWithdrawalLimitUpdatedFilter) -> Self {
            Self::MinWithdrawalLimitUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<NewCommitmentFilter> for VAnchorBaseContractEvents {
        fn from(value: NewCommitmentFilter) -> Self {
            Self::NewCommitmentFilter(value)
        }
    }
    impl ::core::convert::From<NewNullifierFilter> for VAnchorBaseContractEvents {
        fn from(value: NewNullifierFilter) -> Self {
            Self::NewNullifierFilter(value)
        }
    }
    impl ::core::convert::From<PublicKeyFilter> for VAnchorBaseContractEvents {
        fn from(value: PublicKeyFilter) -> Self {
            Self::PublicKeyFilter(value)
        }
    }
    impl ::core::convert::From<SetHandlerFilter> for VAnchorBaseContractEvents {
        fn from(value: SetHandlerFilter) -> Self {
            Self::SetHandlerFilter(value)
        }
    }
    ///Container type for all input parameters for the `EVM_CHAIN_ID_TYPE` function with signature `EVM_CHAIN_ID_TYPE()` and selector `0x8b7e8782`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "EVM_CHAIN_ID_TYPE", abi = "EVM_CHAIN_ID_TYPE()")]
    pub struct EvmChainIdTypeCall;
    ///Container type for all input parameters for the `FIELD_SIZE` function with signature `FIELD_SIZE()` and selector `0x414a37ba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "FIELD_SIZE", abi = "FIELD_SIZE()")]
    pub struct FieldSizeCall;
    ///Container type for all input parameters for the `MAX_EXT_AMOUNT` function with signature `MAX_EXT_AMOUNT()` and selector `0x7fe24ffe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "MAX_EXT_AMOUNT", abi = "MAX_EXT_AMOUNT()")]
    pub struct MaxExtAmountCall;
    ///Container type for all input parameters for the `MAX_FEE` function with signature `MAX_FEE()` and selector `0xbc063e1a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "MAX_FEE", abi = "MAX_FEE()")]
    pub struct MaxFeeCall;
    ///Container type for all input parameters for the `ROOT_HISTORY_SIZE` function with signature `ROOT_HISTORY_SIZE()` and selector `0xcd87a3b4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "ROOT_HISTORY_SIZE", abi = "ROOT_HISTORY_SIZE()")]
    pub struct RootHistorySizeCall;
    ///Container type for all input parameters for the `ZERO_VALUE` function with signature `ZERO_VALUE()` and selector `0xec732959`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "ZERO_VALUE", abi = "ZERO_VALUE()")]
    pub struct ZeroValueCall;
    ///Container type for all input parameters for the `_executeWrapping` function with signature `_executeWrapping(address,address,uint256)` and selector `0x6338bcbc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "_executeWrapping",
        abi = "_executeWrapping(address,address,uint256)"
    )]
    pub struct ExecuteWrappingCall {
        pub from_token_address: ::ethers::core::types::Address,
        pub to_token_address: ::ethers::core::types::Address,
        pub ext_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `_withdrawAndUnwrap` function with signature `_withdrawAndUnwrap(address,address,address,uint256)` and selector `0x509cd41e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "_withdrawAndUnwrap",
        abi = "_withdrawAndUnwrap(address,address,address,uint256)"
    )]
    pub struct WithdrawAndUnwrapCall {
        pub from_token_address: ::ethers::core::types::Address,
        pub to_token_address: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub minus_ext_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculatePublicAmount` function with signature `calculatePublicAmount(int256,uint256)` and selector `0x2570b7b4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "calculatePublicAmount",
        abi = "calculatePublicAmount(int256,uint256)"
    )]
    pub struct CalculatePublicAmountCall {
        pub ext_amount: ::ethers::core::types::I256,
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `commitments` function with signature `commitments(uint256)` and selector `0x49ce8997`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "commitments", abi = "commitments(uint256)")]
    pub struct CommitmentsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `configureMaximumDepositLimit` function with signature `configureMaximumDepositLimit(uint256,uint32)` and selector `0x8c832b13`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "configureMaximumDepositLimit",
        abi = "configureMaximumDepositLimit(uint256,uint32)"
    )]
    pub struct ConfigureMaximumDepositLimitCall {
        pub maximum_deposit_amount: ::ethers::core::types::U256,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `configureMinimalWithdrawalLimit` function with signature `configureMinimalWithdrawalLimit(uint256,uint32)` and selector `0x1f7f99f7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "configureMinimalWithdrawalLimit",
        abi = "configureMinimalWithdrawalLimit(uint256,uint32)"
    )]
    pub struct ConfigureMinimalWithdrawalLimitCall {
        pub minimal_withdrawal_amount: ::ethers::core::types::U256,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `currentNeighborRootIndex` function with signature `currentNeighborRootIndex(uint256)` and selector `0x5d2d766c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "currentNeighborRootIndex",
        abi = "currentNeighborRootIndex(uint256)"
    )]
    pub struct CurrentNeighborRootIndexCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `edgeExistsForChain` function with signature `edgeExistsForChain(uint256)` and selector `0xfa731687`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "edgeExistsForChain", abi = "edgeExistsForChain(uint256)")]
    pub struct EdgeExistsForChainCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `edgeIndex` function with signature `edgeIndex(uint256)` and selector `0xe70ea87c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "edgeIndex", abi = "edgeIndex(uint256)")]
    pub struct EdgeIndexCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `edgeList` function with signature `edgeList(uint256)` and selector `0xdbc916b8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "edgeList", abi = "edgeList(uint256)")]
    pub struct EdgeListCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `filledSubtrees` function with signature `filledSubtrees(uint256)` and selector `0xf178e47c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "filledSubtrees", abi = "filledSubtrees(uint256)")]
    pub struct FilledSubtreesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    ///Container type for all input parameters for the `getChainIdType` function with signature `getChainIdType()` and selector `0x4c830cbd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getChainIdType", abi = "getChainIdType()")]
    pub struct GetChainIdTypeCall;
    ///Container type for all input parameters for the `getHasher` function with signature `getHasher()` and selector `0xea495db0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getHasher", abi = "getHasher()")]
    pub struct GetHasherCall;
    ///Container type for all input parameters for the `getLastRoot` function with signature `getLastRoot()` and selector `0xba70f757`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLastRoot", abi = "getLastRoot()")]
    pub struct GetLastRootCall;
    ///Container type for all input parameters for the `getLatestNeighborEdges` function with signature `getLatestNeighborEdges()` and selector `0x8c0d34d8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getLatestNeighborEdges",
        abi = "getLatestNeighborEdges()"
    )]
    pub struct GetLatestNeighborEdgesCall;
    ///Container type for all input parameters for the `getLatestNeighborRoots` function with signature `getLatestNeighborRoots()` and selector `0x1e627617`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getLatestNeighborRoots",
        abi = "getLatestNeighborRoots()"
    )]
    pub struct GetLatestNeighborRootsCall;
    ///Container type for all input parameters for the `getLevels` function with signature `getLevels()` and selector `0x0c394a60`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLevels", abi = "getLevels()")]
    pub struct GetLevelsCall;
    ///Container type for all input parameters for the `getNextIndex` function with signature `getNextIndex()` and selector `0x0eb7606f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getNextIndex", abi = "getNextIndex()")]
    pub struct GetNextIndexCall;
    ///Container type for all input parameters for the `getProposalNonce` function with signature `getProposalNonce()` and selector `0x0b27fb9a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getProposalNonce", abi = "getProposalNonce()")]
    pub struct GetProposalNonceCall;
    ///Container type for all input parameters for the `getZeroHash` function with signature `getZeroHash(uint32)` and selector `0x305e9eac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getZeroHash", abi = "getZeroHash(uint32)")]
    pub struct GetZeroHashCall {
        pub index: u32,
    }
    ///Container type for all input parameters for the `handler` function with signature `handler()` and selector `0xc80916d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "handler", abi = "handler()")]
    pub struct HandlerCall;
    ///Container type for all input parameters for the `hasEdge` function with signature `hasEdge(uint256)` and selector `0x92156311`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "hasEdge", abi = "hasEdge(uint256)")]
    pub struct HasEdgeCall {
        pub chain_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `hashLeftRight` function with signature `hashLeftRight(uint256,uint256)` and selector `0x5bb93995`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "hashLeftRight", abi = "hashLeftRight(uint256,uint256)")]
    pub struct HashLeftRightCall {
        pub left: ::ethers::core::types::U256,
        pub right: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint256,uint256)` and selector `0xe4a30116`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint256,uint256)")]
    pub struct InitializeCall {
        pub minimal_withdrawal_amount: ::ethers::core::types::U256,
        pub maximum_deposit_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialized` function with signature `initialized()` and selector `0x158ef93e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "initialized", abi = "initialized()")]
    pub struct InitializedCall;
    ///Container type for all input parameters for the `isCorrectExecutionChain` function with signature `isCorrectExecutionChain(bytes32)` and selector `0x830b2f57`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "isCorrectExecutionChain",
        abi = "isCorrectExecutionChain(bytes32)"
    )]
    pub struct IsCorrectExecutionChainCall {
        pub resource_id: [u8; 32],
    }
    ///Container type for all input parameters for the `isCorrectExecutionContext` function with signature `isCorrectExecutionContext(bytes32)` and selector `0xf5fc3d6b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "isCorrectExecutionContext",
        abi = "isCorrectExecutionContext(bytes32)"
    )]
    pub struct IsCorrectExecutionContextCall {
        pub resource_id: [u8; 32],
    }
    ///Container type for all input parameters for the `isKnownNeighborRoot` function with signature `isKnownNeighborRoot(uint256,uint256)` and selector `0x3bfa8d7a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "isKnownNeighborRoot",
        abi = "isKnownNeighborRoot(uint256,uint256)"
    )]
    pub struct IsKnownNeighborRootCall {
        pub neighbor_chain_id: ::ethers::core::types::U256,
        pub root: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `0xa6232a93`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isKnownRoot", abi = "isKnownRoot(uint256)")]
    pub struct IsKnownRootCall {
        pub root: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isSpent` function with signature `isSpent(uint256)` and selector `0x5a129efe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isSpent", abi = "isSpent(uint256)")]
    pub struct IsSpentCall {
        pub nullifier_hash: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isSpentArray` function with signature `isSpentArray(uint256[])` and selector `0xea65ba49`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isSpentArray", abi = "isSpentArray(uint256[])")]
    pub struct IsSpentArrayCall {
        pub nullifier_hashes: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `isValidRoots` function with signature `isValidRoots(uint256[])` and selector `0xb75e6798`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isValidRoots", abi = "isValidRoots(uint256[])")]
    pub struct IsValidRootsCall {
        pub roots: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `lastBalance` function with signature `lastBalance()` and selector `0x8f1c56bd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lastBalance", abi = "lastBalance()")]
    pub struct LastBalanceCall;
    ///Container type for all input parameters for the `maxEdges` function with signature `maxEdges()` and selector `0x71523c32`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "maxEdges", abi = "maxEdges()")]
    pub struct MaxEdgesCall;
    ///Container type for all input parameters for the `maximumDepositAmount` function with signature `maximumDepositAmount()` and selector `0x78abb49b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "maximumDepositAmount", abi = "maximumDepositAmount()")]
    pub struct MaximumDepositAmountCall;
    ///Container type for all input parameters for the `minimalWithdrawalAmount` function with signature `minimalWithdrawalAmount()` and selector `0x840b2791`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "minimalWithdrawalAmount",
        abi = "minimalWithdrawalAmount()"
    )]
    pub struct MinimalWithdrawalAmountCall;
    ///Container type for all input parameters for the `neighborRoots` function with signature `neighborRoots(uint256,uint32)` and selector `0x43e7119f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "neighborRoots", abi = "neighborRoots(uint256,uint32)")]
    pub struct NeighborRootsCall(pub ::ethers::core::types::U256, pub u32);
    ///Container type for all input parameters for the `nullifierHashes` function with signature `nullifierHashes(uint256)` and selector `0x1f79a1e9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "nullifierHashes", abi = "nullifierHashes(uint256)")]
    pub struct NullifierHashesCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `outerLevels` function with signature `outerLevels()` and selector `0xbfbc0a39`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "outerLevels", abi = "outerLevels()")]
    pub struct OuterLevelsCall;
    ///Container type for all input parameters for the `parseChainIdFromResourceId` function with signature `parseChainIdFromResourceId(bytes32)` and selector `0xc2230d6e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "parseChainIdFromResourceId",
        abi = "parseChainIdFromResourceId(bytes32)"
    )]
    pub struct ParseChainIdFromResourceIdCall {
        pub resource_id: [u8; 32],
    }
    ///Container type for all input parameters for the `proposalNonce` function with signature `proposalNonce()` and selector `0xcc3c74a1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "proposalNonce", abi = "proposalNonce()")]
    pub struct ProposalNonceCall;
    ///Container type for all input parameters for the `register` function with signature `register((address,bytes))` and selector `0xb2bc6e0f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "register", abi = "register((address,bytes))")]
    pub struct RegisterCall {
        pub account: Account,
    }
    ///Container type for all input parameters for the `roots` function with signature `roots(uint256)` and selector `0xc2b40ae4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "roots", abi = "roots(uint256)")]
    pub struct RootsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `setHandler` function with signature `setHandler(address,uint32)` and selector `0x72c1ad03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setHandler", abi = "setHandler(address,uint32)")]
    pub struct SetHandlerCall {
        pub handler: ::ethers::core::types::Address,
        pub nonce: u32,
    }
    ///Container type for all input parameters for the `updateEdge` function with signature `updateEdge(uint256,uint32,bytes32)` and selector `0xc1922f9e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "updateEdge", abi = "updateEdge(uint256,uint32,bytes32)")]
    pub struct UpdateEdgeCall {
        pub root: ::ethers::core::types::U256,
        pub leaf_index: u32,
        pub src_resource_id: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum VAnchorBaseContractCalls {
        EvmChainIdType(EvmChainIdTypeCall),
        FieldSize(FieldSizeCall),
        MaxExtAmount(MaxExtAmountCall),
        MaxFee(MaxFeeCall),
        RootHistorySize(RootHistorySizeCall),
        ZeroValue(ZeroValueCall),
        ExecuteWrapping(ExecuteWrappingCall),
        WithdrawAndUnwrap(WithdrawAndUnwrapCall),
        CalculatePublicAmount(CalculatePublicAmountCall),
        Commitments(CommitmentsCall),
        ConfigureMaximumDepositLimit(ConfigureMaximumDepositLimitCall),
        ConfigureMinimalWithdrawalLimit(ConfigureMinimalWithdrawalLimitCall),
        CurrentNeighborRootIndex(CurrentNeighborRootIndexCall),
        EdgeExistsForChain(EdgeExistsForChainCall),
        EdgeIndex(EdgeIndexCall),
        EdgeList(EdgeListCall),
        FilledSubtrees(FilledSubtreesCall),
        GetChainId(GetChainIdCall),
        GetChainIdType(GetChainIdTypeCall),
        GetHasher(GetHasherCall),
        GetLastRoot(GetLastRootCall),
        GetLatestNeighborEdges(GetLatestNeighborEdgesCall),
        GetLatestNeighborRoots(GetLatestNeighborRootsCall),
        GetLevels(GetLevelsCall),
        GetNextIndex(GetNextIndexCall),
        GetProposalNonce(GetProposalNonceCall),
        GetZeroHash(GetZeroHashCall),
        Handler(HandlerCall),
        HasEdge(HasEdgeCall),
        HashLeftRight(HashLeftRightCall),
        Initialize(InitializeCall),
        Initialized(InitializedCall),
        IsCorrectExecutionChain(IsCorrectExecutionChainCall),
        IsCorrectExecutionContext(IsCorrectExecutionContextCall),
        IsKnownNeighborRoot(IsKnownNeighborRootCall),
        IsKnownRoot(IsKnownRootCall),
        IsSpent(IsSpentCall),
        IsSpentArray(IsSpentArrayCall),
        IsValidRoots(IsValidRootsCall),
        LastBalance(LastBalanceCall),
        MaxEdges(MaxEdgesCall),
        MaximumDepositAmount(MaximumDepositAmountCall),
        MinimalWithdrawalAmount(MinimalWithdrawalAmountCall),
        NeighborRoots(NeighborRootsCall),
        NullifierHashes(NullifierHashesCall),
        OuterLevels(OuterLevelsCall),
        ParseChainIdFromResourceId(ParseChainIdFromResourceIdCall),
        ProposalNonce(ProposalNonceCall),
        Register(RegisterCall),
        Roots(RootsCall),
        SetHandler(SetHandlerCall),
        UpdateEdge(UpdateEdgeCall),
    }
    impl ::ethers::core::abi::AbiDecode for VAnchorBaseContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <EvmChainIdTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::EvmChainIdType(decoded));
            }
            if let Ok(decoded) =
                <FieldSizeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FieldSize(decoded));
            }
            if let Ok(decoded) =
                <MaxExtAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::MaxExtAmount(decoded));
            }
            if let Ok(decoded) =
                <MaxFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxFee(decoded));
            }
            if let Ok(decoded) =
                <RootHistorySizeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RootHistorySize(decoded));
            }
            if let Ok(decoded) =
                <ZeroValueCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ZeroValue(decoded));
            }
            if let Ok(decoded) =
                <ExecuteWrappingCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ExecuteWrapping(decoded));
            }
            if let Ok(decoded)
                = <WithdrawAndUnwrapCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawAndUnwrap(decoded));
            }
            if let Ok(decoded)
                = <CalculatePublicAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculatePublicAmount(decoded));
            }
            if let Ok(decoded) =
                <CommitmentsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::Commitments(decoded));
            }
            if let Ok(decoded)
                = <ConfigureMaximumDepositLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConfigureMaximumDepositLimit(decoded));
            }
            if let Ok(decoded)
                = <ConfigureMinimalWithdrawalLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConfigureMinimalWithdrawalLimit(decoded));
            }
            if let Ok(decoded)
                = <CurrentNeighborRootIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CurrentNeighborRootIndex(decoded));
            }
            if let Ok(decoded)
                = <EdgeExistsForChainCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EdgeExistsForChain(decoded));
            }
            if let Ok(decoded) =
                <EdgeIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EdgeIndex(decoded));
            }
            if let Ok(decoded) =
                <EdgeListCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EdgeList(decoded));
            }
            if let Ok(decoded) =
                <FilledSubtreesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::FilledSubtrees(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetChainIdType(decoded));
            }
            if let Ok(decoded) =
                <GetHasherCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetHasher(decoded));
            }
            if let Ok(decoded) =
                <GetLastRootCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetLastRoot(decoded));
            }
            if let Ok(decoded)
                = <GetLatestNeighborEdgesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLatestNeighborEdges(decoded));
            }
            if let Ok(decoded)
                = <GetLatestNeighborRootsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetLatestNeighborRoots(decoded));
            }
            if let Ok(decoded) =
                <GetLevelsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLevels(decoded));
            }
            if let Ok(decoded) =
                <GetNextIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetNextIndex(decoded));
            }
            if let Ok(decoded) =
                <GetProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <GetZeroHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetZeroHash(decoded));
            }
            if let Ok(decoded) =
                <HandlerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasEdgeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HasEdge(decoded));
            }
            if let Ok(decoded) =
                <HashLeftRightCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::HashLeftRight(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InitializedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::Initialized(decoded));
            }
            if let Ok(decoded)
                = <IsCorrectExecutionChainCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsCorrectExecutionChain(decoded));
            }
            if let Ok(decoded)
                = <IsCorrectExecutionContextCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsCorrectExecutionContext(decoded));
            }
            if let Ok(decoded)
                = <IsKnownNeighborRootCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsKnownNeighborRoot(decoded));
            }
            if let Ok(decoded) =
                <IsKnownRootCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsKnownRoot(decoded));
            }
            if let Ok(decoded) =
                <IsSpentCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsSpent(decoded));
            }
            if let Ok(decoded) =
                <IsSpentArrayCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsSpentArray(decoded));
            }
            if let Ok(decoded) =
                <IsValidRootsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::IsValidRoots(decoded));
            }
            if let Ok(decoded) =
                <LastBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LastBalance(decoded));
            }
            if let Ok(decoded) =
                <MaxEdgesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxEdges(decoded));
            }
            if let Ok(decoded)
                = <MaximumDepositAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaximumDepositAmount(decoded));
            }
            if let Ok(decoded)
                = <MinimalWithdrawalAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MinimalWithdrawalAmount(decoded));
            }
            if let Ok(decoded) =
                <NeighborRootsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NeighborRoots(decoded));
            }
            if let Ok(decoded) =
                <NullifierHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NullifierHashes(decoded));
            }
            if let Ok(decoded) =
                <OuterLevelsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::OuterLevels(decoded));
            }
            if let Ok(decoded)
                = <ParseChainIdFromResourceIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ParseChainIdFromResourceId(decoded));
            }
            if let Ok(decoded) =
                <ProposalNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <RegisterCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Register(decoded));
            }
            if let Ok(decoded) =
                <RootsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Roots(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetHandler(decoded));
            }
            if let Ok(decoded) =
                <UpdateEdgeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateEdge(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VAnchorBaseContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EvmChainIdType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FieldSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxExtAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RootHistorySize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteWrapping(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawAndUnwrap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculatePublicAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Commitments(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConfigureMaximumDepositLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConfigureMinimalWithdrawalLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentNeighborRootIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EdgeExistsForChain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EdgeIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EdgeList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FilledSubtrees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainIdType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHasher(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLastRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestNeighborEdges(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestNeighborRoots(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLevels(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetZeroHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Handler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasEdge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashLeftRight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsCorrectExecutionChain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsCorrectExecutionContext(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsKnownNeighborRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsKnownRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSpent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSpentArray(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidRoots(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxEdges(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaximumDepositAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimalWithdrawalAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NeighborRoots(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NullifierHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OuterLevels(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseChainIdFromResourceId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Register(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Roots(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHandler(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateEdge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for VAnchorBaseContractCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::EvmChainIdType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FieldSize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxExtAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::RootHistorySize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZeroValue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteWrapping(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawAndUnwrap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculatePublicAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Commitments(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConfigureMaximumDepositLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConfigureMinimalWithdrawalLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CurrentNeighborRootIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EdgeExistsForChain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EdgeIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EdgeList(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FilledSubtrees(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetChainIdType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetHasher(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLastRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLatestNeighborEdges(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLatestNeighborRoots(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLevels(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNextIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetProposalNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetZeroHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Handler(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasEdge(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashLeftRight(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialized(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsCorrectExecutionChain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsCorrectExecutionContext(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsKnownNeighborRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsKnownRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsSpent(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSpentArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsValidRoots(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxEdges(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaximumDepositAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinimalWithdrawalAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NeighborRoots(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NullifierHashes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OuterLevels(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseChainIdFromResourceId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalNonce(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Register(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Roots(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetHandler(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateEdge(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<EvmChainIdTypeCall> for VAnchorBaseContractCalls {
        fn from(value: EvmChainIdTypeCall) -> Self {
            Self::EvmChainIdType(value)
        }
    }
    impl ::core::convert::From<FieldSizeCall> for VAnchorBaseContractCalls {
        fn from(value: FieldSizeCall) -> Self {
            Self::FieldSize(value)
        }
    }
    impl ::core::convert::From<MaxExtAmountCall> for VAnchorBaseContractCalls {
        fn from(value: MaxExtAmountCall) -> Self {
            Self::MaxExtAmount(value)
        }
    }
    impl ::core::convert::From<MaxFeeCall> for VAnchorBaseContractCalls {
        fn from(value: MaxFeeCall) -> Self {
            Self::MaxFee(value)
        }
    }
    impl ::core::convert::From<RootHistorySizeCall> for VAnchorBaseContractCalls {
        fn from(value: RootHistorySizeCall) -> Self {
            Self::RootHistorySize(value)
        }
    }
    impl ::core::convert::From<ZeroValueCall> for VAnchorBaseContractCalls {
        fn from(value: ZeroValueCall) -> Self {
            Self::ZeroValue(value)
        }
    }
    impl ::core::convert::From<ExecuteWrappingCall> for VAnchorBaseContractCalls {
        fn from(value: ExecuteWrappingCall) -> Self {
            Self::ExecuteWrapping(value)
        }
    }
    impl ::core::convert::From<WithdrawAndUnwrapCall> for VAnchorBaseContractCalls {
        fn from(value: WithdrawAndUnwrapCall) -> Self {
            Self::WithdrawAndUnwrap(value)
        }
    }
    impl ::core::convert::From<CalculatePublicAmountCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: CalculatePublicAmountCall) -> Self {
            Self::CalculatePublicAmount(value)
        }
    }
    impl ::core::convert::From<CommitmentsCall> for VAnchorBaseContractCalls {
        fn from(value: CommitmentsCall) -> Self {
            Self::Commitments(value)
        }
    }
    impl ::core::convert::From<ConfigureMaximumDepositLimitCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: ConfigureMaximumDepositLimitCall) -> Self {
            Self::ConfigureMaximumDepositLimit(value)
        }
    }
    impl ::core::convert::From<ConfigureMinimalWithdrawalLimitCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: ConfigureMinimalWithdrawalLimitCall) -> Self {
            Self::ConfigureMinimalWithdrawalLimit(value)
        }
    }
    impl ::core::convert::From<CurrentNeighborRootIndexCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: CurrentNeighborRootIndexCall) -> Self {
            Self::CurrentNeighborRootIndex(value)
        }
    }
    impl ::core::convert::From<EdgeExistsForChainCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: EdgeExistsForChainCall) -> Self {
            Self::EdgeExistsForChain(value)
        }
    }
    impl ::core::convert::From<EdgeIndexCall> for VAnchorBaseContractCalls {
        fn from(value: EdgeIndexCall) -> Self {
            Self::EdgeIndex(value)
        }
    }
    impl ::core::convert::From<EdgeListCall> for VAnchorBaseContractCalls {
        fn from(value: EdgeListCall) -> Self {
            Self::EdgeList(value)
        }
    }
    impl ::core::convert::From<FilledSubtreesCall> for VAnchorBaseContractCalls {
        fn from(value: FilledSubtreesCall) -> Self {
            Self::FilledSubtrees(value)
        }
    }
    impl ::core::convert::From<GetChainIdCall> for VAnchorBaseContractCalls {
        fn from(value: GetChainIdCall) -> Self {
            Self::GetChainId(value)
        }
    }
    impl ::core::convert::From<GetChainIdTypeCall> for VAnchorBaseContractCalls {
        fn from(value: GetChainIdTypeCall) -> Self {
            Self::GetChainIdType(value)
        }
    }
    impl ::core::convert::From<GetHasherCall> for VAnchorBaseContractCalls {
        fn from(value: GetHasherCall) -> Self {
            Self::GetHasher(value)
        }
    }
    impl ::core::convert::From<GetLastRootCall> for VAnchorBaseContractCalls {
        fn from(value: GetLastRootCall) -> Self {
            Self::GetLastRoot(value)
        }
    }
    impl ::core::convert::From<GetLatestNeighborEdgesCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: GetLatestNeighborEdgesCall) -> Self {
            Self::GetLatestNeighborEdges(value)
        }
    }
    impl ::core::convert::From<GetLatestNeighborRootsCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: GetLatestNeighborRootsCall) -> Self {
            Self::GetLatestNeighborRoots(value)
        }
    }
    impl ::core::convert::From<GetLevelsCall> for VAnchorBaseContractCalls {
        fn from(value: GetLevelsCall) -> Self {
            Self::GetLevels(value)
        }
    }
    impl ::core::convert::From<GetNextIndexCall> for VAnchorBaseContractCalls {
        fn from(value: GetNextIndexCall) -> Self {
            Self::GetNextIndex(value)
        }
    }
    impl ::core::convert::From<GetProposalNonceCall> for VAnchorBaseContractCalls {
        fn from(value: GetProposalNonceCall) -> Self {
            Self::GetProposalNonce(value)
        }
    }
    impl ::core::convert::From<GetZeroHashCall> for VAnchorBaseContractCalls {
        fn from(value: GetZeroHashCall) -> Self {
            Self::GetZeroHash(value)
        }
    }
    impl ::core::convert::From<HandlerCall> for VAnchorBaseContractCalls {
        fn from(value: HandlerCall) -> Self {
            Self::Handler(value)
        }
    }
    impl ::core::convert::From<HasEdgeCall> for VAnchorBaseContractCalls {
        fn from(value: HasEdgeCall) -> Self {
            Self::HasEdge(value)
        }
    }
    impl ::core::convert::From<HashLeftRightCall> for VAnchorBaseContractCalls {
        fn from(value: HashLeftRightCall) -> Self {
            Self::HashLeftRight(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for VAnchorBaseContractCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<InitializedCall> for VAnchorBaseContractCalls {
        fn from(value: InitializedCall) -> Self {
            Self::Initialized(value)
        }
    }
    impl ::core::convert::From<IsCorrectExecutionChainCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: IsCorrectExecutionChainCall) -> Self {
            Self::IsCorrectExecutionChain(value)
        }
    }
    impl ::core::convert::From<IsCorrectExecutionContextCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: IsCorrectExecutionContextCall) -> Self {
            Self::IsCorrectExecutionContext(value)
        }
    }
    impl ::core::convert::From<IsKnownNeighborRootCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: IsKnownNeighborRootCall) -> Self {
            Self::IsKnownNeighborRoot(value)
        }
    }
    impl ::core::convert::From<IsKnownRootCall> for VAnchorBaseContractCalls {
        fn from(value: IsKnownRootCall) -> Self {
            Self::IsKnownRoot(value)
        }
    }
    impl ::core::convert::From<IsSpentCall> for VAnchorBaseContractCalls {
        fn from(value: IsSpentCall) -> Self {
            Self::IsSpent(value)
        }
    }
    impl ::core::convert::From<IsSpentArrayCall> for VAnchorBaseContractCalls {
        fn from(value: IsSpentArrayCall) -> Self {
            Self::IsSpentArray(value)
        }
    }
    impl ::core::convert::From<IsValidRootsCall> for VAnchorBaseContractCalls {
        fn from(value: IsValidRootsCall) -> Self {
            Self::IsValidRoots(value)
        }
    }
    impl ::core::convert::From<LastBalanceCall> for VAnchorBaseContractCalls {
        fn from(value: LastBalanceCall) -> Self {
            Self::LastBalance(value)
        }
    }
    impl ::core::convert::From<MaxEdgesCall> for VAnchorBaseContractCalls {
        fn from(value: MaxEdgesCall) -> Self {
            Self::MaxEdges(value)
        }
    }
    impl ::core::convert::From<MaximumDepositAmountCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: MaximumDepositAmountCall) -> Self {
            Self::MaximumDepositAmount(value)
        }
    }
    impl ::core::convert::From<MinimalWithdrawalAmountCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: MinimalWithdrawalAmountCall) -> Self {
            Self::MinimalWithdrawalAmount(value)
        }
    }
    impl ::core::convert::From<NeighborRootsCall> for VAnchorBaseContractCalls {
        fn from(value: NeighborRootsCall) -> Self {
            Self::NeighborRoots(value)
        }
    }
    impl ::core::convert::From<NullifierHashesCall> for VAnchorBaseContractCalls {
        fn from(value: NullifierHashesCall) -> Self {
            Self::NullifierHashes(value)
        }
    }
    impl ::core::convert::From<OuterLevelsCall> for VAnchorBaseContractCalls {
        fn from(value: OuterLevelsCall) -> Self {
            Self::OuterLevels(value)
        }
    }
    impl ::core::convert::From<ParseChainIdFromResourceIdCall>
        for VAnchorBaseContractCalls
    {
        fn from(value: ParseChainIdFromResourceIdCall) -> Self {
            Self::ParseChainIdFromResourceId(value)
        }
    }
    impl ::core::convert::From<ProposalNonceCall> for VAnchorBaseContractCalls {
        fn from(value: ProposalNonceCall) -> Self {
            Self::ProposalNonce(value)
        }
    }
    impl ::core::convert::From<RegisterCall> for VAnchorBaseContractCalls {
        fn from(value: RegisterCall) -> Self {
            Self::Register(value)
        }
    }
    impl ::core::convert::From<RootsCall> for VAnchorBaseContractCalls {
        fn from(value: RootsCall) -> Self {
            Self::Roots(value)
        }
    }
    impl ::core::convert::From<SetHandlerCall> for VAnchorBaseContractCalls {
        fn from(value: SetHandlerCall) -> Self {
            Self::SetHandler(value)
        }
    }
    impl ::core::convert::From<UpdateEdgeCall> for VAnchorBaseContractCalls {
        fn from(value: UpdateEdgeCall) -> Self {
            Self::UpdateEdge(value)
        }
    }
    ///Container type for all return fields from the `EVM_CHAIN_ID_TYPE` function with signature `EVM_CHAIN_ID_TYPE()` and selector `0x8b7e8782`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EvmChainIdTypeReturn(pub [u8; 2]);
    ///Container type for all return fields from the `FIELD_SIZE` function with signature `FIELD_SIZE()` and selector `0x414a37ba`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FieldSizeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_EXT_AMOUNT` function with signature `MAX_EXT_AMOUNT()` and selector `0x7fe24ffe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MaxExtAmountReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `MAX_FEE` function with signature `MAX_FEE()` and selector `0xbc063e1a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MaxFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ROOT_HISTORY_SIZE` function with signature `ROOT_HISTORY_SIZE()` and selector `0xcd87a3b4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RootHistorySizeReturn(pub u32);
    ///Container type for all return fields from the `ZERO_VALUE` function with signature `ZERO_VALUE()` and selector `0xec732959`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ZeroValueReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_executeWrapping` function with signature `_executeWrapping(address,address,uint256)` and selector `0x6338bcbc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ExecuteWrappingReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `calculatePublicAmount` function with signature `calculatePublicAmount(int256,uint256)` and selector `0x2570b7b4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CalculatePublicAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `commitments` function with signature `commitments(uint256)` and selector `0x49ce8997`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CommitmentsReturn(pub bool);
    ///Container type for all return fields from the `currentNeighborRootIndex` function with signature `currentNeighborRootIndex(uint256)` and selector `0x5d2d766c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CurrentNeighborRootIndexReturn(pub u32);
    ///Container type for all return fields from the `edgeExistsForChain` function with signature `edgeExistsForChain(uint256)` and selector `0xfa731687`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EdgeExistsForChainReturn(pub bool);
    ///Container type for all return fields from the `edgeIndex` function with signature `edgeIndex(uint256)` and selector `0xe70ea87c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EdgeIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `edgeList` function with signature `edgeList(uint256)` and selector `0xdbc916b8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EdgeListReturn {
        pub chain_id: ::ethers::core::types::U256,
        pub root: ::ethers::core::types::U256,
        pub latest_leaf_index: ::ethers::core::types::U256,
        pub src_resource_id: [u8; 32],
    }
    ///Container type for all return fields from the `filledSubtrees` function with signature `filledSubtrees(uint256)` and selector `0xf178e47c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FilledSubtreesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `0x3408e470`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetChainIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getChainIdType` function with signature `getChainIdType()` and selector `0x4c830cbd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetChainIdTypeReturn(pub u64);
    ///Container type for all return fields from the `getHasher` function with signature `getHasher()` and selector `0xea495db0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetHasherReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getLastRoot` function with signature `getLastRoot()` and selector `0xba70f757`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLastRootReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLatestNeighborEdges` function with signature `getLatestNeighborEdges()` and selector `0x8c0d34d8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLatestNeighborEdgesReturn(pub ::std::vec::Vec<Edge>);
    ///Container type for all return fields from the `getLatestNeighborRoots` function with signature `getLatestNeighborRoots()` and selector `0x1e627617`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLatestNeighborRootsReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `getLevels` function with signature `getLevels()` and selector `0x0c394a60`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLevelsReturn(pub u32);
    ///Container type for all return fields from the `getNextIndex` function with signature `getNextIndex()` and selector `0x0eb7606f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetNextIndexReturn(pub u32);
    ///Container type for all return fields from the `getProposalNonce` function with signature `getProposalNonce()` and selector `0x0b27fb9a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetProposalNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getZeroHash` function with signature `getZeroHash(uint32)` and selector `0x305e9eac`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetZeroHashReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `handler` function with signature `handler()` and selector `0xc80916d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HandlerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `hasEdge` function with signature `hasEdge(uint256)` and selector `0x92156311`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HasEdgeReturn(pub bool);
    ///Container type for all return fields from the `hashLeftRight` function with signature `hashLeftRight(uint256,uint256)` and selector `0x5bb93995`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HashLeftRightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `initialized` function with signature `initialized()` and selector `0x158ef93e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct InitializedReturn(pub bool);
    ///Container type for all return fields from the `isCorrectExecutionChain` function with signature `isCorrectExecutionChain(bytes32)` and selector `0x830b2f57`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsCorrectExecutionChainReturn(pub bool);
    ///Container type for all return fields from the `isCorrectExecutionContext` function with signature `isCorrectExecutionContext(bytes32)` and selector `0xf5fc3d6b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsCorrectExecutionContextReturn(pub bool);
    ///Container type for all return fields from the `isKnownNeighborRoot` function with signature `isKnownNeighborRoot(uint256,uint256)` and selector `0x3bfa8d7a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsKnownNeighborRootReturn(pub bool);
    ///Container type for all return fields from the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `0xa6232a93`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsKnownRootReturn(pub bool);
    ///Container type for all return fields from the `isSpent` function with signature `isSpent(uint256)` and selector `0x5a129efe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsSpentReturn(pub bool);
    ///Container type for all return fields from the `isSpentArray` function with signature `isSpentArray(uint256[])` and selector `0xea65ba49`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsSpentArrayReturn(pub ::std::vec::Vec<bool>);
    ///Container type for all return fields from the `isValidRoots` function with signature `isValidRoots(uint256[])` and selector `0xb75e6798`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsValidRootsReturn(pub bool);
    ///Container type for all return fields from the `lastBalance` function with signature `lastBalance()` and selector `0x8f1c56bd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LastBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxEdges` function with signature `maxEdges()` and selector `0x71523c32`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MaxEdgesReturn(pub u8);
    ///Container type for all return fields from the `maximumDepositAmount` function with signature `maximumDepositAmount()` and selector `0x78abb49b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MaximumDepositAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minimalWithdrawalAmount` function with signature `minimalWithdrawalAmount()` and selector `0x840b2791`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MinimalWithdrawalAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `neighborRoots` function with signature `neighborRoots(uint256,uint32)` and selector `0x43e7119f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NeighborRootsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nullifierHashes` function with signature `nullifierHashes(uint256)` and selector `0x1f79a1e9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NullifierHashesReturn(pub bool);
    ///Container type for all return fields from the `outerLevels` function with signature `outerLevels()` and selector `0xbfbc0a39`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OuterLevelsReturn(pub u32);
    ///Container type for all return fields from the `parseChainIdFromResourceId` function with signature `parseChainIdFromResourceId(bytes32)` and selector `0xc2230d6e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ParseChainIdFromResourceIdReturn(pub u64);
    ///Container type for all return fields from the `proposalNonce` function with signature `proposalNonce()` and selector `0xcc3c74a1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProposalNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `roots` function with signature `roots(uint256)` and selector `0xc2b40ae4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RootsReturn {
        pub root: ::ethers::core::types::U256,
        pub latest_leafindex: u32,
    }
    ///`Edge(uint256,uint256,uint256,bytes32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Edge {
        pub chain_id: ::ethers::core::types::U256,
        pub root: ::ethers::core::types::U256,
        pub latest_leaf_index: ::ethers::core::types::U256,
        pub src_resource_id: [u8; 32],
    }
    ///`Account(address,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Account {
        pub owner: ::ethers::core::types::Address,
        pub key_data: ::ethers::core::types::Bytes,
    }
}
