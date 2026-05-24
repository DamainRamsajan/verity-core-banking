use serde::{Deserialize, Serialize};
use vaos_core::types::AgentId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DriftStatus { WithinBounds, Anomalous(AnomalyReport), Critical(AnomalyReport) }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralBaseline { pub agent_id: AgentId, pub mean_vector: Vec<f64>, pub covariance: Vec<f64>, pub samples: u64 }

impl BehavioralBaseline {
    pub fn compute_deviation(&self, _action: &serde_json::Value) -> f64 { 0.1 }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyReport { pub agent_id: AgentId, pub deviation: f64, pub severity: u8, pub timestamp: chrono::DateTime<chrono::Utc> }
