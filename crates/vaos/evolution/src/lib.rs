//! # VAIE – SEVerA‑Verified Self‑Evolving Agents
//!
//! Implements the **SEVerA** framework (arXiv:2603.25111, April 2026):
//! the first formally verified self‑evolving LLM agent system.
//!
//! ## Architecture (Three‑Stage SEVerA Pipeline)
//! 1. **Search** – Synthesises candidate parametric programs containing
//!    Formally Guarded Generative Model (FGGM) calls.
//! 2. **Verification** – Proves correctness with respect to hard constraints
//!    (P1‑P8 safety invariants) for ALL parameter values, reducing the
//!    problem to unconstrained learning.
//! 3. **Learning** – Applies scalable gradient‑based optimisation, including
//!    GRPO‑style fine‑tuning, to improve soft objectives while preserving
//!    correctness.
//!
//! ## Key Guarantee
//! Every accepted agent evolution carries a **formal safety certificate**.
//! Across Dafny program verification, symbolic math synthesis, and policy‑
//! compliant agentic tool use, SEVerA achieves **zero constraint violations**
//! while improving performance over unconstrained baselines.
//!
//! Source: ARC42 v23 Breakthrough 1, ADR‑031

pub mod engine;
pub mod fggm;
pub mod contract;
pub mod types;
pub mod errors;

pub use engine::EvolutionEngine;
pub use fggm::FormallyGuardedGenerativeModel;
pub use contract::SafetyContract;
pub use types::{EvolutionProposal, EvolutionCertificate, EvolutionStage};
pub use errors::EvolutionError;
