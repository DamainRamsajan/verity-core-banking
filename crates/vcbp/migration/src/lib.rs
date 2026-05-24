//! # Verity Core Banking — Legacy Core Migration Toolkit
//!
//! Deterministic COBOL/Java analysis and multi‑LLM retro‑documentation for
//! migrating legacy banking systems to Verity. Every migration is validated
//! by a parallel‑run simulator that compares legacy and Verity outputs.
//!
//! ## Architecture
//! - **COBOL Parser**: tree‑sitter COBOL grammar (arborium-cobol v2.12.0)
//!   for deterministic business rule extraction
//! - **Claude Code Integration**: Anthropic Claude Code for dependency
//!   mapping and incremental refactoring analysis
//! - **Parallel‑Run Simulator**: runs legacy system and Verity Core Banking
//!   simultaneously for ≥90 days, comparing every transaction output
//! - **Multi‑LLM Retro‑Documentation**: BNP Paribas pipeline for generating
//!   functional and technical documentation from COBOL source code
//!
//! ## Migration Phases
//! 1. Discovery — COBOL parsing, business rule extraction, schema mapping
//! 2. Rule Extraction — ASL product definition generation
//! 3. Validation — Parallel‑run with automated comparison
//! 4. Cutover — Phased service cutover with one‑click rollback
//!
//! Source: ARC42 v20.0 §3 VCBP Legacy Core Migration Toolkit, ADR‑010

pub mod engine;
pub mod cobol;
pub mod parallel_run;
pub mod documentation;
pub mod types;
pub mod errors;

pub use engine::MigrationEngine;
pub use cobol::CobolParser;
pub use parallel_run::ParallelRunSimulator;
pub use documentation::DocumentationPipeline;
pub use types::{MigrationConfig, MigrationPhase, MigrationReport};
pub use errors::MigrationError;
