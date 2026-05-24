use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Asset classification per ISO 4217 and FATF guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetClass {
    FiatCurrency,
    TokenizedDeposit,
    DigitalAsset,
    TokenizedSecurity,
    PreciousMetal,
    Cbdc,
}

/// An account's position in a specific asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetPosition {
    pub account_id: Uuid,
    pub asset_class: AssetClass,
    pub currency_code: String,
    pub balance: rust_decimal::Decimal,
    pub reserved: rust_decimal::Decimal,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// A currency pair for FX rate quoting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyPair {
    pub base: String,
    pub quote: String,
    pub rate: rust_decimal::Decimal,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
}
