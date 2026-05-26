use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// An evidence span with contribution measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceSpan {
    pub span_id: Uuid,
    pub source_url: String,
    pub source_text: String,
    pub confidence: f64,
    pub verified: bool,
    /// How much this specific evidence contributed to the learning outcome (0.0–1.0).
    pub contribution_score: f64,
}

/// A learning event recorded by an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    pub event_id: Uuid,
    pub agent_id: vaos_core::types::AgentId,
    pub description: String,
    pub evidence: EvidenceSpan,
    pub learned_at: chrono::DateTime<chrono::Utc>,
    pub deployed: bool,
}

/// An audit record – Merkle‑proofed, cryptographically signed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub record_id: Uuid,
    pub event: LearningEvent,
    pub merkle_proof_hash: [u8; 32],
    pub signature: Vec<u8>,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
}
