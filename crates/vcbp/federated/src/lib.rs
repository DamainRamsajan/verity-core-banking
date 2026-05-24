//! # Verity Core Banking — Federated Learning Mesh
//!
//! Cross‑institution model training without raw data sharing.
//!
//! ## Components
//! - **DSFL**: Dynamic Sharded Federated Learning with O(N·m) communication,
//!   33× latency reduction over Paillier‑based aggregation
//! - **FedSurrogate**: backdoor defense with bidirectional gradient alignment,
//!   FPR<10%, ASR<2.1% under non‑IID data
//! - **FAUN**: Federated Adversarial Unlearning — surgical removal of
//!   poisoned contributions without full retraining
//! - **Federated Ensemble Learning Bridge**: hybrid FL + ensemble methods
//!   for model diversity
//!
//! Source: ARC42 v20.0 §3 VCBP Federated Learning Mesh, ADR‑012

pub mod mesh;
pub mod dsfl;
pub mod defenses;
pub mod ensemble;
pub mod errors;

pub use mesh::FlMesh;
pub use dsfl::DsflAggregator;
pub use defenses::FedSurrogate;
pub use ensemble::EnsembleBridge;
pub use errors::FlError;
