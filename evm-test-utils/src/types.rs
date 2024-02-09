use webb::evm::ethers::{
    abi::{encode, AbiEncode, Address, Bytes, Token, Uint},
    types::H256,
    utils::keccak256,
};

#[allow(clippy::wrong_self_convention)]
pub trait IntoAbiToken {
    fn into_abi_token(&self) -> Token;
    fn encode_abi_token(&self) -> Vec<u8> {
        let token = self.into_abi_token();

        encode(&[token])
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

#[derive(Clone, Debug, typed_builder::TypedBuilder)]
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

impl ExtData {
    /// Generate Abi token for ExtData.
    pub fn into_abi_token(&self) -> Token {
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

    /// Generate hash for abi encoded data.
    pub fn hash(&self) -> H256 {
        let encoded_data = encode(&[self.into_abi_token()]);
        keccak256(encoded_data).into()
    }
}

impl IntoAbiToken for ExtData {
    fn into_abi_token(&self) -> Token {
        self.into_abi_token()
    }
}
