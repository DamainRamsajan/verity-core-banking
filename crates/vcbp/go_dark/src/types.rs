use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A confidential trade intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeIntent {
    pub trade_id: Uuid,
    pub asset_pair: String,
    pub side: TradeSide,
    pub quantity: rust_decimal::Decimal,
    pub limit_price: Option<rust_decimal::Decimal>,
    pub institution_id: Uuid,
    pub compliance_checks: Vec<ComplianceCheck>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeSide { Buy, Sell }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub check_type: String,
    pub passed: bool,
    pub details: Option<String>,
}

/// A zero-knowledge proof that a trade satisfies all compliance rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkTradeProof {
    pub trade_id: Uuid,
    pub proof_bytes: Vec<u8>,
    pub public_inputs: Vec<String>,
    pub proof_system: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub verified: bool,
}

/// Level of disclosure for regulatory reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisclosureLevel {
    /// "Show me a proof" — ZK only, no underlying data
    ProofOnly,
    /// Reveal aggregate statistics
    AggregateOnly,
    /// Full disclosure for regulatory audit
    FullDisclosure,
}
