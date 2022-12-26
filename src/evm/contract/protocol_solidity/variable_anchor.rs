pub use v_anchor_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod v_anchor_contract {
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
    #[doc = "VAnchorContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VANCHORCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"merkleRoot\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EdgeAddition\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"merkleRoot\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"EdgeUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint32\",\"name\":\"leafIndex\",\"type\":\"uint32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Insertion\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"subTreeIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"leafIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewCommitment\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nullifier\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewNullifier\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"key\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PublicKey\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EVM_CHAIN_ID_TYPE\",\"outputs\":[{\"internalType\":\"bytes2\",\"name\":\"\",\"type\":\"bytes2\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FIELD_SIZE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_EXT_AMOUNT\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_FEE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ROOT_HISTORY_SIZE\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ZERO_VALUE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_fromTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_toTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_extAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"_executeWrapping\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_fromTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_toTokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_minusExtAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"_withdrawAndUnwrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"_extAmount\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_fee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"calculatePublicAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureMaximumDepositLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"configureMinimalWithdrawalLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentNeighborRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeExistsForChain\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"edgeList\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"srcResourceID\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"filledSubtrees\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainIdType\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getHasher\",\"outputs\":[{\"internalType\":\"contract IHasher\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastRoot\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestNeighborEdges\",\"outputs\":[{\"internalType\":\"struct Edge[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"srcResourceID\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLatestNeighborRoots\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLevels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNextIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"index\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getZeroHash\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"handler\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_chainID\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasEdge\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_left\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_right\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hashLeftRight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"initialized\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_neighborChainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_root\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isKnownNeighborRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isKnownRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_nullifierHash\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_nullifierHashes\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpentArray\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"\",\"type\":\"bool[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_roots\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidRoots\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxEdges\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maximumDepositAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minimalWithdrawalAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"neighborRoots\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nullifierHashes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"outerLevels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_resourceId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"parseChainIdFromResourceId\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct VAnchorBase.Account\",\"name\":\"_account\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"keyData\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"register\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct VAnchorBase.Account\",\"name\":\"_account\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"keyData\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"_proof\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_auxPublicInputs\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"struct CommonExtData\",\"name\":\"_externalData\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"extAmount\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"refund\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"struct PublicInputs\",\"name\":\"_publicInputs\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"roots\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"extensionRoots\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"inputNullifiers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"outputCommitments\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"publicAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"extDataHash\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct Encryptions\",\"name\":\"_encryptions\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"encryptedOutput1\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput2\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"registerAndTransact\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roots\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"latestLeafindex\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setHandler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setVerifier\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_proof\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_auxPublicInputs\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"struct CommonExtData\",\"name\":\"_externalData\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"extAmount\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"refund\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"struct PublicInputs\",\"name\":\"_publicInputs\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"roots\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"extensionRoots\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"inputNullifiers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"outputCommitments\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"publicAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"extDataHash\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct Encryptions\",\"name\":\"_encryptions\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"encryptedOutput1\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput2\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"transact\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256[8]\",\"name\":\"_proof\",\"type\":\"uint256[8]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"unpackProof\",\"outputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2][2]\",\"name\":\"\",\"type\":\"uint256[2][2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_root\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_leafIndex\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_srcResourceID\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"updateEdge\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifier\",\"outputs\":[{\"internalType\":\"contract IAnchorVerifier\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
    });
    pub struct VAnchorContract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for VAnchorContract<M> {
        fn clone(&self) -> Self {
            VAnchorContract(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for VAnchorContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for VAnchorContract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(VAnchorContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> VAnchorContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                VANCHORCONTRACT_ABI.clone(),
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
        #[doc = "Calls the contract's `_executeWrapping` (0x6338bcbc) function"]
        pub fn execute_wrapping(
            &self,
            from_token_address: ethers::core::types::Address,
            to_token_address: ethers::core::types::Address,
            ext_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash(
                    [99, 56, 188, 188],
                    (from_token_address, to_token_address, ext_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_withdrawAndUnwrap` (0x509cd41e) function"]
        pub fn withdraw_and_unwrap(
            &self,
            from_token_address: ethers::core::types::Address,
            to_token_address: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            minus_ext_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
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
        #[doc = "Calls the contract's `calculatePublicAmount` (0x2570b7b4) function"]
        pub fn calculate_public_amount(
            &self,
            ext_amount: I256,
            fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([37, 112, 183, 180], (ext_amount, fee))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `commitments` (0x49ce8997) function"]
        pub fn commitments(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 206, 137, 151], p0)
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
                ethers::core::types::U256,
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
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
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
        #[doc = "Calls the contract's `getHasher` (0xea495db0) function"]
        pub fn get_hasher(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([234, 73, 93, 176], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastRoot` (0xba70f757) function"]
        pub fn get_last_root(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
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
            ::std::vec::Vec<ethers::core::types::U256>,
        > {
            self.0
                .method_hash([30, 98, 118, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLevels` (0x0c394a60) function"]
        pub fn get_levels(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([12, 57, 74, 96], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNextIndex` (0x0eb7606f) function"]
        pub fn get_next_index(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([14, 183, 96, 111], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getProposalNonce` (0x0b27fb9a) function"]
        pub fn get_proposal_nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([11, 39, 251, 154], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getZeroHash` (0x305e9eac) function"]
        pub fn get_zero_hash(
            &self,
            index: u32,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([48, 94, 158, 172], index)
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
        #[doc = "Calls the contract's `hashLeftRight` (0x5bb93995) function"]
        pub fn hash_left_right(
            &self,
            left: ethers::core::types::U256,
            right: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([91, 185, 57, 149], (left, right))
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
        #[doc = "Calls the contract's `initialized` (0x158ef93e) function"]
        pub fn initialized(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 142, 249, 62], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKnownNeighborRoot` (0x3bfa8d7a) function"]
        pub fn is_known_neighbor_root(
            &self,
            neighbor_chain_id: ethers::core::types::U256,
            root: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([59, 250, 141, 122], (neighbor_chain_id, root))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKnownRoot` (0xa6232a93) function"]
        pub fn is_known_root(
            &self,
            root: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([166, 35, 42, 147], root)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpent` (0x5a129efe) function"]
        pub fn is_spent(
            &self,
            nullifier_hash: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 18, 158, 254], nullifier_hash)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpentArray` (0xea65ba49) function"]
        pub fn is_spent_array(
            &self,
            nullifier_hashes: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>>
        {
            self.0
                .method_hash([234, 101, 186, 73], nullifier_hashes)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isValidRoots` (0xb75e6798) function"]
        pub fn is_valid_roots(
            &self,
            roots: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([183, 94, 103, 152], roots)
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
        #[doc = "Calls the contract's `maxEdges` (0x71523c32) function"]
        pub fn max_edges(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([113, 82, 60, 50], ())
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
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([67, 231, 17, 159], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nullifierHashes` (0x1f79a1e9) function"]
        pub fn nullifier_hashes(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([31, 121, 161, 233], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `outerLevels` (0xbfbc0a39) function"]
        pub fn outer_levels(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([191, 188, 10, 57], ())
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
        #[doc = "Calls the contract's `proposalNonce` (0xcc3c74a1) function"]
        pub fn proposal_nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([204, 60, 116, 161], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `register` (0xb2bc6e0f) function"]
        pub fn register(
            &self,
            account: Account,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 188, 110, 15], (account,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerAndTransact` (0x38a26a09) function"]
        pub fn register_and_transact(
            &self,
            account: Account,
            proof: ethers::core::types::Bytes,
            aux_public_inputs: ethers::core::types::Bytes,
            external_data: CommonExtData,
            public_inputs: PublicInputs,
            encryptions: Encryptions,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [56, 162, 106, 9],
                    (
                        account,
                        proof,
                        aux_public_inputs,
                        external_data,
                        public_inputs,
                        encryptions,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roots` (0xc2b40ae4) function"]
        pub fn roots(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, u32),
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
        #[doc = "Calls the contract's `transact` (0xa38f76e8) function"]
        pub fn transact(
            &self,
            proof: ethers::core::types::Bytes,
            aux_public_inputs: ethers::core::types::Bytes,
            external_data: CommonExtData,
            public_inputs: PublicInputs,
            encryptions: Encryptions,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [163, 143, 118, 232],
                    (
                        proof,
                        aux_public_inputs,
                        external_data,
                        public_inputs,
                        encryptions,
                    ),
                )
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
        #[doc = "Calls the contract's `updateEdge` (0xc1922f9e) function"]
        pub fn update_edge(
            &self,
            root: ethers::core::types::U256,
            leaf_index: u32,
            src_resource_id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [193, 146, 47, 158],
                    (root, leaf_index, src_resource_id),
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
        ) -> ethers::contract::builders::Event<M, VAnchorContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for VAnchorContract<M>
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
        abi = "EdgeAddition(uint256,uint256,uint256)"
    )]
    pub struct EdgeAdditionFilter {
        pub chain_id: ethers::core::types::U256,
        pub latest_leaf_index: ethers::core::types::U256,
        pub merkle_root: ethers::core::types::U256,
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
        abi = "EdgeUpdate(uint256,uint256,uint256)"
    )]
    pub struct EdgeUpdateFilter {
        pub chain_id: ethers::core::types::U256,
        pub latest_leaf_index: ethers::core::types::U256,
        pub merkle_root: ethers::core::types::U256,
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
    #[ethevent(name = "Insertion", abi = "Insertion(uint256,uint32,uint256)")]
    pub struct InsertionFilter {
        #[ethevent(indexed)]
        pub commitment: ethers::core::types::U256,
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
    #[ethevent(
        name = "NewCommitment",
        abi = "NewCommitment(uint256,uint256,uint256,bytes)"
    )]
    pub struct NewCommitmentFilter {
        pub commitment: ethers::core::types::U256,
        pub sub_tree_index: ethers::core::types::U256,
        pub leaf_index: ethers::core::types::U256,
        pub encrypted_output: ethers::core::types::Bytes,
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
    #[ethevent(name = "NewNullifier", abi = "NewNullifier(uint256)")]
    pub struct NewNullifierFilter {
        pub nullifier: ethers::core::types::U256,
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
    pub enum VAnchorContractEvents {
        EdgeAdditionFilter(EdgeAdditionFilter),
        EdgeUpdateFilter(EdgeUpdateFilter),
        InsertionFilter(InsertionFilter),
        NewCommitmentFilter(NewCommitmentFilter),
        NewNullifierFilter(NewNullifierFilter),
        PublicKeyFilter(PublicKeyFilter),
    }
    impl ethers::contract::EthLogDecode for VAnchorContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = EdgeAdditionFilter::decode_log(log) {
                return Ok(VAnchorContractEvents::EdgeAdditionFilter(decoded));
            }
            if let Ok(decoded) = EdgeUpdateFilter::decode_log(log) {
                return Ok(VAnchorContractEvents::EdgeUpdateFilter(decoded));
            }
            if let Ok(decoded) = InsertionFilter::decode_log(log) {
                return Ok(VAnchorContractEvents::InsertionFilter(decoded));
            }
            if let Ok(decoded) = NewCommitmentFilter::decode_log(log) {
                return Ok(VAnchorContractEvents::NewCommitmentFilter(decoded));
            }
            if let Ok(decoded) = NewNullifierFilter::decode_log(log) {
                return Ok(VAnchorContractEvents::NewNullifierFilter(decoded));
            }
            if let Ok(decoded) = PublicKeyFilter::decode_log(log) {
                return Ok(VAnchorContractEvents::PublicKeyFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for VAnchorContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VAnchorContractEvents::EdgeAdditionFilter(element) => {
                    element.fmt(f)
                }
                VAnchorContractEvents::EdgeUpdateFilter(element) => {
                    element.fmt(f)
                }
                VAnchorContractEvents::InsertionFilter(element) => {
                    element.fmt(f)
                }
                VAnchorContractEvents::NewCommitmentFilter(element) => {
                    element.fmt(f)
                }
                VAnchorContractEvents::NewNullifierFilter(element) => {
                    element.fmt(f)
                }
                VAnchorContractEvents::PublicKeyFilter(element) => {
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
    #[doc = "Container type for all input parameters for the `_executeWrapping` function with signature `_executeWrapping(address,address,uint256)` and selector `[99, 56, 188, 188]`"]
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
        abi = "_executeWrapping(address,address,uint256)"
    )]
    pub struct ExecuteWrappingCall {
        pub from_token_address: ethers::core::types::Address,
        pub to_token_address: ethers::core::types::Address,
        pub ext_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `_withdrawAndUnwrap` function with signature `_withdrawAndUnwrap(address,address,address,uint256)` and selector `[80, 156, 212, 30]`"]
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
        name = "_withdrawAndUnwrap",
        abi = "_withdrawAndUnwrap(address,address,address,uint256)"
    )]
    pub struct WithdrawAndUnwrapCall {
        pub from_token_address: ethers::core::types::Address,
        pub to_token_address: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub minus_ext_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `calculatePublicAmount` function with signature `calculatePublicAmount(int256,uint256)` and selector `[37, 112, 183, 180]`"]
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
        name = "calculatePublicAmount",
        abi = "calculatePublicAmount(int256,uint256)"
    )]
    pub struct CalculatePublicAmountCall {
        pub ext_amount: I256,
        pub fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `commitments` function with signature `commitments(uint256)` and selector `[73, 206, 137, 151]`"]
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
    #[ethcall(name = "commitments", abi = "commitments(uint256)")]
    pub struct CommitmentsCall(pub ethers::core::types::U256);
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
    #[doc = "Container type for all input parameters for the `getHasher` function with signature `getHasher()` and selector `[234, 73, 93, 176]`"]
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
    #[ethcall(name = "getHasher", abi = "getHasher()")]
    pub struct GetHasherCall;
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
    #[doc = "Container type for all input parameters for the `getLevels` function with signature `getLevels()` and selector `[12, 57, 74, 96]`"]
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
    #[ethcall(name = "getLevels", abi = "getLevels()")]
    pub struct GetLevelsCall;
    #[doc = "Container type for all input parameters for the `getNextIndex` function with signature `getNextIndex()` and selector `[14, 183, 96, 111]`"]
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
    #[ethcall(name = "getNextIndex", abi = "getNextIndex()")]
    pub struct GetNextIndexCall;
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
    #[doc = "Container type for all input parameters for the `getZeroHash` function with signature `getZeroHash(uint32)` and selector `[48, 94, 158, 172]`"]
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
    #[ethcall(name = "getZeroHash", abi = "getZeroHash(uint32)")]
    pub struct GetZeroHashCall {
        pub index: u32,
    }
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
    #[doc = "Container type for all input parameters for the `hashLeftRight` function with signature `hashLeftRight(uint256,uint256)` and selector `[91, 185, 57, 149]`"]
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
    #[ethcall(name = "hashLeftRight", abi = "hashLeftRight(uint256,uint256)")]
    pub struct HashLeftRightCall {
        pub left: ethers::core::types::U256,
        pub right: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `initialized` function with signature `initialized()` and selector `[21, 142, 249, 62]`"]
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
    #[ethcall(name = "initialized", abi = "initialized()")]
    pub struct InitializedCall;
    #[doc = "Container type for all input parameters for the `isKnownNeighborRoot` function with signature `isKnownNeighborRoot(uint256,uint256)` and selector `[59, 250, 141, 122]`"]
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
        abi = "isKnownNeighborRoot(uint256,uint256)"
    )]
    pub struct IsKnownNeighborRootCall {
        pub neighbor_chain_id: ethers::core::types::U256,
        pub root: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `[166, 35, 42, 147]`"]
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
    #[ethcall(name = "isKnownRoot", abi = "isKnownRoot(uint256)")]
    pub struct IsKnownRootCall {
        pub root: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isSpent` function with signature `isSpent(uint256)` and selector `[90, 18, 158, 254]`"]
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
    #[ethcall(name = "isSpent", abi = "isSpent(uint256)")]
    pub struct IsSpentCall {
        pub nullifier_hash: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isSpentArray` function with signature `isSpentArray(uint256[])` and selector `[234, 101, 186, 73]`"]
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
    #[ethcall(name = "isSpentArray", abi = "isSpentArray(uint256[])")]
    pub struct IsSpentArrayCall {
        pub nullifier_hashes: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `isValidRoots` function with signature `isValidRoots(uint256[])` and selector `[183, 94, 103, 152]`"]
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
    #[ethcall(name = "isValidRoots", abi = "isValidRoots(uint256[])")]
    pub struct IsValidRootsCall {
        pub roots: ::std::vec::Vec<ethers::core::types::U256>,
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
    #[doc = "Container type for all input parameters for the `maxEdges` function with signature `maxEdges()` and selector `[113, 82, 60, 50]`"]
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
    #[doc = "Container type for all input parameters for the `nullifierHashes` function with signature `nullifierHashes(uint256)` and selector `[31, 121, 161, 233]`"]
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
    #[ethcall(name = "nullifierHashes", abi = "nullifierHashes(uint256)")]
    pub struct NullifierHashesCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `outerLevels` function with signature `outerLevels()` and selector `[191, 188, 10, 57]`"]
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
    #[ethcall(name = "outerLevels", abi = "outerLevels()")]
    pub struct OuterLevelsCall;
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
    #[doc = "Container type for all input parameters for the `proposalNonce` function with signature `proposalNonce()` and selector `[204, 60, 116, 161]`"]
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
    #[ethcall(name = "proposalNonce", abi = "proposalNonce()")]
    pub struct ProposalNonceCall;
    #[doc = "Container type for all input parameters for the `register` function with signature `register((address,bytes))` and selector `[178, 188, 110, 15]`"]
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
    #[ethcall(name = "register", abi = "register((address,bytes))")]
    pub struct RegisterCall {
        pub account: Account,
    }
    #[doc = "Container type for all input parameters for the `registerAndTransact` function with signature `registerAndTransact((address,bytes),bytes,bytes,(address,int256,address,uint256,uint256,address),(bytes,bytes,uint256[],uint256[2],uint256,uint256),(bytes,bytes))` and selector `[56, 162, 106, 9]`"]
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
        name = "registerAndTransact",
        abi = "registerAndTransact((address,bytes),bytes,bytes,(address,int256,address,uint256,uint256,address),(bytes,bytes,uint256[],uint256[2],uint256,uint256),(bytes,bytes))"
    )]
    pub struct RegisterAndTransactCall {
        pub account: Account,
        pub proof: ethers::core::types::Bytes,
        pub aux_public_inputs: ethers::core::types::Bytes,
        pub external_data: CommonExtData,
        pub public_inputs: PublicInputs,
        pub encryptions: Encryptions,
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
    #[doc = "Container type for all input parameters for the `setVerifier` function with signature `setVerifier(address,uint32)` and selector `[160, 209, 146, 245]`"]
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
    #[doc = "Container type for all input parameters for the `transact` function with signature `transact(bytes,bytes,(address,int256,address,uint256,uint256,address),(bytes,bytes,uint256[],uint256[2],uint256,uint256),(bytes,bytes))` and selector `[163, 143, 118, 232]`"]
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
        name = "transact",
        abi = "transact(bytes,bytes,(address,int256,address,uint256,uint256,address),(bytes,bytes,uint256[],uint256[2],uint256,uint256),(bytes,bytes))"
    )]
    pub struct TransactCall {
        pub proof: ethers::core::types::Bytes,
        pub aux_public_inputs: ethers::core::types::Bytes,
        pub external_data: CommonExtData,
        pub public_inputs: PublicInputs,
        pub encryptions: Encryptions,
    }
    #[doc = "Container type for all input parameters for the `unpackProof` function with signature `unpackProof(uint256[8])` and selector `[245, 171, 13, 214]`"]
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
    #[doc = "Container type for all input parameters for the `updateEdge` function with signature `updateEdge(uint256,uint32,bytes32)` and selector `[193, 146, 47, 158]`"]
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
    #[ethcall(name = "updateEdge", abi = "updateEdge(uint256,uint32,bytes32)")]
    pub struct UpdateEdgeCall {
        pub root: ethers::core::types::U256,
        pub leaf_index: u32,
        pub src_resource_id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `verifier` function with signature `verifier()` and selector `[43, 122, 195, 243]`"]
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum VAnchorContractCalls {
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
        RegisterAndTransact(RegisterAndTransactCall),
        Roots(RootsCall),
        SetHandler(SetHandlerCall),
        SetVerifier(SetVerifierCall),
        Token(TokenCall),
        Transact(TransactCall),
        UnpackProof(UnpackProofCall),
        UpdateEdge(UpdateEdgeCall),
        Verifier(VerifierCall),
    }
    impl ethers::core::abi::AbiDecode for VAnchorContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <EvmChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::EvmChainIdType(decoded));
            }
            if let Ok(decoded) =
                <FieldSizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::FieldSize(decoded));
            }
            if let Ok(decoded) =
                <MaxExtAmountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::MaxExtAmount(decoded));
            }
            if let Ok(decoded) =
                <MaxFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::MaxFee(decoded));
            }
            if let Ok(decoded) =
                <RootHistorySizeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::RootHistorySize(decoded));
            }
            if let Ok(decoded) =
                <ZeroValueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::ZeroValue(decoded));
            }
            if let Ok(decoded) =
                <ExecuteWrappingCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::ExecuteWrapping(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAndUnwrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::WithdrawAndUnwrap(decoded));
            }
            if let Ok (decoded) = < CalculatePublicAmountCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: CalculatePublicAmount (decoded)) }
            if let Ok(decoded) =
                <CommitmentsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Commitments(decoded));
            }
            if let Ok (decoded) = < ConfigureMaximumDepositLimitCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: ConfigureMaximumDepositLimit (decoded)) }
            if let Ok (decoded) = < ConfigureMinimalWithdrawalLimitCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: ConfigureMinimalWithdrawalLimit (decoded)) }
            if let Ok (decoded) = < CurrentNeighborRootIndexCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: CurrentNeighborRootIndex (decoded)) }
            if let Ok(decoded) =
                <EdgeExistsForChainCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::EdgeExistsForChain(decoded));
            }
            if let Ok(decoded) =
                <EdgeIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::EdgeIndex(decoded));
            }
            if let Ok(decoded) =
                <EdgeListCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::EdgeList(decoded));
            }
            if let Ok(decoded) =
                <FilledSubtreesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::FilledSubtrees(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::GetChainIdType(decoded));
            }
            if let Ok(decoded) =
                <GetHasherCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::GetHasher(decoded));
            }
            if let Ok(decoded) =
                <GetLastRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::GetLastRoot(decoded));
            }
            if let Ok (decoded) = < GetLatestNeighborEdgesCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: GetLatestNeighborEdges (decoded)) }
            if let Ok (decoded) = < GetLatestNeighborRootsCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: GetLatestNeighborRoots (decoded)) }
            if let Ok(decoded) =
                <GetLevelsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::GetLevels(decoded));
            }
            if let Ok(decoded) =
                <GetNextIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::GetNextIndex(decoded));
            }
            if let Ok(decoded) =
                <GetProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::GetProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <GetZeroHashCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::GetZeroHash(decoded));
            }
            if let Ok(decoded) =
                <HandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::HasEdge(decoded));
            }
            if let Ok(decoded) =
                <HashLeftRightCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::HashLeftRight(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InitializedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Initialized(decoded));
            }
            if let Ok (decoded) = < IsKnownNeighborRootCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: IsKnownNeighborRoot (decoded)) }
            if let Ok(decoded) =
                <IsKnownRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::IsKnownRoot(decoded));
            }
            if let Ok(decoded) =
                <IsSpentCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::IsSpent(decoded));
            }
            if let Ok(decoded) =
                <IsSpentArrayCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::IsSpentArray(decoded));
            }
            if let Ok(decoded) =
                <IsValidRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::IsValidRoots(decoded));
            }
            if let Ok(decoded) =
                <LastBalanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::LastBalance(decoded));
            }
            if let Ok(decoded) =
                <MaxEdgesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::MaxEdges(decoded));
            }
            if let Ok (decoded) = < MaximumDepositAmountCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: MaximumDepositAmount (decoded)) }
            if let Ok (decoded) = < MinimalWithdrawalAmountCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: MinimalWithdrawalAmount (decoded)) }
            if let Ok(decoded) =
                <NeighborRootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::NeighborRoots(decoded));
            }
            if let Ok(decoded) =
                <NullifierHashesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::NullifierHashes(decoded));
            }
            if let Ok(decoded) =
                <OuterLevelsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::OuterLevels(decoded));
            }
            if let Ok (decoded) = < ParseChainIdFromResourceIdCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: ParseChainIdFromResourceId (decoded)) }
            if let Ok(decoded) =
                <ProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::ProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <RegisterCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Register(decoded));
            }
            if let Ok (decoded) = < RegisterAndTransactCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: RegisterAndTransact (decoded)) }
            if let Ok(decoded) =
                <RootsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Roots(decoded));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::SetHandler(decoded));
            }
            if let Ok(decoded) =
                <SetVerifierCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::SetVerifier(decoded));
            }
            if let Ok(decoded) =
                <TokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Token(decoded));
            }
            if let Ok(decoded) =
                <TransactCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Transact(decoded));
            }
            if let Ok(decoded) =
                <UnpackProofCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::UnpackProof(decoded));
            }
            if let Ok(decoded) =
                <UpdateEdgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::UpdateEdge(decoded));
            }
            if let Ok(decoded) =
                <VerifierCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Verifier(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for VAnchorContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                VAnchorContractCalls::EvmChainIdType(element) => {
                    element.encode()
                }
                VAnchorContractCalls::FieldSize(element) => element.encode(),
                VAnchorContractCalls::MaxExtAmount(element) => element.encode(),
                VAnchorContractCalls::MaxFee(element) => element.encode(),
                VAnchorContractCalls::RootHistorySize(element) => {
                    element.encode()
                }
                VAnchorContractCalls::ZeroValue(element) => element.encode(),
                VAnchorContractCalls::ExecuteWrapping(element) => {
                    element.encode()
                }
                VAnchorContractCalls::WithdrawAndUnwrap(element) => {
                    element.encode()
                }
                VAnchorContractCalls::CalculatePublicAmount(element) => {
                    element.encode()
                }
                VAnchorContractCalls::Commitments(element) => element.encode(),
                VAnchorContractCalls::ConfigureMaximumDepositLimit(element) => {
                    element.encode()
                }
                VAnchorContractCalls::ConfigureMinimalWithdrawalLimit(
                    element,
                ) => element.encode(),
                VAnchorContractCalls::CurrentNeighborRootIndex(element) => {
                    element.encode()
                }
                VAnchorContractCalls::EdgeExistsForChain(element) => {
                    element.encode()
                }
                VAnchorContractCalls::EdgeIndex(element) => element.encode(),
                VAnchorContractCalls::EdgeList(element) => element.encode(),
                VAnchorContractCalls::FilledSubtrees(element) => {
                    element.encode()
                }
                VAnchorContractCalls::GetChainId(element) => element.encode(),
                VAnchorContractCalls::GetChainIdType(element) => {
                    element.encode()
                }
                VAnchorContractCalls::GetHasher(element) => element.encode(),
                VAnchorContractCalls::GetLastRoot(element) => element.encode(),
                VAnchorContractCalls::GetLatestNeighborEdges(element) => {
                    element.encode()
                }
                VAnchorContractCalls::GetLatestNeighborRoots(element) => {
                    element.encode()
                }
                VAnchorContractCalls::GetLevels(element) => element.encode(),
                VAnchorContractCalls::GetNextIndex(element) => element.encode(),
                VAnchorContractCalls::GetProposalNonce(element) => {
                    element.encode()
                }
                VAnchorContractCalls::GetZeroHash(element) => element.encode(),
                VAnchorContractCalls::Handler(element) => element.encode(),
                VAnchorContractCalls::HasEdge(element) => element.encode(),
                VAnchorContractCalls::HashLeftRight(element) => {
                    element.encode()
                }
                VAnchorContractCalls::Initialize(element) => element.encode(),
                VAnchorContractCalls::Initialized(element) => element.encode(),
                VAnchorContractCalls::IsKnownNeighborRoot(element) => {
                    element.encode()
                }
                VAnchorContractCalls::IsKnownRoot(element) => element.encode(),
                VAnchorContractCalls::IsSpent(element) => element.encode(),
                VAnchorContractCalls::IsSpentArray(element) => element.encode(),
                VAnchorContractCalls::IsValidRoots(element) => element.encode(),
                VAnchorContractCalls::LastBalance(element) => element.encode(),
                VAnchorContractCalls::MaxEdges(element) => element.encode(),
                VAnchorContractCalls::MaximumDepositAmount(element) => {
                    element.encode()
                }
                VAnchorContractCalls::MinimalWithdrawalAmount(element) => {
                    element.encode()
                }
                VAnchorContractCalls::NeighborRoots(element) => {
                    element.encode()
                }
                VAnchorContractCalls::NullifierHashes(element) => {
                    element.encode()
                }
                VAnchorContractCalls::OuterLevels(element) => element.encode(),
                VAnchorContractCalls::ParseChainIdFromResourceId(element) => {
                    element.encode()
                }
                VAnchorContractCalls::ProposalNonce(element) => {
                    element.encode()
                }
                VAnchorContractCalls::Register(element) => element.encode(),
                VAnchorContractCalls::RegisterAndTransact(element) => {
                    element.encode()
                }
                VAnchorContractCalls::Roots(element) => element.encode(),
                VAnchorContractCalls::SetHandler(element) => element.encode(),
                VAnchorContractCalls::SetVerifier(element) => element.encode(),
                VAnchorContractCalls::Token(element) => element.encode(),
                VAnchorContractCalls::Transact(element) => element.encode(),
                VAnchorContractCalls::UnpackProof(element) => element.encode(),
                VAnchorContractCalls::UpdateEdge(element) => element.encode(),
                VAnchorContractCalls::Verifier(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VAnchorContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VAnchorContractCalls::EvmChainIdType(element) => element.fmt(f),
                VAnchorContractCalls::FieldSize(element) => element.fmt(f),
                VAnchorContractCalls::MaxExtAmount(element) => element.fmt(f),
                VAnchorContractCalls::MaxFee(element) => element.fmt(f),
                VAnchorContractCalls::RootHistorySize(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::ZeroValue(element) => element.fmt(f),
                VAnchorContractCalls::ExecuteWrapping(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::WithdrawAndUnwrap(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::CalculatePublicAmount(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::Commitments(element) => element.fmt(f),
                VAnchorContractCalls::ConfigureMaximumDepositLimit(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::ConfigureMinimalWithdrawalLimit(
                    element,
                ) => element.fmt(f),
                VAnchorContractCalls::CurrentNeighborRootIndex(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::EdgeExistsForChain(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::EdgeIndex(element) => element.fmt(f),
                VAnchorContractCalls::EdgeList(element) => element.fmt(f),
                VAnchorContractCalls::FilledSubtrees(element) => element.fmt(f),
                VAnchorContractCalls::GetChainId(element) => element.fmt(f),
                VAnchorContractCalls::GetChainIdType(element) => element.fmt(f),
                VAnchorContractCalls::GetHasher(element) => element.fmt(f),
                VAnchorContractCalls::GetLastRoot(element) => element.fmt(f),
                VAnchorContractCalls::GetLatestNeighborEdges(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::GetLatestNeighborRoots(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::GetLevels(element) => element.fmt(f),
                VAnchorContractCalls::GetNextIndex(element) => element.fmt(f),
                VAnchorContractCalls::GetProposalNonce(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::GetZeroHash(element) => element.fmt(f),
                VAnchorContractCalls::Handler(element) => element.fmt(f),
                VAnchorContractCalls::HasEdge(element) => element.fmt(f),
                VAnchorContractCalls::HashLeftRight(element) => element.fmt(f),
                VAnchorContractCalls::Initialize(element) => element.fmt(f),
                VAnchorContractCalls::Initialized(element) => element.fmt(f),
                VAnchorContractCalls::IsKnownNeighborRoot(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::IsKnownRoot(element) => element.fmt(f),
                VAnchorContractCalls::IsSpent(element) => element.fmt(f),
                VAnchorContractCalls::IsSpentArray(element) => element.fmt(f),
                VAnchorContractCalls::IsValidRoots(element) => element.fmt(f),
                VAnchorContractCalls::LastBalance(element) => element.fmt(f),
                VAnchorContractCalls::MaxEdges(element) => element.fmt(f),
                VAnchorContractCalls::MaximumDepositAmount(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::MinimalWithdrawalAmount(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::NeighborRoots(element) => element.fmt(f),
                VAnchorContractCalls::NullifierHashes(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::OuterLevels(element) => element.fmt(f),
                VAnchorContractCalls::ParseChainIdFromResourceId(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::ProposalNonce(element) => element.fmt(f),
                VAnchorContractCalls::Register(element) => element.fmt(f),
                VAnchorContractCalls::RegisterAndTransact(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::Roots(element) => element.fmt(f),
                VAnchorContractCalls::SetHandler(element) => element.fmt(f),
                VAnchorContractCalls::SetVerifier(element) => element.fmt(f),
                VAnchorContractCalls::Token(element) => element.fmt(f),
                VAnchorContractCalls::Transact(element) => element.fmt(f),
                VAnchorContractCalls::UnpackProof(element) => element.fmt(f),
                VAnchorContractCalls::UpdateEdge(element) => element.fmt(f),
                VAnchorContractCalls::Verifier(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<EvmChainIdTypeCall> for VAnchorContractCalls {
        fn from(var: EvmChainIdTypeCall) -> Self {
            VAnchorContractCalls::EvmChainIdType(var)
        }
    }
    impl ::std::convert::From<FieldSizeCall> for VAnchorContractCalls {
        fn from(var: FieldSizeCall) -> Self {
            VAnchorContractCalls::FieldSize(var)
        }
    }
    impl ::std::convert::From<MaxExtAmountCall> for VAnchorContractCalls {
        fn from(var: MaxExtAmountCall) -> Self {
            VAnchorContractCalls::MaxExtAmount(var)
        }
    }
    impl ::std::convert::From<MaxFeeCall> for VAnchorContractCalls {
        fn from(var: MaxFeeCall) -> Self {
            VAnchorContractCalls::MaxFee(var)
        }
    }
    impl ::std::convert::From<RootHistorySizeCall> for VAnchorContractCalls {
        fn from(var: RootHistorySizeCall) -> Self {
            VAnchorContractCalls::RootHistorySize(var)
        }
    }
    impl ::std::convert::From<ZeroValueCall> for VAnchorContractCalls {
        fn from(var: ZeroValueCall) -> Self {
            VAnchorContractCalls::ZeroValue(var)
        }
    }
    impl ::std::convert::From<ExecuteWrappingCall> for VAnchorContractCalls {
        fn from(var: ExecuteWrappingCall) -> Self {
            VAnchorContractCalls::ExecuteWrapping(var)
        }
    }
    impl ::std::convert::From<WithdrawAndUnwrapCall> for VAnchorContractCalls {
        fn from(var: WithdrawAndUnwrapCall) -> Self {
            VAnchorContractCalls::WithdrawAndUnwrap(var)
        }
    }
    impl ::std::convert::From<CalculatePublicAmountCall> for VAnchorContractCalls {
        fn from(var: CalculatePublicAmountCall) -> Self {
            VAnchorContractCalls::CalculatePublicAmount(var)
        }
    }
    impl ::std::convert::From<CommitmentsCall> for VAnchorContractCalls {
        fn from(var: CommitmentsCall) -> Self {
            VAnchorContractCalls::Commitments(var)
        }
    }
    impl ::std::convert::From<ConfigureMaximumDepositLimitCall>
        for VAnchorContractCalls
    {
        fn from(var: ConfigureMaximumDepositLimitCall) -> Self {
            VAnchorContractCalls::ConfigureMaximumDepositLimit(var)
        }
    }
    impl ::std::convert::From<ConfigureMinimalWithdrawalLimitCall>
        for VAnchorContractCalls
    {
        fn from(var: ConfigureMinimalWithdrawalLimitCall) -> Self {
            VAnchorContractCalls::ConfigureMinimalWithdrawalLimit(var)
        }
    }
    impl ::std::convert::From<CurrentNeighborRootIndexCall>
        for VAnchorContractCalls
    {
        fn from(var: CurrentNeighborRootIndexCall) -> Self {
            VAnchorContractCalls::CurrentNeighborRootIndex(var)
        }
    }
    impl ::std::convert::From<EdgeExistsForChainCall> for VAnchorContractCalls {
        fn from(var: EdgeExistsForChainCall) -> Self {
            VAnchorContractCalls::EdgeExistsForChain(var)
        }
    }
    impl ::std::convert::From<EdgeIndexCall> for VAnchorContractCalls {
        fn from(var: EdgeIndexCall) -> Self {
            VAnchorContractCalls::EdgeIndex(var)
        }
    }
    impl ::std::convert::From<EdgeListCall> for VAnchorContractCalls {
        fn from(var: EdgeListCall) -> Self {
            VAnchorContractCalls::EdgeList(var)
        }
    }
    impl ::std::convert::From<FilledSubtreesCall> for VAnchorContractCalls {
        fn from(var: FilledSubtreesCall) -> Self {
            VAnchorContractCalls::FilledSubtrees(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for VAnchorContractCalls {
        fn from(var: GetChainIdCall) -> Self {
            VAnchorContractCalls::GetChainId(var)
        }
    }
    impl ::std::convert::From<GetChainIdTypeCall> for VAnchorContractCalls {
        fn from(var: GetChainIdTypeCall) -> Self {
            VAnchorContractCalls::GetChainIdType(var)
        }
    }
    impl ::std::convert::From<GetHasherCall> for VAnchorContractCalls {
        fn from(var: GetHasherCall) -> Self {
            VAnchorContractCalls::GetHasher(var)
        }
    }
    impl ::std::convert::From<GetLastRootCall> for VAnchorContractCalls {
        fn from(var: GetLastRootCall) -> Self {
            VAnchorContractCalls::GetLastRoot(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborEdgesCall> for VAnchorContractCalls {
        fn from(var: GetLatestNeighborEdgesCall) -> Self {
            VAnchorContractCalls::GetLatestNeighborEdges(var)
        }
    }
    impl ::std::convert::From<GetLatestNeighborRootsCall> for VAnchorContractCalls {
        fn from(var: GetLatestNeighborRootsCall) -> Self {
            VAnchorContractCalls::GetLatestNeighborRoots(var)
        }
    }
    impl ::std::convert::From<GetLevelsCall> for VAnchorContractCalls {
        fn from(var: GetLevelsCall) -> Self {
            VAnchorContractCalls::GetLevels(var)
        }
    }
    impl ::std::convert::From<GetNextIndexCall> for VAnchorContractCalls {
        fn from(var: GetNextIndexCall) -> Self {
            VAnchorContractCalls::GetNextIndex(var)
        }
    }
    impl ::std::convert::From<GetProposalNonceCall> for VAnchorContractCalls {
        fn from(var: GetProposalNonceCall) -> Self {
            VAnchorContractCalls::GetProposalNonce(var)
        }
    }
    impl ::std::convert::From<GetZeroHashCall> for VAnchorContractCalls {
        fn from(var: GetZeroHashCall) -> Self {
            VAnchorContractCalls::GetZeroHash(var)
        }
    }
    impl ::std::convert::From<HandlerCall> for VAnchorContractCalls {
        fn from(var: HandlerCall) -> Self {
            VAnchorContractCalls::Handler(var)
        }
    }
    impl ::std::convert::From<HasEdgeCall> for VAnchorContractCalls {
        fn from(var: HasEdgeCall) -> Self {
            VAnchorContractCalls::HasEdge(var)
        }
    }
    impl ::std::convert::From<HashLeftRightCall> for VAnchorContractCalls {
        fn from(var: HashLeftRightCall) -> Self {
            VAnchorContractCalls::HashLeftRight(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for VAnchorContractCalls {
        fn from(var: InitializeCall) -> Self {
            VAnchorContractCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InitializedCall> for VAnchorContractCalls {
        fn from(var: InitializedCall) -> Self {
            VAnchorContractCalls::Initialized(var)
        }
    }
    impl ::std::convert::From<IsKnownNeighborRootCall> for VAnchorContractCalls {
        fn from(var: IsKnownNeighborRootCall) -> Self {
            VAnchorContractCalls::IsKnownNeighborRoot(var)
        }
    }
    impl ::std::convert::From<IsKnownRootCall> for VAnchorContractCalls {
        fn from(var: IsKnownRootCall) -> Self {
            VAnchorContractCalls::IsKnownRoot(var)
        }
    }
    impl ::std::convert::From<IsSpentCall> for VAnchorContractCalls {
        fn from(var: IsSpentCall) -> Self {
            VAnchorContractCalls::IsSpent(var)
        }
    }
    impl ::std::convert::From<IsSpentArrayCall> for VAnchorContractCalls {
        fn from(var: IsSpentArrayCall) -> Self {
            VAnchorContractCalls::IsSpentArray(var)
        }
    }
    impl ::std::convert::From<IsValidRootsCall> for VAnchorContractCalls {
        fn from(var: IsValidRootsCall) -> Self {
            VAnchorContractCalls::IsValidRoots(var)
        }
    }
    impl ::std::convert::From<LastBalanceCall> for VAnchorContractCalls {
        fn from(var: LastBalanceCall) -> Self {
            VAnchorContractCalls::LastBalance(var)
        }
    }
    impl ::std::convert::From<MaxEdgesCall> for VAnchorContractCalls {
        fn from(var: MaxEdgesCall) -> Self {
            VAnchorContractCalls::MaxEdges(var)
        }
    }
    impl ::std::convert::From<MaximumDepositAmountCall> for VAnchorContractCalls {
        fn from(var: MaximumDepositAmountCall) -> Self {
            VAnchorContractCalls::MaximumDepositAmount(var)
        }
    }
    impl ::std::convert::From<MinimalWithdrawalAmountCall>
        for VAnchorContractCalls
    {
        fn from(var: MinimalWithdrawalAmountCall) -> Self {
            VAnchorContractCalls::MinimalWithdrawalAmount(var)
        }
    }
    impl ::std::convert::From<NeighborRootsCall> for VAnchorContractCalls {
        fn from(var: NeighborRootsCall) -> Self {
            VAnchorContractCalls::NeighborRoots(var)
        }
    }
    impl ::std::convert::From<NullifierHashesCall> for VAnchorContractCalls {
        fn from(var: NullifierHashesCall) -> Self {
            VAnchorContractCalls::NullifierHashes(var)
        }
    }
    impl ::std::convert::From<OuterLevelsCall> for VAnchorContractCalls {
        fn from(var: OuterLevelsCall) -> Self {
            VAnchorContractCalls::OuterLevels(var)
        }
    }
    impl ::std::convert::From<ParseChainIdFromResourceIdCall>
        for VAnchorContractCalls
    {
        fn from(var: ParseChainIdFromResourceIdCall) -> Self {
            VAnchorContractCalls::ParseChainIdFromResourceId(var)
        }
    }
    impl ::std::convert::From<ProposalNonceCall> for VAnchorContractCalls {
        fn from(var: ProposalNonceCall) -> Self {
            VAnchorContractCalls::ProposalNonce(var)
        }
    }
    impl ::std::convert::From<RegisterCall> for VAnchorContractCalls {
        fn from(var: RegisterCall) -> Self {
            VAnchorContractCalls::Register(var)
        }
    }
    impl ::std::convert::From<RegisterAndTransactCall> for VAnchorContractCalls {
        fn from(var: RegisterAndTransactCall) -> Self {
            VAnchorContractCalls::RegisterAndTransact(var)
        }
    }
    impl ::std::convert::From<RootsCall> for VAnchorContractCalls {
        fn from(var: RootsCall) -> Self {
            VAnchorContractCalls::Roots(var)
        }
    }
    impl ::std::convert::From<SetHandlerCall> for VAnchorContractCalls {
        fn from(var: SetHandlerCall) -> Self {
            VAnchorContractCalls::SetHandler(var)
        }
    }
    impl ::std::convert::From<SetVerifierCall> for VAnchorContractCalls {
        fn from(var: SetVerifierCall) -> Self {
            VAnchorContractCalls::SetVerifier(var)
        }
    }
    impl ::std::convert::From<TokenCall> for VAnchorContractCalls {
        fn from(var: TokenCall) -> Self {
            VAnchorContractCalls::Token(var)
        }
    }
    impl ::std::convert::From<TransactCall> for VAnchorContractCalls {
        fn from(var: TransactCall) -> Self {
            VAnchorContractCalls::Transact(var)
        }
    }
    impl ::std::convert::From<UnpackProofCall> for VAnchorContractCalls {
        fn from(var: UnpackProofCall) -> Self {
            VAnchorContractCalls::UnpackProof(var)
        }
    }
    impl ::std::convert::From<UpdateEdgeCall> for VAnchorContractCalls {
        fn from(var: UpdateEdgeCall) -> Self {
            VAnchorContractCalls::UpdateEdge(var)
        }
    }
    impl ::std::convert::From<VerifierCall> for VAnchorContractCalls {
        fn from(var: VerifierCall) -> Self {
            VAnchorContractCalls::Verifier(var)
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
    #[doc = "Container type for all return fields from the `_executeWrapping` function with signature `_executeWrapping(address,address,uint256)` and selector `[99, 56, 188, 188]`"]
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
    pub struct ExecuteWrappingReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `calculatePublicAmount` function with signature `calculatePublicAmount(int256,uint256)` and selector `[37, 112, 183, 180]`"]
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
    pub struct CalculatePublicAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `commitments` function with signature `commitments(uint256)` and selector `[73, 206, 137, 151]`"]
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
        pub root: ethers::core::types::U256,
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
    pub struct FilledSubtreesReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `getHasher` function with signature `getHasher()` and selector `[234, 73, 93, 176]`"]
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
    pub struct GetHasherReturn(pub ethers::core::types::Address);
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
    pub struct GetLastRootReturn(pub ethers::core::types::U256);
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
            ethers::core::types::U256,
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
    pub struct GetLatestNeighborRootsReturn(
        pub ::std::vec::Vec<ethers::core::types::U256>,
    );
    #[doc = "Container type for all return fields from the `getLevels` function with signature `getLevels()` and selector `[12, 57, 74, 96]`"]
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
    pub struct GetLevelsReturn(pub u32);
    #[doc = "Container type for all return fields from the `getNextIndex` function with signature `getNextIndex()` and selector `[14, 183, 96, 111]`"]
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
    pub struct GetNextIndexReturn(pub u32);
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
    pub struct GetProposalNonceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getZeroHash` function with signature `getZeroHash(uint32)` and selector `[48, 94, 158, 172]`"]
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
    pub struct GetZeroHashReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `hashLeftRight` function with signature `hashLeftRight(uint256,uint256)` and selector `[91, 185, 57, 149]`"]
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
    pub struct HashLeftRightReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `initialized` function with signature `initialized()` and selector `[21, 142, 249, 62]`"]
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
    pub struct InitializedReturn(pub bool);
    #[doc = "Container type for all return fields from the `isKnownNeighborRoot` function with signature `isKnownNeighborRoot(uint256,uint256)` and selector `[59, 250, 141, 122]`"]
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
    #[doc = "Container type for all return fields from the `isKnownRoot` function with signature `isKnownRoot(uint256)` and selector `[166, 35, 42, 147]`"]
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
    #[doc = "Container type for all return fields from the `isSpent` function with signature `isSpent(uint256)` and selector `[90, 18, 158, 254]`"]
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
    #[doc = "Container type for all return fields from the `isSpentArray` function with signature `isSpentArray(uint256[])` and selector `[234, 101, 186, 73]`"]
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
    #[doc = "Container type for all return fields from the `isValidRoots` function with signature `isValidRoots(uint256[])` and selector `[183, 94, 103, 152]`"]
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
    #[doc = "Container type for all return fields from the `maxEdges` function with signature `maxEdges()` and selector `[113, 82, 60, 50]`"]
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
    pub struct MaxEdgesReturn(pub u8);
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
    pub struct NeighborRootsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `nullifierHashes` function with signature `nullifierHashes(uint256)` and selector `[31, 121, 161, 233]`"]
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
    #[doc = "Container type for all return fields from the `outerLevels` function with signature `outerLevels()` and selector `[191, 188, 10, 57]`"]
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
    pub struct OuterLevelsReturn(pub u32);
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
    #[doc = "Container type for all return fields from the `proposalNonce` function with signature `proposalNonce()` and selector `[204, 60, 116, 161]`"]
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
    pub struct ProposalNonceReturn(pub ethers::core::types::U256);
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
        pub root: ethers::core::types::U256,
        pub latest_leafindex: u32,
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
    #[doc = "Container type for all return fields from the `unpackProof` function with signature `unpackProof(uint256[8])` and selector `[245, 171, 13, 214]`"]
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
    pub struct UnpackProofReturn(
        pub [ethers::core::types::U256; 2usize],
        pub [[ethers::core::types::U256; 2usize]; 2usize],
        pub [ethers::core::types::U256; 2usize],
    );
    #[doc = "Container type for all return fields from the `verifier` function with signature `verifier()` and selector `[43, 122, 195, 243]`"]
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
    pub struct VerifierReturn(pub ethers::core::types::Address);
    #[doc = "`CommonExtData(address,int256,address,uint256,uint256,address)`"]
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
    pub struct CommonExtData {
        pub recipient: ethers::core::types::Address,
        pub ext_amount: I256,
        pub relayer: ethers::core::types::Address,
        pub fee: ethers::core::types::U256,
        pub refund: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
    #[doc = "`Edge(uint256,uint256,uint256,bytes32)`"]
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
        pub root: ethers::core::types::U256,
        pub latest_leaf_index: ethers::core::types::U256,
        pub src_resource_id: [u8; 32],
    }
    #[doc = "`Encryptions(bytes,bytes)`"]
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
    pub struct Encryptions {
        pub encrypted_output_1: ethers::core::types::Bytes,
        pub encrypted_output_2: ethers::core::types::Bytes,
    }
    #[doc = "`PublicInputs(bytes,bytes,uint256[],uint256[2],uint256,uint256)`"]
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
    pub struct PublicInputs {
        pub roots: ethers::core::types::Bytes,
        pub extension_roots: ethers::core::types::Bytes,
        pub input_nullifiers: Vec<ethers::core::types::U256>,
        pub output_commitments: [ethers::core::types::U256; 2],
        pub public_amount: ethers::core::types::U256,
        pub ext_data_hash: ethers::core::types::U256,
    }
    #[doc = "`Account(address,bytes)`"]
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
    pub struct Account {
        pub owner: ethers::core::types::Address,
        pub key_data: ethers::core::types::Bytes,
    }
}
