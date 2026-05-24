//! # Verity Core Banking — Real‑Time Regulatory Reporter (R3)
//!
//! Generates regulatory filings directly from the Merkle ledger — no batch ETL.
//! All reports are cryptographically verifiable via ZK‑proof audit packages.
//!
//! ## Supported Reports
//! - **FFIEC 041 Call Report** (quarterly)
//! - **OCC / CFPB / FRB filings**
//! - **FinCEN SAR / CTR** (suspicious activity, currency transaction)
//! - **DORA Register of Information** (XBRL‑CSV)
//! - **ECOA adverse action notices** (plain language, ≤Grade 8)
//!
//! ## Architecture
//! - Reports generated in real time from ledger tags
//! - ZK‑proof audit packages enable regulator verification without
//!   exposing underlying transaction data
//! - ADIC replay‑verification integration: every compliance audit trail
//!   produces a machine‑checkable Lean 4 proof
//!
//! Source: ARC42 v20.0 §3 VCBP Real‑Time Regulatory Reporter

pub mod reporter;
pub mod reports;
pub mod zkp;
pub mod templates;
pub mod errors;

pub use reporter::RegulatoryReporter;
pub use zkp::ZkProofAuditPackage;
pub use errors::ReportError;
