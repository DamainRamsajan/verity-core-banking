use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialNetwork {
    pub nodes: Vec<Institution>,
    pub edges: Vec<ExposureEdge>,
    pub snapshot_date: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub id: Uuid,
    pub name: String,
    pub total_assets: rust_decimal::Decimal,
    pub tier1_capital: rust_decimal::Decimal,
    pub leverage_ratio: f64,
    pub is_sib: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposureEdge {
    pub source: Uuid,
    pub target: Uuid,
    pub amount: rust_decimal::Decimal,
    pub channel: RiskChannel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskChannel { Counterparty, FundingRollover, SecuritiesCrossHolding, FireSale, NbfiAmplification }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContagionResult {
    pub initial_shock: Uuid,
    pub defaulted_institutions: Vec<Uuid>,
    pub total_losses: rust_decimal::Decimal,
    pub cascade_rounds: u32,
    pub systemic_risk_score: f64,
}
