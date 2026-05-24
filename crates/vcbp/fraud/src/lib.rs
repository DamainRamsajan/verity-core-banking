//! # Verity Core Banking — GNN‑Native Real‑Time Fraud Detection
//!
//! Multi‑model ensemble detecting fraud on the Merkle ledger's transaction
//! graph in real time. All models operate with sub‑2ms latency.
//!
//! ## Detection Stack
//! - **SCAFDS** (+15.9pp over GraphSAGE‑AML): edge‑feature graph attention
//!   with attribution‑grounded SAR narrative generation
//! - **AGNAE** (1.12ms per‑tx): RL‑based adaptive exploration for dynamic networks
//! - **GCRMF** (+17.8% F1 cross‑industry AML)
//! - **CMSGNN‑SAO**: spatial attention optimized for large graphs
//! - **Trilemma Detector**: structural invariant — centralized cash‑out patterns
//!
//! Source: ARC42 v20.0 §3 VCBP GNN Fraud Detection Engine

pub mod engine;
pub mod models;
pub mod trilemma;
pub mod types;
pub mod errors;

pub use engine::GnnFraudEngine;
pub use types::{TransactionGraph, FraudScore, FraudAlert};
pub use errors::FraudError;
