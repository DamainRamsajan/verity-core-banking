#[derive(Debug, thiserror::Error)]
pub enum FheError {
    #[error("FHE backend not available: {0:?}")]
    BackendNotAvailable(super::types::FheBackend),

    #[error("FHE scheme mismatch: {a:?} vs {b:?}")]
    SchemeMismatch { a: super::types::FheScheme, b: super::types::FheScheme },

    #[error("Noise budget exhausted: {remaining} bits remaining, {needed} bits needed")]
    NoiseBudgetExhausted { remaining: u32, needed: u32 },

    #[error("FHE encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("FHE operation failed: {0}")]
    OperationFailed(String),
}
