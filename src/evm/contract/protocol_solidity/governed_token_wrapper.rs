pub use governedtokenwrappercontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod governedtokenwrappercontract_mod {
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
    #[doc = "GovernedTokenWrapperContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static GOVERNEDTOKENWRAPPERCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\"},{\"internalType\":\"address payable\",\"name\":\"_feeRecipient\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_governor\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"_isNativeAllowed\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Approval\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"Paused\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"previousAdminRole\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"newAdminRole\",\"type\":\"bytes32\"}],\"name\":\"RoleAdminChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleGranted\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"RoleRevoked\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"Transfer\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"Unpaused\",\"type\":\"event\"},{\"inputs\":[],\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"MINTER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"PAUSER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\"}],\"name\":\"add\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"}],\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"burn\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"burnFrom\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\"}],\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"feeRecipient\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_deposit\",\"type\":\"uint256\"}],\"name\":\"getAmountToWrap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getFee\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amountToWrap\",\"type\":\"uint256\"}],\"name\":\"getFeeFromAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"}],\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"getRoleMember\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"}],\"name\":\"getRoleMemberCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getTokens\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"governor\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"grantRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"historicalTokens\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\"}],\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"isNativeAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"mint\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"pause\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"proposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\"}],\"name\":\"remove\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"renounceRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"revokeRole\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"_feePercentage\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\"}],\"name\":\"setFee\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"_feeRecipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\"}],\"name\":\"setFeeRecipient\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_governor\",\"type\":\"address\"}],\"name\":\"setGovernor\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_isNativeAllowed\",\"type\":\"bool\"}],\"name\":\"setNativeAllowed\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\"}],\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"tokens\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"unpause\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"unwrap\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"unwrapAndSendTo\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"unwrapFor\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\"}],\"name\":\"updateLimit\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"wrap\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"wrapFor\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"wrapForAndSendTo\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"wrappingLimit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct GovernedTokenWrapperContract<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for GovernedTokenWrapperContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for GovernedTokenWrapperContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(GovernedTokenWrapperContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> GovernedTokenWrapperContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                GOVERNEDTOKENWRAPPERCONTRACT_ABI.clone(),
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
        #[doc = "Calls the contract's `MINTER_ROLE` (0xd5391393) function"]
        pub fn minter_role(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([213, 57, 19, 147], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PAUSER_ROLE` (0xe63ab1e9) function"]
        pub fn pauser_role(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([230, 58, 177, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `add` (0xf5d82b6b) function"]
        pub fn add(
            &self,
            token_address: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 216, 43, 107], (token_address, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x42966c68) function"]
        pub fn burn(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burnFrom` (0x79cc6790) function"]
        pub fn burn_from(
            &self,
            account: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 204, 103, 144], (account, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ethers::core::types::Address,
            subtracted_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeRecipient` (0x46904840) function"]
        pub fn fee_recipient(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 144, 72, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountToWrap` (0x96cd4dfe) function"]
        pub fn get_amount_to_wrap(
            &self,
            deposit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([150, 205, 77, 254], deposit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFee` (0xced72f87) function"]
        pub fn get_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([206, 215, 47, 135], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFeeFromAmount` (0x85c00ae8) function"]
        pub fn get_fee_from_amount(
            &self,
            amount_to_wrap: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([133, 192, 10, 232], amount_to_wrap)
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
        #[doc = "Calls the contract's `getTokens` (0xaa6ca808) function"]
        pub fn get_tokens(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([170, 108, 168, 8], ())
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
        #[doc = "Calls the contract's `historicalTokens` (0x85d14834) function"]
        pub fn historical_tokens(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([133, 209, 72, 52], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ethers::core::types::Address,
            added_value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isNativeAllowed` (0xb3e4083f) function"]
        pub fn is_native_allowed(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([179, 228, 8, 63], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(
            &self,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pause` (0x8456cb59) function"]
        pub fn pause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
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
        #[doc = "Calls the contract's `remove` (0xabe7f1ab) function"]
        pub fn remove(
            &self,
            token_address: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 231, 241, 171], (token_address, nonce))
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
        #[doc = "Calls the contract's `setFee` (0x8c6aa3f5) function"]
        pub fn set_fee(
            &self,
            fee_percentage: u8,
            nonce: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 106, 163, 245], (fee_percentage, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeRecipient` (0xc602ac14) function"]
        pub fn set_fee_recipient(
            &self,
            fee_recipient: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 2, 172, 20], (fee_recipient, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGovernor` (0xc42cf535) function"]
        pub fn set_governor(
            &self,
            governor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 44, 245, 53], governor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setNativeAllowed` (0x8b5478b9) function"]
        pub fn set_native_allowed(
            &self,
            is_native_allowed: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 84, 120, 185], is_native_allowed)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokens` (0x4f64b2be) function"]
        pub fn tokens(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([79, 100, 178, 190], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            sender: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrap` (0x39f47693) function"]
        pub fn unwrap(
            &self,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 244, 118, 147], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapAndSendTo` (0x4808285e) function"]
        pub fn unwrap_and_send_to(
            &self,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [72, 8, 40, 94],
                    (token_address, amount, recipient),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapFor` (0x261c80b6) function"]
        pub fn unwrap_for(
            &self,
            sender: ethers::core::types::Address,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [38, 28, 128, 182],
                    (sender, token_address, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateLimit` (0xfae0959a) function"]
        pub fn update_limit(
            &self,
            limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 224, 149, 154], limit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrap` (0xbf376c7a) function"]
        pub fn wrap(
            &self,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 55, 108, 122], (token_address, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapFor` (0x2ca69388) function"]
        pub fn wrap_for(
            &self,
            sender: ethers::core::types::Address,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [44, 166, 147, 136],
                    (sender, token_address, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrapForAndSendTo` (0x7b2e30d6) function"]
        pub fn wrap_for_and_send_to(
            &self,
            sender: ethers::core::types::Address,
            token_address: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [123, 46, 48, 214],
                    (sender, token_address, amount, recipient),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrappingLimit` (0x1f914382) function"]
        pub fn wrapping_limit(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
            self.0
                .method_hash([31, 145, 67, 130], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleAdminChanged` event"]
        pub fn role_admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleAdminChangedFilter>
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
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferFilter> {
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
        ) -> ethers::contract::builders::Event<
            M,
            GovernedTokenWrapperContractEvents,
        > {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum GovernedTokenWrapperContractEvents {
        ApprovalFilter(ApprovalFilter),
        PausedFilter(PausedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        TransferFilter(TransferFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for GovernedTokenWrapperContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(GovernedTokenWrapperContractEvents::ApprovalFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(GovernedTokenWrapperContractEvents::PausedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(
                    GovernedTokenWrapperContractEvents::RoleAdminChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(
                    GovernedTokenWrapperContractEvents::RoleGrantedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(
                    GovernedTokenWrapperContractEvents::RoleRevokedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(GovernedTokenWrapperContractEvents::TransferFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(GovernedTokenWrapperContractEvents::UnpausedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for GovernedTokenWrapperContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GovernedTokenWrapperContractEvents::ApprovalFilter(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractEvents::PausedFilter(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractEvents::RoleAdminChangedFilter(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractEvents::RoleGrantedFilter(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractEvents::RoleRevokedFilter(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractEvents::TransferFilter(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractEvents::UnpausedFilter(element) => {
                    element.fmt(f)
                }
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    #[doc = "Container type for all input parameters for the `MINTER_ROLE`function with signature `MINTER_ROLE()` and selector `[213, 57, 19, 147]`"]
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
    #[ethcall(name = "MINTER_ROLE", abi = "MINTER_ROLE()")]
    pub struct MinterRoleCall;
    #[doc = "Container type for all input parameters for the `PAUSER_ROLE`function with signature `PAUSER_ROLE()` and selector `[230, 58, 177, 233]`"]
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
    #[ethcall(name = "PAUSER_ROLE", abi = "PAUSER_ROLE()")]
    pub struct PauserRoleCall;
    #[doc = "Container type for all input parameters for the `add`function with signature `add(address,uint256)` and selector `[245, 216, 43, 107]`"]
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
    #[ethcall(name = "add", abi = "add(address,uint256)")]
    pub struct AddCall {
        pub token_address: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `burn`function with signature `burn(uint256)` and selector `[66, 150, 108, 104]`"]
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
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `burnFrom`function with signature `burnFrom(address,uint256)` and selector `[121, 204, 103, 144]`"]
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
    #[ethcall(name = "burnFrom", abi = "burnFrom(address,uint256)")]
    pub struct BurnFromCall {
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `decreaseAllowance`function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
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
        name = "decreaseAllowance",
        abi = "decreaseAllowance(address,uint256)"
    )]
    pub struct DecreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub subtracted_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `feeRecipient`function with signature `feeRecipient()` and selector `[70, 144, 72, 64]`"]
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
    #[ethcall(name = "feeRecipient", abi = "feeRecipient()")]
    pub struct FeeRecipientCall;
    #[doc = "Container type for all input parameters for the `getAmountToWrap`function with signature `getAmountToWrap(uint256)` and selector `[150, 205, 77, 254]`"]
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
    #[ethcall(name = "getAmountToWrap", abi = "getAmountToWrap(uint256)")]
    pub struct GetAmountToWrapCall {
        pub deposit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getFee`function with signature `getFee()` and selector `[206, 215, 47, 135]`"]
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
    #[ethcall(name = "getFee", abi = "getFee()")]
    pub struct GetFeeCall;
    #[doc = "Container type for all input parameters for the `getFeeFromAmount`function with signature `getFeeFromAmount(uint256)` and selector `[133, 192, 10, 232]`"]
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
    #[ethcall(name = "getFeeFromAmount", abi = "getFeeFromAmount(uint256)")]
    pub struct GetFeeFromAmountCall {
        pub amount_to_wrap: ethers::core::types::U256,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
        pub role: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getTokens`function with signature `getTokens()` and selector `[170, 108, 168, 8]`"]
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
    #[ethcall(name = "getTokens", abi = "getTokens()")]
    pub struct GetTokensCall;
    #[doc = "Container type for all input parameters for the `governor`function with signature `governor()` and selector `[12, 52, 10, 36]`"]
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
    #[doc = "Container type for all input parameters for the `grantRole`function with signature `grantRole(bytes32,address)` and selector `[47, 47, 241, 93]`"]
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `historicalTokens`function with signature `historicalTokens(uint256)` and selector `[133, 209, 72, 52]`"]
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
    #[ethcall(name = "historicalTokens", abi = "historicalTokens(uint256)")]
    pub struct HistoricalTokensCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `increaseAllowance`function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
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
        name = "increaseAllowance",
        abi = "increaseAllowance(address,uint256)"
    )]
    pub struct IncreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub added_value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isNativeAllowed`function with signature `isNativeAllowed()` and selector `[179, 228, 8, 63]`"]
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
    #[ethcall(name = "isNativeAllowed", abi = "isNativeAllowed()")]
    pub struct IsNativeAllowedCall;
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(address,uint256)` and selector `[64, 193, 15, 25]`"]
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
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `pause`function with signature `pause()` and selector `[132, 86, 203, 89]`"]
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
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    #[doc = "Container type for all input parameters for the `paused`function with signature `paused()` and selector `[92, 151, 90, 187]`"]
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
    #[doc = "Container type for all input parameters for the `proposalNonce`function with signature `proposalNonce()` and selector `[204, 60, 116, 161]`"]
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
    #[doc = "Container type for all input parameters for the `remove`function with signature `remove(address,uint256)` and selector `[171, 231, 241, 171]`"]
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
    #[ethcall(name = "remove", abi = "remove(address,uint256)")]
    pub struct RemoveCall {
        pub token_address: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setFee`function with signature `setFee(uint8,uint256)` and selector `[140, 106, 163, 245]`"]
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
    #[ethcall(name = "setFee", abi = "setFee(uint8,uint256)")]
    pub struct SetFeeCall {
        pub fee_percentage: u8,
        pub nonce: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setFeeRecipient`function with signature `setFeeRecipient(address,uint256)` and selector `[198, 2, 172, 20]`"]
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
        name = "setFeeRecipient",
        abi = "setFeeRecipient(address,uint256)"
    )]
    pub struct SetFeeRecipientCall {
        pub fee_recipient: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setGovernor`function with signature `setGovernor(address)` and selector `[196, 44, 245, 53]`"]
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
    #[ethcall(name = "setGovernor", abi = "setGovernor(address)")]
    pub struct SetGovernorCall {
        pub governor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setNativeAllowed`function with signature `setNativeAllowed(bool)` and selector `[139, 84, 120, 185]`"]
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
    #[ethcall(name = "setNativeAllowed", abi = "setNativeAllowed(bool)")]
    pub struct SetNativeAllowedCall {
        pub is_native_allowed: bool,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface`function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `tokens`function with signature `tokens(uint256)` and selector `[79, 100, 178, 190]`"]
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
    #[ethcall(name = "tokens", abi = "tokens(uint256)")]
    pub struct TokensCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
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
        name = "transferFrom",
        abi = "transferFrom(address,address,uint256)"
    )]
    pub struct TransferFromCall {
        pub sender: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unpause`function with signature `unpause()` and selector `[63, 75, 168, 58]`"]
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
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    #[doc = "Container type for all input parameters for the `unwrap`function with signature `unwrap(address,uint256)` and selector `[57, 244, 118, 147]`"]
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
    #[ethcall(name = "unwrap", abi = "unwrap(address,uint256)")]
    pub struct UnwrapCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unwrapAndSendTo`function with signature `unwrapAndSendTo(address,uint256,address)` and selector `[72, 8, 40, 94]`"]
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
        name = "unwrapAndSendTo",
        abi = "unwrapAndSendTo(address,uint256,address)"
    )]
    pub struct UnwrapAndSendToCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `unwrapFor`function with signature `unwrapFor(address,address,uint256)` and selector `[38, 28, 128, 182]`"]
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
    #[ethcall(name = "unwrapFor", abi = "unwrapFor(address,address,uint256)")]
    pub struct UnwrapForCall {
        pub sender: ethers::core::types::Address,
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateLimit`function with signature `updateLimit(uint256)` and selector `[250, 224, 149, 154]`"]
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
    #[ethcall(name = "updateLimit", abi = "updateLimit(uint256)")]
    pub struct UpdateLimitCall {
        pub limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `wrap`function with signature `wrap(address,uint256)` and selector `[191, 55, 108, 122]`"]
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
    #[ethcall(name = "wrap", abi = "wrap(address,uint256)")]
    pub struct WrapCall {
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `wrapFor`function with signature `wrapFor(address,address,uint256)` and selector `[44, 166, 147, 136]`"]
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
    #[ethcall(name = "wrapFor", abi = "wrapFor(address,address,uint256)")]
    pub struct WrapForCall {
        pub sender: ethers::core::types::Address,
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `wrapForAndSendTo`function with signature `wrapForAndSendTo(address,address,uint256,address)` and selector `[123, 46, 48, 214]`"]
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
        name = "wrapForAndSendTo",
        abi = "wrapForAndSendTo(address,address,uint256,address)"
    )]
    pub struct WrapForAndSendToCall {
        pub sender: ethers::core::types::Address,
        pub token_address: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `wrappingLimit`function with signature `wrappingLimit()` and selector `[31, 145, 67, 130]`"]
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
    #[ethcall(name = "wrappingLimit", abi = "wrappingLimit()")]
    pub struct WrappingLimitCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum GovernedTokenWrapperContractCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        MinterRole(MinterRoleCall),
        PauserRole(PauserRoleCall),
        Add(AddCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        BurnFrom(BurnFromCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        FeeRecipient(FeeRecipientCall),
        GetAmountToWrap(GetAmountToWrapCall),
        GetFee(GetFeeCall),
        GetFeeFromAmount(GetFeeFromAmountCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GetTokens(GetTokensCall),
        Governor(GovernorCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        HistoricalTokens(HistoricalTokensCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        IsNativeAllowed(IsNativeAllowedCall),
        Mint(MintCall),
        Name(NameCall),
        Pause(PauseCall),
        Paused(PausedCall),
        ProposalNonce(ProposalNonceCall),
        Remove(RemoveCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetFee(SetFeeCall),
        SetFeeRecipient(SetFeeRecipientCall),
        SetGovernor(SetGovernorCall),
        SetNativeAllowed(SetNativeAllowedCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        Tokens(TokensCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Unpause(UnpauseCall),
        Unwrap(UnwrapCall),
        UnwrapAndSendTo(UnwrapAndSendToCall),
        UnwrapFor(UnwrapForCall),
        UpdateLimit(UpdateLimitCall),
        Wrap(WrapCall),
        WrapFor(WrapForCall),
        WrapForAndSendTo(WrapForAndSendToCall),
        WrappingLimit(WrappingLimitCall),
    }
    impl ethers::core::abi::AbiDecode for GovernedTokenWrapperContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    GovernedTokenWrapperContractCalls::DefaultAdminRole(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <MinterRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::MinterRole(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PauserRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::PauserRole(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AddCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(GovernedTokenWrapperContractCalls::Add(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Allowance(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::BalanceOf(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <BurnCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <BurnFromCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::BurnFrom(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Decimals(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    GovernedTokenWrapperContractCalls::DecreaseAllowance(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <FeeRecipientCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::FeeRecipient(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetAmountToWrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::GetAmountToWrap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::GetFee(decoded));
            }
            if let Ok(decoded) =
                <GetFeeFromAmountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    GovernedTokenWrapperContractCalls::GetFeeFromAmount(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::GetRoleAdmin(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetRoleMemberCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::GetRoleMember(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetRoleMemberCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    GovernedTokenWrapperContractCalls::GetRoleMemberCount(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <GetTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::GetTokens(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GovernorCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Governor(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::GrantRole(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <HasRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::HasRole(decoded));
            }
            if let Ok(decoded) =
                <HistoricalTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    GovernedTokenWrapperContractCalls::HistoricalTokens(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    GovernedTokenWrapperContractCalls::IncreaseAllowance(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <IsNativeAllowedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::IsNativeAllowed(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MintCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Mint(decoded));
            }
            if let Ok(decoded) =
                <NameCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PauseCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Pause(decoded));
            }
            if let Ok(decoded) =
                <PausedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <ProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::ProposalNonce(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RemoveCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Remove(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::RenounceRole(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::RevokeRole(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::SetFee(decoded));
            }
            if let Ok(decoded) =
                <SetFeeRecipientCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::SetFeeRecipient(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetGovernorCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::SetGovernor(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetNativeAllowedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    GovernedTokenWrapperContractCalls::SetNativeAllowed(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    GovernedTokenWrapperContractCalls::SupportsInterface(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <SymbolCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Tokens(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::TotalSupply(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Transfer(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::TransferFrom(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UnwrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Unwrap(decoded));
            }
            if let Ok(decoded) =
                <UnwrapAndSendToCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::UnwrapAndSendTo(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UnwrapForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::UnwrapFor(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpdateLimitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::UpdateLimit(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::Wrap(decoded));
            }
            if let Ok(decoded) =
                <WrapForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::WrapFor(decoded));
            }
            if let Ok(decoded) =
                <WrapForAndSendToCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    GovernedTokenWrapperContractCalls::WrapForAndSendTo(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <WrappingLimitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(GovernedTokenWrapperContractCalls::WrappingLimit(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for GovernedTokenWrapperContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                GovernedTokenWrapperContractCalls::DefaultAdminRole(
                    element,
                ) => element.encode(),
                GovernedTokenWrapperContractCalls::MinterRole(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::PauserRole(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Add(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Allowance(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Approve(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::BalanceOf(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Burn(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::BurnFrom(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Decimals(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::DecreaseAllowance(
                    element,
                ) => element.encode(),
                GovernedTokenWrapperContractCalls::FeeRecipient(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::GetAmountToWrap(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::GetFee(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::GetFeeFromAmount(
                    element,
                ) => element.encode(),
                GovernedTokenWrapperContractCalls::GetRoleAdmin(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::GetRoleMember(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::GetRoleMemberCount(
                    element,
                ) => element.encode(),
                GovernedTokenWrapperContractCalls::GetTokens(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Governor(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::GrantRole(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::HasRole(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::HistoricalTokens(
                    element,
                ) => element.encode(),
                GovernedTokenWrapperContractCalls::IncreaseAllowance(
                    element,
                ) => element.encode(),
                GovernedTokenWrapperContractCalls::IsNativeAllowed(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Mint(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Name(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Pause(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Paused(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::ProposalNonce(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Remove(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::RenounceRole(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::RevokeRole(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::SetFee(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::SetFeeRecipient(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::SetGovernor(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::SetNativeAllowed(
                    element,
                ) => element.encode(),
                GovernedTokenWrapperContractCalls::SupportsInterface(
                    element,
                ) => element.encode(),
                GovernedTokenWrapperContractCalls::Symbol(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Tokens(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::TotalSupply(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Transfer(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::TransferFrom(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Unpause(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Unwrap(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::UnwrapAndSendTo(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::UnwrapFor(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::UpdateLimit(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::Wrap(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::WrapFor(element) => {
                    element.encode()
                }
                GovernedTokenWrapperContractCalls::WrapForAndSendTo(
                    element,
                ) => element.encode(),
                GovernedTokenWrapperContractCalls::WrappingLimit(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for GovernedTokenWrapperContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                GovernedTokenWrapperContractCalls::DefaultAdminRole(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractCalls::MinterRole(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::PauserRole(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Add(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Allowance(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Approve(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::BalanceOf(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Burn(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::BurnFrom(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Decimals(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::DecreaseAllowance(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractCalls::FeeRecipient(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::GetAmountToWrap(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::GetFee(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::GetFeeFromAmount(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractCalls::GetRoleAdmin(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::GetRoleMember(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::GetRoleMemberCount(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractCalls::GetTokens(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Governor(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::GrantRole(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::HasRole(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::HistoricalTokens(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractCalls::IncreaseAllowance(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractCalls::IsNativeAllowed(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Mint(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Name(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Pause(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Paused(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::ProposalNonce(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Remove(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::RenounceRole(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::RevokeRole(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::SetFee(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::SetFeeRecipient(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::SetGovernor(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::SetNativeAllowed(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractCalls::SupportsInterface(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractCalls::Symbol(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Tokens(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::TotalSupply(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Transfer(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::TransferFrom(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Unpause(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Unwrap(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::UnwrapAndSendTo(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::UnwrapFor(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::UpdateLimit(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::Wrap(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::WrapFor(element) => {
                    element.fmt(f)
                }
                GovernedTokenWrapperContractCalls::WrapForAndSendTo(
                    element,
                ) => element.fmt(f),
                GovernedTokenWrapperContractCalls::WrappingLimit(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<DefaultAdminRoleCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: DefaultAdminRoleCall) -> Self {
            GovernedTokenWrapperContractCalls::DefaultAdminRole(var)
        }
    }
    impl ::std::convert::From<MinterRoleCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: MinterRoleCall) -> Self {
            GovernedTokenWrapperContractCalls::MinterRole(var)
        }
    }
    impl ::std::convert::From<PauserRoleCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: PauserRoleCall) -> Self {
            GovernedTokenWrapperContractCalls::PauserRole(var)
        }
    }
    impl ::std::convert::From<AddCall> for GovernedTokenWrapperContractCalls {
        fn from(var: AddCall) -> Self {
            GovernedTokenWrapperContractCalls::Add(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for GovernedTokenWrapperContractCalls {
        fn from(var: AllowanceCall) -> Self {
            GovernedTokenWrapperContractCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for GovernedTokenWrapperContractCalls {
        fn from(var: ApproveCall) -> Self {
            GovernedTokenWrapperContractCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for GovernedTokenWrapperContractCalls {
        fn from(var: BalanceOfCall) -> Self {
            GovernedTokenWrapperContractCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for GovernedTokenWrapperContractCalls {
        fn from(var: BurnCall) -> Self {
            GovernedTokenWrapperContractCalls::Burn(var)
        }
    }
    impl ::std::convert::From<BurnFromCall> for GovernedTokenWrapperContractCalls {
        fn from(var: BurnFromCall) -> Self {
            GovernedTokenWrapperContractCalls::BurnFrom(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for GovernedTokenWrapperContractCalls {
        fn from(var: DecimalsCall) -> Self {
            GovernedTokenWrapperContractCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: DecreaseAllowanceCall) -> Self {
            GovernedTokenWrapperContractCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<FeeRecipientCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: FeeRecipientCall) -> Self {
            GovernedTokenWrapperContractCalls::FeeRecipient(var)
        }
    }
    impl ::std::convert::From<GetAmountToWrapCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: GetAmountToWrapCall) -> Self {
            GovernedTokenWrapperContractCalls::GetAmountToWrap(var)
        }
    }
    impl ::std::convert::From<GetFeeCall> for GovernedTokenWrapperContractCalls {
        fn from(var: GetFeeCall) -> Self {
            GovernedTokenWrapperContractCalls::GetFee(var)
        }
    }
    impl ::std::convert::From<GetFeeFromAmountCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: GetFeeFromAmountCall) -> Self {
            GovernedTokenWrapperContractCalls::GetFeeFromAmount(var)
        }
    }
    impl ::std::convert::From<GetRoleAdminCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: GetRoleAdminCall) -> Self {
            GovernedTokenWrapperContractCalls::GetRoleAdmin(var)
        }
    }
    impl ::std::convert::From<GetRoleMemberCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: GetRoleMemberCall) -> Self {
            GovernedTokenWrapperContractCalls::GetRoleMember(var)
        }
    }
    impl ::std::convert::From<GetRoleMemberCountCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: GetRoleMemberCountCall) -> Self {
            GovernedTokenWrapperContractCalls::GetRoleMemberCount(var)
        }
    }
    impl ::std::convert::From<GetTokensCall> for GovernedTokenWrapperContractCalls {
        fn from(var: GetTokensCall) -> Self {
            GovernedTokenWrapperContractCalls::GetTokens(var)
        }
    }
    impl ::std::convert::From<GovernorCall> for GovernedTokenWrapperContractCalls {
        fn from(var: GovernorCall) -> Self {
            GovernedTokenWrapperContractCalls::Governor(var)
        }
    }
    impl ::std::convert::From<GrantRoleCall> for GovernedTokenWrapperContractCalls {
        fn from(var: GrantRoleCall) -> Self {
            GovernedTokenWrapperContractCalls::GrantRole(var)
        }
    }
    impl ::std::convert::From<HasRoleCall> for GovernedTokenWrapperContractCalls {
        fn from(var: HasRoleCall) -> Self {
            GovernedTokenWrapperContractCalls::HasRole(var)
        }
    }
    impl ::std::convert::From<HistoricalTokensCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: HistoricalTokensCall) -> Self {
            GovernedTokenWrapperContractCalls::HistoricalTokens(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: IncreaseAllowanceCall) -> Self {
            GovernedTokenWrapperContractCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<IsNativeAllowedCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: IsNativeAllowedCall) -> Self {
            GovernedTokenWrapperContractCalls::IsNativeAllowed(var)
        }
    }
    impl ::std::convert::From<MintCall> for GovernedTokenWrapperContractCalls {
        fn from(var: MintCall) -> Self {
            GovernedTokenWrapperContractCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for GovernedTokenWrapperContractCalls {
        fn from(var: NameCall) -> Self {
            GovernedTokenWrapperContractCalls::Name(var)
        }
    }
    impl ::std::convert::From<PauseCall> for GovernedTokenWrapperContractCalls {
        fn from(var: PauseCall) -> Self {
            GovernedTokenWrapperContractCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for GovernedTokenWrapperContractCalls {
        fn from(var: PausedCall) -> Self {
            GovernedTokenWrapperContractCalls::Paused(var)
        }
    }
    impl ::std::convert::From<ProposalNonceCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: ProposalNonceCall) -> Self {
            GovernedTokenWrapperContractCalls::ProposalNonce(var)
        }
    }
    impl ::std::convert::From<RemoveCall> for GovernedTokenWrapperContractCalls {
        fn from(var: RemoveCall) -> Self {
            GovernedTokenWrapperContractCalls::Remove(var)
        }
    }
    impl ::std::convert::From<RenounceRoleCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: RenounceRoleCall) -> Self {
            GovernedTokenWrapperContractCalls::RenounceRole(var)
        }
    }
    impl ::std::convert::From<RevokeRoleCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: RevokeRoleCall) -> Self {
            GovernedTokenWrapperContractCalls::RevokeRole(var)
        }
    }
    impl ::std::convert::From<SetFeeCall> for GovernedTokenWrapperContractCalls {
        fn from(var: SetFeeCall) -> Self {
            GovernedTokenWrapperContractCalls::SetFee(var)
        }
    }
    impl ::std::convert::From<SetFeeRecipientCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: SetFeeRecipientCall) -> Self {
            GovernedTokenWrapperContractCalls::SetFeeRecipient(var)
        }
    }
    impl ::std::convert::From<SetGovernorCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: SetGovernorCall) -> Self {
            GovernedTokenWrapperContractCalls::SetGovernor(var)
        }
    }
    impl ::std::convert::From<SetNativeAllowedCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: SetNativeAllowedCall) -> Self {
            GovernedTokenWrapperContractCalls::SetNativeAllowed(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: SupportsInterfaceCall) -> Self {
            GovernedTokenWrapperContractCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for GovernedTokenWrapperContractCalls {
        fn from(var: SymbolCall) -> Self {
            GovernedTokenWrapperContractCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TokensCall> for GovernedTokenWrapperContractCalls {
        fn from(var: TokensCall) -> Self {
            GovernedTokenWrapperContractCalls::Tokens(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: TotalSupplyCall) -> Self {
            GovernedTokenWrapperContractCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for GovernedTokenWrapperContractCalls {
        fn from(var: TransferCall) -> Self {
            GovernedTokenWrapperContractCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: TransferFromCall) -> Self {
            GovernedTokenWrapperContractCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for GovernedTokenWrapperContractCalls {
        fn from(var: UnpauseCall) -> Self {
            GovernedTokenWrapperContractCalls::Unpause(var)
        }
    }
    impl ::std::convert::From<UnwrapCall> for GovernedTokenWrapperContractCalls {
        fn from(var: UnwrapCall) -> Self {
            GovernedTokenWrapperContractCalls::Unwrap(var)
        }
    }
    impl ::std::convert::From<UnwrapAndSendToCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: UnwrapAndSendToCall) -> Self {
            GovernedTokenWrapperContractCalls::UnwrapAndSendTo(var)
        }
    }
    impl ::std::convert::From<UnwrapForCall> for GovernedTokenWrapperContractCalls {
        fn from(var: UnwrapForCall) -> Self {
            GovernedTokenWrapperContractCalls::UnwrapFor(var)
        }
    }
    impl ::std::convert::From<UpdateLimitCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: UpdateLimitCall) -> Self {
            GovernedTokenWrapperContractCalls::UpdateLimit(var)
        }
    }
    impl ::std::convert::From<WrapCall> for GovernedTokenWrapperContractCalls {
        fn from(var: WrapCall) -> Self {
            GovernedTokenWrapperContractCalls::Wrap(var)
        }
    }
    impl ::std::convert::From<WrapForCall> for GovernedTokenWrapperContractCalls {
        fn from(var: WrapForCall) -> Self {
            GovernedTokenWrapperContractCalls::WrapFor(var)
        }
    }
    impl ::std::convert::From<WrapForAndSendToCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: WrapForAndSendToCall) -> Self {
            GovernedTokenWrapperContractCalls::WrapForAndSendTo(var)
        }
    }
    impl ::std::convert::From<WrappingLimitCall>
        for GovernedTokenWrapperContractCalls
    {
        fn from(var: WrappingLimitCall) -> Self {
            GovernedTokenWrapperContractCalls::WrappingLimit(var)
        }
    }
}
