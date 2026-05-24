//! Error types for runtime TLA+ checking.

#[derive(Debug, thiserror::Error)]
pub enum TlaError {
    #[error("TLA+ invariant violation: '{invariant}' — {detail}")]
    InvariantViolation { invariant: String, detail: String },

    #[error("Malformed transaction: cannot extract entries")]
    MalformedTransaction,

    #[error("TLA+ specification not loaded")]
    SpecificationNotLoaded,

    #[error("Model check timeout after {0}ms")]
    ModelCheckTimeout(u64),
}
