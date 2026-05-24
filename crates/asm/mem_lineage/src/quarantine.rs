use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use super::types::MemoryEntry;
use super::errors::LineageError;

pub struct QuarantineManager { entries: RwLock<HashMap<Uuid, MemoryEntry>>, ttl_hours: u64 }

impl QuarantineManager {
    pub fn new(ttl_hours: u64) -> Self { Self { entries: RwLock::new(HashMap::new()), ttl_hours } }
    pub async fn isolate(&self, entry: &MemoryEntry) -> Result<(), LineageError> {
        self.entries.write().await.insert(entry.entry_id, entry.clone());
        tracing::warn!(entry_id = %entry.entry_id, "Memory entry quarantined");
        Ok(())
    }
}
