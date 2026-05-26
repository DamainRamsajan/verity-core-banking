#[derive(Debug, thiserror::Error)]
pub enum EhvError {
    #[error("Compliance violation – regulation '{regulation}': action '{action}'")]
    ComplianceViolation { regulation: String, action: String },

    #[error("Policy propagation failed: {0}")]
    PropagationFailed(String),

    #[error("JIT compilation failed: {0}")]
    CompilationFailed(String),
}
