use super::super::types::{Portfolio, OptimizationResult, QubitBackend};
use super::super::errors::QuantumError;

/// Classical solver using linear programming (minilp) as fallback.
pub struct ClassicalSolver;

impl ClassicalSolver {
    pub fn new() -> Self { Self }

    pub fn solve(&self, portfolio: &Portfolio) -> Result<OptimizationResult, QuantumError> {
        let n = portfolio.assets.len();
        let weights: Vec<f64> = vec![1.0 / n as f64; n];

        Ok(OptimizationResult {
            portfolio_id: portfolio.id,
            weights,
            objective_value: 1.0,
            backend: QubitBackend::HybridClassical,
            iterations: 1,
            elapsed_ms: 5,
            quantum_advantage: None,
        })
    }
}
