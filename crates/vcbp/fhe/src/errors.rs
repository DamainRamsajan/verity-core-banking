#[derive(Debug, thiserror::Error)]
pub enum FheError {
    #[error("FHE backend not available: {0:?}")]
    BackendNotAvailable(super::types::FheBackend),
    #[error("FHE scheme mismatch: {a:?} vs {b:?}")]
    SchemeMismatch { a: super::types::FheScheme, b: super::types::FheScheme },
    #[error("Noise budget exhausted")]
    NoiseBudgetExhausted,
}
