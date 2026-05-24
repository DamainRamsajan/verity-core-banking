use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: Uuid,
    pub assets: Vec<Asset>,
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
pub struct OptimizationResult {
    pub portfolio_id: Uuid,
    pub weights: Vec<f64>,
    pub objective_value: f64,
    pub quantum_advantage: Option<f64>,
}
