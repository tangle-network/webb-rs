pub use fungible_token_wrapper_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod fungible_token_wrapper_contract {
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

    #[doc = "FungibleTokenWrapperContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static FUNGIBLETOKENWRAPPERCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"previousAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"newAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleAdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleGranted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleRevoked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MINTER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"PAUSER_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"add\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnFrom\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"subtractedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"decreaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feePercentage\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeRecipient\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_deposit\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountToWrap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFee\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amountToWrap\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFeeFromAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleMember\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleMemberCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTokens\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"grantRole\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"handler\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"historicalTokens\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"addedValue\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"increaseAllowance\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_feePercentage\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_feeRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_isNativeAllowed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_admin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"initialized\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isNativeAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isValidToken\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"remove\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"revokeRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"_feePercentage\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"_feeRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"_nonce\",\"type\":\"uint32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeRecipient\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_handler\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setHandler\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_isNativeAllowed\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setNativeAllowed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokens\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrapAndSendTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrapFor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_limit\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateLimit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrap\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrapFor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"wrapForAndSendTo\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"wrappingLimit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]") . expect ("invalid abi")
    });
    pub struct FungibleTokenWrapperContract<M>(ethers::contract::Contract<M>);
    impl<M: ethers::providers::Middleware> Clone
        for FungibleTokenWrapperContract<M>
    {
        fn clone(&self) -> Self {
            FungibleTokenWrapperContract::from(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for FungibleTokenWrapperContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for FungibleTokenWrapperContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(FungibleTokenWrapperContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> FungibleTokenWrapperContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                FUNGIBLETOKENWRAPPERCONTRACT_ABI.clone(),
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
        #[doc = "Calls the contract's `add` (0xfc97a652) function"]
        pub fn add(
            &self,
            token_address: ethers::core::types::Address,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 151, 166, 82], (token_address, nonce))
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
        #[doc = "Calls the contract's `feePercentage` (0xa001ecdd) function"]
        pub fn fee_percentage(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([160, 1, 236, 221], ())
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
        ) -> ethers::contract::builders::ContractCall<M, u16> {
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
        #[doc = "Calls the contract's `initialize` (0xf63ebb45) function"]
        pub fn initialize(
            &self,
            fee_percentage: u16,
            fee_recipient: ethers::core::types::Address,
            handler: ethers::core::types::Address,
            limit: ethers::core::types::U256,
            is_native_allowed: bool,
            admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [246, 62, 187, 69],
                    (
                        fee_percentage,
                        fee_recipient,
                        handler,
                        limit,
                        is_native_allowed,
                        admin,
                    ),
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
        #[doc = "Calls the contract's `isNativeAllowed` (0xb3e4083f) function"]
        pub fn is_native_allowed(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([179, 228, 8, 63], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isValidToken` (0xc1876453) function"]
        pub fn is_valid_token(
            &self,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([193, 135, 100, 83], token_address)
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
        #[doc = "Calls the contract's `remove` (0x1c4a1436) function"]
        pub fn remove(
            &self,
            token_address: ethers::core::types::Address,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 74, 20, 54], (token_address, nonce))
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
        #[doc = "Calls the contract's `setFeeRecipient` (0x07184f1c) function"]
        pub fn set_fee_recipient(
            &self,
            fee_recipient: ethers::core::types::Address,
            nonce: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 24, 79, 28], (fee_recipient, nonce))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHandler` (0xbac426d0) function"]
        pub fn set_handler(
            &self,
            handler: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 196, 38, 208], handler)
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
        ) -> ethers::contract::builders::Event<
            std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(
            &self,
        ) -> ethers::contract::builders::Event<std::sync::Arc<M>, M, PausedFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleAdminChanged` event"]
        pub fn role_admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleGranted` event"]
        pub fn role_granted_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleRevoked` event"]
        pub fn role_revoked_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(
            &self,
        ) -> ethers::contract::builders::Event<
            std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<
            std::sync::Arc<M>,
            M,
            FungibleTokenWrapperContractEvents,
        > {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for FungibleTokenWrapperContract<M>
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
    pub enum FungibleTokenWrapperContractEvents {
        ApprovalFilter(ApprovalFilter),
        PausedFilter(PausedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        TransferFilter(TransferFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for FungibleTokenWrapperContractEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::ApprovalFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::PausedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::RoleAdminChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::RoleGrantedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(
                    FungibleTokenWrapperContractEvents::RoleRevokedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::TransferFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(FungibleTokenWrapperContractEvents::UnpausedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for FungibleTokenWrapperContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FungibleTokenWrapperContractEvents::ApprovalFilter(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractEvents::PausedFilter(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractEvents::RoleAdminChangedFilter(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractEvents::RoleGrantedFilter(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractEvents::RoleRevokedFilter(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractEvents::TransferFilter(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractEvents::UnpausedFilter(element) => {
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
    #[doc = "Container type for all input parameters for the `add` function with signature `add(address,uint32)` and selector `[252, 151, 166, 82]`"]
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
    #[ethcall(name = "add", abi = "add(address,uint32)")]
    pub struct AddCall {
        pub token_address: ethers::core::types::Address,
        pub nonce: u32,
    }
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
    #[doc = "Container type for all input parameters for the `feePercentage` function with signature `feePercentage()` and selector `[160, 1, 236, 221]`"]
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
    #[ethcall(name = "feePercentage", abi = "feePercentage()")]
    pub struct FeePercentageCall;
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
    #[doc = "Container type for all input parameters for the `getFee` function with signature `getFee()` and selector `[206, 215, 47, 135]`"]
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
    #[doc = "Container type for all input parameters for the `getTokens` function with signature `getTokens()` and selector `[170, 108, 168, 8]`"]
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
    #[doc = "Container type for all input parameters for the `historicalTokens` function with signature `historicalTokens(uint256)` and selector `[133, 209, 72, 52]`"]
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
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(uint16,address,address,uint256,bool,address)` and selector `[246, 62, 187, 69]`"]
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
        name = "initialize",
        abi = "initialize(uint16,address,address,uint256,bool,address)"
    )]
    pub struct InitializeCall {
        pub fee_percentage: u16,
        pub fee_recipient: ethers::core::types::Address,
        pub handler: ethers::core::types::Address,
        pub limit: ethers::core::types::U256,
        pub is_native_allowed: bool,
        pub admin: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `isNativeAllowed` function with signature `isNativeAllowed()` and selector `[179, 228, 8, 63]`"]
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
    #[doc = "Container type for all input parameters for the `isValidToken` function with signature `isValidToken(address)` and selector `[193, 135, 100, 83]`"]
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
    #[ethcall(name = "isValidToken", abi = "isValidToken(address)")]
    pub struct IsValidTokenCall {
        pub token_address: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `remove` function with signature `remove(address,uint32)` and selector `[28, 74, 20, 54]`"]
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
    #[ethcall(name = "remove", abi = "remove(address,uint32)")]
    pub struct RemoveCall {
        pub token_address: ethers::core::types::Address,
        pub nonce: u32,
    }
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
    #[doc = "Container type for all input parameters for the `setFeeRecipient` function with signature `setFeeRecipient(address,uint32)` and selector `[7, 24, 79, 28]`"]
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
        abi = "setFeeRecipient(address,uint32)"
    )]
    pub struct SetFeeRecipientCall {
        pub fee_recipient: ethers::core::types::Address,
        pub nonce: u32,
    }
    #[doc = "Container type for all input parameters for the `setHandler` function with signature `setHandler(address)` and selector `[186, 196, 38, 208]`"]
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
    #[ethcall(name = "setHandler", abi = "setHandler(address)")]
    pub struct SetHandlerCall {
        pub handler: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setNativeAllowed` function with signature `setNativeAllowed(bool)` and selector `[139, 84, 120, 185]`"]
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
    #[doc = "Container type for all input parameters for the `tokens` function with signature `tokens(uint256)` and selector `[79, 100, 178, 190]`"]
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
    #[doc = "Container type for all input parameters for the `updateLimit` function with signature `updateLimit(uint256)` and selector `[250, 224, 149, 154]`"]
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
    #[doc = "Container type for all input parameters for the `wrappingLimit` function with signature `wrappingLimit()` and selector `[31, 145, 67, 130]`"]
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub enum FungibleTokenWrapperContractCalls {
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
        FeePercentage(FeePercentageCall),
        FeeRecipient(FeeRecipientCall),
        GetAmountToWrap(GetAmountToWrapCall),
        GetFee(GetFeeCall),
        GetFeeFromAmount(GetFeeFromAmountCall),
        GetProposalNonce(GetProposalNonceCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GetTokens(GetTokensCall),
        GrantRole(GrantRoleCall),
        Handler(HandlerCall),
        HasRole(HasRoleCall),
        HistoricalTokens(HistoricalTokensCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Initialize(InitializeCall),
        Initialized(InitializedCall),
        IsNativeAllowed(IsNativeAllowedCall),
        IsValidToken(IsValidTokenCall),
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
        SetHandler(SetHandlerCall),
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
    impl ethers::core::abi::AbiDecode for FungibleTokenWrapperContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    FungibleTokenWrapperContractCalls::DefaultAdminRole(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <MinterRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::MinterRole(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PauserRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::PauserRole(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AddCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(FungibleTokenWrapperContractCalls::Add(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Allowance(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::BalanceOf(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <BurnCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <BurnFromCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::BurnFrom(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Decimals(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    FungibleTokenWrapperContractCalls::DecreaseAllowance(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <FeePercentageCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::FeePercentage(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <FeeRecipientCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::FeeRecipient(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetAmountToWrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::GetAmountToWrap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::GetFee(decoded));
            }
            if let Ok(decoded) =
                <GetFeeFromAmountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    FungibleTokenWrapperContractCalls::GetFeeFromAmount(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <GetProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    FungibleTokenWrapperContractCalls::GetProposalNonce(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::GetRoleAdmin(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetRoleMemberCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::GetRoleMember(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetRoleMemberCountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    FungibleTokenWrapperContractCalls::GetRoleMemberCount(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <GetTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::GetTokens(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::GrantRole(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <HandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Handler(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::HasRole(decoded));
            }
            if let Ok(decoded) =
                <HistoricalTokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    FungibleTokenWrapperContractCalls::HistoricalTokens(
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
                    FungibleTokenWrapperContractCalls::IncreaseAllowance(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Initialize(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InitializedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Initialized(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IsNativeAllowedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::IsNativeAllowed(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IsValidTokenCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::IsValidToken(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MintCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Mint(decoded));
            }
            if let Ok(decoded) =
                <NameCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <PauseCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Pause(decoded));
            }
            if let Ok(decoded) =
                <PausedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <ProposalNonceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::ProposalNonce(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RemoveCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Remove(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::RenounceRole(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::RevokeRole(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetFeeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::SetFee(decoded));
            }
            if let Ok(decoded) =
                <SetFeeRecipientCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::SetFeeRecipient(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetHandlerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::SetHandler(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetNativeAllowedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    FungibleTokenWrapperContractCalls::SetNativeAllowed(
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
                    FungibleTokenWrapperContractCalls::SupportsInterface(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <SymbolCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokensCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Tokens(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::TotalSupply(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Transfer(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::TransferFrom(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UnwrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Unwrap(decoded));
            }
            if let Ok(decoded) =
                <UnwrapAndSendToCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::UnwrapAndSendTo(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UnwrapForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::UnwrapFor(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <UpdateLimitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::UpdateLimit(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WrapCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::Wrap(decoded));
            }
            if let Ok(decoded) =
                <WrapForCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::WrapFor(decoded));
            }
            if let Ok(decoded) =
                <WrapForAndSendToCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    FungibleTokenWrapperContractCalls::WrapForAndSendTo(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) =
                <WrappingLimitCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(FungibleTokenWrapperContractCalls::WrappingLimit(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for FungibleTokenWrapperContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                FungibleTokenWrapperContractCalls::DefaultAdminRole(
                    element,
                ) => element.encode(),
                FungibleTokenWrapperContractCalls::MinterRole(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::PauserRole(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Add(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Allowance(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Approve(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::BalanceOf(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Burn(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::BurnFrom(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Decimals(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::DecreaseAllowance(
                    element,
                ) => element.encode(),
                FungibleTokenWrapperContractCalls::FeePercentage(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::FeeRecipient(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::GetAmountToWrap(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::GetFee(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::GetFeeFromAmount(
                    element,
                ) => element.encode(),
                FungibleTokenWrapperContractCalls::GetProposalNonce(
                    element,
                ) => element.encode(),
                FungibleTokenWrapperContractCalls::GetRoleAdmin(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::GetRoleMember(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::GetRoleMemberCount(
                    element,
                ) => element.encode(),
                FungibleTokenWrapperContractCalls::GetTokens(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::GrantRole(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Handler(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::HasRole(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::HistoricalTokens(
                    element,
                ) => element.encode(),
                FungibleTokenWrapperContractCalls::IncreaseAllowance(
                    element,
                ) => element.encode(),
                FungibleTokenWrapperContractCalls::Initialize(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Initialized(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::IsNativeAllowed(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::IsValidToken(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Mint(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Name(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Pause(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Paused(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::ProposalNonce(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Remove(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::RenounceRole(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::RevokeRole(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::SetFee(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::SetFeeRecipient(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::SetHandler(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::SetNativeAllowed(
                    element,
                ) => element.encode(),
                FungibleTokenWrapperContractCalls::SupportsInterface(
                    element,
                ) => element.encode(),
                FungibleTokenWrapperContractCalls::Symbol(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Tokens(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::TotalSupply(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Transfer(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::TransferFrom(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Unpause(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Unwrap(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::UnwrapAndSendTo(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::UnwrapFor(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::UpdateLimit(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::Wrap(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::WrapFor(element) => {
                    element.encode()
                }
                FungibleTokenWrapperContractCalls::WrapForAndSendTo(
                    element,
                ) => element.encode(),
                FungibleTokenWrapperContractCalls::WrappingLimit(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for FungibleTokenWrapperContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                FungibleTokenWrapperContractCalls::DefaultAdminRole(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractCalls::MinterRole(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::PauserRole(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Add(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Allowance(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Approve(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::BalanceOf(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Burn(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::BurnFrom(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Decimals(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::DecreaseAllowance(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractCalls::FeePercentage(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::FeeRecipient(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::GetAmountToWrap(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::GetFee(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::GetFeeFromAmount(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractCalls::GetProposalNonce(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractCalls::GetRoleAdmin(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::GetRoleMember(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::GetRoleMemberCount(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractCalls::GetTokens(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::GrantRole(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Handler(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::HasRole(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::HistoricalTokens(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractCalls::IncreaseAllowance(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractCalls::Initialize(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Initialized(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::IsNativeAllowed(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::IsValidToken(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Mint(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Name(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Pause(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Paused(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::ProposalNonce(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Remove(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::RenounceRole(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::RevokeRole(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::SetFee(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::SetFeeRecipient(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::SetHandler(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::SetNativeAllowed(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractCalls::SupportsInterface(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractCalls::Symbol(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Tokens(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::TotalSupply(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Transfer(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::TransferFrom(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Unpause(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Unwrap(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::UnwrapAndSendTo(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::UnwrapFor(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::UpdateLimit(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::Wrap(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::WrapFor(element) => {
                    element.fmt(f)
                }
                FungibleTokenWrapperContractCalls::WrapForAndSendTo(
                    element,
                ) => element.fmt(f),
                FungibleTokenWrapperContractCalls::WrappingLimit(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<DefaultAdminRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: DefaultAdminRoleCall) -> Self {
            FungibleTokenWrapperContractCalls::DefaultAdminRole(var)
        }
    }
    impl ::std::convert::From<MinterRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: MinterRoleCall) -> Self {
            FungibleTokenWrapperContractCalls::MinterRole(var)
        }
    }
    impl ::std::convert::From<PauserRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: PauserRoleCall) -> Self {
            FungibleTokenWrapperContractCalls::PauserRole(var)
        }
    }
    impl ::std::convert::From<AddCall> for FungibleTokenWrapperContractCalls {
        fn from(var: AddCall) -> Self {
            FungibleTokenWrapperContractCalls::Add(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for FungibleTokenWrapperContractCalls {
        fn from(var: AllowanceCall) -> Self {
            FungibleTokenWrapperContractCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for FungibleTokenWrapperContractCalls {
        fn from(var: ApproveCall) -> Self {
            FungibleTokenWrapperContractCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for FungibleTokenWrapperContractCalls {
        fn from(var: BalanceOfCall) -> Self {
            FungibleTokenWrapperContractCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for FungibleTokenWrapperContractCalls {
        fn from(var: BurnCall) -> Self {
            FungibleTokenWrapperContractCalls::Burn(var)
        }
    }
    impl ::std::convert::From<BurnFromCall> for FungibleTokenWrapperContractCalls {
        fn from(var: BurnFromCall) -> Self {
            FungibleTokenWrapperContractCalls::BurnFrom(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for FungibleTokenWrapperContractCalls {
        fn from(var: DecimalsCall) -> Self {
            FungibleTokenWrapperContractCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: DecreaseAllowanceCall) -> Self {
            FungibleTokenWrapperContractCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<FeePercentageCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: FeePercentageCall) -> Self {
            FungibleTokenWrapperContractCalls::FeePercentage(var)
        }
    }
    impl ::std::convert::From<FeeRecipientCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: FeeRecipientCall) -> Self {
            FungibleTokenWrapperContractCalls::FeeRecipient(var)
        }
    }
    impl ::std::convert::From<GetAmountToWrapCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: GetAmountToWrapCall) -> Self {
            FungibleTokenWrapperContractCalls::GetAmountToWrap(var)
        }
    }
    impl ::std::convert::From<GetFeeCall> for FungibleTokenWrapperContractCalls {
        fn from(var: GetFeeCall) -> Self {
            FungibleTokenWrapperContractCalls::GetFee(var)
        }
    }
    impl ::std::convert::From<GetFeeFromAmountCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: GetFeeFromAmountCall) -> Self {
            FungibleTokenWrapperContractCalls::GetFeeFromAmount(var)
        }
    }
    impl ::std::convert::From<GetProposalNonceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: GetProposalNonceCall) -> Self {
            FungibleTokenWrapperContractCalls::GetProposalNonce(var)
        }
    }
    impl ::std::convert::From<GetRoleAdminCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: GetRoleAdminCall) -> Self {
            FungibleTokenWrapperContractCalls::GetRoleAdmin(var)
        }
    }
    impl ::std::convert::From<GetRoleMemberCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: GetRoleMemberCall) -> Self {
            FungibleTokenWrapperContractCalls::GetRoleMember(var)
        }
    }
    impl ::std::convert::From<GetRoleMemberCountCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: GetRoleMemberCountCall) -> Self {
            FungibleTokenWrapperContractCalls::GetRoleMemberCount(var)
        }
    }
    impl ::std::convert::From<GetTokensCall> for FungibleTokenWrapperContractCalls {
        fn from(var: GetTokensCall) -> Self {
            FungibleTokenWrapperContractCalls::GetTokens(var)
        }
    }
    impl ::std::convert::From<GrantRoleCall> for FungibleTokenWrapperContractCalls {
        fn from(var: GrantRoleCall) -> Self {
            FungibleTokenWrapperContractCalls::GrantRole(var)
        }
    }
    impl ::std::convert::From<HandlerCall> for FungibleTokenWrapperContractCalls {
        fn from(var: HandlerCall) -> Self {
            FungibleTokenWrapperContractCalls::Handler(var)
        }
    }
    impl ::std::convert::From<HasRoleCall> for FungibleTokenWrapperContractCalls {
        fn from(var: HasRoleCall) -> Self {
            FungibleTokenWrapperContractCalls::HasRole(var)
        }
    }
    impl ::std::convert::From<HistoricalTokensCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: HistoricalTokensCall) -> Self {
            FungibleTokenWrapperContractCalls::HistoricalTokens(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: IncreaseAllowanceCall) -> Self {
            FungibleTokenWrapperContractCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<InitializeCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: InitializeCall) -> Self {
            FungibleTokenWrapperContractCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<InitializedCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: InitializedCall) -> Self {
            FungibleTokenWrapperContractCalls::Initialized(var)
        }
    }
    impl ::std::convert::From<IsNativeAllowedCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: IsNativeAllowedCall) -> Self {
            FungibleTokenWrapperContractCalls::IsNativeAllowed(var)
        }
    }
    impl ::std::convert::From<IsValidTokenCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: IsValidTokenCall) -> Self {
            FungibleTokenWrapperContractCalls::IsValidToken(var)
        }
    }
    impl ::std::convert::From<MintCall> for FungibleTokenWrapperContractCalls {
        fn from(var: MintCall) -> Self {
            FungibleTokenWrapperContractCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for FungibleTokenWrapperContractCalls {
        fn from(var: NameCall) -> Self {
            FungibleTokenWrapperContractCalls::Name(var)
        }
    }
    impl ::std::convert::From<PauseCall> for FungibleTokenWrapperContractCalls {
        fn from(var: PauseCall) -> Self {
            FungibleTokenWrapperContractCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for FungibleTokenWrapperContractCalls {
        fn from(var: PausedCall) -> Self {
            FungibleTokenWrapperContractCalls::Paused(var)
        }
    }
    impl ::std::convert::From<ProposalNonceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: ProposalNonceCall) -> Self {
            FungibleTokenWrapperContractCalls::ProposalNonce(var)
        }
    }
    impl ::std::convert::From<RemoveCall> for FungibleTokenWrapperContractCalls {
        fn from(var: RemoveCall) -> Self {
            FungibleTokenWrapperContractCalls::Remove(var)
        }
    }
    impl ::std::convert::From<RenounceRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: RenounceRoleCall) -> Self {
            FungibleTokenWrapperContractCalls::RenounceRole(var)
        }
    }
    impl ::std::convert::From<RevokeRoleCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: RevokeRoleCall) -> Self {
            FungibleTokenWrapperContractCalls::RevokeRole(var)
        }
    }
    impl ::std::convert::From<SetFeeCall> for FungibleTokenWrapperContractCalls {
        fn from(var: SetFeeCall) -> Self {
            FungibleTokenWrapperContractCalls::SetFee(var)
        }
    }
    impl ::std::convert::From<SetFeeRecipientCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: SetFeeRecipientCall) -> Self {
            FungibleTokenWrapperContractCalls::SetFeeRecipient(var)
        }
    }
    impl ::std::convert::From<SetHandlerCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: SetHandlerCall) -> Self {
            FungibleTokenWrapperContractCalls::SetHandler(var)
        }
    }
    impl ::std::convert::From<SetNativeAllowedCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: SetNativeAllowedCall) -> Self {
            FungibleTokenWrapperContractCalls::SetNativeAllowed(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: SupportsInterfaceCall) -> Self {
            FungibleTokenWrapperContractCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for FungibleTokenWrapperContractCalls {
        fn from(var: SymbolCall) -> Self {
            FungibleTokenWrapperContractCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TokensCall> for FungibleTokenWrapperContractCalls {
        fn from(var: TokensCall) -> Self {
            FungibleTokenWrapperContractCalls::Tokens(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: TotalSupplyCall) -> Self {
            FungibleTokenWrapperContractCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for FungibleTokenWrapperContractCalls {
        fn from(var: TransferCall) -> Self {
            FungibleTokenWrapperContractCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: TransferFromCall) -> Self {
            FungibleTokenWrapperContractCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for FungibleTokenWrapperContractCalls {
        fn from(var: UnpauseCall) -> Self {
            FungibleTokenWrapperContractCalls::Unpause(var)
        }
    }
    impl ::std::convert::From<UnwrapCall> for FungibleTokenWrapperContractCalls {
        fn from(var: UnwrapCall) -> Self {
            FungibleTokenWrapperContractCalls::Unwrap(var)
        }
    }
    impl ::std::convert::From<UnwrapAndSendToCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: UnwrapAndSendToCall) -> Self {
            FungibleTokenWrapperContractCalls::UnwrapAndSendTo(var)
        }
    }
    impl ::std::convert::From<UnwrapForCall> for FungibleTokenWrapperContractCalls {
        fn from(var: UnwrapForCall) -> Self {
            FungibleTokenWrapperContractCalls::UnwrapFor(var)
        }
    }
    impl ::std::convert::From<UpdateLimitCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: UpdateLimitCall) -> Self {
            FungibleTokenWrapperContractCalls::UpdateLimit(var)
        }
    }
    impl ::std::convert::From<WrapCall> for FungibleTokenWrapperContractCalls {
        fn from(var: WrapCall) -> Self {
            FungibleTokenWrapperContractCalls::Wrap(var)
        }
    }
    impl ::std::convert::From<WrapForCall> for FungibleTokenWrapperContractCalls {
        fn from(var: WrapForCall) -> Self {
            FungibleTokenWrapperContractCalls::WrapFor(var)
        }
    }
    impl ::std::convert::From<WrapForAndSendToCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: WrapForAndSendToCall) -> Self {
            FungibleTokenWrapperContractCalls::WrapForAndSendTo(var)
        }
    }
    impl ::std::convert::From<WrappingLimitCall>
        for FungibleTokenWrapperContractCalls
    {
        fn from(var: WrappingLimitCall) -> Self {
            FungibleTokenWrapperContractCalls::WrappingLimit(var)
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
    #[doc = "Container type for all return fields from the `feePercentage` function with signature `feePercentage()` and selector `[160, 1, 236, 221]`"]
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
    pub struct FeePercentageReturn(pub u16);
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
    #[doc = "Container type for all return fields from the `getFee` function with signature `getFee()` and selector `[206, 215, 47, 135]`"]
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
    pub struct GetFeeReturn(pub u16);
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
    #[doc = "Container type for all return fields from the `getTokens` function with signature `getTokens()` and selector `[170, 108, 168, 8]`"]
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
    pub struct GetTokensReturn(
        pub ::std::vec::Vec<ethers::core::types::Address>,
    );
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
    #[doc = "Container type for all return fields from the `historicalTokens` function with signature `historicalTokens(uint256)` and selector `[133, 209, 72, 52]`"]
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
    pub struct HistoricalTokensReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `isNativeAllowed` function with signature `isNativeAllowed()` and selector `[179, 228, 8, 63]`"]
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
    pub struct IsNativeAllowedReturn(pub bool);
    #[doc = "Container type for all return fields from the `isValidToken` function with signature `isValidToken(address)` and selector `[193, 135, 100, 83]`"]
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
    pub struct IsValidTokenReturn(pub bool);
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
    #[doc = "Container type for all return fields from the `tokens` function with signature `tokens(uint256)` and selector `[79, 100, 178, 190]`"]
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
    pub struct TokensReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `wrappingLimit` function with signature `wrappingLimit()` and selector `[31, 145, 67, 130]`"]
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
    pub struct WrappingLimitReturn(pub ethers::core::types::U256);
}
