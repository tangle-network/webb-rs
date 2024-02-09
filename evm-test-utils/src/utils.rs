use ark_std::{collections::BTreeMap, rand::rngs::OsRng};
use circom_proving::verify_proof;
use core::fmt::Debug;
use num_traits::Zero;
use webb::evm::ethers::types::{H256, U256};

use ark_ff::{BigInteger, PrimeField};

use arkworks_native_gadgets::poseidon::Poseidon;
pub use arkworks_setups::common::{
    prove, prove_unchecked, setup_tree_and_create_path, verify_unchecked_raw,
};
use arkworks_setups::{
    common::{setup_params, Leaf},
    r1cs::{mixer::MixerR1CSProver, vanchor::VAnchorR1CSProver},
    utxo::{self, Utxo},
    Curve, MixerProver, VAnchorProver,
};

use ark_bn254::{Bn254, Fr};
use ark_circom::WitnessCalculator;
use ark_groth16::{
    create_proof_with_reduction_and_matrices, prepare_verifying_key,
    verify_proof as ark_verify_proof, Proof as ArkProof, ProvingKey,
    VerifyingKey,
};
use ark_relations::r1cs::ConstraintMatrices;
use ark_std::{rand::thread_rng, vec::Vec, UniformRand};
use num_bigint::BigInt;
use std::sync::Mutex;

type Bn254Fr = ark_bn254::Fr;

pub const DEFAULT_LEAF: [u8; 32] = [
    47, 229, 76, 96, 211, 172, 171, 243, 52, 58, 53, 182, 235, 161, 93, 180,
    130, 27, 52, 15, 118, 231, 65, 226, 36, 150, 133, 237, 72, 153, 175, 108,
];

#[allow(non_camel_case_types)]
type VAnchorProver_Bn254_30_2_2_2 =
    VAnchorR1CSProver<Bn254, TREE_DEPTH, ANCHOR_CT, NUM_UTXOS, NUM_UTXOS>;

use ark_bn254::{Fq, Fq2, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_circom::CircomReduction;

use ark_relations::r1cs::SynthesisError;

use cfg_if::cfg_if;

use num_bigint::{BigUint, Sign};
use once_cell::sync::OnceCell;
use serde_json::Value;

use std::{convert::TryFrom, result::Result, str::FromStr};
use thiserror::Error;
use wasmer::{Module, Store};

#[derive(Error, Debug)]
pub enum ProofError {
    #[error("Error reading circuit key: {0}")]
    CircuitKeyError(#[from] std::io::Error),
    #[error("Error producing witness: {0}")]
    WitnessError(color_eyre::Report),
    #[error("Error producing proof: {0}")]
    SynthesisError(#[from] SynthesisError),
}

#[cfg(not(target_arch = "wasm32"))]
static WITNESS_CALCULATOR: OnceCell<Mutex<WitnessCalculator>> = OnceCell::new();

// Utilities to convert a json verification key in a groth16::VerificationKey
fn fq_from_str(s: &str) -> Fq {
    Fq::try_from(BigUint::from_str(s).unwrap()).unwrap()
}

// Extracts the element in G1 corresponding to its JSON serialization
fn json_to_g1(json: &Value, key: &str) -> G1Affine {
    let els: Vec<String> = json
        .get(key)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i.as_str().unwrap().to_string())
        .collect();
    G1Affine::from(G1Projective::new(
        fq_from_str(&els[0]),
        fq_from_str(&els[1]),
        fq_from_str(&els[2]),
    ))
}

// Extracts the vector of G1 elements corresponding to its JSON serialization
fn json_to_g1_vec(json: &Value, key: &str) -> Vec<G1Affine> {
    let els: Vec<Vec<String>> = json
        .get(key)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|i| {
            i.as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    els.iter()
        .map(|coords| {
            G1Affine::from(G1Projective::new(
                fq_from_str(&coords[0]),
                fq_from_str(&coords[1]),
                fq_from_str(&coords[2]),
            ))
        })
        .collect()
}

// Extracts the element in G2 corresponding to its JSON serialization
fn json_to_g2(json: &Value, key: &str) -> G2Affine {
    let els: Vec<Vec<String>> = json
        .get(key)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|i| {
            i.as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    let x = Fq2::new(fq_from_str(&els[0][0]), fq_from_str(&els[0][1]));
    let y = Fq2::new(fq_from_str(&els[1][0]), fq_from_str(&els[1][1]));
    let z = Fq2::new(fq_from_str(&els[2][0]), fq_from_str(&els[2][1]));
    G2Affine::from(G2Projective::new(x, y, z))
}

// Converts JSON to a VerifyingKey
fn to_verifying_key(json: serde_json::Value) -> VerifyingKey<Bn254> {
    VerifyingKey {
        alpha_g1: json_to_g1(&json, "vk_alpha_1"),
        beta_g2: json_to_g2(&json, "vk_beta_2"),
        gamma_g2: json_to_g2(&json, "vk_gamma_2"),
        delta_g2: json_to_g2(&json, "vk_delta_2"),
        gamma_abc_g1: json_to_g1_vec(&json, "IC"),
    }
}

// Computes the verification key from its JSON serialization
fn vk_from_json(vk_path: &str) -> VerifyingKey<Bn254> {
    let json = std::fs::read_to_string(vk_path).unwrap();
    let json: Value = serde_json::from_str(&json).unwrap();

    to_verifying_key(json)
}

pub fn generate_proof(
    #[cfg(not(target_arch = "wasm32"))] witness_calculator: &Mutex<
        WitnessCalculator,
    >,
    #[cfg(target_arch = "wasm32")] witness_calculator: &mut WitnessCalculator,
    proving_key: &(ProvingKey<Bn254>, ConstraintMatrices<Fr>),
    vanchor_witness: [(&str, Vec<BigInt>); 15],
) -> Result<(ArkProof<Bn254>, Vec<Fr>), ProofError> {
    let inputs = vanchor_witness
        .into_iter()
        .map(|(name, values)| (name.to_string(), values));

    println!("inputs {inputs:?}");

    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let full_assignment = witness_calculator
            .calculate_witness_element::<Bn254, _>(inputs, false)
            .map_err(ProofError::WitnessError)?;
        } else {
            let full_assignment = witness_calculator
            .lock()
            .expect("witness_calculator mutex should not get poisoned")
            .calculate_witness_element::<Bn254, _>(inputs, false)
            .map_err(ProofError::WitnessError)?;
        }
    }

    // Random Values
    let mut rng = thread_rng();
    let r = Fr::rand(&mut rng);
    let s = Fr::rand(&mut rng);

    let proof = create_proof_with_reduction_and_matrices::<_, CircomReduction>(
        &proving_key.0,
        r,
        s,
        &proving_key.1,
        proving_key.1.num_instance_variables,
        proving_key.1.num_constraints,
        full_assignment.as_slice(),
    )?;

    Ok((proof, full_assignment))
}

// Initializes the witness calculator using a bytes vector
#[cfg(not(target_arch = "wasm32"))]
pub fn circom_from_raw(
    wasm_buffer: Vec<u8>,
) -> &'static Mutex<WitnessCalculator> {
    WITNESS_CALCULATOR.get_or_init(|| {
        let store = Store::default();
        let module = Module::new(&store, wasm_buffer).unwrap();
        let result = WitnessCalculator::from_module(module)
            .expect("Failed to create witness calculator");
        Mutex::new(result)
    })
}

// Initializes the witness calculator
#[cfg(not(target_arch = "wasm32"))]
pub fn circom_from_folder(
    wasm_path: &str,
) -> &'static Mutex<WitnessCalculator> {
    // We read the wasm file
    let wasm_buffer = std::fs::read(wasm_path).unwrap();
    circom_from_raw(wasm_buffer)
}

const TREE_DEPTH: usize = 30;
const ANCHOR_CT: usize = 2;
pub const NUM_UTXOS: usize = 2;

pub fn setup_utxos(
    // Transaction inputs
    chain_ids: [u64; NUM_UTXOS],
    amounts: [u128; NUM_UTXOS],
    indices: Option<[u64; NUM_UTXOS]>,
) -> [Utxo<Bn254Fr>; NUM_UTXOS] {
    let curve = Curve::Bn254;
    let rng = &mut thread_rng();
    // Input Utxos
    let indices: [Option<u64>; NUM_UTXOS] = if indices.is_some() {
        let ind_unw = indices.unwrap();
        ind_unw.map(Some)
    } else {
        [None; NUM_UTXOS]
    };
    let utxo1 = VAnchorProver_Bn254_30_2_2_2::create_random_utxo(
        curve,
        chain_ids[0],
        amounts[0],
        indices[0],
        rng,
    )
    .unwrap();
    let utxo2 = VAnchorProver_Bn254_30_2_2_2::create_random_utxo(
        curve,
        chain_ids[1],
        amounts[1],
        indices[1],
        rng,
    )
    .unwrap();

    [utxo1, utxo2]
}

pub fn setup_vanchor_circuit(
    // Metadata inputs
    public_amount: i128,
    chain_id: u64,
    ext_data_hash: Vec<u8>,
    in_utxos: [Utxo<Bn254Fr>; NUM_UTXOS],
    out_utxos: [Utxo<Bn254Fr>; NUM_UTXOS],
    root: U256,
    neighbor_roots: [U256; ANCHOR_CT - 1],
    leaves: Vec<Vec<u8>>,
    circom_params: &(ProvingKey<Bn254>, ConstraintMatrices<Bn254Fr>),
    #[cfg(not(target_arch = "wasm32"))] wc: &Mutex<WitnessCalculator>,
    #[cfg(target_arch = "wasm32")] wc: &mut WitnessCalculator,
) -> (ArkProof<Bn254>, Vec<Bn254Fr>) {
    let curve = Curve::Bn254;
    let _rng = &mut thread_rng();

    let leaves_f: Vec<Bn254Fr> = leaves
        .iter()
        .map(|x| Bn254Fr::from_be_bytes_mod_order(x))
        .collect();

    let mut in_leaves: BTreeMap<u64, Vec<Vec<u8>>> = BTreeMap::new();
    in_leaves.insert(chain_id, leaves);

    let in_indices = [
        in_utxos[0].get_index().unwrap(),
        in_utxos[1].get_index().unwrap(),
    ];

    let params3 = setup_params::<Bn254Fr>(curve, 5, 3);
    let poseidon3 = Poseidon::new(params3);
    let (tree, _) = setup_tree_and_create_path::<
        Bn254Fr,
        Poseidon<Bn254Fr>,
        TREE_DEPTH,
    >(&poseidon3, &leaves_f, 0, &DEFAULT_LEAF)
    .unwrap();

    let in_paths: Vec<_> = in_indices
        .iter()
        .map(|i| tree.generate_membership_proof(*i))
        .collect();

    let roots_f: [Bn254Fr; ANCHOR_CT] = vec![if root != U256::zero() {
        let mut be_bytes = [0u8; 32];
        root.to_big_endian(&mut be_bytes);
        Bn254Fr::from_be_bytes_mod_order(&be_bytes)
    } else {
        tree.root()
    }]
    .iter()
    .chain(
        neighbor_roots
            .iter()
            .map(|r| {
                let mut be_bytes = [0u8; 32];
                r.to_big_endian(&mut be_bytes);
                Bn254Fr::from_be_bytes_mod_order(&be_bytes)
            })
            .collect::<Vec<Bn254Fr>>()
            .iter(),
    )
    .cloned()
    .collect::<Vec<Bn254Fr>>()
    .try_into()
    .unwrap();

    let in_root_set = roots_f.map(|x| x.into_repr().to_bytes_be());

    let params4 = setup_params::<Bn254Fr>(Curve::Bn254, 5, 4);
    let nullifier_hasher = Poseidon::<Bn254Fr> { params: params4 };

    // Make Inputs
    let public_amount_as_vec = if public_amount > 0 {
        vec![BigInt::from_bytes_be(
            Sign::Plus,
            &public_amount.to_be_bytes(),
        )]
    } else {
        vec![BigInt::from_bytes_be(
            Sign::Minus,
            &(-public_amount).to_be_bytes(),
        )]
    };

    let ext_data_hash_as_vec =
        vec![BigInt::from_bytes_be(Sign::Plus, &ext_data_hash)];
    let mut input_nullifier_as_vec = Vec::new();
    let mut output_commitment_as_vec = Vec::new();
    for i in 0..NUM_UTXOS {
        input_nullifier_as_vec.push(BigInt::from_bytes_be(
            Sign::Plus,
            &in_utxos[i]
                .calculate_nullifier(&nullifier_hasher)
                .unwrap()
                .into_repr()
                .to_bytes_be(),
        ));
        output_commitment_as_vec.push(BigInt::from_bytes_be(
            Sign::Plus,
            &out_utxos[i].commitment.into_repr().to_bytes_be(),
        ));
    }

    let chain_id_as_vec =
        vec![BigInt::from_bytes_be(Sign::Plus, &chain_id.to_be_bytes())];

    let roots_as_vec = in_root_set
        .iter()
        .map(|x| BigInt::from_bytes_be(Sign::Plus, x))
        .collect::<Vec<BigInt>>();

    let mut in_amount_as_vec = Vec::new();
    let mut in_private_key_as_vec = Vec::new();
    let mut in_blinding_as_vec = Vec::new();
    let mut in_path_indices_as_vec = Vec::new();
    let mut in_path_elements_as_vec = Vec::new();
    let mut out_chain_id_as_vec = Vec::new();
    let mut out_amount_as_vec = Vec::new();
    let mut out_pub_key_as_vec = Vec::new();
    let mut out_blinding_as_vec = Vec::new();

    for i in 0..NUM_UTXOS {
        in_amount_as_vec.push(BigInt::from_bytes_be(
            Sign::Plus,
            &in_utxos[i].amount.into_repr().to_bytes_be(),
        ));
        in_private_key_as_vec.push(BigInt::from_bytes_be(
            Sign::Plus,
            &in_utxos[i]
                .keypair
                .secret_key
                .unwrap()
                .into_repr()
                .to_bytes_be(),
        ));
        in_blinding_as_vec.push(BigInt::from_bytes_be(
            Sign::Plus,
            &in_utxos[i].blinding.into_repr().to_bytes_be(),
        ));
        in_path_indices_as_vec
            .push(BigInt::from(in_utxos[i].get_index().unwrap()));
        for j in 0..TREE_DEPTH {
            let neighbor_elt: Bn254Fr = if in_indices[i] == 0 {
                in_paths[i].path[j].1
            } else {
                in_paths[i].path[j].0
            };
            in_path_elements_as_vec.push(BigInt::from_bytes_be(
                Sign::Plus,
                &neighbor_elt.into_repr().to_bytes_be(),
            ));
        }

        out_chain_id_as_vec.push(BigInt::from_bytes_be(
            Sign::Plus,
            &out_utxos[i].chain_id.into_repr().to_bytes_be(),
        ));

        out_amount_as_vec.push(BigInt::from_bytes_be(
            Sign::Plus,
            &out_utxos[i].amount.into_repr().to_bytes_be(),
        ));

        out_pub_key_as_vec.push(BigInt::from_bytes_be(
            Sign::Plus,
            &out_utxos[i].keypair.public_key.into_repr().to_bytes_be(),
        ));

        out_blinding_as_vec.push(BigInt::from_bytes_be(
            Sign::Plus,
            &out_utxos[i].blinding.into_repr().to_bytes_be(),
        ));
    }

    let inputs_for_proof = [
        ("publicAmount", public_amount_as_vec),
        ("extDataHash", ext_data_hash_as_vec),
        ("inputNullifier", input_nullifier_as_vec.clone()),
        ("inAmount", in_amount_as_vec.clone()),
        ("inPrivateKey", in_private_key_as_vec.clone()),
        ("inBlinding", in_blinding_as_vec.clone()),
        ("inPathIndices", in_path_indices_as_vec.clone()),
        ("inPathElements", in_path_elements_as_vec.clone()),
        ("outputCommitment", output_commitment_as_vec.clone()),
        ("outChainID", out_chain_id_as_vec.clone()),
        ("outAmount", out_amount_as_vec.clone()),
        ("outPubkey", out_pub_key_as_vec.clone()),
        ("outBlinding", out_blinding_as_vec.clone()),
        ("chainID", chain_id_as_vec),
        ("roots", roots_as_vec),
    ];

    let x = generate_proof(wc, circom_params, inputs_for_proof.clone());

    let num_inputs = circom_params.1.num_instance_variables;

    let (proof, full_assignment) = x.unwrap();

    let public_inputs = &full_assignment[1..num_inputs];

    let did_proof_work =
        verify_proof(&circom_params.0.vk, &proof, public_inputs.to_vec())
            .unwrap();
    assert!(did_proof_work);

    (proof, public_inputs.to_vec())
}

pub fn deconstruct_public_inputs(
    public_inputs: &Vec<Bn254Fr>,
) -> (
    Bn254Fr,      // Chain Id
    Bn254Fr,      // Public amount
    Vec<Bn254Fr>, // Roots
    Vec<Bn254Fr>, // Input tx Nullifiers
    Vec<Bn254Fr>, // Output tx commitments
    Bn254Fr,      // External data hash
) {
    let public_amount = public_inputs[0];
    let ext_data_hash = public_inputs[1];
    let nullifiers = public_inputs[2..4].to_vec();
    let commitments = public_inputs[4..6].to_vec();
    let chain_id = public_inputs[6];
    let root_set = public_inputs[7..9].to_vec();
    (
        chain_id,
        public_amount,
        root_set,
        nullifiers,
        commitments,
        ext_data_hash,
    )
}

pub fn deconstruct_public_inputs_el(
    public_inputs_f: &Vec<Bn254Fr>,
) -> (
    u64,       // Chain Id
    U256,      // Public amount
    Vec<U256>, // Roots
    Vec<U256>, // Input tx Nullifiers
    [U256; 2], // Output tx commitments
    U256,      // External data hash
) {
    let (
        chain_id,
        public_amount,
        roots,
        nullifiers,
        commitments,
        ext_data_hash,
    ) = deconstruct_public_inputs(public_inputs_f);
    let chain_id_el =
        U256::from_big_endian(&chain_id.into_repr().to_bytes_be());
    let public_amount_el =
        U256::from_big_endian(&public_amount.into_repr().to_bytes_be());
    let root_set_el = roots
        .iter()
        .map(|x| U256::from_big_endian(&x.into_repr().to_bytes_be()))
        .collect();
    let nullifiers_el = nullifiers
        .iter()
        .map(|x| U256::from_big_endian(&x.into_repr().to_bytes_be()))
        .collect();
    let commitments_el: [U256; 2] = commitments
        .iter()
        .map(|x| U256::from_big_endian(&x.into_repr().to_bytes_be()))
        .collect::<Vec<U256>>()
        .try_into() // Try to convert Vec<U256> to [U256; 2]
        .expect("Failed to convert Vec<U256> to [U256; 2]");
    let ext_data_hash_el =
        U256::from_big_endian(&ext_data_hash.into_repr().to_bytes_be());
    (
        chain_id_el.as_u64(),
        public_amount_el,
        root_set_el,
        nullifiers_el,
        commitments_el,
        ext_data_hash_el,
    )
}
