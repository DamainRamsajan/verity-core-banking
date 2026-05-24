use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub from_account: Uuid,
    pub to_account: String,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub rail_type: RailType,
    pub priority: PaymentPriority,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RailType { FedNow, Swift, Ach, FedWire, Chips, Rtp, Iso20022Direct }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentPriority { Low, Normal, High, Critical }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentReceipt {
    pub payment_id: Uuid,
    pub rail_reference: String,
    pub status: PaymentStatus,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub fee: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentStatus { Pending, Accepted, Settled, Rejected, Failed }

#[async_trait]
pub trait PaymentRail: Send + Sync {
    async fn send(&self, payment: &Payment) -> Result<PaymentReceipt, super::PaymentError>;
    fn rail_type(&self) -> RailType;
    fn is_available(&self) -> bool;
    fn supports(&self, currency: &str, amount: rust_decimal::Decimal) -> bool;
}
