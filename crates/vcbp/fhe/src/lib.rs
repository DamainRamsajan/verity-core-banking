//! # Verity Core Banking — FHE Hardware Acceleration Abstraction Layer
//!
//! Provides a unified interface for Fully Homomorphic Encryption operations
//! across software (TFHE-rs), GPU, and ASIC (Intel Heracles) backends.
//!
//! Source: ARC42 v20.0 §3 VCBP FHE Hardware Acceleration Abstraction Layer

pub mod engine;
pub mod backends;
pub mod types;
pub mod errors;

pub use engine::FheEngine;
pub use types::{FheBackend, FheCiphertext, FhePlaintext, FheScheme};
pub use errors::FheError;
