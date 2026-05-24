#[derive(Debug, thiserror::Error)]
pub enum InclusiveError {
    #[error("Accessibility profile not found: {0}")]
    ProfileNotFound(uuid::Uuid),
    #[error("Compliance level insufficient")]
    ComplianceInsufficient,
}
