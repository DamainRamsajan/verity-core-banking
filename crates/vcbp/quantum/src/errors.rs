#[derive(Debug, thiserror::Error)]
pub enum QuantumError { #[error("Solver timeout")] SolverTimeout }
