pub use vanchorcontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod vanchorcontract_mod {
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
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"contract IAnchorVerifier\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"_levels\",\"type\":\"uint32\"},{\"internalType\":\"contract IPoseidonT3\",\"name\":\"_hasher\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\"},{\"internalType\":\"uint8\",\"name\":\"_maxEdges\",\"type\":\"uint8\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\"}],\"name\":\"EdgeAddition\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"merkleRoot\",\"type\":\"bytes32\"}],\"name\":\"EdgeUpdate\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint32\",\"name\":\"leafIndex\",\"type\":\"uint32\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\"}],\"name\":\"Insertion\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"encryptedOutput\",\"type\":\"bytes\"}],\"name\":\"NewCommitment\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"nullifier\",\"type\":\"bytes32\"}],\"name\":\"NewNullifier\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bytes\",\"name\":\"key\",\"type\":\"bytes\"}],\"name\":\"PublicKey\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"EVM_CHAIN_ID_TYPE\",\"outputs\":[{\"internalType\":\"bytes2\",\"name\":\"\",\"type\":\"bytes2\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"FIELD_SIZE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MAX_EXT_AMOUNT\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MAX_FEE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ROOT_HISTORY_SIZE\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ZERO_VALUE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_extAmount\",\"type\":\"uint256\"}],\"name\":\"_executeWrapping\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"_extAmount\",\"type\":\"int256\"},{\"internalType\":\"uint256\",\"name\":\"_fee\",\"type\":\"uint256\"}],\"name\":\"calculatePublicAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\"}],\"name\":\"configureMaximumDepositLimit\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\"}],\"name\":\"configureMinimalWithdrawalLimit\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"currentNeighborRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"currentRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeExistsForChain\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"edgeList\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"target\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"filledSubtrees\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getChainIdType\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getLastRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getLatestNeighborEdges\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"latestLeafIndex\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"target\",\"type\":\"bytes32\"}],\"internalType\":\"struct LinkableTree.Edge[]\",\"name\":\"\",\"type\":\"tuple[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getLatestNeighborRoots\",\"outputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"\",\"type\":\"bytes32[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getProposalNonce\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"handler\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_chainID\",\"type\":\"uint256\"}],\"name\":\"hasEdge\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract IPoseidonT3\",\"name\":\"_hasher\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"_left\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_right\",\"type\":\"bytes32\"}],\"name\":\"hashLeftRight\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"hasher\",\"outputs\":[{\"internalType\":\"contract IPoseidonT3\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_minimalWithdrawalAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_maximumDepositAmount\",\"type\":\"uint256\"}],\"name\":\"initialize\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_neighborChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"}],\"name\":\"isKnownNeighborRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"}],\"name\":\"isKnownRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\"}],\"name\":\"isSpent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_nullifierHashes\",\"type\":\"bytes32[]\"}],\"name\":\"isSpentArray\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"\",\"type\":\"bool[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_roots\",\"type\":\"bytes32[]\"}],\"name\":\"isValidRoots\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"lastBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"levels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"maxEdges\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"maximumDepositAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"minimalWithdrawalAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"name\":\"neighborRoots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"nextIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"nullifierHashes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"publicKey\",\"type\":\"bytes\"}],\"internalType\":\"struct VAnchorBase.Account\",\"name\":\"_account\",\"type\":\"tuple\"}],\"name\":\"register\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"publicKey\",\"type\":\"bytes\"}],\"internalType\":\"struct VAnchorBase.Account\",\"name\":\"_account\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"roots\",\"type\":\"bytes\"},{\"internalType\":\"bytes32[]\",\"name\":\"inputNullifiers\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes32[2]\",\"name\":\"outputCommitments\",\"type\":\"bytes32[2]\"},{\"internalType\":\"uint256\",\"name\":\"publicAmount\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"extDataHash\",\"type\":\"bytes32\"}],\"internalType\":\"struct VAnchorEncodeInputs.Proof\",\"name\":\"_proofArgs\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"int256\",\"name\":\"extAmount\",\"type\":\"int256\"},{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput1\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput2\",\"type\":\"bytes\"}],\"internalType\":\"struct VAnchorBase.ExtData\",\"name\":\"_extData\",\"type\":\"tuple\"}],\"name\":\"registerAndTransact\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"publicKey\",\"type\":\"bytes\"}],\"internalType\":\"struct VAnchorBase.Account\",\"name\":\"_account\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"roots\",\"type\":\"bytes\"},{\"internalType\":\"bytes32[]\",\"name\":\"inputNullifiers\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes32[2]\",\"name\":\"outputCommitments\",\"type\":\"bytes32[2]\"},{\"internalType\":\"uint256\",\"name\":\"publicAmount\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"extDataHash\",\"type\":\"bytes32\"}],\"internalType\":\"struct VAnchorEncodeInputs.Proof\",\"name\":\"_proofArgs\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"int256\",\"name\":\"extAmount\",\"type\":\"int256\"},{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput1\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput2\",\"type\":\"bytes\"}],\"internalType\":\"struct VAnchorBase.ExtData\",\"name\":\"_extData\",\"type\":\"tuple\"},{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\"}],\"name\":\"registerAndTransactWrap\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"roots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\"}],\"name\":\"setHandler\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_verifier\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\"}],\"name\":\"setVerifier\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"token\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"roots\",\"type\":\"bytes\"},{\"internalType\":\"bytes32[]\",\"name\":\"inputNullifiers\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes32[2]\",\"name\":\"outputCommitments\",\"type\":\"bytes32[2]\"},{\"internalType\":\"uint256\",\"name\":\"publicAmount\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"extDataHash\",\"type\":\"bytes32\"}],\"internalType\":\"struct VAnchorEncodeInputs.Proof\",\"name\":\"_args\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"int256\",\"name\":\"extAmount\",\"type\":\"int256\"},{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput1\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput2\",\"type\":\"bytes\"}],\"internalType\":\"struct VAnchorBase.ExtData\",\"name\":\"_extData\",\"type\":\"tuple\"}],\"name\":\"transact\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"bytes\",\"name\":\"proof\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"roots\",\"type\":\"bytes\"},{\"internalType\":\"bytes32[]\",\"name\":\"inputNullifiers\",\"type\":\"bytes32[]\"},{\"internalType\":\"bytes32[2]\",\"name\":\"outputCommitments\",\"type\":\"bytes32[2]\"},{\"internalType\":\"uint256\",\"name\":\"publicAmount\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"extDataHash\",\"type\":\"bytes32\"}],\"internalType\":\"struct VAnchorEncodeInputs.Proof\",\"name\":\"_args\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"int256\",\"name\":\"extAmount\",\"type\":\"int256\"},{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput1\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"encryptedOutput2\",\"type\":\"bytes\"}],\"internalType\":\"struct VAnchorBase.ExtData\",\"name\":\"_extData\",\"type\":\"tuple\"},{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\"}],\"name\":\"transactWrap\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256[8]\",\"name\":\"_proof\",\"type\":\"uint256[8]\"}],\"name\":\"unpackProof\",\"outputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\"},{\"internalType\":\"uint256[2][2]\",\"name\":\"\",\"type\":\"uint256[2][2]\"},{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"unwrapIntoNative\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"unwrapIntoToken\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_sourceChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"_leafIndex\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_target\",\"type\":\"bytes32\"}],\"name\":\"updateEdge\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"verifier\",\"outputs\":[{\"internalType\":\"contract IAnchorVerifier\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_minusExtAmount\",\"type\":\"uint256\"}],\"name\":\"withdrawAndUnwrap\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"wrapNative\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"wrapToken\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"}],\"name\":\"zeros\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct VAnchorContract<M>(ethers::contract::Contract<M>);
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
    impl<'a, M: ethers::providers::Middleware> VAnchorContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                VANCHORCONTRACT_ABI.clone(),
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
            ext_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 238, 224, 96], (token_address, ext_amount))
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
        #[doc = "Calls the contract's `commitments` (0x839df945) function"]
        pub fn commitments(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([131, 157, 249, 69], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `configureMaximumDepositLimit` (0xdad878a5) function"]
        pub fn configure_maximum_deposit_limit(
            &self,
            maximum_deposit_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 216, 120, 165], maximum_deposit_amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `configureMinimalWithdrawalLimit` (0x2d48470c) function"]
        pub fn configure_minimal_withdrawal_limit(
            &self,
            minimal_withdrawal_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 72, 71, 12], minimal_withdrawal_amount)
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
        #[doc = "Calls the contract's `register` (0xb2bc6e0f) function"]
        pub fn register(
            &self,
            account: Account,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 188, 110, 15], (account,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerAndTransact` (0xc9be7250) function"]
        pub fn register_and_transact(
            &self,
            account: Account,
            proof_args: Proof,
            ext_data: ExtData,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 190, 114, 80],
                    (account, proof_args, ext_data),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerAndTransactWrap` (0xb7566a67) function"]
        pub fn register_and_transact_wrap(
            &self,
            account: Account,
            proof_args: Proof,
            ext_data: ExtData,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [183, 86, 106, 103],
                    (account, proof_args, ext_data, token_address),
                )
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
        #[doc = "Calls the contract's `transact` (0x9bbca3a9) function"]
        pub fn transact(
            &self,
            args: Proof,
            ext_data: ExtData,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 188, 163, 169], (args, ext_data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transactWrap` (0x4167bb1e) function"]
        pub fn transact_wrap(
            &self,
            args: Proof,
            ext_data: ExtData,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [65, 103, 187, 30],
                    (args, ext_data, token_address),
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
        #[doc = "Calls the contract's `withdrawAndUnwrap` (0x95c87d1a) function"]
        pub fn withdraw_and_unwrap(
            &self,
            token_address: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            minus_ext_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [149, 200, 125, 26],
                    (token_address, recipient, minus_ext_amount),
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
        abi = "NewCommitment(bytes32,uint256,bytes)"
    )]
    pub struct NewCommitmentFilter {
        pub commitment: [u8; 32],
        pub index: ethers::core::types::U256,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
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
    #[doc = "Container type for all input parameters for the `MAX_EXT_AMOUNT`function with signature `MAX_EXT_AMOUNT()` and selector `[127, 226, 79, 254]`"]
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
    #[doc = "Container type for all input parameters for the `MAX_FEE`function with signature `MAX_FEE()` and selector `[188, 6, 62, 26]`"]
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
    #[doc = "Container type for all input parameters for the `_executeWrapping`function with signature `_executeWrapping(address,uint256)` and selector `[148, 238, 224, 96]`"]
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
        pub ext_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `calculatePublicAmount`function with signature `calculatePublicAmount(int256,uint256)` and selector `[37, 112, 183, 180]`"]
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
    #[doc = "Container type for all input parameters for the `configureMaximumDepositLimit`function with signature `configureMaximumDepositLimit(uint256)` and selector `[218, 216, 120, 165]`"]
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
        abi = "configureMaximumDepositLimit(uint256)"
    )]
    pub struct ConfigureMaximumDepositLimitCall {
        pub maximum_deposit_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `configureMinimalWithdrawalLimit`function with signature `configureMinimalWithdrawalLimit(uint256)` and selector `[45, 72, 71, 12]`"]
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
        abi = "configureMinimalWithdrawalLimit(uint256)"
    )]
    pub struct ConfigureMinimalWithdrawalLimitCall {
        pub minimal_withdrawal_amount: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(uint256,uint256)` and selector `[228, 163, 1, 22]`"]
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
    #[doc = "Container type for all input parameters for the `lastBalance`function with signature `lastBalance()` and selector `[143, 28, 86, 189]`"]
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
    #[doc = "Container type for all input parameters for the `maximumDepositAmount`function with signature `maximumDepositAmount()` and selector `[120, 171, 180, 155]`"]
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
    #[doc = "Container type for all input parameters for the `minimalWithdrawalAmount`function with signature `minimalWithdrawalAmount()` and selector `[132, 11, 39, 145]`"]
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
    #[doc = "Container type for all input parameters for the `register`function with signature `register((address,bytes))` and selector `[178, 188, 110, 15]`"]
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
    #[doc = "Container type for all input parameters for the `registerAndTransact`function with signature `registerAndTransact((address,bytes),(bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes))` and selector `[201, 190, 114, 80]`"]
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
        abi = "registerAndTransact((address,bytes),(bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes))"
    )]
    pub struct RegisterAndTransactCall {
        pub account: Account,
        pub proof_args: Proof,
        pub ext_data: ExtData,
    }
    #[doc = "Container type for all input parameters for the `registerAndTransactWrap`function with signature `registerAndTransactWrap((address,bytes),(bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes),address)` and selector `[183, 86, 106, 103]`"]
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
        name = "registerAndTransactWrap",
        abi = "registerAndTransactWrap((address,bytes),(bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes),address)"
    )]
    pub struct RegisterAndTransactWrapCall {
        pub account: Account,
        pub proof_args: Proof,
        pub ext_data: ExtData,
        pub token_address: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `token`function with signature `token()` and selector `[252, 12, 84, 106]`"]
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
    #[doc = "Container type for all input parameters for the `transact`function with signature `transact((bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes))` and selector `[155, 188, 163, 169]`"]
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
        abi = "transact((bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes))"
    )]
    pub struct TransactCall {
        pub args: Proof,
        pub ext_data: ExtData,
    }
    #[doc = "Container type for all input parameters for the `transactWrap`function with signature `transactWrap((bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes),address)` and selector `[65, 103, 187, 30]`"]
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
        name = "transactWrap",
        abi = "transactWrap((bytes,bytes,bytes32[],bytes32[2],uint256,bytes32),(address,int256,address,uint256,bytes,bytes),address)"
    )]
    pub struct TransactWrapCall {
        pub args: Proof,
        pub ext_data: ExtData,
        pub token_address: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `unwrapIntoNative`function with signature `unwrapIntoNative(address,uint256)` and selector `[159, 248, 0, 99]`"]
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
    #[doc = "Container type for all input parameters for the `unwrapIntoToken`function with signature `unwrapIntoToken(address,uint256)` and selector `[79, 64, 18, 65]`"]
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
    #[doc = "Container type for all input parameters for the `withdrawAndUnwrap`function with signature `withdrawAndUnwrap(address,address,uint256)` and selector `[149, 200, 125, 26]`"]
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
        abi = "withdrawAndUnwrap(address,address,uint256)"
    )]
    pub struct WithdrawAndUnwrapCall {
        pub token_address: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub minus_ext_amount: ethers::core::types::U256,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "zeros", abi = "zeros(uint256)")]
    pub struct ZerosCall {
        pub i: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum VAnchorContractCalls {
        EvmChainIdType(EvmChainIdTypeCall),
        FieldSize(FieldSizeCall),
        MaxExtAmount(MaxExtAmountCall),
        MaxFee(MaxFeeCall),
        RootHistorySize(RootHistorySizeCall),
        ZeroValue(ZeroValueCall),
        ExecuteWrapping(ExecuteWrappingCall),
        CalculatePublicAmount(CalculatePublicAmountCall),
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
        MaxEdges(MaxEdgesCall),
        MaximumDepositAmount(MaximumDepositAmountCall),
        MinimalWithdrawalAmount(MinimalWithdrawalAmountCall),
        NeighborRoots(NeighborRootsCall),
        NextIndex(NextIndexCall),
        NullifierHashes(NullifierHashesCall),
        Register(RegisterCall),
        RegisterAndTransact(RegisterAndTransactCall),
        RegisterAndTransactWrap(RegisterAndTransactWrapCall),
        Roots(RootsCall),
        SetHandler(SetHandlerCall),
        SetVerifier(SetVerifierCall),
        Token(TokenCall),
        Transact(TransactCall),
        TransactWrap(TransactWrapCall),
        UnpackProof(UnpackProofCall),
        UnwrapIntoNative(UnwrapIntoNativeCall),
        UnwrapIntoToken(UnwrapIntoTokenCall),
        UpdateEdge(UpdateEdgeCall),
        Verifier(VerifierCall),
        WithdrawAndUnwrap(WithdrawAndUnwrapCall),
        WrapNative(WrapNativeCall),
        WrapToken(WrapTokenCall),
        Zeros(ZerosCall),
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
                <CurrentRootIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::CurrentRootIndex(decoded));
            }
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
                <GetLastRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::GetLastRoot(decoded));
            }
            if let Ok (decoded) = < GetLatestNeighborEdgesCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: GetLatestNeighborEdges (decoded)) }
            if let Ok (decoded) = < GetLatestNeighborRootsCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: GetLatestNeighborRoots (decoded)) }
            if let Ok(decoded) =
                <GetProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::GetProposalNonce(decoded));
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
                <HasherCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Hasher(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Initialize(decoded));
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
                <LevelsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Levels(decoded));
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
                <NextIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::NextIndex(decoded));
            }
            if let Ok(decoded) =
                <NullifierHashesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::NullifierHashes(decoded));
            }
            if let Ok(decoded) =
                <RegisterCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Register(decoded));
            }
            if let Ok (decoded) = < RegisterAndTransactCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: RegisterAndTransact (decoded)) }
            if let Ok (decoded) = < RegisterAndTransactWrapCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (VAnchorContractCalls :: RegisterAndTransactWrap (decoded)) }
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
                <TransactWrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::TransactWrap(decoded));
            }
            if let Ok(decoded) =
                <UnpackProofCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::UnpackProof(decoded));
            }
            if let Ok(decoded) =
                <UnwrapIntoNativeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::UnwrapIntoNative(decoded));
            }
            if let Ok(decoded) =
                <UnwrapIntoTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::UnwrapIntoToken(decoded));
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
            if let Ok(decoded) =
                <WithdrawAndUnwrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::WithdrawAndUnwrap(decoded));
            }
            if let Ok(decoded) =
                <WrapNativeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::WrapNative(decoded));
            }
            if let Ok(decoded) =
                <WrapTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::WrapToken(decoded));
            }
            if let Ok(decoded) =
                <ZerosCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(VAnchorContractCalls::Zeros(decoded));
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
                VAnchorContractCalls::CurrentRootIndex(element) => {
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
                VAnchorContractCalls::GetLastRoot(element) => element.encode(),
                VAnchorContractCalls::GetLatestNeighborEdges(element) => {
                    element.encode()
                }
                VAnchorContractCalls::GetLatestNeighborRoots(element) => {
                    element.encode()
                }
                VAnchorContractCalls::GetProposalNonce(element) => {
                    element.encode()
                }
                VAnchorContractCalls::Handler(element) => element.encode(),
                VAnchorContractCalls::HasEdge(element) => element.encode(),
                VAnchorContractCalls::HashLeftRight(element) => {
                    element.encode()
                }
                VAnchorContractCalls::Hasher(element) => element.encode(),
                VAnchorContractCalls::Initialize(element) => element.encode(),
                VAnchorContractCalls::IsKnownNeighborRoot(element) => {
                    element.encode()
                }
                VAnchorContractCalls::IsKnownRoot(element) => element.encode(),
                VAnchorContractCalls::IsSpent(element) => element.encode(),
                VAnchorContractCalls::IsSpentArray(element) => element.encode(),
                VAnchorContractCalls::IsValidRoots(element) => element.encode(),
                VAnchorContractCalls::LastBalance(element) => element.encode(),
                VAnchorContractCalls::Levels(element) => element.encode(),
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
                VAnchorContractCalls::NextIndex(element) => element.encode(),
                VAnchorContractCalls::NullifierHashes(element) => {
                    element.encode()
                }
                VAnchorContractCalls::Register(element) => element.encode(),
                VAnchorContractCalls::RegisterAndTransact(element) => {
                    element.encode()
                }
                VAnchorContractCalls::RegisterAndTransactWrap(element) => {
                    element.encode()
                }
                VAnchorContractCalls::Roots(element) => element.encode(),
                VAnchorContractCalls::SetHandler(element) => element.encode(),
                VAnchorContractCalls::SetVerifier(element) => element.encode(),
                VAnchorContractCalls::Token(element) => element.encode(),
                VAnchorContractCalls::Transact(element) => element.encode(),
                VAnchorContractCalls::TransactWrap(element) => element.encode(),
                VAnchorContractCalls::UnpackProof(element) => element.encode(),
                VAnchorContractCalls::UnwrapIntoNative(element) => {
                    element.encode()
                }
                VAnchorContractCalls::UnwrapIntoToken(element) => {
                    element.encode()
                }
                VAnchorContractCalls::UpdateEdge(element) => element.encode(),
                VAnchorContractCalls::Verifier(element) => element.encode(),
                VAnchorContractCalls::WithdrawAndUnwrap(element) => {
                    element.encode()
                }
                VAnchorContractCalls::WrapNative(element) => element.encode(),
                VAnchorContractCalls::WrapToken(element) => element.encode(),
                VAnchorContractCalls::Zeros(element) => element.encode(),
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
                VAnchorContractCalls::CurrentRootIndex(element) => {
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
                VAnchorContractCalls::GetLastRoot(element) => element.fmt(f),
                VAnchorContractCalls::GetLatestNeighborEdges(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::GetLatestNeighborRoots(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::GetProposalNonce(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::Handler(element) => element.fmt(f),
                VAnchorContractCalls::HasEdge(element) => element.fmt(f),
                VAnchorContractCalls::HashLeftRight(element) => element.fmt(f),
                VAnchorContractCalls::Hasher(element) => element.fmt(f),
                VAnchorContractCalls::Initialize(element) => element.fmt(f),
                VAnchorContractCalls::IsKnownNeighborRoot(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::IsKnownRoot(element) => element.fmt(f),
                VAnchorContractCalls::IsSpent(element) => element.fmt(f),
                VAnchorContractCalls::IsSpentArray(element) => element.fmt(f),
                VAnchorContractCalls::IsValidRoots(element) => element.fmt(f),
                VAnchorContractCalls::LastBalance(element) => element.fmt(f),
                VAnchorContractCalls::Levels(element) => element.fmt(f),
                VAnchorContractCalls::MaxEdges(element) => element.fmt(f),
                VAnchorContractCalls::MaximumDepositAmount(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::MinimalWithdrawalAmount(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::NeighborRoots(element) => element.fmt(f),
                VAnchorContractCalls::NextIndex(element) => element.fmt(f),
                VAnchorContractCalls::NullifierHashes(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::Register(element) => element.fmt(f),
                VAnchorContractCalls::RegisterAndTransact(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::RegisterAndTransactWrap(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::Roots(element) => element.fmt(f),
                VAnchorContractCalls::SetHandler(element) => element.fmt(f),
                VAnchorContractCalls::SetVerifier(element) => element.fmt(f),
                VAnchorContractCalls::Token(element) => element.fmt(f),
                VAnchorContractCalls::Transact(element) => element.fmt(f),
                VAnchorContractCalls::TransactWrap(element) => element.fmt(f),
                VAnchorContractCalls::UnpackProof(element) => element.fmt(f),
                VAnchorContractCalls::UnwrapIntoNative(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::UnwrapIntoToken(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::UpdateEdge(element) => element.fmt(f),
                VAnchorContractCalls::Verifier(element) => element.fmt(f),
                VAnchorContractCalls::WithdrawAndUnwrap(element) => {
                    element.fmt(f)
                }
                VAnchorContractCalls::WrapNative(element) => element.fmt(f),
                VAnchorContractCalls::WrapToken(element) => element.fmt(f),
                VAnchorContractCalls::Zeros(element) => element.fmt(f),
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
    impl ::std::convert::From<CurrentRootIndexCall> for VAnchorContractCalls {
        fn from(var: CurrentRootIndexCall) -> Self {
            VAnchorContractCalls::CurrentRootIndex(var)
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
    impl ::std::convert::From<GetProposalNonceCall> for VAnchorContractCalls {
        fn from(var: GetProposalNonceCall) -> Self {
            VAnchorContractCalls::GetProposalNonce(var)
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
    impl ::std::convert::From<HasherCall> for VAnchorContractCalls {
        fn from(var: HasherCall) -> Self {
            VAnchorContractCalls::Hasher(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for VAnchorContractCalls {
        fn from(var: InitializeCall) -> Self {
            VAnchorContractCalls::Initialize(var)
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
    impl ::std::convert::From<LevelsCall> for VAnchorContractCalls {
        fn from(var: LevelsCall) -> Self {
            VAnchorContractCalls::Levels(var)
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
    impl ::std::convert::From<NextIndexCall> for VAnchorContractCalls {
        fn from(var: NextIndexCall) -> Self {
            VAnchorContractCalls::NextIndex(var)
        }
    }
    impl ::std::convert::From<NullifierHashesCall> for VAnchorContractCalls {
        fn from(var: NullifierHashesCall) -> Self {
            VAnchorContractCalls::NullifierHashes(var)
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
    impl ::std::convert::From<RegisterAndTransactWrapCall>
        for VAnchorContractCalls
    {
        fn from(var: RegisterAndTransactWrapCall) -> Self {
            VAnchorContractCalls::RegisterAndTransactWrap(var)
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
    impl ::std::convert::From<TransactWrapCall> for VAnchorContractCalls {
        fn from(var: TransactWrapCall) -> Self {
            VAnchorContractCalls::TransactWrap(var)
        }
    }
    impl ::std::convert::From<UnpackProofCall> for VAnchorContractCalls {
        fn from(var: UnpackProofCall) -> Self {
            VAnchorContractCalls::UnpackProof(var)
        }
    }
    impl ::std::convert::From<UnwrapIntoNativeCall> for VAnchorContractCalls {
        fn from(var: UnwrapIntoNativeCall) -> Self {
            VAnchorContractCalls::UnwrapIntoNative(var)
        }
    }
    impl ::std::convert::From<UnwrapIntoTokenCall> for VAnchorContractCalls {
        fn from(var: UnwrapIntoTokenCall) -> Self {
            VAnchorContractCalls::UnwrapIntoToken(var)
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
    impl ::std::convert::From<WithdrawAndUnwrapCall> for VAnchorContractCalls {
        fn from(var: WithdrawAndUnwrapCall) -> Self {
            VAnchorContractCalls::WithdrawAndUnwrap(var)
        }
    }
    impl ::std::convert::From<WrapNativeCall> for VAnchorContractCalls {
        fn from(var: WrapNativeCall) -> Self {
            VAnchorContractCalls::WrapNative(var)
        }
    }
    impl ::std::convert::From<WrapTokenCall> for VAnchorContractCalls {
        fn from(var: WrapTokenCall) -> Self {
            VAnchorContractCalls::WrapToken(var)
        }
    }
    impl ::std::convert::From<ZerosCall> for VAnchorContractCalls {
        fn from(var: ZerosCall) -> Self {
            VAnchorContractCalls::Zeros(var)
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
    #[doc = "`Account(address,bytes)`"]
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
    pub struct Account {
        pub owner: ethers::core::types::Address,
        pub public_key: ethers::core::types::Bytes,
    }
    #[doc = "`ExtData(address,int256,address,uint256,bytes,bytes)`"]
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
    pub struct ExtData {
        pub recipient: ethers::core::types::Address,
        pub ext_amount: I256,
        pub relayer: ethers::core::types::Address,
        pub fee: ethers::core::types::U256,
        pub encrypted_output_1: ethers::core::types::Bytes,
        pub encrypted_output_2: ethers::core::types::Bytes,
    }
    #[doc = "`Proof(bytes,bytes,bytes32[],bytes32[2],uint256,bytes32)`"]
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
    pub struct Proof {
        pub proof: ethers::core::types::Bytes,
        pub roots: ethers::core::types::Bytes,
        pub input_nullifiers: Vec<[u8; 32]>,
        pub output_commitments: [[u8; 32]; 2],
        pub public_amount: ethers::core::types::U256,
        pub ext_data_hash: [u8; 32],
    }
}
