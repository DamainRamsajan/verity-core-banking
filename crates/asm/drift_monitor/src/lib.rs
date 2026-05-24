//! # Verity ASM — DriftMonitor Behavioral Anomaly Detection
//!
//! Real-time ML model per agent type learning normal behavior and flagging
//! deviations. Targets Silent Override attacks — parameter mutations
//! executed by agents without explicit user intent.
//!
//! Uses anomstream-core for streaming anomaly detection (Random Cut Forest,
//! per-feature EWMA / CUSUM, drift detectors, streaming stats).
//!
//! Source: ARC42 v20.0 Addendum v17.0 §A-14

pub mod engine;
pub mod detectors;
pub mod types;
pub mod errors;

pub use engine::DriftMonitor;
pub use types::{DriftStatus, BehavioralBaseline, AnomalyReport};
pub use errors::DriftError;
