pub use poseidon_hasher_contract::*;
#[doc = r" This module was auto-generated with ethers-rs Abigen."]
#[doc = r" More information at: <https://github.com/gakonst/ethers-rs>"]
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod poseidon_hasher_contract {
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"array\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hash11\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[3]\",\"name\":\"array\",\"type\":\"uint256[3]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hash3\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[5]\",\"name\":\"array\",\"type\":\"uint256[5]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hash5\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_left\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_right\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hashLeftRight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"zeros\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]" ;
    #[doc = "The parsed JSON ABI of the contract."]
    pub static POSEIDONHASHERCONTRACT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct PoseidonHasherContract<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PoseidonHasherContract<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PoseidonHasherContract<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PoseidonHasherContract<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PoseidonHasherContract<M> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(PoseidonHasherContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PoseidonHasherContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers` client at"]
        #[doc = r" `address`. The contract derefs to a `ethers::Contract` object."]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                POSEIDONHASHERCONTRACT_ABI.clone(),
                client,
            ))
        }
        #[doc = "Calls the contract's `hash11` (0x8a1a52d2) function"]
        pub fn hash_11(
            &self,
            array: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([138, 26, 82, 210], array)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hash3` (0xbea140b3) function"]
        pub fn hash_3(
            &self,
            array: [::ethers::core::types::U256; 3],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([190, 161, 64, 179], array)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hash5` (0x9cfced97) function"]
        pub fn hash_5(
            &self,
            array: [::ethers::core::types::U256; 5],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([156, 252, 237, 151], array)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hashLeftRight` (0x5bb93995) function"]
        pub fn hash_left_right(
            &self,
            left: ::ethers::core::types::U256,
            right: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::U256,
        > {
            self.0
                .method_hash([91, 185, 57, 149], (left, right))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `zeros` (0xe8295588) function"]
        pub fn zeros(
            &self,
            i: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([232, 41, 85, 136], i)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware>
        From<::ethers::contract::Contract<M>> for PoseidonHasherContract<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[doc = "Container type for all input parameters for the `hash11` function with signature `hash11(uint256[])` and selector `0x8a1a52d2`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "hash11", abi = "hash11(uint256[])")]
    pub struct Hash11Call {
        pub array: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `hash3` function with signature `hash3(uint256[3])` and selector `0xbea140b3`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "hash3", abi = "hash3(uint256[3])")]
    pub struct Hash3Call {
        pub array: [::ethers::core::types::U256; 3],
    }
    #[doc = "Container type for all input parameters for the `hash5` function with signature `hash5(uint256[5])` and selector `0x9cfced97`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "hash5", abi = "hash5(uint256[5])")]
    pub struct Hash5Call {
        pub array: [::ethers::core::types::U256; 5],
    }
    #[doc = "Container type for all input parameters for the `hashLeftRight` function with signature `hashLeftRight(uint256,uint256)` and selector `0x5bb93995`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "hashLeftRight", abi = "hashLeftRight(uint256,uint256)")]
    pub struct HashLeftRightCall {
        pub left: ::ethers::core::types::U256,
        pub right: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `zeros` function with signature `zeros(uint256)` and selector `0xe8295588`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "zeros", abi = "zeros(uint256)")]
    pub struct ZerosCall {
        pub i: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all of the contract's call "]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum PoseidonHasherContractCalls {
        Hash11(Hash11Call),
        Hash3(Hash3Call),
        Hash5(Hash5Call),
        HashLeftRight(HashLeftRightCall),
        Zeros(ZerosCall),
    }
    impl ::ethers::core::abi::AbiDecode for PoseidonHasherContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError>
        {
            let data = data.as_ref();
            if let Ok(decoded) =
                <Hash11Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Hash11(decoded));
            }
            if let Ok(decoded) =
                <Hash3Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Hash3(decoded));
            }
            if let Ok(decoded) =
                <Hash5Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Hash5(decoded));
            }
            if let Ok(decoded) =
                <HashLeftRightCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::HashLeftRight(decoded));
            }
            if let Ok(decoded) =
                <ZerosCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Zeros(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PoseidonHasherContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Hash11(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Hash3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Hash5(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashLeftRight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Zeros(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PoseidonHasherContractCalls {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            match self {
                Self::Hash11(element) => ::core::fmt::Display::fmt(element, f),
                Self::Hash3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Hash5(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashLeftRight(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Zeros(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Hash11Call> for PoseidonHasherContractCalls {
        fn from(value: Hash11Call) -> Self {
            Self::Hash11(value)
        }
    }
    impl ::core::convert::From<Hash3Call> for PoseidonHasherContractCalls {
        fn from(value: Hash3Call) -> Self {
            Self::Hash3(value)
        }
    }
    impl ::core::convert::From<Hash5Call> for PoseidonHasherContractCalls {
        fn from(value: Hash5Call) -> Self {
            Self::Hash5(value)
        }
    }
    impl ::core::convert::From<HashLeftRightCall> for PoseidonHasherContractCalls {
        fn from(value: HashLeftRightCall) -> Self {
            Self::HashLeftRight(value)
        }
    }
    impl ::core::convert::From<ZerosCall> for PoseidonHasherContractCalls {
        fn from(value: ZerosCall) -> Self {
            Self::Zeros(value)
        }
    }
    #[doc = "Container type for all return fields from the `hash11` function with signature `hash11(uint256[])` and selector `0x8a1a52d2`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Hash11Return(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `hash3` function with signature `hash3(uint256[3])` and selector `0xbea140b3`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Hash3Return(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `hash5` function with signature `hash5(uint256[5])` and selector `0x9cfced97`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Hash5Return(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `hashLeftRight` function with signature `hashLeftRight(uint256,uint256)` and selector `0x5bb93995`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HashLeftRightReturn(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `zeros` function with signature `zeros(uint256)` and selector `0xe8295588`"]
    #[derive(
        Clone,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        serde :: Serialize,
        serde :: Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ZerosReturn(pub [u8; 32]);
}
