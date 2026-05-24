use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeConfig {
    pub node_id: String,
    pub reservation_limit: rust_decimal::Decimal,
    pub sync_interval_secs: u64,
}

impl Default for EdgeConfig {
    fn default() -> Self {
        Self {
            node_id: format!("EDGE-{}", Uuid::new_v4()),
            reservation_limit: rust_decimal::Decimal::new(100_000, 0),
            sync_interval_secs: 300,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineTransaction {
    pub id: Uuid,
    pub from_account: Uuid,
    pub to_account: String,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub signature: Vec<u8>,
    pub synced: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus { Online, Offline, Syncing }
