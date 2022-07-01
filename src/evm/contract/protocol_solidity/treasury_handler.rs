pub use treasuryhandlercontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod treasuryhandlercontract_mod {
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
    #[doc = "TreasuryHandlerContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TREASURYHANDLERCONTRACT_ABI: ethers::contract::Lazy<
        ethers::core::abi::Abi,
    > = ethers::contract::Lazy::new(|| {
        serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"bridgeAddress\",\"type\":\"address\"},{\"internalType\":\"bytes32[]\",\"name\":\"initialResourceIDs\",\"type\":\"bytes32[]\"},{\"internalType\":\"address[]\",\"name\":\"initialContractAddresses\",\"type\":\"address[]\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"_bridgeAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"_contractAddressToResourceID\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"_contractWhitelist\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"_counts\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"_resourceIDToContractAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"_updateRecords\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_treasuryAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_executionChainID\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\"},{\"internalType\":\"bytes4\",\"name\":\"functionSig\",\"type\":\"bytes4\"},{\"internalType\":\"bytes32\",\"name\":\"_resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_updateValue\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"executeProposal\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"updateNonce\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"executionChainId\",\"type\":\"uint256\"}],\"name\":\"getUpdateRecord\",\"outputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"_treasuryAddress\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_executionChainID\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\"},{\"internalType\":\"bytes4\",\"name\":\"functionSig\",\"type\":\"bytes4\"},{\"internalType\":\"bytes32\",\"name\":\"_resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"_updateValue\",\"type\":\"bytes32\"}],\"internalType\":\"struct TreasuryHandler.UpdateRecord\",\"name\":\"\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newBridge\",\"type\":\"address\"}],\"name\":\"migrateBridge\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"resourceID\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"contractAddress\",\"type\":\"address\"}],\"name\":\"setResource\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]") . expect ("invalid abi")
    });
    #[derive(Clone)]
    pub struct TreasuryHandlerContract<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for TreasuryHandlerContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug
        for TreasuryHandlerContract<M>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TreasuryHandlerContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> TreasuryHandlerContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                TREASURYHANDLERCONTRACT_ABI.clone(),
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
        ) -> ethers::contract::builders::ContractCall<
            M,
            ethers::core::types::U256,
        > {
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
    #[doc = "Container type for all input parameters for the `_bridgeAddress`function with signature `_bridgeAddress()` and selector `[49, 140, 19, 110]`"]
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
    #[doc = "Container type for all input parameters for the `_contractAddressToResourceID`function with signature `_contractAddressToResourceID(address)` and selector `[236, 151, 211, 180]`"]
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
    #[doc = "Container type for all input parameters for the `_contractWhitelist`function with signature `_contractWhitelist(address)` and selector `[127, 121, 190, 168]`"]
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
    #[doc = "Container type for all input parameters for the `_counts`function with signature `_counts(uint256)` and selector `[215, 90, 6, 131]`"]
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
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
        serde :: Serialize,
        serde :: Deserialize,
    )]
    #[ethcall(name = "executeProposal", abi = "executeProposal(bytes32,bytes)")]
    pub struct ExecuteProposalCall {
        pub resource_id: [u8; 32],
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `getUpdateRecord`function with signature `getUpdateRecord(uint256,uint256)` and selector `[107, 132, 114, 74]`"]
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
    #[doc = "Container type for all input parameters for the `migrateBridge`function with signature `migrateBridge(address)` and selector `[215, 245, 179, 89]`"]
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
    #[doc = "Container type for all input parameters for the `setResource`function with signature `setResource(bytes32,address)` and selector `[184, 250, 55, 54]`"]
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TreasuryHandlerContractCalls {
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
    impl ethers::core::abi::AbiDecode for TreasuryHandlerContractCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BridgeAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TreasuryHandlerContractCalls::BridgeAddress(decoded));
            }
            if let Ok (decoded) = < ContractAddressToResourceIDCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (TreasuryHandlerContractCalls :: ContractAddressToResourceID (decoded)) }
            if let Ok(decoded) =
                <ContractWhitelistCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TreasuryHandlerContractCalls::ContractWhitelist(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CountsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TreasuryHandlerContractCalls::Counts(decoded));
            }
            if let Ok (decoded) = < ResourceIDToContractAddressCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (TreasuryHandlerContractCalls :: ResourceIDToContractAddress (decoded)) }
            if let Ok(decoded) =
                <UpdateRecordsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TreasuryHandlerContractCalls::UpdateRecords(decoded));
            }
            if let Ok(decoded) =
                <ExecuteProposalCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TreasuryHandlerContractCalls::ExecuteProposal(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetUpdateRecordCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TreasuryHandlerContractCalls::GetUpdateRecord(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MigrateBridgeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TreasuryHandlerContractCalls::MigrateBridge(decoded));
            }
            if let Ok(decoded) =
                <SetResourceCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(TreasuryHandlerContractCalls::SetResource(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TreasuryHandlerContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                TreasuryHandlerContractCalls::BridgeAddress(element) => {
                    element.encode()
                }
                TreasuryHandlerContractCalls::ContractAddressToResourceID(
                    element,
                ) => element.encode(),
                TreasuryHandlerContractCalls::ContractWhitelist(element) => {
                    element.encode()
                }
                TreasuryHandlerContractCalls::Counts(element) => {
                    element.encode()
                }
                TreasuryHandlerContractCalls::ResourceIDToContractAddress(
                    element,
                ) => element.encode(),
                TreasuryHandlerContractCalls::UpdateRecords(element) => {
                    element.encode()
                }
                TreasuryHandlerContractCalls::ExecuteProposal(element) => {
                    element.encode()
                }
                TreasuryHandlerContractCalls::GetUpdateRecord(element) => {
                    element.encode()
                }
                TreasuryHandlerContractCalls::MigrateBridge(element) => {
                    element.encode()
                }
                TreasuryHandlerContractCalls::SetResource(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for TreasuryHandlerContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TreasuryHandlerContractCalls::BridgeAddress(element) => {
                    element.fmt(f)
                }
                TreasuryHandlerContractCalls::ContractAddressToResourceID(
                    element,
                ) => element.fmt(f),
                TreasuryHandlerContractCalls::ContractWhitelist(element) => {
                    element.fmt(f)
                }
                TreasuryHandlerContractCalls::Counts(element) => element.fmt(f),
                TreasuryHandlerContractCalls::ResourceIDToContractAddress(
                    element,
                ) => element.fmt(f),
                TreasuryHandlerContractCalls::UpdateRecords(element) => {
                    element.fmt(f)
                }
                TreasuryHandlerContractCalls::ExecuteProposal(element) => {
                    element.fmt(f)
                }
                TreasuryHandlerContractCalls::GetUpdateRecord(element) => {
                    element.fmt(f)
                }
                TreasuryHandlerContractCalls::MigrateBridge(element) => {
                    element.fmt(f)
                }
                TreasuryHandlerContractCalls::SetResource(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<BridgeAddressCall> for TreasuryHandlerContractCalls {
        fn from(var: BridgeAddressCall) -> Self {
            TreasuryHandlerContractCalls::BridgeAddress(var)
        }
    }
    impl ::std::convert::From<ContractAddressToResourceIDCall>
        for TreasuryHandlerContractCalls
    {
        fn from(var: ContractAddressToResourceIDCall) -> Self {
            TreasuryHandlerContractCalls::ContractAddressToResourceID(var)
        }
    }
    impl ::std::convert::From<ContractWhitelistCall>
        for TreasuryHandlerContractCalls
    {
        fn from(var: ContractWhitelistCall) -> Self {
            TreasuryHandlerContractCalls::ContractWhitelist(var)
        }
    }
    impl ::std::convert::From<CountsCall> for TreasuryHandlerContractCalls {
        fn from(var: CountsCall) -> Self {
            TreasuryHandlerContractCalls::Counts(var)
        }
    }
    impl ::std::convert::From<ResourceIDToContractAddressCall>
        for TreasuryHandlerContractCalls
    {
        fn from(var: ResourceIDToContractAddressCall) -> Self {
            TreasuryHandlerContractCalls::ResourceIDToContractAddress(var)
        }
    }
    impl ::std::convert::From<UpdateRecordsCall> for TreasuryHandlerContractCalls {
        fn from(var: UpdateRecordsCall) -> Self {
            TreasuryHandlerContractCalls::UpdateRecords(var)
        }
    }
    impl ::std::convert::From<ExecuteProposalCall>
        for TreasuryHandlerContractCalls
    {
        fn from(var: ExecuteProposalCall) -> Self {
            TreasuryHandlerContractCalls::ExecuteProposal(var)
        }
    }
    impl ::std::convert::From<GetUpdateRecordCall>
        for TreasuryHandlerContractCalls
    {
        fn from(var: GetUpdateRecordCall) -> Self {
            TreasuryHandlerContractCalls::GetUpdateRecord(var)
        }
    }
    impl ::std::convert::From<MigrateBridgeCall> for TreasuryHandlerContractCalls {
        fn from(var: MigrateBridgeCall) -> Self {
            TreasuryHandlerContractCalls::MigrateBridge(var)
        }
    }
    impl ::std::convert::From<SetResourceCall> for TreasuryHandlerContractCalls {
        fn from(var: SetResourceCall) -> Self {
            TreasuryHandlerContractCalls::SetResource(var)
        }
    }
    #[doc = "`UpdateRecord(address,uint256,uint256,bytes4,bytes32,bytes32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        serde :: Serialize,
        serde :: Deserialize,
    )]
    pub struct UpdateRecord {
        pub treasury_address: ethers::core::types::Address,
        pub execution_chain_id: ethers::core::types::U256,
        pub nonce: ethers::core::types::U256,
        pub function_sig: [u8; 4],
        pub resource_id: [u8; 32],
        pub update_value: [u8; 32],
    }
}
