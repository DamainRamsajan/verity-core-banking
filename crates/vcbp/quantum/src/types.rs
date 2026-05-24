use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A portfolio of assets to optimise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: Uuid,
    pub assets: Vec<Asset>,
    pub constraints: Vec<PortfolioConstraint>,
    pub objective: OptimizationObjective,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub symbol: String,
    pub expected_return: f64,
    pub volatility: f64,
    pub weight_min: f64,
    pub weight_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioConstraint {
    pub constraint_type: ConstraintType,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    MaxPosition,
    MinPosition,
    SectorLimit(String),
    TurnoverLimit,
    LiquidityRatio,
    BaselCapitalCharge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationObjective {
    MaxSharpeRatio,
    MinVariance,
    MaxReturn { risk_budget: f64 },
    RiskParity,
    MaxKCut { num_clusters: usize },
}

/// Result of an optimisation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub portfolio_id: Uuid,
    pub weights: Vec<f64>,
    pub objective_value: f64,
    pub backend: QubitBackend,
    pub iterations: u64,
    pub elapsed_ms: u64,
    pub quantum_advantage: Option<f64>,
}

/// Available quantum backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QubitBackend {
    Simulator,
    IonQ,
    IBMQ,
    Rigetti,
    HybridClassical,
}
