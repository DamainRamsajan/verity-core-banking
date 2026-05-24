//! # Verity Core Banking — PQC Migration & Cryptographic Dependency Scanner
//!
//! Manages the transition of VeriChain and all Verity cryptographic operations
//! from classical (Ed25519, RSA) to post-quantum (ML-DSA-44, ML-KEM-768).
//!
//! ## Migration Timeline
//! - **Phase 1 (2026 H2)**: Discovery & Inventory — PQC keys generated in parallel
//! - **Phase 2 (mid-2027)**: Hybrid signing on non-critical paths
//! - **Phase 3 (2029)**: Classical algorithm deprecation begins
//!
//! ## Standards
//! - NIST FIPS 203 (ML-KEM) — key encapsulation
//! - NIST FIPS 204 (ML-DSA) — digital signatures
//! - NIST FIPS 205 (SLH-DSA) — stateless hash-based signatures
//! - G7 CEG PQC Roadmap (January 2026)
//! - Google 2029 PQC migration target
//!
//! Source: ARC42 v20.0 §3 VCBP PQC Migration, ADR-011, ADR-023

pub mod engine;
pub mod migration;
pub mod scanner;
pub mod reencrypt;
pub mod types;
pub mod errors;

pub use engine::PqcEngine;
pub use migration::MigrationManager;
pub use scanner::CryptoDependencyScanner;
pub use reencrypt::LongLivedReencryptor;
pub use types::{MigrationPhase, PqcAlgorithm, HybridSignature};
pub use errors::PqcError;
