#[derive(Debug, thiserror::Error)]
pub enum GuardError {
    #[error("Security violation: {0:?}")]
    SecurityViolation(Vec<super::types::SecurityEvent>),
    #[error("Sandbox execution failed: {0}")]
    SandboxExecutionFailed(String),
}
