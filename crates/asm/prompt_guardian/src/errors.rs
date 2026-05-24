#[derive(Debug, thiserror::Error)]
pub enum GuardianError {
    #[error("Input exceeds maximum length: {0} bytes")]
    InputTooLarge(usize),
    #[error("Injection detection failed: {0}")]
    DetectionFailed(String),
    #[error("Encoded content decode failed: {0}")]
    DecodeError(String),
}
