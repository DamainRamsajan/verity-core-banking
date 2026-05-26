#[derive(Debug, thiserror::Error)]
pub enum EvolutionError {
    #[error("Daily proposal limit exceeded (max {max})")]
    DailyLimitExceeded { max: u32 },

    #[error("Safety contract violation: {0}")]
    ContractViolation(String),

    #[error("FGGM verification failed: {0}")]
    VerificationFailed(String),

    #[error("Proposal not found: {0}")]
    ProposalNotFound(uuid::Uuid),

    #[error("Z3 solver error: {0}")]
    SolverError(String),
}