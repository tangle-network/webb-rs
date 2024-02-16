use serde::{Deserialize, Serialize};
use webb::evm::ethers::abi::{encode, AbiEncode, Address, Bytes, Token, Uint};
use webb::evm::ethers::types::U256;

#[allow(clippy::wrong_self_convention)]
pub trait IntoAbiToken {
    fn into_abi_token(&self) -> Token;
    fn encode_abi_token(&self) -> Vec<u8> {
        let token = self.into_abi_token();

        encode(&[token])
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProofData {
    pub proof: Bytes,
    pub roots: Bytes,
    pub extension_roots: Bytes,
    pub input_nullifiers: Vec<U256>,
    pub output_commitments: [U256; 2],
    pub public_amount: U256,
    pub ext_data_hash: U256,
}

#[derive(Clone, Debug, Serialize, Deserialize, typed_builder::TypedBuilder)]
pub struct ExtData {
    pub recipient: Address,
    pub relayer: Address,
    pub ext_amount: i128,
    pub fee: Uint,
    pub refund: Uint,
    pub token: Address,
    pub encrypted_output1: Bytes,
    pub encrypted_output2: Bytes,
}

impl IntoAbiToken for ExtData {
    fn into_abi_token(&self) -> Token {
        Token::Tuple(vec![
            Token::Address(self.recipient),
            self.ext_amount.into_abi_token(),
            Token::Address(self.relayer),
            Token::Uint(self.fee),
            Token::Uint(self.refund),
            Token::Address(self.token),
            Token::Bytes(self.encrypted_output1.clone()),
            Token::Bytes(self.encrypted_output2.clone()),
        ])
    }
}

impl IntoAbiToken for i128 {
    fn into_abi_token(&self) -> Token {
        let bytes = self.encode();
        let mut bytes32: [u8; 32] = [0; 32];
        for (i, byte) in bytes.iter().enumerate() {
            bytes32[i] = *byte;
        }
        Token::Int(bytes32.into())
    }
}

impl IntoAbiToken for [u8; 32] {
    fn into_abi_token(&self) -> Token {
        Token::Bytes(self.to_vec())
    }
}
