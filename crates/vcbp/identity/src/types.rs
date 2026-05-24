use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub agent_id: vaos_core::types::AgentId,
    pub binary_hash: [u8; 32],
    pub did: String,
    pub kya_credential_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartAccount {
    pub account_id: Uuid,
    pub spending_limit: SpendingLimit,
    pub human_principal: Option<String>,
    pub frozen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingLimit {
    pub daily: rust_decimal::Decimal,
    pub per_transaction: rust_decimal::Decimal,
}
