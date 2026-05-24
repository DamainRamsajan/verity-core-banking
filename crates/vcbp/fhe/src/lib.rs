//! # Verity Core Banking — FHE Hardware Acceleration Abstraction Layer
//!
//! Provides a unified interface for Fully Homomorphic Encryption operations
//! across software (TFHE-rs), GPU, and ASIC (Intel Heracles) backends.
//!
//! ## Performance
//! - **Software**: TFHE-rs v1.6.1 — pure Rust, 10-50× faster than C++ reference
//! - **GPU**: HEonGPU — CUDA-accelerated CKKS/BFV bootstrapping
//! - **ASIC**: Intel Heracles — 5,000× speedup over Xeon server CPUs
//!   (ISSCC 2026 demonstration: 14µs vs 15ms for encrypted DB query)
//!
//! ## Intel Heracles Specifications
//! - 3nm process, 64 tile-pair compute cores
//! - 48GB HBM2E memory, 819 GB/s bandwidth
//! - Dedicated NTT hardware units for CKKS and BGV schemes
//! - Target: <50µs per FHE transaction
//!
//! Source: ARC42 v20.0 §3 VCBP FHE Hardware Acceleration Abstraction Layer

pub mod engine;
pub mod backends;
pub mod types;
pub mod errors;

pub use engine::FheEngine;
pub use types::{FheBackend, FheCiphertext, FhePlaintext, FheScheme};
pub use errors::FheError;
