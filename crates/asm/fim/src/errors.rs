#[derive(Debug, thiserror::Error)]
pub enum FimError {
    #[error("Financial invariant violation: {parameter} — {reason}")]
    InvariantViolation { parameter: String, reason: String },
}
