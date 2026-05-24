#[derive(Debug, thiserror::Error)]
pub enum GuardError {
    #[error("Security violation: {0:?}")]
    SecurityViolation(Vec<super::types::SecurityEvent>),
    #[error("Sandbox execution failed: {0}")]
    SandboxExecutionFailed(String),
    #[error("MCP tool descriptor validation failed")]
    McpValidationFailed,
    #[error("Boiling the Frog pattern detected (cumulative risk: {0})")]
    BoilingFrogDetected(f64),
}
