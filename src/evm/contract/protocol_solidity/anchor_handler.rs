pub use anchorhandlercontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod anchorhandlercontract_mod {
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
    #[doc = "AnchorHandlerContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ANCHORHANDLERCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"bridgeAddress\",\"type\":\"address\"},{\"internalType\":\"bytes32[]\",\"name\":\"initialResourceIDs\",\"type\":\"bytes32[]\"},{\"internalType\":\"address[]\",\"name\":\"initialContractAddresses\",\"type\":\"address[]\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"_bridgeAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"_contractAddressToResourceID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"_contractWhitelist\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"_counts\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"_resourceIDToContractAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"_updateRecords\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_sourceChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_merkleRoot\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"_leafIndex\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"executeProposal\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"updateNonce\",\"type\":\"uint64\"},{\"internalType\":\"uint256\",\"name\":\"sourceChainId\",\"type\":\"uint256\"}],\"name\":\"getUpdateRecord\",\"outputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_sourceChainID\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"_resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_merkleRoot\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"_leafIndex\",\"type\":\"uint256\"}],\"internalType\":\"struct AnchorHandler.UpdateRecord\",\"name\":\"\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newBridge\",\"type\":\"address\"}],\"name\":\"migrateBridge\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"contractAddress\",\"type\":\"address\"}],\"name\":\"setResource\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct AnchorHandlerContract<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AnchorHandlerContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for AnchorHandlerContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AnchorHandlerContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AnchorHandlerContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                ANCHORHANDLERCONTRACT_ABI.clone(),
                client,
            );
            Self(contract)
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
        #[doc = "Calls the contract's `_counts` (0xd75a0683) function"]
        pub fn counts(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([215, 90, 6, 131], p0)
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
                [u8; 32],
                [u8; 32],
                ethers::core::types::U256,
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
        #[doc = "Calls the contract's `getUpdateRecord` (0xe07384a8) function"]
        pub fn get_update_record(
            &self,
            update_nonce: u64,
            source_chain_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, UpdateRecord> {
            self.0
                .method_hash(
                    [224, 115, 132, 168],
                    (update_nonce, source_chain_id),
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
    #[doc = "Container type for all input parameters for the `_bridgeAddress`function with signature `_bridgeAddress()` and selector `[49, 140, 19, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_bridgeAddress", abi = "_bridgeAddress()")]
    pub struct BridgeAddressCall;
    #[doc = "Container type for all input parameters for the `_contractAddressToResourceID`function with signature `_contractAddressToResourceID(address)` and selector `[236, 151, 211, 180]`"]
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
        name = "_contractAddressToResourceID",
        abi = "_contractAddressToResourceID(address)"
    )]
    pub struct ContractAddressToResourceIDCall(
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `_contractWhitelist`function with signature `_contractWhitelist(address)` and selector `[127, 121, 190, 168]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_contractWhitelist", abi = "_contractWhitelist(address)")]
    pub struct ContractWhitelistCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `_resourceIDToContractAddress`function with signature `_resourceIDToContractAddress(bytes32)` and selector `[197, 76, 42, 17]`"]
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
        name = "_resourceIDToContractAddress",
        abi = "_resourceIDToContractAddress(bytes32)"
    )]
    pub struct ResourceIDToContractAddressCall(pub [u8; 32]);
    #[doc = "Container type for all input parameters for the `_updateRecords`function with signature `_updateRecords(uint256,uint256)` and selector `[12, 158, 158, 20]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "_updateRecords", abi = "_updateRecords(uint256,uint256)")]
    pub struct UpdateRecordsCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `executeProposal`function with signature `executeProposal(bytes32,bytes)` and selector `[226, 72, 207, 242]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "executeProposal", abi = "executeProposal(bytes32,bytes)")]
    pub struct ExecuteProposalCall {
        pub resource_id: [u8; 32],
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `getUpdateRecord`function with signature `getUpdateRecord(uint64,uint256)` and selector `[224, 115, 132, 168]`"]
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
        name = "getUpdateRecord",
        abi = "getUpdateRecord(uint64,uint256)"
    )]
    pub struct GetUpdateRecordCall {
        pub update_nonce: u64,
        pub source_chain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `migrateBridge`function with signature `migrateBridge(address)` and selector `[215, 245, 179, 89]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "migrateBridge", abi = "migrateBridge(address)")]
    pub struct MigrateBridgeCall {
        pub new_bridge: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setResource`function with signature `setResource(bytes32,address)` and selector `[184, 250, 55, 54]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setResource", abi = "setResource(bytes32,address)")]
    pub struct SetResourceCall {
        pub resource_id: [u8; 32],
        pub contract_address: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AnchorHandlerContractCalls {
        BridgeAddress(BridgeAddressCall),
        ContractAddressToResourceID(ContractAddressToResourceIDCall),
        ContractWhitelist(ContractWhitelistCall),
        Counts(CountsCall),
        ResourceIDToContractAddress(ResourceIDToContractAddressCall),
        UpdateRecords(UpdateRecordsCall),
        ExecuteProposal(ExecuteProposalCall),
        GetUpdateRecord(GetUpdateRecordCall),
        MigrateBridge(MigrateBridgeCall),
        SetResource(SetResourceCall),
    }
    impl ethers::core::abi::AbiDecode for AnchorHandlerContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BridgeAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorHandlerContractCalls::BridgeAddress(decoded));
            }
            if let Ok (decoded) = < ContractAddressToResourceIDCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (AnchorHandlerContractCalls :: ContractAddressToResourceID (decoded)) }
            if let Ok(decoded) =
                <ContractWhitelistCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorHandlerContractCalls::ContractWhitelist(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CountsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorHandlerContractCalls::Counts(decoded));
            }
            if let Ok (decoded) = < ResourceIDToContractAddressCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (AnchorHandlerContractCalls :: ResourceIDToContractAddress (decoded)) }
            if let Ok(decoded) =
                <UpdateRecordsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorHandlerContractCalls::UpdateRecords(decoded));
            }
            if let Ok(decoded) =
                <ExecuteProposalCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorHandlerContractCalls::ExecuteProposal(decoded));
            }
            if let Ok(decoded) =
                <GetUpdateRecordCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorHandlerContractCalls::GetUpdateRecord(decoded));
            }
            if let Ok(decoded) =
                <MigrateBridgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorHandlerContractCalls::MigrateBridge(decoded));
            }
            if let Ok(decoded) =
                <SetResourceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AnchorHandlerContractCalls::SetResource(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AnchorHandlerContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AnchorHandlerContractCalls::BridgeAddress(element) => {
                    element.encode()
                }
                AnchorHandlerContractCalls::ContractAddressToResourceID(
                    element,
                ) => element.encode(),
                AnchorHandlerContractCalls::ContractWhitelist(element) => {
                    element.encode()
                }
                AnchorHandlerContractCalls::Counts(element) => element.encode(),
                AnchorHandlerContractCalls::ResourceIDToContractAddress(
                    element,
                ) => element.encode(),
                AnchorHandlerContractCalls::UpdateRecords(element) => {
                    element.encode()
                }
                AnchorHandlerContractCalls::ExecuteProposal(element) => {
                    element.encode()
                }
                AnchorHandlerContractCalls::GetUpdateRecord(element) => {
                    element.encode()
                }
                AnchorHandlerContractCalls::MigrateBridge(element) => {
                    element.encode()
                }
                AnchorHandlerContractCalls::SetResource(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for AnchorHandlerContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AnchorHandlerContractCalls::BridgeAddress(element) => {
                    element.fmt(f)
                }
                AnchorHandlerContractCalls::ContractAddressToResourceID(
                    element,
                ) => element.fmt(f),
                AnchorHandlerContractCalls::ContractWhitelist(element) => {
                    element.fmt(f)
                }
                AnchorHandlerContractCalls::Counts(element) => element.fmt(f),
                AnchorHandlerContractCalls::ResourceIDToContractAddress(
                    element,
                ) => element.fmt(f),
                AnchorHandlerContractCalls::UpdateRecords(element) => {
                    element.fmt(f)
                }
                AnchorHandlerContractCalls::ExecuteProposal(element) => {
                    element.fmt(f)
                }
                AnchorHandlerContractCalls::GetUpdateRecord(element) => {
                    element.fmt(f)
                }
                AnchorHandlerContractCalls::MigrateBridge(element) => {
                    element.fmt(f)
                }
                AnchorHandlerContractCalls::SetResource(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<BridgeAddressCall> for AnchorHandlerContractCalls {
        fn from(var: BridgeAddressCall) -> Self {
            AnchorHandlerContractCalls::BridgeAddress(var)
        }
    }
    impl ::std::convert::From<ContractAddressToResourceIDCall>
        for AnchorHandlerContractCalls
    {
        fn from(var: ContractAddressToResourceIDCall) -> Self {
            AnchorHandlerContractCalls::ContractAddressToResourceID(var)
        }
    }
    impl ::std::convert::From<ContractWhitelistCall>
        for AnchorHandlerContractCalls
    {
        fn from(var: ContractWhitelistCall) -> Self {
            AnchorHandlerContractCalls::ContractWhitelist(var)
        }
    }
    impl ::std::convert::From<CountsCall> for AnchorHandlerContractCalls {
        fn from(var: CountsCall) -> Self {
            AnchorHandlerContractCalls::Counts(var)
        }
    }
    impl ::std::convert::From<ResourceIDToContractAddressCall>
        for AnchorHandlerContractCalls
    {
        fn from(var: ResourceIDToContractAddressCall) -> Self {
            AnchorHandlerContractCalls::ResourceIDToContractAddress(var)
        }
    }
    impl ::std::convert::From<UpdateRecordsCall> for AnchorHandlerContractCalls {
        fn from(var: UpdateRecordsCall) -> Self {
            AnchorHandlerContractCalls::UpdateRecords(var)
        }
    }
    impl ::std::convert::From<ExecuteProposalCall> for AnchorHandlerContractCalls {
        fn from(var: ExecuteProposalCall) -> Self {
            AnchorHandlerContractCalls::ExecuteProposal(var)
        }
    }
    impl ::std::convert::From<GetUpdateRecordCall> for AnchorHandlerContractCalls {
        fn from(var: GetUpdateRecordCall) -> Self {
            AnchorHandlerContractCalls::GetUpdateRecord(var)
        }
    }
    impl ::std::convert::From<MigrateBridgeCall> for AnchorHandlerContractCalls {
        fn from(var: MigrateBridgeCall) -> Self {
            AnchorHandlerContractCalls::MigrateBridge(var)
        }
    }
    impl ::std::convert::From<SetResourceCall> for AnchorHandlerContractCalls {
        fn from(var: SetResourceCall) -> Self {
            AnchorHandlerContractCalls::SetResource(var)
        }
    }
    #[doc = "`UpdateRecord(address,uint256,bytes32,bytes32,uint256)`"]
    #[derive(
        Clone, Debug, Default, Eq, PartialEq, ethers :: contract :: EthAbiType,
    )]
    pub struct UpdateRecord {
        pub token_address: ethers::core::types::Address,
        pub source_chain_id: ethers::core::types::U256,
        pub resource_id: [u8; 32],
        pub merkle_root: [u8; 32],
        pub leaf_index: ethers::core::types::U256,
    }
}
