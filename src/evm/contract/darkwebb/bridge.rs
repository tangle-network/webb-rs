pub use bridgecontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod bridgecontract_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
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
    #[doc = "BridgeContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BRIDGECONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"address[]\",\"name\":\"initialRelayers\",\"type\":\"address[]\"},{\"internalType\":\"uint256\",\"name\":\"initialRelayerThreshold\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"Paused\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"originChainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"indexed\":false,\"internalType\":\"enum Bridge.ProposalStatus\",\"name\":\"status\",\"type\":\"uint8\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"ProposalEvent\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"originChainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"indexed\":false,\"internalType\":\"enum Bridge.ProposalStatus\",\"name\":\"status\",\"type\":\"uint8\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"ProposalVote\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"}],\"name\":\"RelayerAdded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"}],\"name\":\"RelayerRemoved\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"newThreshold\",\"type\":\"uint256\"}],\"name\":\"RelayerThresholdChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleGranted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleRevoked\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"Unpaused\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MAX_RELAYERS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"RELAYER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_chainID\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"_counts\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_expiry\",\"outputs\":[{\"internalType\":\"uint40\",\"name\":\"\",\"type\":\"uint40\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_fee\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint72\",\"name\":\"destNonce\",\"type\":\"uint72\"},{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"}],\"name\":\"_hasVotedOnProposal\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_relayerThreshold\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"_resourceIDToHandlerAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_totalRelayers\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayerAddress\",\"type\":\"address\"}],\"name\":\"adminAddRelayer\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newThreshold\",\"type\":\"uint256\"}],\"name\":\"adminChangeRelayerThreshold\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"adminPauseTransfers\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayerAddress\",\"type\":\"address\"}],\"name\":\"adminRemoveRelayer\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"handlerAddress\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"executionContextAddress\",\"type\":\"address\"}],\"name\":\"adminSetResource\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"adminUnpauseTransfers\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"cancelProposal\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"}],\"name\":\"executeProposal\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"originChainID\",\"type\":\"uint256\"},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"getProposal\",\"outputs\":[{\"components\":[{\"internalType\":\"enum Bridge.ProposalStatus\",\"name\":\"_status\",\"type\":\"uint8\"},{\"internalType\":\"uint200\",\"name\":\"_yesVotes\",\"type\":\"uint200\"},{\"internalType\":\"uint8\",\"name\":\"_yesVotesTotal\",\"type\":\"uint8\"},{\"internalType\":\"uint40\",\"name\":\"_proposedBlock\",\"type\":\"uint40\"}],\"internalType\":\"struct Bridge.Proposal\",\"name\":\"\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"}],\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"getRoleMember\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"}],\"name\":\"getRoleMemberCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"getRoleMemberIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"grantRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"}],\"name\":\"isRelayer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\"}],\"name\":\"renounceAdmin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"renounceRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"revokeRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"voteProposal\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct BridgeContract<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for BridgeContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for BridgeContract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BridgeContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> BridgeContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                BRIDGECONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function"]
        pub fn default_admin_role(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_RELAYERS` (0x9debb3bd) function"]
        pub fn max_relayers(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([157, 235, 179, 189], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `RELAYER_ROLE` (0x926d7d7f) function"]
        pub fn relayer_role(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([146, 109, 125, 127], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_chainID` (0xbeab7131) function"]
        pub fn chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([190, 171, 113, 49], ())
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `_expiry` (0xc5ec8970) function"]
        pub fn expiry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([197, 236, 137, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_fee` (0xc5b37c22) function"]
        pub fn fee(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([197, 179, 124, 34], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_hasVotedOnProposal` (0x7febe63f) function"]
        pub fn has_voted_on_proposal(
            &self,
            dest_nonce: u128,
            data_hash: [u8; 32],
            relayer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [127, 235, 230, 63],
                    (dest_nonce, data_hash, relayer),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_relayerThreshold` (0xd7a9cd79) function"]
        pub fn relayer_threshold(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([215, 169, 205, 121], ())
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
        #[doc = "Calls the contract's `_totalRelayers` (0x802aabe8) function"]
        pub fn total_relayers(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([128, 42, 171, 232], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `adminAddRelayer` (0xcdb0f73a) function"]
        pub fn admin_add_relayer(
            &self,
            relayer_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 176, 247, 58], relayer_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `adminChangeRelayerThreshold` (0x4e056005) function"]
        pub fn admin_change_relayer_threshold(
            &self,
            new_threshold: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 5, 96, 5], new_threshold)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `adminPauseTransfers` (0x80ae1c28) function"]
        pub fn admin_pause_transfers(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 174, 28, 40], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `adminRemoveRelayer` (0x9d82dd63) function"]
        pub fn admin_remove_relayer(
            &self,
            relayer_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 130, 221, 99], relayer_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `adminSetResource` (0xcb10f215) function"]
        pub fn admin_set_resource(
            &self,
            handler_address: ethers::core::types::Address,
            resource_id: [u8; 32],
            execution_context_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [203, 16, 242, 21],
                    (handler_address, resource_id, execution_context_address),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `adminUnpauseTransfers` (0xffaac0eb) function"]
        pub fn admin_unpause_transfers(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 170, 192, 235], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelProposal` (0x709940e6) function"]
        pub fn cancel_proposal(
            &self,
            chain_id: ethers::core::types::U256,
            nonce: u64,
            data_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 153, 64, 230], (chain_id, nonce, data_hash))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeProposal` (0xdfef9a53) function"]
        pub fn execute_proposal(
            &self,
            chain_id: ethers::core::types::U256,
            nonce: u64,
            data: Vec<u8>,
            resource_id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [223, 239, 154, 83],
                    (chain_id, nonce, data, resource_id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getProposal` (0x9b303d7a) function"]
        pub fn get_proposal(
            &self,
            origin_chain_id: ethers::core::types::U256,
            nonce: u64,
            data_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (u8, ethers::core::types::U256, u8, u64),
        > {
            self.0
                .method_hash(
                    [155, 48, 61, 122],
                    (origin_chain_id, nonce, data_hash),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleAdmin` (0x248a9ca3) function"]
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleMember` (0x9010d07c) function"]
        pub fn get_role_member(
            &self,
            role: [u8; 32],
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([144, 16, 208, 124], (role, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleMemberCount` (0xca15c873) function"]
        pub fn get_role_member_count(
            &self,
            role: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([202, 21, 200, 115], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleMemberIndex` (0x4e0df3f6) function"]
        pub fn get_role_member_index(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([78, 13, 243, 246], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grantRole` (0x2f2ff15d) function"]
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasRole` (0x91d14854) function"]
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isRelayer` (0x541d5548) function"]
        pub fn is_relayer(
            &self,
            relayer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([84, 29, 85, 72], relayer)
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
        #[doc = "Calls the contract's `renounceAdmin` (0x5e1fab0f) function"]
        pub fn renounce_admin(
            &self,
            new_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 31, 171, 15], new_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceRole` (0x36568abe) function"]
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeRole` (0xd547741f) function"]
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `voteProposal` (0xa06fa09b) function"]
        pub fn vote_proposal(
            &self,
            chain_id: ethers::core::types::U256,
            nonce: u64,
            resource_id: [u8; 32],
            data_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [160, 111, 160, 155],
                    (chain_id, nonce, resource_id, data_hash),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProposalEvent` event"]
        pub fn proposal_event_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProposalEventFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProposalVote` event"]
        pub fn proposal_vote_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProposalVoteFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RelayerAdded` event"]
        pub fn relayer_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RelayerAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RelayerRemoved` event"]
        pub fn relayer_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RelayerRemovedFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `RelayerThresholdChanged` event"]
        pub fn relayer_threshold_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RelayerThresholdChangedFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleGranted` event"]
        pub fn role_granted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleGrantedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleRevoked` event"]
        pub fn role_revoked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleRevokedFilter> {
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
        ) -> ethers::contract::builders::Event<M, BridgeContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ethers::core::types::Address,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "ProposalEvent",
        abi = "ProposalEvent(uint256,uint64,uint8,bytes32)"
    )]
    pub struct ProposalEventFilter {
        pub origin_chain_id: ethers::core::types::U256,
        pub nonce: u64,
        pub status: u8,
        pub data_hash: [u8; 32],
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "ProposalVote",
        abi = "ProposalVote(uint256,uint64,uint8,bytes32)"
    )]
    pub struct ProposalVoteFilter {
        pub origin_chain_id: ethers::core::types::U256,
        pub nonce: u64,
        pub status: u8,
        pub data_hash: [u8; 32],
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "RelayerAdded", abi = "RelayerAdded(address)")]
    pub struct RelayerAddedFilter {
        pub relayer: ethers::core::types::Address,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "RelayerRemoved", abi = "RelayerRemoved(address)")]
    pub struct RelayerRemovedFilter {
        pub relayer: ethers::core::types::Address,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "RelayerThresholdChanged",
        abi = "RelayerThresholdChanged(uint256)"
    )]
    pub struct RelayerThresholdChangedFilter {
        pub new_threshold: ethers::core::types::U256,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "RoleGranted",
        abi = "RoleGranted(bytes32,address,address)"
    )]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(
        name = "RoleRevoked",
        abi = "RoleRevoked(bytes32,address,address)"
    )]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
    }
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthEvent,
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum BridgeContractEvents {
        PausedFilter(PausedFilter),
        ProposalEventFilter(ProposalEventFilter),
        ProposalVoteFilter(ProposalVoteFilter),
        RelayerAddedFilter(RelayerAddedFilter),
        RelayerRemovedFilter(RelayerRemovedFilter),
        RelayerThresholdChangedFilter(RelayerThresholdChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::core::abi::Tokenizable for BridgeContractEvents {
        fn from_token(
            token: ethers::core::abi::Token,
        ) -> Result<Self, ethers::core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = PausedFilter::from_token(token.clone()) {
                return Ok(BridgeContractEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = ProposalEventFilter::from_token(token.clone())
            {
                return Ok(BridgeContractEvents::ProposalEventFilter(decoded));
            }
            if let Ok(decoded) = ProposalVoteFilter::from_token(token.clone()) {
                return Ok(BridgeContractEvents::ProposalVoteFilter(decoded));
            }
            if let Ok(decoded) = RelayerAddedFilter::from_token(token.clone()) {
                return Ok(BridgeContractEvents::RelayerAddedFilter(decoded));
            }
            if let Ok(decoded) = RelayerRemovedFilter::from_token(token.clone())
            {
                return Ok(BridgeContractEvents::RelayerRemovedFilter(decoded));
            }
            if let Ok(decoded) =
                RelayerThresholdChangedFilter::from_token(token.clone())
            {
                return Ok(BridgeContractEvents::RelayerThresholdChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RoleGrantedFilter::from_token(token.clone()) {
                return Ok(BridgeContractEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::from_token(token.clone()) {
                return Ok(BridgeContractEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::from_token(token.clone()) {
                return Ok(BridgeContractEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers::core::abi::Token {
            match self {
                BridgeContractEvents::PausedFilter(element) => {
                    element.into_token()
                }
                BridgeContractEvents::ProposalEventFilter(element) => {
                    element.into_token()
                }
                BridgeContractEvents::ProposalVoteFilter(element) => {
                    element.into_token()
                }
                BridgeContractEvents::RelayerAddedFilter(element) => {
                    element.into_token()
                }
                BridgeContractEvents::RelayerRemovedFilter(element) => {
                    element.into_token()
                }
                BridgeContractEvents::RelayerThresholdChangedFilter(
                    element,
                ) => element.into_token(),
                BridgeContractEvents::RoleGrantedFilter(element) => {
                    element.into_token()
                }
                BridgeContractEvents::RoleRevokedFilter(element) => {
                    element.into_token()
                }
                BridgeContractEvents::UnpausedFilter(element) => {
                    element.into_token()
                }
            }
        }
    }
    impl ethers::core::abi::TokenizableItem for BridgeContractEvents {}
    impl ethers::contract::EthLogDecode for BridgeContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(BridgeContractEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = ProposalEventFilter::decode_log(log) {
                return Ok(BridgeContractEvents::ProposalEventFilter(decoded));
            }
            if let Ok(decoded) = ProposalVoteFilter::decode_log(log) {
                return Ok(BridgeContractEvents::ProposalVoteFilter(decoded));
            }
            if let Ok(decoded) = RelayerAddedFilter::decode_log(log) {
                return Ok(BridgeContractEvents::RelayerAddedFilter(decoded));
            }
            if let Ok(decoded) = RelayerRemovedFilter::decode_log(log) {
                return Ok(BridgeContractEvents::RelayerRemovedFilter(decoded));
            }
            if let Ok(decoded) = RelayerThresholdChangedFilter::decode_log(log)
            {
                return Ok(BridgeContractEvents::RelayerThresholdChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(BridgeContractEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(BridgeContractEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(BridgeContractEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    #[doc = "`Proposal(uint8,uint200,uint8,uint40)`"]
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthAbiType,
    )]
    pub struct Proposal {
        pub status: u8,
        pub yes_votes: ethers::core::types::U256,
        pub yes_votes_total: u8,
        pub proposed_block: u64,
    }
}
