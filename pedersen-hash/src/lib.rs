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

impl pedersen::Window for PedersenWindow {
    const WINDOW_SIZE: usize = 4;
    const NUM_WINDOWS: usize = 128;
}

pub fn hash(data: &[u8]) -> Result<[u8; 32], Error> {
    const SEED: &[u8; 32] = b"WebbToolsPedersenHasherSeedBytes";
    let mut rng = rand_chacha::ChaChaRng::from_seed(*SEED);
    let crh_params = PedersenHasher::setup(&mut rng)?;
    let result = PedersenHasher::evaluate(&crh_params, data)?;
    let hash = result
        .into_repr()
        .to_bytes_le()
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
            "ad92702fabf2ef92930e51b6b6f40de4c006b7ed129ed84811956c98b3c09302";
        assert_eq!(hex::encode(out), actual);
    }
}
