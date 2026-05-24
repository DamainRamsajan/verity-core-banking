use super::types::{Portfolio, OptimizationResult, QubitBackend};

/// Hybrid quantum‑classical benchmark framework.
///
/// Compares quantum solutions against classical solvers on identical
/// problem instances. Invokes quantum backend only when demonstrable
/// advantage exists.
pub struct HybridBenchmark {
    history: Vec<BenchmarkRun>,
}

#[derive(Debug, Clone)]
pub struct BenchmarkRun {
    pub portfolio_id: uuid::Uuid,
    pub quantum_result: OptimizationResult,
    pub classical_result: OptimizationResult,
    pub advantage_ratio: f64,
}

impl HybridBenchmark {
    pub fn new() -> Self { Self { history: Vec::new() } }

    pub fn record(&mut self, quantum: OptimizationResult, classical: OptimizationResult) {
        let advantage = if classical.objective_value > 0.0 {
            quantum.objective_value / classical.objective_value - 1.0
        } else {
            0.0
        };
        self.history.push(BenchmarkRun {
            portfolio_id: quantum.portfolio_id,
            quantum_result: quantum,
            classical_result: classical,
            advantage_ratio: advantage,
        });
    }

    pub fn advantage_demonstrated(&self) -> Option<f64> {
        if self.history.is_empty() { None }
        else {
            Some(self.history.iter().map(|r| r.advantage_ratio).sum::<f64>() / self.history.len() as f64)
        }
    }
}
