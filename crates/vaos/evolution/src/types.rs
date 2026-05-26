use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A proposed agent improvement (new behaviour, optimised route, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionProposal {
    pub proposal_id: Uuid,
    pub agent_id: vaos_core::types::AgentId,
    pub description: String,
    pub proposed_code: String,
    pub safety_invariants: Vec<String>,
    pub performance_metrics: serde_json::Value,
    pub proposed_at: chrono::DateTime<chrono::Utc>,
}

/// A formal certificate proving an evolution satisfies all safety invariants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionCertificate {
    pub proposal_id: Uuid,
    pub verified: bool,
    pub invariants_checked: Vec<String>,
    pub counterexample: Option<String>,
    pub proof_hash: [u8; 32],
    pub certified_at: chrono::DateTime<chrono::Utc>,
}

/// The three stages of the SEVerA pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionStage {
    Search,
    Verification,
    Learning,
    Accepted,
    Rejected,
}
