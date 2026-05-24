#[derive(Debug, thiserror::Error)]
pub enum TlaError {
    #[error("TLA+ specification parse error: {0}")]
    SpecParseError(String),
    #[error("TLA+ invariant violation: {0}")]
    InvariantViolation(String),
}
