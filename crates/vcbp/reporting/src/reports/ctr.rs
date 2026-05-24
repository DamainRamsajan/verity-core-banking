use serde::{Deserialize, Serialize};

/// Currency Transaction Report (FinCEN CTR) — cash transactions >$10,000.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtrReport {
    pub id: uuid::Uuid,
    pub transaction_id: uuid::Uuid,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub filed_at: chrono::DateTime<chrono::Utc>,
}
