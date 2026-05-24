#[derive(Debug, thiserror::Error)]
pub enum QuantumError {
    #[error("Problem too large: {qubits_needed} qubits needed, {qubits_available} available")]
    ProblemTooLarge { qubits_needed: usize, qubits_available: usize },
    #[error("Solver timeout")]
    SolverTimeout,
    #[error("Quantum backend unavailable: {0:?}")]
    BackendUnavailable(super::types::QubitBackend),
    #[error("Invalid portfolio: {0}")]
    InvalidPortfolio(String),
}
