use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeIntent {
    pub trade_id: Uuid,
    pub asset_pair: String,
    pub side: TradeSide,
    pub quantity: rust_decimal::Decimal,
    pub limit_price: Option<rust_decimal::Decimal>,
    pub institution_id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeSide { Buy, Sell }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkTradeProof {
    pub trade_id: Uuid,
    pub proof_bytes: Vec<u8>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub verified: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisclosureLevel { ProofOnly, AggregateOnly, FullDisclosure }
