//! A simple wrapper around the `zkm_verifier` crate.

use wasm_bindgen::prelude::*;
use zkm_verifier::{
    Groth16Verifier, PlonkVerifier, StarkVerifier, GROTH16_VK_BYTES, PLONK_VK_BYTES,
};

// Enable console.log! macro for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Set up panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// Wrapper around [`zkm_verifier::StarkVerifier::verify`].
///
/// public_inputs: public values committed by the guest as a bincode-serialized byte array.
///     For example:
///     ```
///     // Write the output of the program.
///     //
///     // Behind the scenes, this also compiles down to a system call which handles writing
///     // outputs to the prover.
///     // zkm_zkvm::io::commit(&block_hash);
///     ```
/// Relative to `verify_stark_proof()`, it performs a consistency check between
/// user-supplied public values and those committed in the proof.
#[wasm_bindgen]
pub fn verify_stark_with_public_inputs(proof: &[u8], public_inputs: &[u8], zkm_vk: &[u8]) -> bool {
    StarkVerifier::verify(proof, public_inputs, zkm_vk).is_ok()
}

/// Wrapper around [`zkm_verifier::StarkVerifier::verify_proof`].
#[wasm_bindgen]
pub fn verify_stark(proof: &[u8], zkm_vk: &[u8]) -> bool {
    console_log!(
        "Starting verification with proof size: {}, zkm_vk size: {}",
        proof.len(),
        zkm_vk.len()
    );

    StarkVerifier::verify_proof(proof, zkm_vk).is_ok()
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
