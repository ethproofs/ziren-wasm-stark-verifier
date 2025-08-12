//! A simple wrapper around the `zkm_verifier` crate.

use wasm_bindgen::prelude::wasm_bindgen;
use zkm_verifier::{
    Groth16Verifier, PlonkVerifier, StarkVerifier, GROTH16_VK_BYTES, PLONK_VK_BYTES,
};

/// Wrapper around [`zkm_verifier::StarkVerifier::verify`].
#[wasm_bindgen]
pub fn verify_stark(proof: &[u8], public_inputs: &[u8], zkm_vk: &[u8]) -> bool {
    StarkVerifier::verify(proof, public_inputs, zkm_vk).is_ok()
}

/// Wrapper around [`zkm_verifier::Groth16Verifier::verify`].
///
/// We hardcode the Groth16 VK bytes to only verify Ziren proofs.
#[wasm_bindgen]
pub fn verify_groth16(proof: &[u8], public_inputs: &[u8], zkm_vk_hash: &str) -> bool {
    Groth16Verifier::verify(proof, public_inputs, zkm_vk_hash, *GROTH16_VK_BYTES).is_ok()
}

/// Wrapper around [`zkm_verifier::PlonkVerifier::verify`].
///
/// We hardcode the Plonk VK bytes to only verify Ziren proofs.
#[wasm_bindgen]
pub fn verify_plonk(proof: &[u8], public_inputs: &[u8], zkm_vk_hash: &str) -> bool {
    PlonkVerifier::verify(proof, public_inputs, zkm_vk_hash, *PLONK_VK_BYTES).is_ok()
}
