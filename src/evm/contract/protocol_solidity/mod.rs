#![allow(clippy::all, clippy::exhaustive_enums)]

pub mod anchor_handler;
pub mod erc20_preset_minter_pauser;
pub mod fungible_token_wrapper;
pub mod masp_vanchor;
pub mod masp_vanchor_batch_tree;
pub mod masp_vanchor_tree;
pub mod open_variable_anchor;
pub mod poseidon_hasher;
pub mod poseidon_t3;
pub mod poseidon_t4;
pub mod poseidon_t6;
pub mod signature_bridge;
pub mod token_wrapper;
pub mod token_wrapper_handler;
pub mod treasury;
pub mod treasury_handler;
pub mod vanchor_base;
pub mod vanchor_encode_inputs;
pub mod variable_anchor;
pub mod variable_anchor_tree;

pub mod poseidon_hasher_factory {
    use std::sync::Arc;

    use ethers::prelude::*;
    use ethers::solc::artifacts::ContractBytecode;

    use super::poseidon_hasher::POSEIDONHASHERCONTRACT_ABI;

    fn contract_bytecode() -> std::io::Result<ContractBytecode> {
        const ARTIFACT: &str = include_str!(
            "../../../../contracts/protocol-solidity/PoseidonHasher.json"
        );
        let artifact = serde_json::from_str::<HardhatArtifact>(&ARTIFACT)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let contract = artifact.into_contract_bytecode();
        let bytecode: ContractBytecode = contract.into();
        Ok(bytecode)
    }

    pub fn create<M>(
        t3: Address,
        t4: Address,
        t6: Address,
        client: Arc<M>,
    ) -> std::io::Result<ContractFactory<M>>
    where
        M: Middleware,
    {
        let abi = POSEIDONHASHERCONTRACT_ABI.clone();
        let mut bytecode_unlinked = contract_bytecode()?;
        if let Some(ref mut bytecode) = bytecode_unlinked.bytecode {
            bytecode.link_all_fully_qualified([
                ("contracts/hashers/Poseidon.sol:PoseidonT3", t3),
                ("contracts/hashers/Poseidon.sol:PoseidonT4", t4),
                ("contracts/hashers/Poseidon.sol:PoseidonT6", t6),
            ]);
        }
        let bytecode = bytecode_unlinked.bytecode.ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "missing bytecode")
        })?;
        debug_assert!(
            bytecode.object.is_bytecode(),
            "bytecode is not fully linked"
        );
        let raw_bytecode = bytecode.object.into_bytes().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "bytecode is not fully linked",
            )
        })?;
        let factory = ContractFactory::new(abi, raw_bytecode, client);
        Ok(factory)
    }
}

pub mod vanchor_tree_factory {
    use std::sync::Arc;

    use ethers::prelude::*;
    use ethers::solc::artifacts::ContractBytecode;

    use super::variable_anchor_tree::VANCHORTREECONTRACT_ABI;

    fn contract_bytecode() -> std::io::Result<ContractBytecode> {
        const ARTIFACT: &str = include_str!(
            "../../../../contracts/protocol-solidity/VAnchorTree.json"
        );
        let artifact = serde_json::from_str::<HardhatArtifact>(&ARTIFACT)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let contract = artifact.into_contract_bytecode();
        let bytecode: ContractBytecode = contract.into();
        Ok(bytecode)
    }

    pub fn create<M>(
        vanchor_encode_inputs: Address,
        client: Arc<M>,
    ) -> std::io::Result<ContractFactory<M>>
    where
        M: Middleware,
    {
        let abi = VANCHORTREECONTRACT_ABI.clone();
        let mut bytecode_unlinked = contract_bytecode()?;
        if let Some(ref mut bytecode) = bytecode_unlinked.bytecode {
            bytecode.link_all_fully_qualified([(
                "contracts/libs/VAnchorEncodeInputs.sol:VAnchorEncodeInputs",
                vanchor_encode_inputs,
            )]);
        }
        let bytecode = bytecode_unlinked.bytecode.ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "missing bytecode")
        })?;
        debug_assert!(
            bytecode.object.is_bytecode(),
            "bytecode is not fully linked"
        );
        let raw_bytecode = bytecode.object.into_bytes().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "bytecode is not fully linked",
            )
        })?;
        let factory = ContractFactory::new(abi, raw_bytecode, client);
        Ok(factory)
    }
}
