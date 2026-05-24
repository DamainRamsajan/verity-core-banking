//! # Verity ASM — MemLineage Memory Integrity Guardian
//!
//! Lineage-guided enforcement for agent memory. Attaches cryptographic
//! provenance and LLM-mediated derivation lineage to every memory entry.
//! MemLineage is "the only configuration that drives all three columns
//! to zero ASR, while sub-millisecond per-operation overhead."
//!
//! ## Architecture
//! - **RFC-6962 Merkle log** over per-principal Ed25519-signed entries
//! - **Weighted derivation DAG**: tracks how each memory entry was derived
//! - **Quarantine partitioning**: suspicious memories isolated in graph partition
//! - **Untrusted-Path Persistence**: chains whose attribution edges remain
//!   above threshold are blocked from influencing agent decisions
//!
//! ## Defenses
//! - ShadowMerge (93.8% ASR) — blocked
//! - Trojan Hippo (85-100% ASR) — dormant payload detection
//! - OEP (self-evolving poison) — non-transferable experience detection
//!
//! Source: ARC42 v20.0 Addendum v17.0 §A-11, MemLineage paper (May 2026)

pub mod engine;
pub mod merkle;
pub mod dag;
pub mod quarantine;
pub mod types;
pub mod errors;

pub use engine::MemLineageEngine;
pub use types::{MemoryEntry, LineageProof, DerivationEdge, QuarantineStatus};
pub use errors::LineageError;
