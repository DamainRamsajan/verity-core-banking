//! # VAIE – EVE‑Agent Evidence‑Verifiable Learning Audit
//!
//! Implements the **EVE‑Agent** framework (arXiv:2605.22905, May 2026):
//! evidence‑verifiable self‑evolution where every training example carries
//! an inspectable source span that explains why it should be trusted.
//!
//! ## Architecture
//! - **Evidence Span Generation** – every agent learning event carries a
//!   source‑grounded, inspectable reference explaining why the improvement
//!   is valid.
//! - **Auditable Curriculum** – the resulting curriculum is not merely
//!   self‑generated but auditable by construction.
//! - **Mer‑kle‑Proofed Audit Trail** – every evidence span is appended to
//!   the Merkle‑proofed provenance log for regulatory audit.
//!
//! ## Key Guarantee
//! "Every lesson our agents learn carries a source reference explaining
//! why it should be trusted. Here is the audit log of everything our
//! Fraud Agent learned this week, with evidence for every conclusion."
//!
//! Source: ARC42 v23 Breakthrough 6, ADR‑036

pub mod engine;
pub mod audit;
pub mod types;
pub mod errors;

pub use engine::EvidenceEngine;
pub use audit::LearningAuditLog;
pub use types::{EvidenceSpan, LearningEvent, AuditRecord};
pub use errors::EvidenceError;
