use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A regulatory policy update distributed via CRDT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyUpdate {
    pub update_id: Uuid,
    pub regulation: String,
    pub description: String,
    pub formal_rule: String,
    pub published_at: chrono::DateTime<chrono::Utc>,
    pub effective_at: chrono::DateTime<chrono::Utc>,
    pub propagated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Where the policy is enforced in the agent pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyEnforcementPoint {
    PreInference,
    InlineJIT,
    PostInference,
    RuntimeOnly,
}

/// Governance latency measurement.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GovernanceLatency {
    pub regulation_published_at: chrono::DateTime<chrono::Utc>,
    pub policy_propagated_at: chrono::DateTime<chrono::Utc>,
    pub agents_compliant_at: chrono::DateTime<chrono::Utc>,
    pub total_latency_ms: u64,
}
