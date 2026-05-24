use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryEntryType { Observation, Inference, ToolOutput, ExternalInput, Consolidation }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuarantineStatus { Clean, Suspicious, Quarantined, Rejected }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub entry_id: Uuid,
    pub agent_id: vaos_core::types::AgentId,
    pub content: serde_json::Value,
    pub entry_type: MemoryEntryType,
    pub quarantine_status: QuarantineStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
