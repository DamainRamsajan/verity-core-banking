use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub from_account: Uuid,
    pub to_account: String,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub rail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub payment_id: Uuid,
    pub status: String,
    pub rail_reference: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
