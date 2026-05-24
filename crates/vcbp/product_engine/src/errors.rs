#[derive(Debug, thiserror::Error)]
pub enum ProductError {
    #[error("Compilation failed: {0}")]
    CompilationFailed(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Temporal contract violation: {contract}: {reason}")]
    TemporalContractViolation { contract: String, reason: String },

    #[error("Regulatory invariant violation: {invariant}")]
    RegulatoryViolation { invariant: String },

    #[error("ASL syntax error: line {line}, column {col}: {message}")]
    SyntaxError { line: usize, col: usize, message: String },

    #[error("Capability not found: {0}")]
    CapabilityNotFound(String),
}
