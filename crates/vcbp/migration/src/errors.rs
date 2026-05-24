#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("Migration mismatch threshold exceeded")]
    MismatchThresholdExceeded,
    #[error("Cutover not authorised: {days_completed}/{min_days} days")]
    CutoverNotAuthorised { days_completed: u32, min_days: u32 },
}
