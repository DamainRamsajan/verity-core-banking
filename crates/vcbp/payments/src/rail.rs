use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::errors::PaymentError;

/// Payment instruction to be sent over a rail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub from_account: uuid::Uuid,
    pub to_account: String,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub rail_type: RailType,
    pub priority: PaymentPriority,
    pub capability_token: vaos_core::types::CapabilityToken,
    pub metadata: serde_json::Value,
}

/// Receipt confirming payment was accepted by the rail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentReceipt {
    pub payment_id: Uuid,
    pub rail_reference: String,
    pub status: PaymentStatus,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub fee: Option<rust_decimal::Decimal>,
}

/// Which payment rail to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RailType {
    FedNow,
    Swift,
    Ach,
    FedWire,
    Chips,
    Rtp,
    Iso20022Direct,
    ProjectKeystone,
}

/// Payment priority for smart routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentPriority {
    Low,       // Batch‑ok (ACH)
    Normal,    // Same‑day
    High,      // Real‑time (FedNow, RTP)
    Critical,  // Immediate with fallback (FedWire)
}

/// Final status of a payment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Accepted,
    Settled,
    Rejected,
    Failed,
}

/// The core trait for any payment rail.
#[async_trait]
pub trait PaymentRail: Send + Sync {
    /// Send a payment over this rail.
    async fn send(&self, payment: &Payment) -> Result<PaymentReceipt, PaymentError>;

    /// The type of rail this is.
    fn rail_type(&self) -> RailType;

    /// Whether this rail is currently available.
    fn is_available(&self) -> bool;

    /// Whether this rail supports the given currency and amount.
    fn supports(&self, currency: &str, amount: rust_decimal::Decimal) -> bool;
}
