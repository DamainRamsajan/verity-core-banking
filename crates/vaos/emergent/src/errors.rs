//! Error types for emergent protocol learning.

#[derive(Debug, thiserror::Error)]
pub enum EmergentError {
    #[error("Protocol unsafe: {0}")]
    ProtocolUnsafe(String),

    #[error("Learning failed: insufficient training data")]
    InsufficientData,
}
