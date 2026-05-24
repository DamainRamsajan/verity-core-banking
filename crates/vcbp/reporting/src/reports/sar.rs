use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarReport {
    pub id: uuid::Uuid,
    pub filing_institution: String,
    pub suspicious_activity: String,
    pub amount: rust_decimal::Decimal,
    pub account_ids: Vec<uuid::Uuid>,
    pub filed_at: chrono::DateTime<chrono::Utc>,
}
