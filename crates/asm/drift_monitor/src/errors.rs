#[derive(Debug, thiserror::Error)]
pub enum DriftError { #[error("Baseline not established for agent {0:?}")] BaselineNotEstablished(vaos_core::types::AgentId), #[error("Anomaly detection failed: {0}")] DetectionFailed(String) }
