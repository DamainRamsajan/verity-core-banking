use serde::{Deserialize, Serialize};
use vaos_core::types::AgentId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KillLevel { Pause, Suspend, Terminate }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillSwitchAction { pub agent_id: AgentId, pub level: KillLevel, pub reason: String, pub timestamp: chrono::DateTime<chrono::Utc>, pub snapshot: Option<ForensicSnapshot> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicSnapshot { pub agent_id: AgentId, pub snapshot_hash: [u8; 32], pub captured_at: chrono::DateTime<chrono::Utc>, pub memory_size_bytes: u64 }
