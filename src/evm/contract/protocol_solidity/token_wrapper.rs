pub use token_wrapper_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod token_wrapper_contract {
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
    #[doc = "TokenWrapperContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TOKENWRAPPERCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"previousAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"newAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleAdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleGranted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleRevoked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MINTER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"PAUSER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnFrom\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeRecipient\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_deposit\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountToWrap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amountToWrap\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFeeFromAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleMember\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleMemberCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"grantRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"revokeRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_feePercentage\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrapAndSendTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrapFor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrapFor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrapForAndSendTo\",\"outputs\":[]}]") . expect ("invalid abi")
    });
    pub struct TokenWrapperContract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for TokenWrapperContract<M> {
        fn clone(&self) -> Self {
            TokenWrapperContract(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for TokenWrapperContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for TokenWrapperContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TokenWrapperContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> TokenWrapperContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                TOKENWRAPPERCONTRACT_ABI.clone(),
                client,
            )
            .into()
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
        #[doc = "Calls the contract's `setFee` (0xc2ae4720) function"]
        pub fn set_fee(
            &self,
            fee_percentage: u16,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 174, 71, 32], (fee_percentage, nonce))
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
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
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
        ) -> ethers::contract::builders::Event<M, TokenWrapperContractEvents>
        {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for TokenWrapperContract<M>
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum TokenWrapperContractEvents {
        ApprovalFilter(ApprovalFilter),
        PausedFilter(PausedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        TransferFilter(TransferFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for TokenWrapperContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(TokenWrapperContractEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(TokenWrapperContractEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(TokenWrapperContractEvents::RoleAdminChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(TokenWrapperContractEvents::RoleGrantedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(TokenWrapperContractEvents::RoleRevokedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(TokenWrapperContractEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(TokenWrapperContractEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for TokenWrapperContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TokenWrapperContractEvents::ApprovalFilter(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractEvents::PausedFilter(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractEvents::RoleAdminChangedFilter(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractEvents::RoleGrantedFilter(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractEvents::RoleRevokedFilter(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractEvents::TransferFilter(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractEvents::UnpausedFilter(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23, 253, 223]`"]
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
    #[doc = "Container type for all input parameters for the `MINTER_ROLE` function with signature `MINTER_ROLE()` and selector `[213, 57, 19, 147]`"]
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
    #[doc = "Container type for all input parameters for the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `[230, 58, 177, 233]`"]
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
    #[doc = "Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
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
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
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
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
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
    #[doc = "Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `[66, 150, 108, 104]`"]
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
    #[doc = "Container type for all input parameters for the `burnFrom` function with signature `burnFrom(address,uint256)` and selector `[121, 204, 103, 144]`"]
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
    #[doc = "Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
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
    #[doc = "Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
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
    #[doc = "Container type for all input parameters for the `feeRecipient` function with signature `feeRecipient()` and selector `[70, 144, 72, 64]`"]
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
    #[doc = "Container type for all input parameters for the `getAmountToWrap` function with signature `getAmountToWrap(uint256)` and selector `[150, 205, 77, 254]`"]
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
    #[doc = "Container type for all input parameters for the `getFeeFromAmount` function with signature `getFeeFromAmount(uint256)` and selector `[133, 192, 10, 232]`"]
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
    #[doc = "Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156, 163]`"]
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
    #[doc = "Container type for all input parameters for the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `[144, 16, 208, 124]`"]
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
    #[doc = "Container type for all input parameters for the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `[202, 21, 200, 115]`"]
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
    #[doc = "Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `[47, 47, 241, 93]`"]
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
    #[doc = "Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`"]
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
    #[doc = "Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
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
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `[64, 193, 15, 25]`"]
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
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
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
    #[doc = "Container type for all input parameters for the `pause` function with signature `pause()` and selector `[132, 86, 203, 89]`"]
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
    #[doc = "Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `[54, 86, 138, 190]`"]
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
    #[doc = "Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `[213, 71, 116, 31]`"]
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
    #[doc = "Container type for all input parameters for the `setFee` function with signature `setFee(uint16,uint32)` and selector `[194, 174, 71, 32]`"]
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
    #[ethcall(name = "setFee", abi = "setFee(uint16,uint32)")]
    pub struct SetFeeCall {
        pub fee_percentage: u16,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
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
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
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
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
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
    #[doc = "Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
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
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
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
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `[63, 75, 168, 58]`"]
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
    #[doc = "Container type for all input parameters for the `unwrap` function with signature `unwrap(address,uint256)` and selector `[57, 244, 118, 147]`"]
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
    #[doc = "Container type for all input parameters for the `unwrapAndSendTo` function with signature `unwrapAndSendTo(address,uint256,address)` and selector `[72, 8, 40, 94]`"]
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
    #[doc = "Container type for all input parameters for the `unwrapFor` function with signature `unwrapFor(address,address,uint256)` and selector `[38, 28, 128, 182]`"]
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
    #[doc = "Container type for all input parameters for the `wrap` function with signature `wrap(address,uint256)` and selector `[191, 55, 108, 122]`"]
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
    #[doc = "Container type for all input parameters for the `wrapFor` function with signature `wrapFor(address,address,uint256)` and selector `[44, 166, 147, 136]`"]
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
    #[doc = "Container type for all input parameters for the `wrapForAndSendTo` function with signature `wrapForAndSendTo(address,address,uint256,address)` and selector `[123, 46, 48, 214]`"]
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum TokenWrapperContractCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        MinterRole(MinterRoleCall),
        PauserRole(PauserRoleCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        BurnFrom(BurnFromCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        FeeRecipient(FeeRecipientCall),
        GetAmountToWrap(GetAmountToWrapCall),
        GetFeeFromAmount(GetFeeFromAmountCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Mint(MintCall),
        Name(NameCall),
        Pause(PauseCall),
        Paused(PausedCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetFee(SetFeeCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Unpause(UnpauseCall),
        Unwrap(UnwrapCall),
        UnwrapAndSendTo(UnwrapAndSendToCall),
        UnwrapFor(UnwrapForCall),
        Wrap(WrapCall),
        WrapFor(WrapForCall),
        WrapForAndSendTo(WrapForAndSendToCall),
    }
    impl ethers::core::abi::AbiDecode for TokenWrapperContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::DefaultAdminRole(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MinterRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::MinterRole(decoded));
            }
            if let Ok(decoded) =
                <PauserRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::PauserRole(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BurnCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <BurnFromCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::BurnFrom(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::DecreaseAllowance(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <FeeRecipientCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::FeeRecipient(decoded));
            }
            if let Ok(decoded) =
                <GetAmountToWrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::GetAmountToWrap(decoded));
            }
            if let Ok(decoded) =
                <GetFeeFromAmountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::GetFeeFromAmount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetRoleMemberCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::GetRoleMember(decoded));
            }
            if let Ok(decoded) =
                <GetRoleMemberCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::GetRoleMemberCount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::GrantRole(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::HasRole(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::IncreaseAllowance(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MintCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Mint(decoded));
            }
            if let Ok(decoded) =
                <NameCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PauseCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Pause(decoded));
            }
            if let Ok(decoded) =
                <PausedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <SetFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::SetFee(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::SupportsInterface(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SymbolCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UnwrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Unwrap(decoded));
            }
            if let Ok(decoded) =
                <UnwrapAndSendToCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::UnwrapAndSendTo(decoded));
            }
            if let Ok(decoded) =
                <UnwrapForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::UnwrapFor(decoded));
            }
            if let Ok(decoded) =
                <WrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::Wrap(decoded));
            }
            if let Ok(decoded) =
                <WrapForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::WrapFor(decoded));
            }
            if let Ok(decoded) =
                <WrapForAndSendToCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperContractCalls::WrapForAndSendTo(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TokenWrapperContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                TokenWrapperContractCalls::DefaultAdminRole(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::MinterRole(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::PauserRole(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::Allowance(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::Approve(element) => element.encode(),
                TokenWrapperContractCalls::BalanceOf(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::Burn(element) => element.encode(),
                TokenWrapperContractCalls::BurnFrom(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::Decimals(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::DecreaseAllowance(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::FeeRecipient(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::GetAmountToWrap(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::GetFeeFromAmount(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::GetRoleAdmin(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::GetRoleMember(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::GetRoleMemberCount(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::GrantRole(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::HasRole(element) => element.encode(),
                TokenWrapperContractCalls::IncreaseAllowance(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::Mint(element) => element.encode(),
                TokenWrapperContractCalls::Name(element) => element.encode(),
                TokenWrapperContractCalls::Pause(element) => element.encode(),
                TokenWrapperContractCalls::Paused(element) => element.encode(),
                TokenWrapperContractCalls::RenounceRole(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::RevokeRole(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::SetFee(element) => element.encode(),
                TokenWrapperContractCalls::SupportsInterface(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::Symbol(element) => element.encode(),
                TokenWrapperContractCalls::TotalSupply(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::Transfer(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::TransferFrom(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::Unpause(element) => element.encode(),
                TokenWrapperContractCalls::Unwrap(element) => element.encode(),
                TokenWrapperContractCalls::UnwrapAndSendTo(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::UnwrapFor(element) => {
                    element.encode()
                }
                TokenWrapperContractCalls::Wrap(element) => element.encode(),
                TokenWrapperContractCalls::WrapFor(element) => element.encode(),
                TokenWrapperContractCalls::WrapForAndSendTo(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for TokenWrapperContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TokenWrapperContractCalls::DefaultAdminRole(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::MinterRole(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::PauserRole(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::Allowance(element) => element.fmt(f),
                TokenWrapperContractCalls::Approve(element) => element.fmt(f),
                TokenWrapperContractCalls::BalanceOf(element) => element.fmt(f),
                TokenWrapperContractCalls::Burn(element) => element.fmt(f),
                TokenWrapperContractCalls::BurnFrom(element) => element.fmt(f),
                TokenWrapperContractCalls::Decimals(element) => element.fmt(f),
                TokenWrapperContractCalls::DecreaseAllowance(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::FeeRecipient(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::GetAmountToWrap(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::GetFeeFromAmount(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::GetRoleAdmin(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::GetRoleMember(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::GetRoleMemberCount(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::GrantRole(element) => element.fmt(f),
                TokenWrapperContractCalls::HasRole(element) => element.fmt(f),
                TokenWrapperContractCalls::IncreaseAllowance(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::Mint(element) => element.fmt(f),
                TokenWrapperContractCalls::Name(element) => element.fmt(f),
                TokenWrapperContractCalls::Pause(element) => element.fmt(f),
                TokenWrapperContractCalls::Paused(element) => element.fmt(f),
                TokenWrapperContractCalls::RenounceRole(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::RevokeRole(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::SetFee(element) => element.fmt(f),
                TokenWrapperContractCalls::SupportsInterface(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::Symbol(element) => element.fmt(f),
                TokenWrapperContractCalls::TotalSupply(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::Transfer(element) => element.fmt(f),
                TokenWrapperContractCalls::TransferFrom(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::Unpause(element) => element.fmt(f),
                TokenWrapperContractCalls::Unwrap(element) => element.fmt(f),
                TokenWrapperContractCalls::UnwrapAndSendTo(element) => {
                    element.fmt(f)
                }
                TokenWrapperContractCalls::UnwrapFor(element) => element.fmt(f),
                TokenWrapperContractCalls::Wrap(element) => element.fmt(f),
                TokenWrapperContractCalls::WrapFor(element) => element.fmt(f),
                TokenWrapperContractCalls::WrapForAndSendTo(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<DefaultAdminRoleCall> for TokenWrapperContractCalls {
        fn from(var: DefaultAdminRoleCall) -> Self {
            TokenWrapperContractCalls::DefaultAdminRole(var)
        }
    }
    impl ::std::convert::From<MinterRoleCall> for TokenWrapperContractCalls {
        fn from(var: MinterRoleCall) -> Self {
            TokenWrapperContractCalls::MinterRole(var)
        }
    }
    impl ::std::convert::From<PauserRoleCall> for TokenWrapperContractCalls {
        fn from(var: PauserRoleCall) -> Self {
            TokenWrapperContractCalls::PauserRole(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for TokenWrapperContractCalls {
        fn from(var: AllowanceCall) -> Self {
            TokenWrapperContractCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for TokenWrapperContractCalls {
        fn from(var: ApproveCall) -> Self {
            TokenWrapperContractCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for TokenWrapperContractCalls {
        fn from(var: BalanceOfCall) -> Self {
            TokenWrapperContractCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for TokenWrapperContractCalls {
        fn from(var: BurnCall) -> Self {
            TokenWrapperContractCalls::Burn(var)
        }
    }
    impl ::std::convert::From<BurnFromCall> for TokenWrapperContractCalls {
        fn from(var: BurnFromCall) -> Self {
            TokenWrapperContractCalls::BurnFrom(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for TokenWrapperContractCalls {
        fn from(var: DecimalsCall) -> Self {
            TokenWrapperContractCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for TokenWrapperContractCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            TokenWrapperContractCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<FeeRecipientCall> for TokenWrapperContractCalls {
        fn from(var: FeeRecipientCall) -> Self {
            TokenWrapperContractCalls::FeeRecipient(var)
        }
    }
    impl ::std::convert::From<GetAmountToWrapCall> for TokenWrapperContractCalls {
        fn from(var: GetAmountToWrapCall) -> Self {
            TokenWrapperContractCalls::GetAmountToWrap(var)
        }
    }
    impl ::std::convert::From<GetFeeFromAmountCall> for TokenWrapperContractCalls {
        fn from(var: GetFeeFromAmountCall) -> Self {
            TokenWrapperContractCalls::GetFeeFromAmount(var)
        }
    }
    impl ::std::convert::From<GetRoleAdminCall> for TokenWrapperContractCalls {
        fn from(var: GetRoleAdminCall) -> Self {
            TokenWrapperContractCalls::GetRoleAdmin(var)
        }
    }
    impl ::std::convert::From<GetRoleMemberCall> for TokenWrapperContractCalls {
        fn from(var: GetRoleMemberCall) -> Self {
            TokenWrapperContractCalls::GetRoleMember(var)
        }
    }
    impl ::std::convert::From<GetRoleMemberCountCall>
        for TokenWrapperContractCalls
    {
        fn from(var: GetRoleMemberCountCall) -> Self {
            TokenWrapperContractCalls::GetRoleMemberCount(var)
        }
    }
    impl ::std::convert::From<GrantRoleCall> for TokenWrapperContractCalls {
        fn from(var: GrantRoleCall) -> Self {
            TokenWrapperContractCalls::GrantRole(var)
        }
    }
    impl ::std::convert::From<HasRoleCall> for TokenWrapperContractCalls {
        fn from(var: HasRoleCall) -> Self {
            TokenWrapperContractCalls::HasRole(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for TokenWrapperContractCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            TokenWrapperContractCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<MintCall> for TokenWrapperContractCalls {
        fn from(var: MintCall) -> Self {
            TokenWrapperContractCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for TokenWrapperContractCalls {
        fn from(var: NameCall) -> Self {
            TokenWrapperContractCalls::Name(var)
        }
    }
    impl ::std::convert::From<PauseCall> for TokenWrapperContractCalls {
        fn from(var: PauseCall) -> Self {
            TokenWrapperContractCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for TokenWrapperContractCalls {
        fn from(var: PausedCall) -> Self {
            TokenWrapperContractCalls::Paused(var)
        }
    }
    impl ::std::convert::From<RenounceRoleCall> for TokenWrapperContractCalls {
        fn from(var: RenounceRoleCall) -> Self {
            TokenWrapperContractCalls::RenounceRole(var)
        }
    }
    impl ::std::convert::From<RevokeRoleCall> for TokenWrapperContractCalls {
        fn from(var: RevokeRoleCall) -> Self {
            TokenWrapperContractCalls::RevokeRole(var)
        }
    }
    impl ::std::convert::From<SetFeeCall> for TokenWrapperContractCalls {
        fn from(var: SetFeeCall) -> Self {
            TokenWrapperContractCalls::SetFee(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for TokenWrapperContractCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            TokenWrapperContractCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for TokenWrapperContractCalls {
        fn from(var: SymbolCall) -> Self {
            TokenWrapperContractCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for TokenWrapperContractCalls {
        fn from(var: TotalSupplyCall) -> Self {
            TokenWrapperContractCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for TokenWrapperContractCalls {
        fn from(var: TransferCall) -> Self {
            TokenWrapperContractCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for TokenWrapperContractCalls {
        fn from(var: TransferFromCall) -> Self {
            TokenWrapperContractCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for TokenWrapperContractCalls {
        fn from(var: UnpauseCall) -> Self {
            TokenWrapperContractCalls::Unpause(var)
        }
    }
    impl ::std::convert::From<UnwrapCall> for TokenWrapperContractCalls {
        fn from(var: UnwrapCall) -> Self {
            TokenWrapperContractCalls::Unwrap(var)
        }
    }
    impl ::std::convert::From<UnwrapAndSendToCall> for TokenWrapperContractCalls {
        fn from(var: UnwrapAndSendToCall) -> Self {
            TokenWrapperContractCalls::UnwrapAndSendTo(var)
        }
    }
    impl ::std::convert::From<UnwrapForCall> for TokenWrapperContractCalls {
        fn from(var: UnwrapForCall) -> Self {
            TokenWrapperContractCalls::UnwrapFor(var)
        }
    }
    impl ::std::convert::From<WrapCall> for TokenWrapperContractCalls {
        fn from(var: WrapCall) -> Self {
            TokenWrapperContractCalls::Wrap(var)
        }
    }
    impl ::std::convert::From<WrapForCall> for TokenWrapperContractCalls {
        fn from(var: WrapForCall) -> Self {
            TokenWrapperContractCalls::WrapFor(var)
        }
    }
    impl ::std::convert::From<WrapForAndSendToCall> for TokenWrapperContractCalls {
        fn from(var: WrapForAndSendToCall) -> Self {
            TokenWrapperContractCalls::WrapForAndSendTo(var)
        }
    }
    #[doc = "Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23, 253, 223]`"]
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `MINTER_ROLE` function with signature `MINTER_ROLE()` and selector `[213, 57, 19, 147]`"]
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
    pub struct MinterRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `PAUSER_ROLE` function with signature `PAUSER_ROLE()` and selector `[230, 58, 177, 233]`"]
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
    pub struct PauserRoleReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
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
    pub struct AllowanceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
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
    pub struct ApproveReturn(pub bool);
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
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
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `decimals` function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
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
    pub struct DecimalsReturn(pub u8);
    #[doc = "Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
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
    pub struct DecreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `feeRecipient` function with signature `feeRecipient()` and selector `[70, 144, 72, 64]`"]
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
    pub struct FeeRecipientReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getAmountToWrap` function with signature `getAmountToWrap(uint256)` and selector `[150, 205, 77, 254]`"]
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
    pub struct GetAmountToWrapReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getFeeFromAmount` function with signature `getFeeFromAmount(uint256)` and selector `[133, 192, 10, 232]`"]
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
    pub struct GetFeeFromAmountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156, 163]`"]
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `[144, 16, 208, 124]`"]
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
    pub struct GetRoleMemberReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `[202, 21, 200, 115]`"]
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
    pub struct GetRoleMemberCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`"]
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
    pub struct HasRoleReturn(pub bool);
    #[doc = "Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
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
    pub struct IncreaseAllowanceReturn(pub bool);
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
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
    pub struct NameReturn(pub String);
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
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
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
    pub struct SupportsInterfaceReturn(pub bool);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
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
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
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
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
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
    pub struct TransferReturn(pub bool);
    #[doc = "Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
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
    pub struct TransferFromReturn(pub bool);
}
