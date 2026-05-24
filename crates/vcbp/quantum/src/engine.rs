use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{Portfolio, OptimizationResult, QubitBackend, OptimizationObjective};
use super::solvers::{QaoaSolver, MaxKCutSolver, ClassicalSolver};
use super::benchmark::HybridBenchmark;
use super::errors::QuantumError;

/// Central quantum optimisation engine.
///
/// Routes optimisation problems to the appropriate solver: quantum (QAOA)
/// when advantage is expected, classical (minilp) when it is not.
pub struct QuantumEngine {
    qaoa: QaoaSolver,
    maxkcut: MaxKCutSolver,
    classical: ClassicalSolver,
    benchmark: HybridBenchmark,
    config: QuantumConfig,
    stats: RwLock<QuantumStats>,
}

#[derive(Debug, Clone)]
pub struct QuantumConfig {
    pub max_qubits: usize,
    pub default_backend: QubitBackend,
    pub min_quantum_advantage: f64,
}

impl Default for QuantumConfig {
    fn default() -> Self {
        Self { max_qubits: 64, default_backend: QubitBackend::Simulator, min_quantum_advantage: 0.01 }
    }
}

#[derive(Debug, Default, Clone)]
pub struct QuantumStats {
    pub optimizations_run: u64,
    pub quantum_solutions: u64,
    pub classical_fallbacks: u64,
    pub advantage_demonstrated: u64,
}

impl QuantumEngine {
    pub fn new(config: QuantumConfig) -> Self {
        Self {
            qaoa: QaoaSolver::new(config.max_qubits),
            maxkcut: MaxKCutSolver::new(config.max_qubits),
            classical: ClassicalSolver::new(),
            benchmark: HybridBenchmark::new(),
            config,
            stats: RwLock::new(QuantumStats::default()),
        }
    }

    /// Optimise a portfolio using the best available solver.
    #[tracing::instrument(name = "quantum.optimize", level = "info", skip(self))]
    pub async fn optimize(
        &self,
        portfolio: &Portfolio,
    ) -> Result<OptimizationResult, QuantumError> {
        let mut stats = self.stats.write().await;
        stats.optimizations_run += 1;

        // Select solver based on objective type and problem size
        match &portfolio.objective {
            OptimizationObjective::MaxKCut { num_clusters } => {
                // Use JPMorgan Max‑k‑Cut QAOA formulation
                let result = self.maxkcut.solve(portfolio, *num_clusters).await?;
                stats.quantum_solutions += 1;
                if result.quantum_advantage.unwrap_or(0.0) > self.config.min_quantum_advantage {
                    stats.advantage_demonstrated += 1;
                }
                Ok(result)
            }
            OptimizationObjective::MaxSharpeRatio | OptimizationObjective::MinVariance => {
                // Use two‑step QAOA for integrated portfolio + risk
                let result = self.qaoa.solve(portfolio).await?;
                stats.quantum_solutions += 1;
                Ok(result)
            }
            _ => {
                // Classical fallback for non‑QAOA objectives
                stats.classical_fallbacks += 1;
                self.classical.solve(portfolio)
            }
        }
    }
}
