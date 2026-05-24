//! # Verity Core Banking — Edge Banking Runtime
//!
//! Lightweight offline‑first banking runtime for branches, ATMs, and IoT
//! edge devices. Implements the **Crunchfish Governed Offline Payments**
//! architecture: a reservation‑based Layer‑2 that preserves central ledger
//! authority while enabling disconnected operation.
//!
//! ## Architecture
//! - **Reserve–Pay–Settle lifecycle**: offline wallets hold a pre‑reserved
//!   balance; transactions spend against the reservation; settlement syncs
//!   on reconnection
//! - **Mesh synchronisation**: cryptographic conflict resolution when
//!   multiple offline nodes reconnect
//! - **Bounded exposure**: risk is borne by the issuer, not the payee;
//!   offline spending cannot exceed the reservation
//!
//! ## Market Validation
//! - **Insolify FinCore**: 300+ banks across Africa and Middle East using
//!   predictive edge computing for offline transaction processing
//! - **Crunchfish**: patented architecture deployed in production payment
//!   systems globally
//!
//! Source: ARC42 v20.0 §3 VCBP Edge Banking Runtime, ADR-009

pub mod runtime;
pub mod mesh;
pub mod reservation;
pub mod types;
pub mod errors;

pub use runtime::EdgeRuntime;
pub use mesh::MeshSync;
pub use reservation::ReservationPool;
pub use types::{EdgeConfig, OfflineTransaction, SyncStatus};
pub use errors::EdgeError;
