#[derive(Debug, thiserror::Error)]
pub enum PqcError {
    #[error("PQC signature generation failed")]
    SignatureGenerationFailed,
    #[error("PQC key generation failed")]
    KeyGenerationFailed,
    #[error("Migration liveness condition not met: Δeff={effective_window}, required={required}")]
    LivenessConditionFailed { effective_window: f64, required: f64 },
    #[error("Dependency scan failed: {0}")]
    ScanFailed(String),
    #[error("Re-encryption failed: {0}")]
    ReencryptionFailed(String),
}
