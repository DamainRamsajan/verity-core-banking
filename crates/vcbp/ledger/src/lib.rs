//! # Verity Core Banking — Merkle Double‑Entry Ledger
//!
//! Append‑only, event‑sourced, CQRS‑separated ledger with Merkle proofs
//! and TLA+‑verified capital safety. Every transaction produces balanced
//! debit/credit entries with O(log N) inclusion proofs.
//!
//! ## Architecture
//! - **Event‑sourced**: all state derived from the immutable transaction log
//! - **CQRS**: strict separation between write‑side (command) and read‑side (query)
//! - **Merkle proofs**: BLAKE3‑based Merkle tree over all transaction hashes
//! - **TLA+ verified**: Conservation of Value (Σ entries = 0) enforced at
//!   compile time by the TLA+ spec and continuously validated at runtime
//! - **Compliance‑in‑the‑write‑path**: regulatory rules checked at commit time
//!
//! ## Performance
//! - <50ms P99 ledger append (local)
//! - Zero over‑commitment vs. optimistic locking's 509.3%
//!
//! Source: ARC42 v20.0 §3 VCBP Merkle Double‑Entry Ledger, ADR‑002

pub mod merkle_ledger;
pub mod event_store;
pub mod proof;
pub mod positions;
pub mod tla_verifier;
pub mod fim;
pub mod types;
pub mod errors;

pub use merkle_ledger::MerkleLedger;
pub use event_store::EventStore;
pub use proof::MerkleProof;
pub use positions::PositionKeeper;
pub use types::{Transaction, Entry, AccountId, Currency, Balance};
pub use errors::LedgerError;
