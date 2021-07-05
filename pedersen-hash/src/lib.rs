use std::convert::TryInto;

use ark_crypto_primitives::crh::{
    injective_map::{PedersenCRHCompressor, TECompressor},
    pedersen, CRH,
};
use ark_ed_on_bn254::EdwardsProjective;
use ark_ff::{fields::PrimeField, BigInteger};

pub type PedersenHasher =
    PedersenCRHCompressor<EdwardsProjective, TECompressor, PedersenWindow>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PedersenWindow;

impl pedersen::Window for PedersenWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 64;
}

pub fn hash(data: &[u8]) -> [u8; 32] {
    let crh_params = PedersenHasher::setup(&mut rand::rngs::OsRng).unwrap();
    let result = PedersenHasher::evaluate(&crh_params, data).unwrap();
    result.into_repr().to_bytes_le().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use rand::rngs::OsRng;
    use rand::Rng;
    fn circom_pedersen_hash(data: &[u8]) -> [u8; 32] {
        let val = hex::encode(data);
        let result = std::process::Command::new("node")
            .arg("scripts/circom_pedersen_hash.js")
            .arg(val)
            .output()
            .unwrap();
        let output = hex::decode(result.stdout).expect("hex value");
        output.try_into().unwrap()
    }

    fn rust_pedersen_hash(data: &[u8]) -> [u8; 32] {
        super::hash(data)
    }

    fn assert_hash_eq(left: [u8; 32], right: [u8; 32]) {
        assert_eq!(hex::encode(&left), hex::encode(&right));
    }

    #[test]
    fn should_fail() {
        let mut data = [0u8; 32];
        let mut rng = OsRng::default();
        rng.fill(&mut data);
        assert_hash_eq(circom_pedersen_hash(&data), rust_pedersen_hash(&data));
    }
}
