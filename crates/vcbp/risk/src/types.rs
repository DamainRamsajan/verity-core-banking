use serde::{Deserialize, Serialize};

/// A financial network for contagion simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialNetwork {
    pub nodes: Vec<Institution>,
    pub edges: Vec<ExposureEdge>,
    pub snapshot_date: chrono::NaiveDate,
}

/// A financial institution in the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub id: uuid::Uuid,
    pub name: String,
    pub total_assets: rust_decimal::Decimal,
    pub tier1_capital: rust_decimal::Decimal,
    pub leverage_ratio: f64,
    pub is_sib: bool,
}

/// A directed exposure between two institutions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposureEdge {
    pub source: uuid::Uuid,
    pub target: uuid::Uuid,
    pub amount: rust_decimal::Decimal,
    pub channel: RiskChannel,
}

/// Propagation channels per IMF/ECB multilayer model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskChannel {
    Counterparty,
    FundingRollover,
    SecuritiesCrossHolding,
    FireSale,
    NbfiAmplification,
}

/// Result of a contagion simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContagionResult {
    pub initial_shock: uuid::Uuid,
    pub defaulted_institutions: Vec<uuid::Uuid>,
    pub total_losses: rust_decimal::Decimal,
    pub cascade_rounds: u32,
    pub capital_depletion_pct: f64,
    pub systemic_risk_score: f64,
}
