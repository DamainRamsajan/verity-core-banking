use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A memory entry with cryptographic provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub entry_id: Uuid,
    pub agent_id: vaos_core::types::AgentId,
    pub content: serde_json::Value,
    pub entry_type: MemoryEntryType,
    pub lineage_proof: LineageProof,
    pub quarantine_status: QuarantineStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryEntryType { Observation, Inference, ToolOutput, ExternalInput, Consolidation }

/// Cryptographic provenance for a memory entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageProof {
    pub merkle_leaf_hash: [u8; 32],
    pub merkle_proof: Vec<[u8; 32]>,
    pub derivation_edges: Vec<DerivationEdge>,
    pub signature: Vec<u8>,
    pub provenance_score: f64,
}

/// An edge in the derivation DAG — how this entry was derived.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivationEdge {
    pub parent_entry_id: Uuid,
    pub derivation_type: DerivationType,
    pub attribution_weight: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivationType { DirectCopy, Summarization, Inference, ExternalAttribution, Consolidation }

/// Quarantine status of a memory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuarantineStatus {
    Clean,
    Suspicious,
    Quarantined,
    Rejected,
}
