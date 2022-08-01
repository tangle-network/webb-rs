pub use signature_bridge_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod signature_bridge_contract {
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
        ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"initialGovernor\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"GovernanceOwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recovered\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RecoveredAddress\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EVM_CHAIN_ID_TYPE\",\"outputs\":[{\"internalType\":\"bytes2\",\"name\":\"\",\"type\":\"bytes2\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_resourceIDToHandlerAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"functionSig\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newResourceID\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"handlerAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"executionContextAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"adminSetResourceWithSignature\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"averageSessionLengthInMillisecs\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentVotingPeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeProposalWithSignature\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainIdType\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"governor\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isGovernor\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSignatureFromGovernor\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastGovernorUpdateTime\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numOfProposers\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposerSetRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposerSetUpdateNonce\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"recover\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"refreshNonce\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sessionLengthMultiplier\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"publicKey\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"nonce\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"sig\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnershipWithSignaturePubKey\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_proposerSetRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"_averageSessionLengthInMillisecs\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_numOfProposers\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_proposerSetUpdateNonce\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_sig\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateProposerSetData\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct Governable.Vote\",\"name\":\"vote\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leafIndex\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"siblingPathNodes\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposedGovernor\",\"type\":\"address\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"voteInFavorForceSetGovernor\",\"outputs\":[]}]") . expect ("invalid abi")
    });
    pub struct SignatureBridgeContract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for SignatureBridgeContract<M> {
        fn clone(&self) -> Self {
            SignatureBridgeContract(self.0.clone())
        }
    }
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
    impl<M: ethers::providers::Middleware> SignatureBridgeContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                SIGNATUREBRIDGECONTRACT_ABI.clone(),
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
        #[doc = "Calls the contract's `adminSetResourceWithSignature` (0xc944e408) function"]
        pub fn admin_set_resource_with_signature(
            &self,
            resource_id: [u8; 32],
            function_sig: [u8; 4],
            nonce: u32,
            new_resource_id: [u8; 32],
            handler_address: ethers::core::types::Address,
            execution_context_address: ethers::core::types::Address,
            sig: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 68, 228, 8],
                    (
                        resource_id,
                        function_sig,
                        nonce,
                        new_resource_id,
                        handler_address,
                        execution_context_address,
                        sig,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `averageSessionLengthInMillisecs` (0x016737bb) function"]
        pub fn average_session_length_in_millisecs(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([1, 103, 55, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentVotingPeriod` (0x3a049e02) function"]
        pub fn current_voting_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([58, 4, 158, 2], ())
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
        #[doc = "Calls the contract's `getChainIdType` (0x4c830cbd) function"]
        pub fn get_chain_id_type(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([76, 131, 12, 189], ())
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
        #[doc = "Calls the contract's `lastGovernorUpdateTime` (0x9e09583c) function"]
        pub fn last_governor_update_time(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([158, 9, 88, 60], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `numOfProposers` (0xbac163a2) function"]
        pub fn num_of_proposers(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([186, 193, 99, 162], ())
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
        #[doc = "Calls the contract's `proposerSetRoot` (0xc5eb6b1f) function"]
        pub fn proposer_set_root(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([197, 235, 107, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proposerSetUpdateNonce` (0x93596700) function"]
        pub fn proposer_set_update_nonce(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([147, 89, 103, 0], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `recover` (0x1ed13d1b) function"]
        pub fn recover(
            &self,
            data: ethers::core::types::Bytes,
            sig: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
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
        #[doc = "Calls the contract's `sessionLengthMultiplier` (0xbdfadc84) function"]
        pub fn session_length_multiplier(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([189, 250, 220, 132], ())
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
        #[doc = "Calls the contract's `updateProposerSetData` (0xf3d23d54) function"]
        pub fn update_proposer_set_data(
            &self,
            proposer_set_root: [u8; 32],
            average_session_length_in_millisecs: u64,
            num_of_proposers: u32,
            proposer_set_update_nonce: u32,
            sig: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [243, 210, 61, 84],
                    (
                        proposer_set_root,
                        average_session_length_in_millisecs,
                        num_of_proposers,
                        proposer_set_update_nonce,
                        sig,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `voteInFavorForceSetGovernor` (0x24118804) function"]
        pub fn vote_in_favor_force_set_governor(
            &self,
            vote: Vote,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 17, 136, 4], (vote,))
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
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for SignatureBridgeContract<M>
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `_resourceIDToHandlerAddress` function with signature `_resourceIDToHandlerAddress(bytes32)` and selector `[132, 219, 128, 159]`"]
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
        name = "_resourceIDToHandlerAddress",
        abi = "_resourceIDToHandlerAddress(bytes32)"
    )]
    pub struct ResourceIDToHandlerAddressCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `adminSetResourceWithSignature` function with signature `adminSetResourceWithSignature(bytes32,bytes4,uint32,bytes32,address,address,bytes)` and selector `[201, 68, 228, 8]`"]
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
        name = "adminSetResourceWithSignature",
        abi = "adminSetResourceWithSignature(bytes32,bytes4,uint32,bytes32,address,address,bytes)"
    )]
    pub struct AdminSetResourceWithSignatureCall {
        pub resource_id: [u8; 32],
        pub function_sig: [u8; 4],
        pub nonce: u32,
        pub new_resource_id: [u8; 32],
        pub handler_address: ethers::core::types::Address,
        pub execution_context_address: ethers::core::types::Address,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `averageSessionLengthInMillisecs` function with signature `averageSessionLengthInMillisecs()` and selector `[1, 103, 55, 187]`"]
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
        name = "averageSessionLengthInMillisecs",
        abi = "averageSessionLengthInMillisecs()"
    )]
    pub struct AverageSessionLengthInMillisecsCall;
    #[doc = "Container type for all input parameters for the `currentVotingPeriod` function with signature `currentVotingPeriod()` and selector `[58, 4, 158, 2]`"]
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
    #[ethcall(name = "currentVotingPeriod", abi = "currentVotingPeriod()")]
    pub struct CurrentVotingPeriodCall;
    #[doc = "Container type for all input parameters for the `executeProposalWithSignature` function with signature `executeProposalWithSignature(bytes,bytes)` and selector `[157, 43, 30, 215]`"]
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
        name = "executeProposalWithSignature",
        abi = "executeProposalWithSignature(bytes,bytes)"
    )]
    pub struct ExecuteProposalWithSignatureCall {
        pub data: ethers::core::types::Bytes,
        pub sig: ethers::core::types::Bytes,
    }
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
    #[doc = "Container type for all input parameters for the `governor` function with signature `governor()` and selector `[12, 52, 10, 36]`"]
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
    #[ethcall(name = "governor", abi = "governor()")]
    pub struct GovernorCall;
    #[doc = "Container type for all input parameters for the `isGovernor` function with signature `isGovernor()` and selector `[199, 175, 51, 82]`"]
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
    #[ethcall(name = "isGovernor", abi = "isGovernor()")]
    pub struct IsGovernorCall;
    #[doc = "Container type for all input parameters for the `isSignatureFromGovernor` function with signature `isSignatureFromGovernor(bytes,bytes)` and selector `[135, 85, 188, 173]`"]
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
        name = "isSignatureFromGovernor",
        abi = "isSignatureFromGovernor(bytes,bytes)"
    )]
    pub struct IsSignatureFromGovernorCall {
        pub data: ethers::core::types::Bytes,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `lastGovernorUpdateTime` function with signature `lastGovernorUpdateTime()` and selector `[158, 9, 88, 60]`"]
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
        name = "lastGovernorUpdateTime",
        abi = "lastGovernorUpdateTime()"
    )]
    pub struct LastGovernorUpdateTimeCall;
    #[doc = "Container type for all input parameters for the `numOfProposers` function with signature `numOfProposers()` and selector `[186, 193, 99, 162]`"]
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
    #[ethcall(name = "numOfProposers", abi = "numOfProposers()")]
    pub struct NumOfProposersCall;
    #[doc = "Container type for all input parameters for the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
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
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
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
    #[doc = "Container type for all input parameters for the `proposerSetRoot` function with signature `proposerSetRoot()` and selector `[197, 235, 107, 31]`"]
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
    #[ethcall(name = "proposerSetRoot", abi = "proposerSetRoot()")]
    pub struct ProposerSetRootCall;
    #[doc = "Container type for all input parameters for the `proposerSetUpdateNonce` function with signature `proposerSetUpdateNonce()` and selector `[147, 89, 103, 0]`"]
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
        name = "proposerSetUpdateNonce",
        abi = "proposerSetUpdateNonce()"
    )]
    pub struct ProposerSetUpdateNonceCall;
    #[doc = "Container type for all input parameters for the `recover` function with signature `recover(bytes,bytes)` and selector `[30, 209, 61, 27]`"]
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
    #[ethcall(name = "recover", abi = "recover(bytes,bytes)")]
    pub struct RecoverCall {
        pub data: ethers::core::types::Bytes,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `refreshNonce` function with signature `refreshNonce()` and selector `[19, 203, 1, 249]`"]
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
    #[ethcall(name = "refreshNonce", abi = "refreshNonce()")]
    pub struct RefreshNonceCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `sessionLengthMultiplier` function with signature `sessionLengthMultiplier()` and selector `[189, 250, 220, 132]`"]
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
        name = "sessionLengthMultiplier",
        abi = "sessionLengthMultiplier()"
    )]
    pub struct SessionLengthMultiplierCall;
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address,uint32)` and selector `[166, 233, 76, 145]`"]
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
        name = "transferOwnership",
        abi = "transferOwnership(address,uint32)"
    )]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `transferOwnershipWithSignaturePubKey` function with signature `transferOwnershipWithSignaturePubKey(bytes,uint32,bytes)` and selector `[114, 150, 181, 216]`"]
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
        name = "transferOwnershipWithSignaturePubKey",
        abi = "transferOwnershipWithSignaturePubKey(bytes,uint32,bytes)"
    )]
    pub struct TransferOwnershipWithSignaturePubKeyCall {
        pub public_key: ethers::core::types::Bytes,
        pub nonce: u32,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `updateProposerSetData` function with signature `updateProposerSetData(bytes32,uint64,uint32,uint32,bytes)` and selector `[243, 210, 61, 84]`"]
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
        name = "updateProposerSetData",
        abi = "updateProposerSetData(bytes32,uint64,uint32,uint32,bytes)"
    )]
    pub struct UpdateProposerSetDataCall {
        pub proposer_set_root: [u8; 32],
        pub average_session_length_in_millisecs: u64,
        pub num_of_proposers: u32,
        pub proposer_set_update_nonce: u32,
        pub sig: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `voteInFavorForceSetGovernor` function with signature `voteInFavorForceSetGovernor((uint32,bytes32[],address))` and selector `[36, 17, 136, 4]`"]
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
        name = "voteInFavorForceSetGovernor",
        abi = "voteInFavorForceSetGovernor((uint32,bytes32[],address))"
    )]
    pub struct VoteInFavorForceSetGovernorCall {
        pub vote: Vote,
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
    pub enum SignatureBridgeContractCalls {
        EvmChainIdType(EvmChainIdTypeCall),
        ResourceIDToHandlerAddress(ResourceIDToHandlerAddressCall),
        AdminSetResourceWithSignature(AdminSetResourceWithSignatureCall),
        AverageSessionLengthInMillisecs(AverageSessionLengthInMillisecsCall),
        CurrentVotingPeriod(CurrentVotingPeriodCall),
        ExecuteProposalWithSignature(ExecuteProposalWithSignatureCall),
        GetChainId(GetChainIdCall),
        GetChainIdType(GetChainIdTypeCall),
        Governor(GovernorCall),
        IsGovernor(IsGovernorCall),
        IsSignatureFromGovernor(IsSignatureFromGovernorCall),
        LastGovernorUpdateTime(LastGovernorUpdateTimeCall),
        NumOfProposers(NumOfProposersCall),
        Paused(PausedCall),
        ProposalNonce(ProposalNonceCall),
        ProposerSetRoot(ProposerSetRootCall),
        ProposerSetUpdateNonce(ProposerSetUpdateNonceCall),
        Recover(RecoverCall),
        RefreshNonce(RefreshNonceCall),
        RenounceOwnership(RenounceOwnershipCall),
        SessionLengthMultiplier(SessionLengthMultiplierCall),
        TransferOwnership(TransferOwnershipCall),
        TransferOwnershipWithSignaturePubKey(
            TransferOwnershipWithSignaturePubKeyCall,
        ),
        UpdateProposerSetData(UpdateProposerSetDataCall),
        VoteInFavorForceSetGovernor(VoteInFavorForceSetGovernorCall),
    }
    impl ethers::core::abi::AbiDecode for SignatureBridgeContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <EvmChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::EvmChainIdType(
                    decoded,
                ));
            }
            if let Ok (decoded) = < ResourceIDToHandlerAddressCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: ResourceIDToHandlerAddress (decoded)) }
            if let Ok (decoded) = < AdminSetResourceWithSignatureCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: AdminSetResourceWithSignature (decoded)) }
            if let Ok (decoded) = < AverageSessionLengthInMillisecsCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: AverageSessionLengthInMillisecs (decoded)) }
            if let Ok (decoded) = < CurrentVotingPeriodCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: CurrentVotingPeriod (decoded)) }
            if let Ok (decoded) = < ExecuteProposalWithSignatureCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: ExecuteProposalWithSignature (decoded)) }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdTypeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::GetChainIdType(
                    decoded,
                ));
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
            if let Ok (decoded) = < LastGovernorUpdateTimeCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: LastGovernorUpdateTime (decoded)) }
            if let Ok(decoded) =
                <NumOfProposersCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::NumOfProposers(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PausedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <ProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::ProposalNonce(decoded));
            }
            if let Ok(decoded) =
                <ProposerSetRootCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::ProposerSetRoot(
                    decoded,
                ));
            }
            if let Ok (decoded) = < ProposerSetUpdateNonceCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: ProposerSetUpdateNonce (decoded)) }
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
            if let Ok (decoded) = < SessionLengthMultiplierCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: SessionLengthMultiplier (decoded)) }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SignatureBridgeContractCalls::TransferOwnership(
                    decoded,
                ));
            }
            if let Ok (decoded) = < TransferOwnershipWithSignaturePubKeyCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: TransferOwnershipWithSignaturePubKey (decoded)) }
            if let Ok (decoded) = < UpdateProposerSetDataCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: UpdateProposerSetData (decoded)) }
            if let Ok (decoded) = < VoteInFavorForceSetGovernorCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (SignatureBridgeContractCalls :: VoteInFavorForceSetGovernor (decoded)) }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for SignatureBridgeContractCalls {
        fn encode(self) -> Vec<u8> {
            match self { SignatureBridgeContractCalls :: EvmChainIdType (element) => element . encode () , SignatureBridgeContractCalls :: ResourceIDToHandlerAddress (element) => element . encode () , SignatureBridgeContractCalls :: AdminSetResourceWithSignature (element) => element . encode () , SignatureBridgeContractCalls :: AverageSessionLengthInMillisecs (element) => element . encode () , SignatureBridgeContractCalls :: CurrentVotingPeriod (element) => element . encode () , SignatureBridgeContractCalls :: ExecuteProposalWithSignature (element) => element . encode () , SignatureBridgeContractCalls :: GetChainId (element) => element . encode () , SignatureBridgeContractCalls :: GetChainIdType (element) => element . encode () , SignatureBridgeContractCalls :: Governor (element) => element . encode () , SignatureBridgeContractCalls :: IsGovernor (element) => element . encode () , SignatureBridgeContractCalls :: IsSignatureFromGovernor (element) => element . encode () , SignatureBridgeContractCalls :: LastGovernorUpdateTime (element) => element . encode () , SignatureBridgeContractCalls :: NumOfProposers (element) => element . encode () , SignatureBridgeContractCalls :: Paused (element) => element . encode () , SignatureBridgeContractCalls :: ProposalNonce (element) => element . encode () , SignatureBridgeContractCalls :: ProposerSetRoot (element) => element . encode () , SignatureBridgeContractCalls :: ProposerSetUpdateNonce (element) => element . encode () , SignatureBridgeContractCalls :: Recover (element) => element . encode () , SignatureBridgeContractCalls :: RefreshNonce (element) => element . encode () , SignatureBridgeContractCalls :: RenounceOwnership (element) => element . encode () , SignatureBridgeContractCalls :: SessionLengthMultiplier (element) => element . encode () , SignatureBridgeContractCalls :: TransferOwnership (element) => element . encode () , SignatureBridgeContractCalls :: TransferOwnershipWithSignaturePubKey (element) => element . encode () , SignatureBridgeContractCalls :: UpdateProposerSetData (element) => element . encode () , SignatureBridgeContractCalls :: VoteInFavorForceSetGovernor (element) => element . encode () }
        }
    }
    impl ::std::fmt::Display for SignatureBridgeContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self { SignatureBridgeContractCalls :: EvmChainIdType (element) => element . fmt (f) , SignatureBridgeContractCalls :: ResourceIDToHandlerAddress (element) => element . fmt (f) , SignatureBridgeContractCalls :: AdminSetResourceWithSignature (element) => element . fmt (f) , SignatureBridgeContractCalls :: AverageSessionLengthInMillisecs (element) => element . fmt (f) , SignatureBridgeContractCalls :: CurrentVotingPeriod (element) => element . fmt (f) , SignatureBridgeContractCalls :: ExecuteProposalWithSignature (element) => element . fmt (f) , SignatureBridgeContractCalls :: GetChainId (element) => element . fmt (f) , SignatureBridgeContractCalls :: GetChainIdType (element) => element . fmt (f) , SignatureBridgeContractCalls :: Governor (element) => element . fmt (f) , SignatureBridgeContractCalls :: IsGovernor (element) => element . fmt (f) , SignatureBridgeContractCalls :: IsSignatureFromGovernor (element) => element . fmt (f) , SignatureBridgeContractCalls :: LastGovernorUpdateTime (element) => element . fmt (f) , SignatureBridgeContractCalls :: NumOfProposers (element) => element . fmt (f) , SignatureBridgeContractCalls :: Paused (element) => element . fmt (f) , SignatureBridgeContractCalls :: ProposalNonce (element) => element . fmt (f) , SignatureBridgeContractCalls :: ProposerSetRoot (element) => element . fmt (f) , SignatureBridgeContractCalls :: ProposerSetUpdateNonce (element) => element . fmt (f) , SignatureBridgeContractCalls :: Recover (element) => element . fmt (f) , SignatureBridgeContractCalls :: RefreshNonce (element) => element . fmt (f) , SignatureBridgeContractCalls :: RenounceOwnership (element) => element . fmt (f) , SignatureBridgeContractCalls :: SessionLengthMultiplier (element) => element . fmt (f) , SignatureBridgeContractCalls :: TransferOwnership (element) => element . fmt (f) , SignatureBridgeContractCalls :: TransferOwnershipWithSignaturePubKey (element) => element . fmt (f) , SignatureBridgeContractCalls :: UpdateProposerSetData (element) => element . fmt (f) , SignatureBridgeContractCalls :: VoteInFavorForceSetGovernor (element) => element . fmt (f) }
        }
    }
    impl ::std::convert::From<EvmChainIdTypeCall> for SignatureBridgeContractCalls {
        fn from(var: EvmChainIdTypeCall) -> Self {
            SignatureBridgeContractCalls::EvmChainIdType(var)
        }
    }
    impl ::std::convert::From<ResourceIDToHandlerAddressCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: ResourceIDToHandlerAddressCall) -> Self {
            SignatureBridgeContractCalls::ResourceIDToHandlerAddress(var)
        }
    }
    impl ::std::convert::From<AdminSetResourceWithSignatureCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: AdminSetResourceWithSignatureCall) -> Self {
            SignatureBridgeContractCalls::AdminSetResourceWithSignature(var)
        }
    }
    impl ::std::convert::From<AverageSessionLengthInMillisecsCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: AverageSessionLengthInMillisecsCall) -> Self {
            SignatureBridgeContractCalls::AverageSessionLengthInMillisecs(var)
        }
    }
    impl ::std::convert::From<CurrentVotingPeriodCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: CurrentVotingPeriodCall) -> Self {
            SignatureBridgeContractCalls::CurrentVotingPeriod(var)
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
    impl ::std::convert::From<GetChainIdTypeCall> for SignatureBridgeContractCalls {
        fn from(var: GetChainIdTypeCall) -> Self {
            SignatureBridgeContractCalls::GetChainIdType(var)
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
    impl ::std::convert::From<LastGovernorUpdateTimeCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: LastGovernorUpdateTimeCall) -> Self {
            SignatureBridgeContractCalls::LastGovernorUpdateTime(var)
        }
    }
    impl ::std::convert::From<NumOfProposersCall> for SignatureBridgeContractCalls {
        fn from(var: NumOfProposersCall) -> Self {
            SignatureBridgeContractCalls::NumOfProposers(var)
        }
    }
    impl ::std::convert::From<PausedCall> for SignatureBridgeContractCalls {
        fn from(var: PausedCall) -> Self {
            SignatureBridgeContractCalls::Paused(var)
        }
    }
    impl ::std::convert::From<ProposalNonceCall> for SignatureBridgeContractCalls {
        fn from(var: ProposalNonceCall) -> Self {
            SignatureBridgeContractCalls::ProposalNonce(var)
        }
    }
    impl ::std::convert::From<ProposerSetRootCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: ProposerSetRootCall) -> Self {
            SignatureBridgeContractCalls::ProposerSetRoot(var)
        }
    }
    impl ::std::convert::From<ProposerSetUpdateNonceCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: ProposerSetUpdateNonceCall) -> Self {
            SignatureBridgeContractCalls::ProposerSetUpdateNonce(var)
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
    impl ::std::convert::From<SessionLengthMultiplierCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: SessionLengthMultiplierCall) -> Self {
            SignatureBridgeContractCalls::SessionLengthMultiplier(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: TransferOwnershipCall) -> Self {
            SignatureBridgeContractCalls::TransferOwnership(var)
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
    impl ::std::convert::From<UpdateProposerSetDataCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: UpdateProposerSetDataCall) -> Self {
            SignatureBridgeContractCalls::UpdateProposerSetData(var)
        }
    }
    impl ::std::convert::From<VoteInFavorForceSetGovernorCall>
        for SignatureBridgeContractCalls
    {
        fn from(var: VoteInFavorForceSetGovernorCall) -> Self {
            SignatureBridgeContractCalls::VoteInFavorForceSetGovernor(var)
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
    #[doc = "Container type for all return fields from the `_resourceIDToHandlerAddress` function with signature `_resourceIDToHandlerAddress(bytes32)` and selector `[132, 219, 128, 159]`"]
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
    pub struct ResourceIDToHandlerAddressReturn(
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all return fields from the `averageSessionLengthInMillisecs` function with signature `averageSessionLengthInMillisecs()` and selector `[1, 103, 55, 187]`"]
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
    pub struct AverageSessionLengthInMillisecsReturn(pub u64);
    #[doc = "Container type for all return fields from the `currentVotingPeriod` function with signature `currentVotingPeriod()` and selector `[58, 4, 158, 2]`"]
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
    pub struct CurrentVotingPeriodReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `governor` function with signature `governor()` and selector `[12, 52, 10, 36]`"]
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
    pub struct GovernorReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isGovernor` function with signature `isGovernor()` and selector `[199, 175, 51, 82]`"]
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
    pub struct IsGovernorReturn(pub bool);
    #[doc = "Container type for all return fields from the `isSignatureFromGovernor` function with signature `isSignatureFromGovernor(bytes,bytes)` and selector `[135, 85, 188, 173]`"]
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
    pub struct IsSignatureFromGovernorReturn(pub bool);
    #[doc = "Container type for all return fields from the `lastGovernorUpdateTime` function with signature `lastGovernorUpdateTime()` and selector `[158, 9, 88, 60]`"]
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
    pub struct LastGovernorUpdateTimeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `numOfProposers` function with signature `numOfProposers()` and selector `[186, 193, 99, 162]`"]
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
    pub struct NumOfProposersReturn(pub u32);
    #[doc = "Container type for all return fields from the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
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
    pub struct PausedReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `proposerSetRoot` function with signature `proposerSetRoot()` and selector `[197, 235, 107, 31]`"]
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
    pub struct ProposerSetRootReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `proposerSetUpdateNonce` function with signature `proposerSetUpdateNonce()` and selector `[147, 89, 103, 0]`"]
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
    pub struct ProposerSetUpdateNonceReturn(pub u32);
    #[doc = "Container type for all return fields from the `recover` function with signature `recover(bytes,bytes)` and selector `[30, 209, 61, 27]`"]
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
    pub struct RecoverReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `refreshNonce` function with signature `refreshNonce()` and selector `[19, 203, 1, 249]`"]
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
    pub struct RefreshNonceReturn(pub u32);
    #[doc = "Container type for all return fields from the `sessionLengthMultiplier` function with signature `sessionLengthMultiplier()` and selector `[189, 250, 220, 132]`"]
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
    pub struct SessionLengthMultiplierReturn(pub ethers::core::types::U256);
    #[doc = "`Vote(uint32,bytes32[],address)`"]
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
    pub struct Vote {
        pub leaf_index: u32,
        pub sibling_path_nodes: Vec<[u8; 32]>,
        pub proposed_governor: ethers::core::types::Address,
    }
}
