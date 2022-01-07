pub use signaturebridgecontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod signaturebridgecontract_mod {
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
    #[doc = "SignatureBridgeContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static SIGNATUREBRIDGECONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"initialGovernor\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"GovernanceOwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"Paused\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"recovered\",\"type\":\"address\"}],\"name\":\"RecoveredAddress\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"Unpaused\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"_counts\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"_resourceIDToHandlerAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"resourceIDs\",\"type\":\"bytes32[]\"},{\"internalType\":\"address\",\"name\":\"newBridge\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\"}],\"name\":\"adminMigrateBridgeWithSignature\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"handlerAddress\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"executionContextAddress\",\"type\":\"address\"},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\"}],\"name\":\"adminSetResourceWithSignature\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"pubkey\",\"type\":\"bytes\"}],\"name\":\"checkPubKey\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\"}],\"name\":\"executeProposalWithSignature\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"governor\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"isGovernor\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\"}],\"name\":\"isSignatureFromGovernor\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\"}],\"name\":\"recover\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"refreshNonce\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\"},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\"}],\"name\":\"transferOwnershipWithSignature\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"publicKey\",\"type\":\"bytes\"},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\"},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\"}],\"name\":\"transferOwnershipWithSignaturePubKey\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\"},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\"}],\"name\":\"verify\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct SignatureBridgeContract<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for SignatureBridgeContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for SignatureBridgeContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(SignatureBridgeContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> SignatureBridgeContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                SIGNATUREBRIDGECONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `_counts` (0xd75a0683) function"]
        pub fn counts(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([215, 90, 6, 131], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_resourceIDToHandlerAddress` (0x84db809f) function"]
        pub fn resource_id_to_handler_address(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([132, 219, 128, 159], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `adminMigrateBridgeWithSignature` (0x87ac4f51) function"]
        pub fn admin_migrate_bridge_with_signature(
            &self,
            resource_i_ds: ::std::vec::Vec<[u8; 32]>,
            new_bridge: ethers::core::types::Address,
            sig: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [135, 172, 79, 81],
                    (resource_i_ds, new_bridge, sig),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `adminSetResourceWithSignature` (0x1eee6bc8) function"]
        pub fn admin_set_resource_with_signature(
            &self,
            handler_address: ethers::core::types::Address,
            resource_id: [u8; 32],
            execution_context_address: ethers::core::types::Address,
            sig: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [30, 238, 107, 200],
                    (
                        handler_address,
                        resource_id,
                        execution_context_address,
                        sig,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkPubKey` (0xd4066f4c) function"]
        pub fn check_pub_key(
            &self,
            pubkey: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([212, 6, 111, 76], pubkey)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeProposalWithSignature` (0x9d2b1ed7) function"]
        pub fn execute_proposal_with_signature(
            &self,
            data: ethers::core::types::Bytes,
            sig: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 43, 30, 215], (data, sig))
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
        #[doc = "Calls the contract's `governor` (0x0c340a24) function"]
        pub fn governor(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([12, 52, 10, 36], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isGovernor` (0xc7af3352) function"]
        pub fn is_governor(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([199, 175, 51, 82], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isSignatureFromGovernor` (0x8755bcad) function"]
        pub fn is_signature_from_governor(
            &self,
            data: ethers::core::types::Bytes,
            sig: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([135, 85, 188, 173], (data, sig))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `recover` (0x1ed13d1b) function"]
        pub fn recover(
            &self,
            data: ethers::core::types::Bytes,
            sig: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 209, 61, 27], (data, sig))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refreshNonce` (0x13cb01f9) function"]
        pub fn refresh_nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([19, 203, 1, 249], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xa6e94c91) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 233, 76, 145], (new_owner, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnershipWithSignature` (0x911005e7) function"]
        pub fn transfer_ownership_with_signature(
            &self,
            new_owner: ethers::core::types::Address,
            nonce: u32,
            sig: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 16, 5, 231], (new_owner, nonce, sig))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnershipWithSignaturePubKey` (0x7296b5d8) function"]
        pub fn transfer_ownership_with_signature_pub_key(
            &self,
            public_key: ethers::core::types::Bytes,
            nonce: u32,
            sig: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 150, 181, 216], (public_key, nonce, sig))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verify` (0xf1835db7) function"]
        pub fn verify(
            &self,
            hash: [u8; 32],
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([241, 131, 93, 183], (hash, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `GovernanceOwnershipTransferred` event"]
        pub fn governance_ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            M,
            GovernanceOwnershipTransferredFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RecoveredAddress` event"]
        pub fn recovered_address_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RecoveredAddressFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, SignatureBridgeContractEvents>
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
    #[ethevent(
        name = "GovernanceOwnershipTransferred",
        abi = "GovernanceOwnershipTransferred(address,address)"
    )]
    pub struct GovernanceOwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ethers::core::types::Address,
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
    #[ethevent(name = "RecoveredAddress", abi = "RecoveredAddress(address)")]
    pub struct RecoveredAddressFilter {
        #[ethevent(indexed)]
        pub recovered: ethers::core::types::Address,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum SignatureBridgeContractEvents {
        GovernanceOwnershipTransferredFilter(
            GovernanceOwnershipTransferredFilter,
        ),
        PausedFilter(PausedFilter),
        RecoveredAddressFilter(RecoveredAddressFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for SignatureBridgeContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) =
                GovernanceOwnershipTransferredFilter::decode_log(log)
            {
                return Ok (SignatureBridgeContractEvents :: GovernanceOwnershipTransferredFilter (decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(SignatureBridgeContractEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RecoveredAddressFilter::decode_log(log) {
                return Ok(
                    SignatureBridgeContractEvents::RecoveredAddressFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(SignatureBridgeContractEvents::UnpausedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for SignatureBridgeContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self { SignatureBridgeContractEvents :: GovernanceOwnershipTransferredFilter (element) => element . fmt (f) , SignatureBridgeContractEvents :: PausedFilter (element) => element . fmt (f) , SignatureBridgeContractEvents :: RecoveredAddressFilter (element) => element . fmt (f) , SignatureBridgeContractEvents :: UnpausedFilter (element) => element . fmt (f) }
        }
    }
    #[doc = "Container type for all input parameters for the `_counts`function with signature `_counts(uint256)` and selector `[215, 90, 6, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_counts", abi = "_counts(uint256)")]
    pub struct CountsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `_resourceIDToHandlerAddress`function with signature `_resourceIDToHandlerAddress(bytes32)` and selector `[132, 219, 128, 159]`"]
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
        name = "_resourceIDToHandlerAddress",
        abi = "_resourceIDToHandlerAddress(bytes32)"
    )]
    pub struct ResourceIDToHandlerAddressCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `adminMigrateBridgeWithSignature`function with signature `adminMigrateBridgeWithSignature(bytes32[],address,bytes)` and selector `[135, 172, 79, 81]`"]
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
        name = "adminMigrateBridgeWithSignature",
        abi = "adminMigrateBridgeWithSignature(bytes32[],address,bytes)"
    )]
    pub struct AdminMigrateBridgeWithSignatureCall {
        pub resource_i_ds: ::std::vec::Vec<[u8; 32]>,
        pub new_bridge: ethers::core::types::Address,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `adminSetResourceWithSignature`function with signature `adminSetResourceWithSignature(address,bytes32,address,bytes)` and selector `[30, 238, 107, 200]`"]
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
        name = "adminSetResourceWithSignature",
        abi = "adminSetResourceWithSignature(address,bytes32,address,bytes)"
    )]
    pub struct AdminSetResourceWithSignatureCall {
        pub handler_address: ethers::core::types::Address,
        pub resource_id: [u8; 32],
        pub execution_context_address: ethers::core::types::Address,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `checkPubKey`function with signature `checkPubKey(bytes)` and selector `[212, 6, 111, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "checkPubKey", abi = "checkPubKey(bytes)")]
    pub struct CheckPubKeyCall {
        pub pubkey: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `executeProposalWithSignature`function with signature `executeProposalWithSignature(bytes,bytes)` and selector `[157, 43, 30, 215]`"]
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
        name = "executeProposalWithSignature",
        abi = "executeProposalWithSignature(bytes,bytes)"
    )]
    pub struct ExecuteProposalWithSignatureCall {
        pub data: ethers::core::types::Bytes,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `getChainId`function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    #[doc = "Container type for all input parameters for the `governor`function with signature `governor()` and selector `[12, 52, 10, 36]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "governor", abi = "governor()")]
    pub struct GovernorCall;
    #[doc = "Container type for all input parameters for the `isGovernor`function with signature `isGovernor()` and selector `[199, 175, 51, 82]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isGovernor", abi = "isGovernor()")]
    pub struct IsGovernorCall;
    #[doc = "Container type for all input parameters for the `isSignatureFromGovernor`function with signature `isSignatureFromGovernor(bytes,bytes)` and selector `[135, 85, 188, 173]`"]
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
        name = "isSignatureFromGovernor",
        abi = "isSignatureFromGovernor(bytes,bytes)"
    )]
    pub struct IsSignatureFromGovernorCall {
        pub data: ethers::core::types::Bytes,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `paused`function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    #[doc = "Container type for all input parameters for the `recover`function with signature `recover(bytes,bytes)` and selector `[30, 209, 61, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "recover", abi = "recover(bytes,bytes)")]
    pub struct RecoverCall {
        pub data: ethers::core::types::Bytes,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `refreshNonce`function with signature `refreshNonce()` and selector `[19, 203, 1, 249]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "refreshNonce", abi = "refreshNonce()")]
    pub struct RefreshNonceCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership`function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `transferOwnership`function with signature `transferOwnership(address,uint32)` and selector `[166, 233, 76, 145]`"]
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
        name = "transferOwnership",
        abi = "transferOwnership(address,uint32)"
    )]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `transferOwnershipWithSignature`function with signature `transferOwnershipWithSignature(address,uint32,bytes)` and selector `[145, 16, 5, 231]`"]
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
        name = "transferOwnershipWithSignature",
        abi = "transferOwnershipWithSignature(address,uint32,bytes)"
    )]
    pub struct TransferOwnershipWithSignatureCall {
        pub new_owner: ethers::core::types::Address,
        pub nonce: u32,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `transferOwnershipWithSignaturePubKey`function with signature `transferOwnershipWithSignaturePubKey(bytes,uint32,bytes)` and selector `[114, 150, 181, 216]`"]
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
        name = "transferOwnershipWithSignaturePubKey",
        abi = "transferOwnershipWithSignaturePubKey(bytes,uint32,bytes)"
    )]
    pub struct TransferOwnershipWithSignaturePubKeyCall {
        pub public_key: ethers::core::types::Bytes,
        pub nonce: u32,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `verify`function with signature `verify(bytes32,uint8,bytes32,bytes32)` and selector `[241, 131, 93, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "verify", abi = "verify(bytes32,uint8,bytes32,bytes32)")]
    pub struct VerifyCall {
        pub hash: [u8; 32],
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum SignatureBridgeContractCalls {
        Counts(CountsCall),
        ResourceIDToHandlerAddress(ResourceIDToHandlerAddressCall),
        AdminMigrateBridgeWithSignature(AdminMigrateBridgeWithSignatureCall),
        AdminSetResourceWithSignature(AdminSetResourceWithSignatureCall),
        CheckPubKey(CheckPubKeyCall),
        ExecuteProposalWithSignature(ExecuteProposalWithSignatureCall),
        GetChainId(GetChainIdCall),
        Governor(GovernorCall),
        IsGovernor(IsGovernorCall),
        IsSignatureFromGovernor(IsSignatureFromGovernorCall),
        Paused(PausedCall),
        Recover(RecoverCall),
        RefreshNonce(RefreshNonceCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        TransferOwnershipWithSignature(TransferOwnershipWithSignatureCall),
        TransferOwnershipWithSignaturePubKey(
            TransferOwnershipWithSignaturePubKeyCall,
        ),
        Verify(VerifyCall),
    }
    impl ethers::core::abi::AbiDecode for SignatureBridgeContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CountsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::Counts(decoded));
            }
            if let Ok (decoded) = < ResourceIDToHandlerAddressCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: ResourceIDToHandlerAddress (decoded)) }
            if let Ok (decoded) = < AdminMigrateBridgeWithSignatureCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: AdminMigrateBridgeWithSignature (decoded)) }
            if let Ok (decoded) = < AdminSetResourceWithSignatureCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: AdminSetResourceWithSignature (decoded)) }
            if let Ok(decoded) =
                <CheckPubKeyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::CheckPubKey(decoded));
            }
            if let Ok (decoded) = < ExecuteProposalWithSignatureCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: ExecuteProposalWithSignature (decoded)) }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GovernorCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::Governor(decoded));
            }
            if let Ok(decoded) =
                <IsGovernorCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::IsGovernor(decoded));
            }
            if let Ok (decoded) = < IsSignatureFromGovernorCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: IsSignatureFromGovernor (decoded)) }
            if let Ok(decoded) =
                <PausedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <RecoverCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::Recover(decoded));
            }
            if let Ok(decoded) =
                <RefreshNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::RefreshNonce(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::RenounceOwnership(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::TransferOwnership(
                    decoded,
                ));
            }
            if let Ok (decoded) = < TransferOwnershipWithSignatureCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: TransferOwnershipWithSignature (decoded)) }
            if let Ok (decoded) = < TransferOwnershipWithSignaturePubKeyCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: TransferOwnershipWithSignaturePubKey (decoded)) }
            if let Ok(decoded) =
                <VerifyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::Verify(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for SignatureBridgeContractCalls {
        fn encode(self) -> Vec<u8> {
            match self { SignatureBridgeContractCalls :: Counts (element) => element . encode () , SignatureBridgeContractCalls :: ResourceIDToHandlerAddress (element) => element . encode () , SignatureBridgeContractCalls :: AdminMigrateBridgeWithSignature (element) => element . encode () , SignatureBridgeContractCalls :: AdminSetResourceWithSignature (element) => element . encode () , SignatureBridgeContractCalls :: CheckPubKey (element) => element . encode () , SignatureBridgeContractCalls :: ExecuteProposalWithSignature (element) => element . encode () , SignatureBridgeContractCalls :: GetChainId (element) => element . encode () , SignatureBridgeContractCalls :: Governor (element) => element . encode () , SignatureBridgeContractCalls :: IsGovernor (element) => element . encode () , SignatureBridgeContractCalls :: IsSignatureFromGovernor (element) => element . encode () , SignatureBridgeContractCalls :: Paused (element) => element . encode () , SignatureBridgeContractCalls :: Recover (element) => element . encode () , SignatureBridgeContractCalls :: RefreshNonce (element) => element . encode () , SignatureBridgeContractCalls :: RenounceOwnership (element) => element . encode () , SignatureBridgeContractCalls :: TransferOwnership (element) => element . encode () , SignatureBridgeContractCalls :: TransferOwnershipWithSignature (element) => element . encode () , SignatureBridgeContractCalls :: TransferOwnershipWithSignaturePubKey (element) => element . encode () , SignatureBridgeContractCalls :: Verify (element) => element . encode () }
        }
    }
    impl ::std::fmt::Display for SignatureBridgeContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self { SignatureBridgeContractCalls :: Counts (element) => element . fmt (f) , SignatureBridgeContractCalls :: ResourceIDToHandlerAddress (element) => element . fmt (f) , SignatureBridgeContractCalls :: AdminMigrateBridgeWithSignature (element) => element . fmt (f) , SignatureBridgeContractCalls :: AdminSetResourceWithSignature (element) => element . fmt (f) , SignatureBridgeContractCalls :: CheckPubKey (element) => element . fmt (f) , SignatureBridgeContractCalls :: ExecuteProposalWithSignature (element) => element . fmt (f) , SignatureBridgeContractCalls :: GetChainId (element) => element . fmt (f) , SignatureBridgeContractCalls :: Governor (element) => element . fmt (f) , SignatureBridgeContractCalls :: IsGovernor (element) => element . fmt (f) , SignatureBridgeContractCalls :: IsSignatureFromGovernor (element) => element . fmt (f) , SignatureBridgeContractCalls :: Paused (element) => element . fmt (f) , SignatureBridgeContractCalls :: Recover (element) => element . fmt (f) , SignatureBridgeContractCalls :: RefreshNonce (element) => element . fmt (f) , SignatureBridgeContractCalls :: RenounceOwnership (element) => element . fmt (f) , SignatureBridgeContractCalls :: TransferOwnership (element) => element . fmt (f) , SignatureBridgeContractCalls :: TransferOwnershipWithSignature (element) => element . fmt (f) , SignatureBridgeContractCalls :: TransferOwnershipWithSignaturePubKey (element) => element . fmt (f) , SignatureBridgeContractCalls :: Verify (element) => element . fmt (f) }
        }
    }
    impl ::std::convert::From<CountsCall> for SignatureBridgeContractCalls {
        fn from(var: CountsCall) -> Self {
            SignatureBridgeContractCalls::Counts(var)
        }
    }
    impl ::std::convert::From<ResourceIDToHandlerAddressCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: ResourceIDToHandlerAddressCall) -> Self {
            SignatureBridgeContractCalls::ResourceIDToHandlerAddress(var)
        }
    }
    impl ::std::convert::From<AdminMigrateBridgeWithSignatureCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: AdminMigrateBridgeWithSignatureCall) -> Self {
            SignatureBridgeContractCalls::AdminMigrateBridgeWithSignature(var)
        }
    }
    impl ::std::convert::From<AdminSetResourceWithSignatureCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: AdminSetResourceWithSignatureCall) -> Self {
            SignatureBridgeContractCalls::AdminSetResourceWithSignature(var)
        }
    }
    impl ::std::convert::From<CheckPubKeyCall> for SignatureBridgeContractCalls {
        fn from(var: CheckPubKeyCall) -> Self {
            SignatureBridgeContractCalls::CheckPubKey(var)
        }
    }
    impl ::std::convert::From<ExecuteProposalWithSignatureCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: ExecuteProposalWithSignatureCall) -> Self {
            SignatureBridgeContractCalls::ExecuteProposalWithSignature(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for SignatureBridgeContractCalls {
        fn from(var: GetChainIdCall) -> Self {
            SignatureBridgeContractCalls::GetChainId(var)
        }
    }
    impl ::std::convert::From<GovernorCall> for SignatureBridgeContractCalls {
        fn from(var: GovernorCall) -> Self {
            SignatureBridgeContractCalls::Governor(var)
        }
    }
    impl ::std::convert::From<IsGovernorCall> for SignatureBridgeContractCalls {
        fn from(var: IsGovernorCall) -> Self {
            SignatureBridgeContractCalls::IsGovernor(var)
        }
    }
    impl ::std::convert::From<IsSignatureFromGovernorCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: IsSignatureFromGovernorCall) -> Self {
            SignatureBridgeContractCalls::IsSignatureFromGovernor(var)
        }
    }
    impl ::std::convert::From<PausedCall> for SignatureBridgeContractCalls {
        fn from(var: PausedCall) -> Self {
            SignatureBridgeContractCalls::Paused(var)
        }
    }
    impl ::std::convert::From<RecoverCall> for SignatureBridgeContractCalls {
        fn from(var: RecoverCall) -> Self {
            SignatureBridgeContractCalls::Recover(var)
        }
    }
    impl ::std::convert::From<RefreshNonceCall> for SignatureBridgeContractCalls {
        fn from(var: RefreshNonceCall) -> Self {
            SignatureBridgeContractCalls::RefreshNonce(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: RenounceOwnershipCall) -> Self {
            SignatureBridgeContractCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: TransferOwnershipCall) -> Self {
            SignatureBridgeContractCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipWithSignatureCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: TransferOwnershipWithSignatureCall) -> Self {
            SignatureBridgeContractCalls::TransferOwnershipWithSignature(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipWithSignaturePubKeyCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: TransferOwnershipWithSignaturePubKeyCall) -> Self {
            SignatureBridgeContractCalls::TransferOwnershipWithSignaturePubKey(
                var,
            )
        }
    }
    impl ::std::convert::From<VerifyCall> for SignatureBridgeContractCalls {
        fn from(var: VerifyCall) -> Self {
            SignatureBridgeContractCalls::Verify(var)
        }
    }
}
