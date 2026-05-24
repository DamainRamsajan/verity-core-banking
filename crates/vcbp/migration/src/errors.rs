#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("Parse failed: {0}")]
    ParseFailed(String),
    #[error("Parallel‑run mismatch threshold exceeded")]
    MismatchThresholdExceeded,
    #[error("Cutover not authorised: {days_completed}/{min_days} days")]
    CutoverNotAuthorised { days_completed: u32, min_days: u32 },
    #[error("COBOL file not found: {0}")]
    FileNotFound(String),
}
