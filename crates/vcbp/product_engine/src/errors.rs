#[derive(Debug, thiserror::Error)]
pub enum ProductError {
    #[error("Compilation failed: {0}")]
    CompilationFailed(String),
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    #[error("Temporal contract violation: {contract}: {reason}")]
    TemporalContractViolation { contract: String, reason: String },
}
