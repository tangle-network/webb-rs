pub use bridgecontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod bridgecontract_mod {
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
    #[doc = "BridgeContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static BRIDGECONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"address[]\",\"name\":\"initialRelayers\",\"type\":\"address[]\"},{\"internalType\":\"uint256\",\"name\":\"initialRelayerThreshold\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"destinationChainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"}],\"name\":\"Deposit\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"Paused\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"originChainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"indexed\":false,\"internalType\":\"enum Bridge.ProposalStatus\",\"name\":\"status\",\"type\":\"uint8\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"ProposalEvent\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"originChainID\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"indexed\":false,\"internalType\":\"enum Bridge.ProposalStatus\",\"name\":\"status\",\"type\":\"uint8\"},{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"ProposalVote\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"}],\"name\":\"RelayerAdded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"}],\"name\":\"RelayerRemoved\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"newThreshold\",\"type\":\"uint256\"}],\"name\":\"RelayerThresholdChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleGranted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleRevoked\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"Unpaused\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MAX_RELAYERS\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"RELAYER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_chainID\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"_counts\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_expiry\",\"outputs\":[{\"internalType\":\"uint40\",\"name\":\"\",\"type\":\"uint40\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_fee\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint72\",\"name\":\"destNonce\",\"type\":\"uint72\"},{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"}],\"name\":\"_hasVotedOnProposal\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_relayerThreshold\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"_resourceIDToHandlerAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"_totalRelayers\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayerAddress\",\"type\":\"address\"}],\"name\":\"adminAddRelayer\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newFee\",\"type\":\"uint256\"}],\"name\":\"adminChangeFee\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newThreshold\",\"type\":\"uint256\"}],\"name\":\"adminChangeRelayerThreshold\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"adminPauseTransfers\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayerAddress\",\"type\":\"address\"}],\"name\":\"adminRemoveRelayer\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"handlerAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"}],\"name\":\"adminSetBurnable\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"handlerAddress\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"executionContextAddress\",\"type\":\"address\"}],\"name\":\"adminSetResource\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"adminUnpauseTransfers\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"handlerAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amountOrTokenID\",\"type\":\"uint256\"}],\"name\":\"adminWithdraw\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"cancelProposal\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint32\",\"name\":\"destinationChainID\",\"type\":\"uint32\"},{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"deposit\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"},{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"}],\"name\":\"executeProposal\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"originChainID\",\"type\":\"uint256\"},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"getProposal\",\"outputs\":[{\"components\":[{\"internalType\":\"enum Bridge.ProposalStatus\",\"name\":\"_status\",\"type\":\"uint8\"},{\"internalType\":\"uint200\",\"name\":\"_yesVotes\",\"type\":\"uint200\"},{\"internalType\":\"uint8\",\"name\":\"_yesVotesTotal\",\"type\":\"uint8\"},{\"internalType\":\"uint40\",\"name\":\"_proposedBlock\",\"type\":\"uint40\"}],\"internalType\":\"struct Bridge.Proposal\",\"name\":\"\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"}],\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"getRoleMember\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"}],\"name\":\"getRoleMemberCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"getRoleMemberIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"grantRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\"}],\"name\":\"isRelayer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\"}],\"name\":\"renounceAdmin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"renounceRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"revokeRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address payable[]\",\"name\":\"addrs\",\"type\":\"address[]\"},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\"}],\"name\":\"transferFunds\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"chainID\",\"type\":\"uint256\"},{\"internalType\":\"uint64\",\"name\":\"nonce\",\"type\":\"uint64\"},{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\"}],\"name\":\"voteProposal\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
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
        #[doc = "Calls the contract's `adminChangeFee` (0x91c404ac) function"]
        pub fn admin_change_fee(
            &self,
            new_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 196, 4, 172], new_fee)
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
        #[doc = "Calls the contract's `adminSetBurnable` (0x8c0c2631) function"]
        pub fn admin_set_burnable(
            &self,
            handler_address: ethers::core::types::Address,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [140, 12, 38, 49],
                    (handler_address, token_address),
                )
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
        #[doc = "Calls the contract's `adminWithdraw` (0x780cf004) function"]
        pub fn admin_withdraw(
            &self,
            handler_address: ethers::core::types::Address,
            token_address: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount_or_token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [120, 12, 240, 4],
                    (
                        handler_address,
                        token_address,
                        recipient,
                        amount_or_token_id,
                    ),
                )
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
        #[doc = "Calls the contract's `deposit` (0xa44f5fe6) function"]
        pub fn deposit(
            &self,
            destination_chain_id: u32,
            resource_id: [u8; 32],
            data: Vec<u8>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [164, 79, 95, 230],
                    (destination_chain_id, resource_id, data),
                )
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
        #[doc = "Calls the contract's `transferFunds` (0x4603ae38) function"]
        pub fn transfer_funds(
            &self,
            addrs: ::std::vec::Vec<ethers::core::types::Address>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 3, 174, 56], (addrs, amounts))
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
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
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
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(uint256,bytes32,uint64)")]
    pub struct DepositFilter {
        pub destination_chain_id: ethers::core::types::U256,
        pub resource_id: [u8; 32],
        pub nonce: u64,
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
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
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
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "RelayerAdded", abi = "RelayerAdded(address)")]
    pub struct RelayerAddedFilter {
        pub relayer: ethers::core::types::Address,
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
    #[ethevent(name = "RelayerRemoved", abi = "RelayerRemoved(address)")]
    pub struct RelayerRemovedFilter {
        pub relayer: ethers::core::types::Address,
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
        name = "RelayerThresholdChanged",
        abi = "RelayerThresholdChanged(uint256)"
    )]
    pub struct RelayerThresholdChangedFilter {
        pub new_threshold: ethers::core::types::U256,
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
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
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
    pub enum BridgeContractEvents {
        DepositFilter(DepositFilter),
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
    impl ethers::contract::EthLogDecode for BridgeContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(BridgeContractEvents::DepositFilter(decoded));
            }
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
                return Ok(
                    BridgeContractEvents::RelayerThresholdChangedFilter(
                        decoded,
                    ),
                );
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
    impl ::std::fmt::Display for BridgeContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BridgeContractEvents::DepositFilter(element) => element.fmt(f),
                BridgeContractEvents::PausedFilter(element) => element.fmt(f),
                BridgeContractEvents::ProposalEventFilter(element) => {
                    element.fmt(f)
                }
                BridgeContractEvents::ProposalVoteFilter(element) => {
                    element.fmt(f)
                }
                BridgeContractEvents::RelayerAddedFilter(element) => {
                    element.fmt(f)
                }
                BridgeContractEvents::RelayerRemovedFilter(element) => {
                    element.fmt(f)
                }
                BridgeContractEvents::RelayerThresholdChangedFilter(
                    element,
                ) => element.fmt(f),
                BridgeContractEvents::RoleGrantedFilter(element) => {
                    element.fmt(f)
                }
                BridgeContractEvents::RoleRevokedFilter(element) => {
                    element.fmt(f)
                }
                BridgeContractEvents::UnpausedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DEFAULT_ADMIN_ROLE`function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23, 253, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    #[doc = "Container type for all input parameters for the `MAX_RELAYERS`function with signature `MAX_RELAYERS()` and selector `[157, 235, 179, 189]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MAX_RELAYERS", abi = "MAX_RELAYERS()")]
    pub struct MaxRelayersCall;
    #[doc = "Container type for all input parameters for the `RELAYER_ROLE`function with signature `RELAYER_ROLE()` and selector `[146, 109, 125, 127]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "RELAYER_ROLE", abi = "RELAYER_ROLE()")]
    pub struct RelayerRoleCall;
    #[doc = "Container type for all input parameters for the `_chainID`function with signature `_chainID()` and selector `[190, 171, 113, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_chainID", abi = "_chainID()")]
    pub struct ChainIDCall;
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
    #[doc = "Container type for all input parameters for the `_expiry`function with signature `_expiry()` and selector `[197, 236, 137, 112]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_expiry", abi = "_expiry()")]
    pub struct ExpiryCall;
    #[doc = "Container type for all input parameters for the `_fee`function with signature `_fee()` and selector `[197, 179, 124, 34]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_fee", abi = "_fee()")]
    pub struct FeeCall;
    #[doc = "Container type for all input parameters for the `_hasVotedOnProposal`function with signature `_hasVotedOnProposal(uint72,bytes32,address)` and selector `[127, 235, 230, 63]`"]
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
        name = "_hasVotedOnProposal",
        abi = "_hasVotedOnProposal(uint72,bytes32,address)"
    )]
    pub struct HasVotedOnProposalCall {
        pub dest_nonce: u128,
        pub data_hash: [u8; 32],
        pub relayer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `_relayerThreshold`function with signature `_relayerThreshold()` and selector `[215, 169, 205, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_relayerThreshold", abi = "_relayerThreshold()")]
    pub struct RelayerThresholdCall;
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
    #[doc = "Container type for all input parameters for the `_totalRelayers`function with signature `_totalRelayers()` and selector `[128, 42, 171, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_totalRelayers", abi = "_totalRelayers()")]
    pub struct TotalRelayersCall;
    #[doc = "Container type for all input parameters for the `adminAddRelayer`function with signature `adminAddRelayer(address)` and selector `[205, 176, 247, 58]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "adminAddRelayer", abi = "adminAddRelayer(address)")]
    pub struct AdminAddRelayerCall {
        pub relayer_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `adminChangeFee`function with signature `adminChangeFee(uint256)` and selector `[145, 196, 4, 172]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "adminChangeFee", abi = "adminChangeFee(uint256)")]
    pub struct AdminChangeFeeCall {
        pub new_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `adminChangeRelayerThreshold`function with signature `adminChangeRelayerThreshold(uint256)` and selector `[78, 5, 96, 5]`"]
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
        name = "adminChangeRelayerThreshold",
        abi = "adminChangeRelayerThreshold(uint256)"
    )]
    pub struct AdminChangeRelayerThresholdCall {
        pub new_threshold: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `adminPauseTransfers`function with signature `adminPauseTransfers()` and selector `[128, 174, 28, 40]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "adminPauseTransfers", abi = "adminPauseTransfers()")]
    pub struct AdminPauseTransfersCall;
    #[doc = "Container type for all input parameters for the `adminRemoveRelayer`function with signature `adminRemoveRelayer(address)` and selector `[157, 130, 221, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "adminRemoveRelayer", abi = "adminRemoveRelayer(address)")]
    pub struct AdminRemoveRelayerCall {
        pub relayer_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `adminSetBurnable`function with signature `adminSetBurnable(address,address)` and selector `[140, 12, 38, 49]`"]
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
        name = "adminSetBurnable",
        abi = "adminSetBurnable(address,address)"
    )]
    pub struct AdminSetBurnableCall {
        pub handler_address: ethers::core::types::Address,
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `adminSetResource`function with signature `adminSetResource(address,bytes32,address)` and selector `[203, 16, 242, 21]`"]
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
        name = "adminSetResource",
        abi = "adminSetResource(address,bytes32,address)"
    )]
    pub struct AdminSetResourceCall {
        pub handler_address: ethers::core::types::Address,
        pub resource_id: [u8; 32],
        pub execution_context_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `adminUnpauseTransfers`function with signature `adminUnpauseTransfers()` and selector `[255, 170, 192, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "adminUnpauseTransfers", abi = "adminUnpauseTransfers()")]
    pub struct AdminUnpauseTransfersCall;
    #[doc = "Container type for all input parameters for the `adminWithdraw`function with signature `adminWithdraw(address,address,address,uint256)` and selector `[120, 12, 240, 4]`"]
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
        name = "adminWithdraw",
        abi = "adminWithdraw(address,address,address,uint256)"
    )]
    pub struct AdminWithdrawCall {
        pub handler_address: ethers::core::types::Address,
        pub token_address: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount_or_token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `cancelProposal`function with signature `cancelProposal(uint256,uint64,bytes32)` and selector `[112, 153, 64, 230]`"]
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
        name = "cancelProposal",
        abi = "cancelProposal(uint256,uint64,bytes32)"
    )]
    pub struct CancelProposalCall {
        pub chain_id: ethers::core::types::U256,
        pub nonce: u64,
        pub data_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint32,bytes32,bytes)` and selector `[164, 79, 95, 230]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint32,bytes32,bytes)")]
    pub struct DepositCall {
        pub destination_chain_id: u32,
        pub resource_id: [u8; 32],
        pub data: Vec<u8>,
    }
    #[doc = "Container type for all input parameters for the `executeProposal`function with signature `executeProposal(uint256,uint64,bytes,bytes32)` and selector `[223, 239, 154, 83]`"]
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
        name = "executeProposal",
        abi = "executeProposal(uint256,uint64,bytes,bytes32)"
    )]
    pub struct ExecuteProposalCall {
        pub chain_id: ethers::core::types::U256,
        pub nonce: u64,
        pub data: Vec<u8>,
        pub resource_id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getProposal`function with signature `getProposal(uint256,uint64,bytes32)` and selector `[155, 48, 61, 122]`"]
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
        name = "getProposal",
        abi = "getProposal(uint256,uint64,bytes32)"
    )]
    pub struct GetProposalCall {
        pub origin_chain_id: ethers::core::types::U256,
        pub nonce: u64,
        pub data_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getRoleAdmin`function with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getRoleMember`function with signature `getRoleMember(bytes32,uint256)` and selector `[144, 16, 208, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRoleMember", abi = "getRoleMember(bytes32,uint256)")]
    pub struct GetRoleMemberCall {
        pub role: [u8; 32],
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getRoleMemberCount`function with signature `getRoleMemberCount(bytes32)` and selector `[202, 21, 200, 115]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
        pub role: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getRoleMemberIndex`function with signature `getRoleMemberIndex(bytes32,address)` and selector `[78, 13, 243, 246]`"]
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
        name = "getRoleMemberIndex",
        abi = "getRoleMemberIndex(bytes32,address)"
    )]
    pub struct GetRoleMemberIndexCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `grantRole`function with signature `grantRole(bytes32,address)` and selector `[47, 47, 241, 93]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hasRole`function with signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `isRelayer`function with signature `isRelayer(address)` and selector `[84, 29, 85, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isRelayer", abi = "isRelayer(address)")]
    pub struct IsRelayerCall {
        pub relayer: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `renounceAdmin`function with signature `renounceAdmin(address)` and selector `[94, 31, 171, 15]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceAdmin", abi = "renounceAdmin(address)")]
    pub struct RenounceAdminCall {
        pub new_admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `renounceRole`function with signature `renounceRole(bytes32,address)` and selector `[54, 86, 138, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `revokeRole`function with signature `revokeRole(bytes32,address)` and selector `[213, 71, 116, 31]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transferFunds`function with signature `transferFunds(address[],uint256[])` and selector `[70, 3, 174, 56]`"]
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
        name = "transferFunds",
        abi = "transferFunds(address[],uint256[])"
    )]
    pub struct TransferFundsCall {
        pub addrs: ::std::vec::Vec<ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `voteProposal`function with signature `voteProposal(uint256,uint64,bytes32,bytes32)` and selector `[160, 111, 160, 155]`"]
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
        name = "voteProposal",
        abi = "voteProposal(uint256,uint64,bytes32,bytes32)"
    )]
    pub struct VoteProposalCall {
        pub chain_id: ethers::core::types::U256,
        pub nonce: u64,
        pub resource_id: [u8; 32],
        pub data_hash: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum BridgeContractCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        MaxRelayers(MaxRelayersCall),
        RelayerRole(RelayerRoleCall),
        ChainID(ChainIDCall),
        Counts(CountsCall),
        Expiry(ExpiryCall),
        Fee(FeeCall),
        HasVotedOnProposal(HasVotedOnProposalCall),
        RelayerThreshold(RelayerThresholdCall),
        ResourceIDToHandlerAddress(ResourceIDToHandlerAddressCall),
        TotalRelayers(TotalRelayersCall),
        AdminAddRelayer(AdminAddRelayerCall),
        AdminChangeFee(AdminChangeFeeCall),
        AdminChangeRelayerThreshold(AdminChangeRelayerThresholdCall),
        AdminPauseTransfers(AdminPauseTransfersCall),
        AdminRemoveRelayer(AdminRemoveRelayerCall),
        AdminSetBurnable(AdminSetBurnableCall),
        AdminSetResource(AdminSetResourceCall),
        AdminUnpauseTransfers(AdminUnpauseTransfersCall),
        AdminWithdraw(AdminWithdrawCall),
        CancelProposal(CancelProposalCall),
        Deposit(DepositCall),
        ExecuteProposal(ExecuteProposalCall),
        GetProposal(GetProposalCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GetRoleMemberIndex(GetRoleMemberIndexCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IsRelayer(IsRelayerCall),
        Paused(PausedCall),
        RenounceAdmin(RenounceAdminCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        TransferFunds(TransferFundsCall),
        VoteProposal(VoteProposalCall),
    }
    impl ethers::core::abi::AbiDecode for BridgeContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <MaxRelayersCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::MaxRelayers(decoded));
            }
            if let Ok(decoded) =
                <RelayerRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::RelayerRole(decoded));
            }
            if let Ok(decoded) =
                <ChainIDCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::ChainID(decoded));
            }
            if let Ok(decoded) =
                <CountsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::Counts(decoded));
            }
            if let Ok(decoded) =
                <ExpiryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::Expiry(decoded));
            }
            if let Ok(decoded) =
                <FeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(BridgeContractCalls::Fee(decoded));
            }
            if let Ok(decoded) =
                <HasVotedOnProposalCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::HasVotedOnProposal(decoded));
            }
            if let Ok(decoded) =
                <RelayerThresholdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::RelayerThreshold(decoded));
            }
            if let Ok (decoded) = < ResourceIDToHandlerAddressCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (BridgeContractCalls :: ResourceIDToHandlerAddress (decoded)) }
            if let Ok(decoded) =
                <TotalRelayersCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::TotalRelayers(decoded));
            }
            if let Ok(decoded) =
                <AdminAddRelayerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::AdminAddRelayer(decoded));
            }
            if let Ok(decoded) =
                <AdminChangeFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::AdminChangeFee(decoded));
            }
            if let Ok (decoded) = < AdminChangeRelayerThresholdCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (BridgeContractCalls :: AdminChangeRelayerThreshold (decoded)) }
            if let Ok (decoded) = < AdminPauseTransfersCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (BridgeContractCalls :: AdminPauseTransfers (decoded)) }
            if let Ok(decoded) =
                <AdminRemoveRelayerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::AdminRemoveRelayer(decoded));
            }
            if let Ok(decoded) =
                <AdminSetBurnableCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::AdminSetBurnable(decoded));
            }
            if let Ok(decoded) =
                <AdminSetResourceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::AdminSetResource(decoded));
            }
            if let Ok (decoded) = < AdminUnpauseTransfersCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (BridgeContractCalls :: AdminUnpauseTransfers (decoded)) }
            if let Ok(decoded) =
                <AdminWithdrawCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::AdminWithdraw(decoded));
            }
            if let Ok(decoded) =
                <CancelProposalCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::CancelProposal(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <ExecuteProposalCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::ExecuteProposal(decoded));
            }
            if let Ok(decoded) =
                <GetProposalCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::GetProposal(decoded));
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetRoleMemberCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::GetRoleMember(decoded));
            }
            if let Ok(decoded) =
                <GetRoleMemberCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::GetRoleMemberCount(decoded));
            }
            if let Ok(decoded) =
                <GetRoleMemberIndexCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::GetRoleMemberIndex(decoded));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::GrantRole(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::HasRole(decoded));
            }
            if let Ok(decoded) =
                <IsRelayerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::IsRelayer(decoded));
            }
            if let Ok(decoded) =
                <PausedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <RenounceAdminCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::RenounceAdmin(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <TransferFundsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::TransferFunds(decoded));
            }
            if let Ok(decoded) =
                <VoteProposalCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(BridgeContractCalls::VoteProposal(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for BridgeContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                BridgeContractCalls::DefaultAdminRole(element) => {
                    element.encode()
                }
                BridgeContractCalls::MaxRelayers(element) => element.encode(),
                BridgeContractCalls::RelayerRole(element) => element.encode(),
                BridgeContractCalls::ChainID(element) => element.encode(),
                BridgeContractCalls::Counts(element) => element.encode(),
                BridgeContractCalls::Expiry(element) => element.encode(),
                BridgeContractCalls::Fee(element) => element.encode(),
                BridgeContractCalls::HasVotedOnProposal(element) => {
                    element.encode()
                }
                BridgeContractCalls::RelayerThreshold(element) => {
                    element.encode()
                }
                BridgeContractCalls::ResourceIDToHandlerAddress(element) => {
                    element.encode()
                }
                BridgeContractCalls::TotalRelayers(element) => element.encode(),
                BridgeContractCalls::AdminAddRelayer(element) => {
                    element.encode()
                }
                BridgeContractCalls::AdminChangeFee(element) => {
                    element.encode()
                }
                BridgeContractCalls::AdminChangeRelayerThreshold(element) => {
                    element.encode()
                }
                BridgeContractCalls::AdminPauseTransfers(element) => {
                    element.encode()
                }
                BridgeContractCalls::AdminRemoveRelayer(element) => {
                    element.encode()
                }
                BridgeContractCalls::AdminSetBurnable(element) => {
                    element.encode()
                }
                BridgeContractCalls::AdminSetResource(element) => {
                    element.encode()
                }
                BridgeContractCalls::AdminUnpauseTransfers(element) => {
                    element.encode()
                }
                BridgeContractCalls::AdminWithdraw(element) => element.encode(),
                BridgeContractCalls::CancelProposal(element) => {
                    element.encode()
                }
                BridgeContractCalls::Deposit(element) => element.encode(),
                BridgeContractCalls::ExecuteProposal(element) => {
                    element.encode()
                }
                BridgeContractCalls::GetProposal(element) => element.encode(),
                BridgeContractCalls::GetRoleAdmin(element) => element.encode(),
                BridgeContractCalls::GetRoleMember(element) => element.encode(),
                BridgeContractCalls::GetRoleMemberCount(element) => {
                    element.encode()
                }
                BridgeContractCalls::GetRoleMemberIndex(element) => {
                    element.encode()
                }
                BridgeContractCalls::GrantRole(element) => element.encode(),
                BridgeContractCalls::HasRole(element) => element.encode(),
                BridgeContractCalls::IsRelayer(element) => element.encode(),
                BridgeContractCalls::Paused(element) => element.encode(),
                BridgeContractCalls::RenounceAdmin(element) => element.encode(),
                BridgeContractCalls::RenounceRole(element) => element.encode(),
                BridgeContractCalls::RevokeRole(element) => element.encode(),
                BridgeContractCalls::TransferFunds(element) => element.encode(),
                BridgeContractCalls::VoteProposal(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for BridgeContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                BridgeContractCalls::DefaultAdminRole(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::MaxRelayers(element) => element.fmt(f),
                BridgeContractCalls::RelayerRole(element) => element.fmt(f),
                BridgeContractCalls::ChainID(element) => element.fmt(f),
                BridgeContractCalls::Counts(element) => element.fmt(f),
                BridgeContractCalls::Expiry(element) => element.fmt(f),
                BridgeContractCalls::Fee(element) => element.fmt(f),
                BridgeContractCalls::HasVotedOnProposal(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::RelayerThreshold(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::ResourceIDToHandlerAddress(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::TotalRelayers(element) => element.fmt(f),
                BridgeContractCalls::AdminAddRelayer(element) => element.fmt(f),
                BridgeContractCalls::AdminChangeFee(element) => element.fmt(f),
                BridgeContractCalls::AdminChangeRelayerThreshold(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::AdminPauseTransfers(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::AdminRemoveRelayer(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::AdminSetBurnable(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::AdminSetResource(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::AdminUnpauseTransfers(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::AdminWithdraw(element) => element.fmt(f),
                BridgeContractCalls::CancelProposal(element) => element.fmt(f),
                BridgeContractCalls::Deposit(element) => element.fmt(f),
                BridgeContractCalls::ExecuteProposal(element) => element.fmt(f),
                BridgeContractCalls::GetProposal(element) => element.fmt(f),
                BridgeContractCalls::GetRoleAdmin(element) => element.fmt(f),
                BridgeContractCalls::GetRoleMember(element) => element.fmt(f),
                BridgeContractCalls::GetRoleMemberCount(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::GetRoleMemberIndex(element) => {
                    element.fmt(f)
                }
                BridgeContractCalls::GrantRole(element) => element.fmt(f),
                BridgeContractCalls::HasRole(element) => element.fmt(f),
                BridgeContractCalls::IsRelayer(element) => element.fmt(f),
                BridgeContractCalls::Paused(element) => element.fmt(f),
                BridgeContractCalls::RenounceAdmin(element) => element.fmt(f),
                BridgeContractCalls::RenounceRole(element) => element.fmt(f),
                BridgeContractCalls::RevokeRole(element) => element.fmt(f),
                BridgeContractCalls::TransferFunds(element) => element.fmt(f),
                BridgeContractCalls::VoteProposal(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DefaultAdminRoleCall> for BridgeContractCalls {
        fn from(var: DefaultAdminRoleCall) -> Self {
            BridgeContractCalls::DefaultAdminRole(var)
        }
    }
    impl ::std::convert::From<MaxRelayersCall> for BridgeContractCalls {
        fn from(var: MaxRelayersCall) -> Self {
            BridgeContractCalls::MaxRelayers(var)
        }
    }
    impl ::std::convert::From<RelayerRoleCall> for BridgeContractCalls {
        fn from(var: RelayerRoleCall) -> Self {
            BridgeContractCalls::RelayerRole(var)
        }
    }
    impl ::std::convert::From<ChainIDCall> for BridgeContractCalls {
        fn from(var: ChainIDCall) -> Self {
            BridgeContractCalls::ChainID(var)
        }
    }
    impl ::std::convert::From<CountsCall> for BridgeContractCalls {
        fn from(var: CountsCall) -> Self {
            BridgeContractCalls::Counts(var)
        }
    }
    impl ::std::convert::From<ExpiryCall> for BridgeContractCalls {
        fn from(var: ExpiryCall) -> Self {
            BridgeContractCalls::Expiry(var)
        }
    }
    impl ::std::convert::From<FeeCall> for BridgeContractCalls {
        fn from(var: FeeCall) -> Self {
            BridgeContractCalls::Fee(var)
        }
    }
    impl ::std::convert::From<HasVotedOnProposalCall> for BridgeContractCalls {
        fn from(var: HasVotedOnProposalCall) -> Self {
            BridgeContractCalls::HasVotedOnProposal(var)
        }
    }
    impl ::std::convert::From<RelayerThresholdCall> for BridgeContractCalls {
        fn from(var: RelayerThresholdCall) -> Self {
            BridgeContractCalls::RelayerThreshold(var)
        }
    }
    impl ::std::convert::From<ResourceIDToHandlerAddressCall>
        for BridgeContractCalls
    {
        fn from(var: ResourceIDToHandlerAddressCall) -> Self {
            BridgeContractCalls::ResourceIDToHandlerAddress(var)
        }
    }
    impl ::std::convert::From<TotalRelayersCall> for BridgeContractCalls {
        fn from(var: TotalRelayersCall) -> Self {
            BridgeContractCalls::TotalRelayers(var)
        }
    }
    impl ::std::convert::From<AdminAddRelayerCall> for BridgeContractCalls {
        fn from(var: AdminAddRelayerCall) -> Self {
            BridgeContractCalls::AdminAddRelayer(var)
        }
    }
    impl ::std::convert::From<AdminChangeFeeCall> for BridgeContractCalls {
        fn from(var: AdminChangeFeeCall) -> Self {
            BridgeContractCalls::AdminChangeFee(var)
        }
    }
    impl ::std::convert::From<AdminChangeRelayerThresholdCall>
        for BridgeContractCalls
    {
        fn from(var: AdminChangeRelayerThresholdCall) -> Self {
            BridgeContractCalls::AdminChangeRelayerThreshold(var)
        }
    }
    impl ::std::convert::From<AdminPauseTransfersCall> for BridgeContractCalls {
        fn from(var: AdminPauseTransfersCall) -> Self {
            BridgeContractCalls::AdminPauseTransfers(var)
        }
    }
    impl ::std::convert::From<AdminRemoveRelayerCall> for BridgeContractCalls {
        fn from(var: AdminRemoveRelayerCall) -> Self {
            BridgeContractCalls::AdminRemoveRelayer(var)
        }
    }
    impl ::std::convert::From<AdminSetBurnableCall> for BridgeContractCalls {
        fn from(var: AdminSetBurnableCall) -> Self {
            BridgeContractCalls::AdminSetBurnable(var)
        }
    }
    impl ::std::convert::From<AdminSetResourceCall> for BridgeContractCalls {
        fn from(var: AdminSetResourceCall) -> Self {
            BridgeContractCalls::AdminSetResource(var)
        }
    }
    impl ::std::convert::From<AdminUnpauseTransfersCall> for BridgeContractCalls {
        fn from(var: AdminUnpauseTransfersCall) -> Self {
            BridgeContractCalls::AdminUnpauseTransfers(var)
        }
    }
    impl ::std::convert::From<AdminWithdrawCall> for BridgeContractCalls {
        fn from(var: AdminWithdrawCall) -> Self {
            BridgeContractCalls::AdminWithdraw(var)
        }
    }
    impl ::std::convert::From<CancelProposalCall> for BridgeContractCalls {
        fn from(var: CancelProposalCall) -> Self {
            BridgeContractCalls::CancelProposal(var)
        }
    }
    impl ::std::convert::From<DepositCall> for BridgeContractCalls {
        fn from(var: DepositCall) -> Self {
            BridgeContractCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<ExecuteProposalCall> for BridgeContractCalls {
        fn from(var: ExecuteProposalCall) -> Self {
            BridgeContractCalls::ExecuteProposal(var)
        }
    }
    impl ::std::convert::From<GetProposalCall> for BridgeContractCalls {
        fn from(var: GetProposalCall) -> Self {
            BridgeContractCalls::GetProposal(var)
        }
    }
    impl ::std::convert::From<GetRoleAdminCall> for BridgeContractCalls {
        fn from(var: GetRoleAdminCall) -> Self {
            BridgeContractCalls::GetRoleAdmin(var)
        }
    }
    impl ::std::convert::From<GetRoleMemberCall> for BridgeContractCalls {
        fn from(var: GetRoleMemberCall) -> Self {
            BridgeContractCalls::GetRoleMember(var)
        }
    }
    impl ::std::convert::From<GetRoleMemberCountCall> for BridgeContractCalls {
        fn from(var: GetRoleMemberCountCall) -> Self {
            BridgeContractCalls::GetRoleMemberCount(var)
        }
    }
    impl ::std::convert::From<GetRoleMemberIndexCall> for BridgeContractCalls {
        fn from(var: GetRoleMemberIndexCall) -> Self {
            BridgeContractCalls::GetRoleMemberIndex(var)
        }
    }
    impl ::std::convert::From<GrantRoleCall> for BridgeContractCalls {
        fn from(var: GrantRoleCall) -> Self {
            BridgeContractCalls::GrantRole(var)
        }
    }
    impl ::std::convert::From<HasRoleCall> for BridgeContractCalls {
        fn from(var: HasRoleCall) -> Self {
            BridgeContractCalls::HasRole(var)
        }
    }
    impl ::std::convert::From<IsRelayerCall> for BridgeContractCalls {
        fn from(var: IsRelayerCall) -> Self {
            BridgeContractCalls::IsRelayer(var)
        }
    }
    impl ::std::convert::From<PausedCall> for BridgeContractCalls {
        fn from(var: PausedCall) -> Self {
            BridgeContractCalls::Paused(var)
        }
    }
    impl ::std::convert::From<RenounceAdminCall> for BridgeContractCalls {
        fn from(var: RenounceAdminCall) -> Self {
            BridgeContractCalls::RenounceAdmin(var)
        }
    }
    impl ::std::convert::From<RenounceRoleCall> for BridgeContractCalls {
        fn from(var: RenounceRoleCall) -> Self {
            BridgeContractCalls::RenounceRole(var)
        }
    }
    impl ::std::convert::From<RevokeRoleCall> for BridgeContractCalls {
        fn from(var: RevokeRoleCall) -> Self {
            BridgeContractCalls::RevokeRole(var)
        }
    }
    impl ::std::convert::From<TransferFundsCall> for BridgeContractCalls {
        fn from(var: TransferFundsCall) -> Self {
            BridgeContractCalls::TransferFunds(var)
        }
    }
    impl ::std::convert::From<VoteProposalCall> for BridgeContractCalls {
        fn from(var: VoteProposalCall) -> Self {
            BridgeContractCalls::VoteProposal(var)
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
