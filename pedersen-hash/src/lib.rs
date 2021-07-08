use std::convert::TryInto;

use ark_crypto_primitives::crh::{
    injective_map::{PedersenCRHCompressor, TECompressor},
    pedersen, CRH,
};
use ark_crypto_primitives::Error;
use ark_ed_on_bn254::EdwardsProjective;
use ark_ff::{fields::PrimeField, BigInteger};
use rand_chacha::rand_core::SeedableRng;

pub type PedersenHasher =
    PedersenCRHCompressor<EdwardsProjective, TECompressor, PedersenWindow>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PedersenWindow;

const MAX_INPUT_SIZE: usize = 64;

impl pedersen::Window for PedersenWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = (8 * MAX_INPUT_SIZE) / Self::WINDOW_SIZE;
}

pub fn hash(data: &[u8]) -> Result<[u8; 32], Error> {
    const SEED: &[u8; 32] = b"WebbToolsPedersenHasherSeedBytes";
    let mut rng = rand_chacha::ChaChaRng::from_seed(*SEED);
    let crh_params = PedersenHasher::setup(&mut rng)?;
    let result = PedersenHasher::evaluate(&crh_params, data)?;
    let hash = result
        .into_repr()
        .to_bytes_be() // always use big-endian for bignumbers!!
        .try_into()
        .expect("Hash is always 32 bytes!");
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_hash_data() {
        let data = b"Pedersen";
        let out = hash(data).unwrap();
        let actual =
            "0293c0b3986c951148d89e12edb706c0e40df4b6b6510e9392eff2ab2f7092ad";
        assert_eq!(hex::encode(out), actual);
    }
}
