#[derive(Debug, thiserror::Error)]
pub enum GuardianError {
    #[error("Input exceeds maximum length: {0} bytes")]
    InputTooLarge(usize),
    #[error("JailGuard classification failed: {0}")]
    JailGuardError(String),
    #[error("Armorer scan failed: {0}")]
    ArmorerError(String),
    #[error("llm-guard scan failed: {0}")]
    LlmGuardError(String),
    #[error("Encoded content decode failed: {0}")]
    DecodeError(String),
}
