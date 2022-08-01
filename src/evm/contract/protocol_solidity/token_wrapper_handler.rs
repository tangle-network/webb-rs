pub use token_wrapper_handler_contract::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod token_wrapper_handler_contract {
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
    #[doc = "TokenWrapperHandlerContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TOKENWRAPPERHANDLERCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"bridgeAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"initialResourceIDs\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"initialContractAddresses\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_bridgeAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_contractAddressToResourceID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_contractWhitelist\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_resourceIDToContractAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_updateRecords\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_tokenWrapperAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_executionChainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"functionSig\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_resourceID\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_updateValue\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeProposal\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"updateNonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"executionChainId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUpdateRecord\",\"outputs\":[{\"internalType\":\"struct TokenWrapperHandler.UpdateRecord\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"_tokenWrapperAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_executionChainID\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"functionSig\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_resourceID\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_updateValue\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newBridge\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"migrateBridge\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"contractAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setResource\",\"outputs\":[]}]") . expect ("invalid abi")
    });
    pub struct TokenWrapperHandlerContract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for TokenWrapperHandlerContract<M> {
        fn clone(&self) -> Self {
            TokenWrapperHandlerContract(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for TokenWrapperHandlerContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for TokenWrapperHandlerContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TokenWrapperHandlerContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> TokenWrapperHandlerContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                TOKENWRAPPERHANDLERCONTRACT_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `_bridgeAddress` (0x318c136e) function"]
        pub fn bridge_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([49, 140, 19, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_contractAddressToResourceID` (0xec97d3b4) function"]
        pub fn contract_address_to_resource_id(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([236, 151, 211, 180], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_contractWhitelist` (0x7f79bea8) function"]
        pub fn contract_whitelist(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([127, 121, 190, 168], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_resourceIDToContractAddress` (0xc54c2a11) function"]
        pub fn resource_id_to_contract_address(
            &self,
            p0: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::Address,
        > {
            self.0
                .method_hash([197, 76, 42, 17], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_updateRecords` (0x0c9e9e14) function"]
        pub fn update_records(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::U256,
                [u8; 4],
                [u8; 32],
                [u8; 32],
            ),
        > {
            self.0
                .method_hash([12, 158, 158, 20], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeProposal` (0xe248cff2) function"]
        pub fn execute_proposal(
            &self,
            resource_id: [u8; 32],
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 72, 207, 242], (resource_id, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUpdateRecord` (0x6b84724a) function"]
        pub fn get_update_record(
            &self,
            update_nonce: ethers::core::types::U256,
            execution_chain_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, UpdateRecord> {
            self.0
                .method_hash(
                    [107, 132, 114, 74],
                    (update_nonce, execution_chain_id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `migrateBridge` (0xd7f5b359) function"]
        pub fn migrate_bridge(
            &self,
            new_bridge: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 245, 179, 89], new_bridge)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setResource` (0xb8fa3736) function"]
        pub fn set_resource(
            &self,
            resource_id: [u8; 32],
            contract_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [184, 250, 55, 54],
                    (resource_id, contract_address),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for TokenWrapperHandlerContract<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `_bridgeAddress` function with signature `_bridgeAddress()` and selector `[49, 140, 19, 110]`"]
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
    #[ethcall(name = "_bridgeAddress", abi = "_bridgeAddress()")]
    pub struct BridgeAddressCall;
    #[doc = "Container type for all input parameters for the `_contractAddressToResourceID` function with signature `_contractAddressToResourceID(address)` and selector `[236, 151, 211, 180]`"]
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
        name = "_contractAddressToResourceID",
        abi = "_contractAddressToResourceID(address)"
    )]
    pub struct ContractAddressToResourceIDCall(
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `_contractWhitelist` function with signature `_contractWhitelist(address)` and selector `[127, 121, 190, 168]`"]
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
    #[ethcall(name = "_contractWhitelist", abi = "_contractWhitelist(address)")]
    pub struct ContractWhitelistCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `_resourceIDToContractAddress` function with signature `_resourceIDToContractAddress(bytes32)` and selector `[197, 76, 42, 17]`"]
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
        name = "_resourceIDToContractAddress",
        abi = "_resourceIDToContractAddress(bytes32)"
    )]
    pub struct ResourceIDToContractAddressCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `_updateRecords` function with signature `_updateRecords(uint256,uint256)` and selector `[12, 158, 158, 20]`"]
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
    #[ethcall(name = "_updateRecords", abi = "_updateRecords(uint256,uint256)")]
    pub struct UpdateRecordsCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `executeProposal` function with signature `executeProposal(bytes32,bytes)` and selector `[226, 72, 207, 242]`"]
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
    #[ethcall(name = "executeProposal", abi = "executeProposal(bytes32,bytes)")]
    pub struct ExecuteProposalCall {
        pub resource_id: [u8; 32],
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `getUpdateRecord` function with signature `getUpdateRecord(uint256,uint256)` and selector `[107, 132, 114, 74]`"]
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
        name = "getUpdateRecord",
        abi = "getUpdateRecord(uint256,uint256)"
    )]
    pub struct GetUpdateRecordCall {
        pub update_nonce: ethers::core::types::U256,
        pub execution_chain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `migrateBridge` function with signature `migrateBridge(address)` and selector `[215, 245, 179, 89]`"]
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
    #[ethcall(name = "migrateBridge", abi = "migrateBridge(address)")]
    pub struct MigrateBridgeCall {
        pub new_bridge: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setResource` function with signature `setResource(bytes32,address)` and selector `[184, 250, 55, 54]`"]
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
    #[ethcall(name = "setResource", abi = "setResource(bytes32,address)")]
    pub struct SetResourceCall {
        pub resource_id: [u8; 32],
        pub contract_address: ethers::core::types::Address,
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
    pub enum TokenWrapperHandlerContractCalls {
        BridgeAddress(BridgeAddressCall),
        ContractAddressToResourceID(ContractAddressToResourceIDCall),
        ContractWhitelist(ContractWhitelistCall),
        ResourceIDToContractAddress(ResourceIDToContractAddressCall),
        UpdateRecords(UpdateRecordsCall),
        ExecuteProposal(ExecuteProposalCall),
        GetUpdateRecord(GetUpdateRecordCall),
        MigrateBridge(MigrateBridgeCall),
        SetResource(SetResourceCall),
    }
    impl ethers::core::abi::AbiDecode for TokenWrapperHandlerContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BridgeAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperHandlerContractCalls::BridgeAddress(
                    decoded,
                ));
            }
            if let Ok (decoded) = < ContractAddressToResourceIDCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (TokenWrapperHandlerContractCalls :: ContractAddressToResourceID (decoded)) }
            if let Ok(decoded) =
                <ContractWhitelistCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(
                    TokenWrapperHandlerContractCalls::ContractWhitelist(
                        decoded,
                    ),
                );
            }
            if let Ok (decoded) = < ResourceIDToContractAddressCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (TokenWrapperHandlerContractCalls :: ResourceIDToContractAddress (decoded)) }
            if let Ok(decoded) =
                <UpdateRecordsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperHandlerContractCalls::UpdateRecords(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ExecuteProposalCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperHandlerContractCalls::ExecuteProposal(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetUpdateRecordCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperHandlerContractCalls::GetUpdateRecord(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MigrateBridgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperHandlerContractCalls::MigrateBridge(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetResourceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TokenWrapperHandlerContractCalls::SetResource(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TokenWrapperHandlerContractCalls {
        fn encode(self) -> Vec<u8> {
            match self { TokenWrapperHandlerContractCalls :: BridgeAddress (element) => element . encode () , TokenWrapperHandlerContractCalls :: ContractAddressToResourceID (element) => element . encode () , TokenWrapperHandlerContractCalls :: ContractWhitelist (element) => element . encode () , TokenWrapperHandlerContractCalls :: ResourceIDToContractAddress (element) => element . encode () , TokenWrapperHandlerContractCalls :: UpdateRecords (element) => element . encode () , TokenWrapperHandlerContractCalls :: ExecuteProposal (element) => element . encode () , TokenWrapperHandlerContractCalls :: GetUpdateRecord (element) => element . encode () , TokenWrapperHandlerContractCalls :: MigrateBridge (element) => element . encode () , TokenWrapperHandlerContractCalls :: SetResource (element) => element . encode () }
        }
    }
    impl ::std::fmt::Display for TokenWrapperHandlerContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self { TokenWrapperHandlerContractCalls :: BridgeAddress (element) => element . fmt (f) , TokenWrapperHandlerContractCalls :: ContractAddressToResourceID (element) => element . fmt (f) , TokenWrapperHandlerContractCalls :: ContractWhitelist (element) => element . fmt (f) , TokenWrapperHandlerContractCalls :: ResourceIDToContractAddress (element) => element . fmt (f) , TokenWrapperHandlerContractCalls :: UpdateRecords (element) => element . fmt (f) , TokenWrapperHandlerContractCalls :: ExecuteProposal (element) => element . fmt (f) , TokenWrapperHandlerContractCalls :: GetUpdateRecord (element) => element . fmt (f) , TokenWrapperHandlerContractCalls :: MigrateBridge (element) => element . fmt (f) , TokenWrapperHandlerContractCalls :: SetResource (element) => element . fmt (f) }
        }
    }
    impl ::std::convert::From<BridgeAddressCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(var: BridgeAddressCall) -> Self {
            TokenWrapperHandlerContractCalls::BridgeAddress(var)
        }
    }
    impl ::std::convert::From<ContractAddressToResourceIDCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(var: ContractAddressToResourceIDCall) -> Self {
            TokenWrapperHandlerContractCalls::ContractAddressToResourceID(var)
        }
    }
    impl ::std::convert::From<ContractWhitelistCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(var: ContractWhitelistCall) -> Self {
            TokenWrapperHandlerContractCalls::ContractWhitelist(var)
        }
    }
    impl ::std::convert::From<ResourceIDToContractAddressCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(var: ResourceIDToContractAddressCall) -> Self {
            TokenWrapperHandlerContractCalls::ResourceIDToContractAddress(var)
        }
    }
    impl ::std::convert::From<UpdateRecordsCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(var: UpdateRecordsCall) -> Self {
            TokenWrapperHandlerContractCalls::UpdateRecords(var)
        }
    }
    impl ::std::convert::From<ExecuteProposalCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(var: ExecuteProposalCall) -> Self {
            TokenWrapperHandlerContractCalls::ExecuteProposal(var)
        }
    }
    impl ::std::convert::From<GetUpdateRecordCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(var: GetUpdateRecordCall) -> Self {
            TokenWrapperHandlerContractCalls::GetUpdateRecord(var)
        }
    }
    impl ::std::convert::From<MigrateBridgeCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(var: MigrateBridgeCall) -> Self {
            TokenWrapperHandlerContractCalls::MigrateBridge(var)
        }
    }
    impl ::std::convert::From<SetResourceCall>
        for TokenWrapperHandlerContractCalls
    {
        fn from(var: SetResourceCall) -> Self {
            TokenWrapperHandlerContractCalls::SetResource(var)
        }
    }
    #[doc = "Container type for all return fields from the `_bridgeAddress` function with signature `_bridgeAddress()` and selector `[49, 140, 19, 110]`"]
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
    pub struct BridgeAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `_contractAddressToResourceID` function with signature `_contractAddressToResourceID(address)` and selector `[236, 151, 211, 180]`"]
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
    pub struct ContractAddressToResourceIDReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `_contractWhitelist` function with signature `_contractWhitelist(address)` and selector `[127, 121, 190, 168]`"]
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
    pub struct ContractWhitelistReturn(pub bool);
    #[doc = "Container type for all return fields from the `_resourceIDToContractAddress` function with signature `_resourceIDToContractAddress(bytes32)` and selector `[197, 76, 42, 17]`"]
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
    pub struct ResourceIDToContractAddressReturn(
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all return fields from the `_updateRecords` function with signature `_updateRecords(uint256,uint256)` and selector `[12, 158, 158, 20]`"]
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
    pub struct UpdateRecordsReturn {
        pub token_wrapper_address: ethers::core::types::Address,
        pub execution_chain_id: ethers::core::types::U256,
        pub nonce: ethers::core::types::U256,
        pub function_sig: [u8; 4],
        pub resource_id: [u8; 32],
        pub update_value: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `getUpdateRecord` function with signature `getUpdateRecord(uint256,uint256)` and selector `[107, 132, 114, 74]`"]
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
    pub struct GetUpdateRecordReturn(
        pub  (
            ethers::core::types::Address,
            ethers::core::types::U256,
            ethers::core::types::U256,
            [u8; 4],
            [u8; 32],
            [u8; 32],
        ),
    );
    #[doc = "`UpdateRecord(address,uint256,uint256,bytes4,bytes32,bytes32)`"]
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
    pub struct UpdateRecord {
        pub token_wrapper_address: ethers::core::types::Address,
        pub execution_chain_id: ethers::core::types::U256,
        pub nonce: ethers::core::types::U256,
        pub function_sig: [u8; 4],
        pub resource_id: [u8; 32],
        pub update_value: [u8; 32],
    }
}
