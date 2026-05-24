//! # Verity Core Banking — Capability‑Based Banking Operations
//!
//! Maps every banking action to a specific capability token. Enforces the
//! **four‑eyes principle** as a structural invariant at the VM level:
//! critical operations (wire transfers above $10K, loan approvals, GL postings)
//! require tokens from two separate principals — not a policy check, but a
//! **compile‑time guarantee**.
//!
//! ## Token Ontology
//! | Banking Operation | Required Capability Token(s) |
//! |-------------------|------------------------------|
//! | Account debit     | `debit:account:<id>` |
//! | Account credit    | `credit:account:<id>` |
//! | Wire transfer >$10K | `wire:transfer` + `approval:level_2` |
//! | Loan approval     | `loan:approve:<id>` + `risk:signoff` |
//! | GL posting        | `gl:post:<account>` |
//! | Regulatory filing | `regulatory:file:<type>` |
//!
//! ## Safety Guarantees
//! - OWASP Excessive Agency (ASI03) eliminated — agent cannot act without token
//! - Four‑eyes principle is VM‑enforced, not configurational
//! - All operations produce provenance capsules for audit
//!
//! Source: ARC42 v20.0 §3 VCBP Capability‑Based Banking Operations, ADR‑003

pub mod operations;
pub mod tokens;
pub mod dual_control;
pub mod engine;
pub mod errors;

pub use operations::{BankingOperation, DebitOp, CreditOp, WireTransferOp, LoanApprovalOp, GlPostingOp};
pub use tokens::TokenOntology;
pub use dual_control::DualControlEnforcer;
pub use engine::BankingOpsEngine;
pub use errors::BankingOpsError;
