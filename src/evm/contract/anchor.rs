pub use anchorcontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod anchorcontract_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            self as ethers_contract,
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            self as ethers_core,
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::{self as ethers_providers, Middleware},
    };
    #[doc = "AnchorContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ANCHORCONTRACT_ABI: ethers_contract::Lazy<
        ethers_core::abi::Abi,
    > = ethers_contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"commitment\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint32\",\"name\":\"leafIndex\",\"type\":\"uint32\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\"}],\"name\":\"Deposit\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"nullifierHash\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\"}],\"name\":\"Withdrawal\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"FIELD_SIZE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ROOT_HISTORY_SIZE\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ZERO_VALUE\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"commitments\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"currentRootIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"denomination\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"filledSubtrees\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getLastRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"contract IHasher\",\"name\":\"_hasher\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"_left\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_right\",\"type\":\"bytes32\"}],\"name\":\"hashLeftRight\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"hasher\",\"outputs\":[{\"internalType\":\"contract IHasher\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"}],\"name\":\"isKnownRoot\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"levels\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"nextIndex\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"nullifierHashes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"roots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"verifier\",\"outputs\":[{\"internalType\":\"contract IVerifier\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"}],\"name\":\"zeros\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_commitment\",\"type\":\"bytes32\"}],\"name\":\"deposit\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"_proof\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\"},{\"internalType\":\"address payable\",\"name\":\"_recipient\",\"type\":\"address\"},{\"internalType\":\"address payable\",\"name\":\"_relayer\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_fee\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_refund\",\"type\":\"uint256\"}],\"name\":\"withdraw\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_nullifierHash\",\"type\":\"bytes32\"}],\"name\":\"isSpent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_nullifierHashes\",\"type\":\"bytes32[]\"}],\"name\":\"isSpentArray\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"spent\",\"type\":\"bool[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct AnchorContract<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for AnchorContract<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for AnchorContract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AnchorContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> AnchorContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                ANCHORCONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `FIELD_SIZE` (0x414a37ba) function"]
        pub fn field_size(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256>
        {
            self.0
                .method_hash([65, 74, 55, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ROOT_HISTORY_SIZE` (0xcd87a3b4) function"]
        pub fn root_history_size(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([205, 135, 163, 180], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ZERO_VALUE` (0xec732959) function"]
        pub fn zero_value(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256>
        {
            self.0
                .method_hash([236, 115, 41, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `commitments` (0x839df945) function"]
        pub fn commitments(
            &self,
            p0: [u8; 32],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([131, 157, 249, 69], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentRootIndex` (0x90eeb02b) function"]
        pub fn current_root_index(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([144, 238, 176, 43], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `denomination` (0x8bca6d16) function"]
        pub fn denomination(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256>
        {
            self.0
                .method_hash([139, 202, 109, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xb214faa5) function"]
        pub fn deposit(
            &self,
            commitment: [u8; 32],
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 20, 250, 165], commitment)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `filledSubtrees` (0xf178e47c) function"]
        pub fn filled_subtrees(
            &self,
            p0: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([241, 120, 228, 124], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastRoot` (0xba70f757) function"]
        pub fn get_last_root(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([186, 112, 247, 87], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hashLeftRight` (0x8ea3099e) function"]
        pub fn hash_left_right(
            &self,
            hasher: ethers_core::types::Address,
            left: [u8; 32],
            right: [u8; 32],
        ) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([142, 163, 9, 158], (hasher, left, right))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasher` (0xed33639f) function"]
        pub fn hasher(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            ethers_core::types::Address,
        > {
            self.0
                .method_hash([237, 51, 99, 159], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isKnownRoot` (0x6d9833e3) function"]
        pub fn is_known_root(
            &self,
            root: [u8; 32],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 152, 51, 227], root)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpent` (0xe5285dcc) function"]
        pub fn is_spent(
            &self,
            nullifier_hash: [u8; 32],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([229, 40, 93, 204], nullifier_hash)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSpentArray` (0x9fa12d0b) function"]
        pub fn is_spent_array(
            &self,
            nullifier_hashes: Vec<[u8; 32]>,
        ) -> ethers_contract::builders::ContractCall<M, Vec<bool>> {
            self.0
                .method_hash([159, 161, 45, 11], nullifier_hashes)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `levels` (0x4ecf518b) function"]
        pub fn levels(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([78, 207, 81, 139], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nextIndex` (0xfc7e9c6f) function"]
        pub fn next_index(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([252, 126, 156, 111], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nullifierHashes` (0x17cc915c) function"]
        pub fn nullifier_hashes(
            &self,
            p0: [u8; 32],
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([23, 204, 145, 92], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `roots` (0xc2b40ae4) function"]
        pub fn roots(
            &self,
            p0: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([194, 180, 10, 228], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifier` (0x2b7ac3f3) function"]
        pub fn verifier(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            ethers_core::types::Address,
        > {
            self.0
                .method_hash([43, 122, 195, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x21a0adb6) function"]
        pub fn withdraw(
            &self,
            proof: Vec<u8>,
            root: [u8; 32],
            nullifier_hash: [u8; 32],
            recipient: ethers_core::types::Address,
            relayer: ethers_core::types::Address,
            fee: ethers_core::types::U256,
            refund: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [33, 160, 173, 182],
                    (
                        proof,
                        root,
                        nullifier_hash,
                        recipient,
                        relayer,
                        fee,
                        refund,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `zeros` (0xe8295588) function"]
        pub fn zeros(
            &self,
            i: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([232, 41, 85, 136], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, DepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdrawal` event"]
        pub fn withdrawal_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, WithdrawalFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers_contract::builders::Event<M, AnchorContractEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(bytes32,uint32,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub commitment: [u8; 32],
        pub leaf_index: u32,
        pub timestamp: ethers_core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent,
    )]
    #[ethevent(
        name = "Withdrawal",
        abi = "Withdrawal(address,bytes32,address,uint256)"
    )]
    pub struct WithdrawalFilter {
        pub to: ethers_core::types::Address,
        pub nullifier_hash: [u8; 32],
        #[ethevent(indexed)]
        pub relayer: ethers_core::types::Address,
        pub fee: ethers_core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum AnchorContractEvents {
        DepositFilter(DepositFilter),
        WithdrawalFilter(WithdrawalFilter),
    }
    impl ethers_core::abi::Tokenizable for AnchorContractEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DepositFilter::from_token(token.clone()) {
                return Ok(AnchorContractEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalFilter::from_token(token.clone()) {
                return Ok(AnchorContractEvents::WithdrawalFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                AnchorContractEvents::DepositFilter(element) => {
                    element.into_token()
                }
                AnchorContractEvents::WithdrawalFilter(element) => {
                    element.into_token()
                }
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for AnchorContractEvents {}
    impl ethers_contract::EthLogDecode for AnchorContractEvents {
        fn decode_log(
            log: &ethers_core::abi::RawLog,
        ) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(AnchorContractEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalFilter::decode_log(log) {
                return Ok(AnchorContractEvents::WithdrawalFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}
