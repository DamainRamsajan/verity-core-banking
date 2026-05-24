use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::types::{MemoryEntry, MemoryEntryType, QuarantineStatus};
use super::merkle::MerkleLog;
use super::errors::LineageError;

#[allow(dead_code)]
pub struct MemLineageEngine {
    memory: RwLock<HashMap<Uuid, MemoryEntry>>,
    merkle: RwLock<MerkleLog>,
    config: LineageConfig,
}

#[derive(Debug, Clone)]
pub struct LineageConfig {
    pub max_derivation_depth: u32,
    pub provenance_threshold: f64,
    pub quarantine_ttl_hours: u64,
}

impl Default for LineageConfig {
    fn default() -> Self {
        Self { max_derivation_depth: 10, provenance_threshold: 0.5, quarantine_ttl_hours: 720 }
    }
}

impl MemLineageEngine {
    pub fn new(config: LineageConfig) -> Self {
        Self {
            memory: RwLock::new(HashMap::new()),
            merkle: RwLock::new(MerkleLog::new()),
            config,
        }
    }

    pub async fn write(
        &self,
        agent_id: vaos_core::types::AgentId,
        content: serde_json::Value,
        entry_type: MemoryEntryType,
    ) -> Result<MemoryEntry, LineageError> {
        let entry_id = Uuid::new_v4();
        let entry = MemoryEntry {
            entry_id,
            agent_id,
            content,
            entry_type,
            quarantine_status: QuarantineStatus::Clean,
            created_at: chrono::Utc::now(),
        };

        // Insert into Merkle log
        self.merkle.write().await.insert(entry_id)?;

        // Store
        self.memory.write().await.insert(entry_id, entry.clone());

        Ok(entry)
    }

    pub async fn read(&self, entry_id: Uuid) -> Result<MemoryEntry, LineageError> {
        let mem = self.memory.read().await;
        mem.get(&entry_id).cloned().ok_or(LineageError::EntryNotFound(entry_id))
    }

    pub async fn quarantine(&self, entry_id: Uuid) -> Result<(), LineageError> {
        let mut mem = self.memory.write().await;
        if let Some(entry) = mem.get_mut(&entry_id) {
            entry.quarantine_status = QuarantineStatus::Quarantined;
            Ok(())
        } else {
            Err(LineageError::EntryNotFound(entry_id))
        }
    }
}
