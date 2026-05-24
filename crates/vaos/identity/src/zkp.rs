//! Zero-knowledge proof identity verification.
//!
//! Source: DIAP protocol — Noir ZKP circuits (4 constraints, ~192-byte proofs,
//! 3-5ms verification).

use serde::{Deserialize, Serialize};

/// A zero-knowledge proof of identity (DIAP ZKP-on-CID).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkpIdentityProof {
    /// Noir ZKP proof bytes (~192 bytes)
    pub proof_bytes: Vec<u8>,
    /// Public inputs to the ZKP circuit
    pub public_inputs: Vec<String>,
    /// Circuit version
    pub circuit_version: String,
}
