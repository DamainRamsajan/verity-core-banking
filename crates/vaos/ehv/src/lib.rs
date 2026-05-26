//! # VAIE – EHV‑Style Governance‑Aware JIT Compiler
//!
//! Implements the **EHV** architecture (arXiv:2605.17909, May 2026):
//! a Governance‑Aware Just‑In‑Time Compiler that relocates the Policy
//! Enforcement Point (PEP) into the inference pipeline.
//!
//! ## Architecture
//! - **CRDT‑Synchronised Policy Network** – regulatory changes are distributed
//!   globally via Conflict‑free Replicated Data Types, achieving O(1)
//!   propagation latency.
//! - **Governance‑Aware JIT Compiler** – inlines policy checks into every
//!   agent's inference path at compile‑time, making non‑compliant actions
//!   **computationally unreachable**.
//! - **TLA+ Formal Verification** – proves that non‑compliant actions cannot
//!   be reached within the system's bounded operating state space.
//!
//! ## Key Guarantee
//! Reduces Governance Latency from O(days) – the 14‑30 day auditing gap
//! in current frameworks like ISO/IEC 42001 and NIST AI RMF – to O(1).
//! "When a regulator publishes a new rule at 9:00 AM, every Verity agent
//! worldwide is compliant by 9:00:01 AM."
//!
//! Source: ARC42 v23 Breakthrough 2, ADR‑032

pub mod engine;
pub mod compiler;
pub mod policy;
pub mod types;
pub mod errors;

pub use engine::EhvEngine;
pub use compiler::GovernanceJitCompiler;
pub use policy::PolicyNetwork;
pub use types::{PolicyUpdate, PolicyEnforcementPoint, GovernanceLatency};
pub use errors::EhvError;
