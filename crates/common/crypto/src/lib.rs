//! # Verity Common — Shared Cryptographic Primitives
//!
//! Provides unified cryptographic utilities across all Verity crates:
//! - BLAKE3 hashing for ledger and provenance
//! - Ed25519 signing for capability tokens and provenance capsules
//! - ML-DSA-44 post-quantum signatures (via dcrypt)
//! - Constant-time comparison for cryptographic operations
//!
//! Source: ARC42 v20.0 §6 Security, C8 (PQC readiness)

pub mod hash;
pub mod sign;
pub mod constant_time;
pub mod types;
pub mod errors;

pub use hash::HashExt;
pub use sign::SignExt;
pub use types::KeyPair;
pub use errors::CryptoError;
