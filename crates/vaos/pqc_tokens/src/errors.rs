//! Error types for PQC token engine.

#[derive(Debug, thiserror::Error)]
pub enum PqcError {
    #[error("PQC signature invalid")]
    PqcSignatureInvalid,

    #[error("Hybrid signature mismatch: classical valid but PQC failed")]
    HybridSignatureMismatch,

    #[error("Migration key mismatch")]
    MigrationKeyMismatch,

    #[error("Algorithm not supported in current migration phase")]
    AlgorithmNotSupported,
}
