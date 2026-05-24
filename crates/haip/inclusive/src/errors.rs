#[derive(Debug, thiserror::Error)]
pub enum InclusiveError {
    #[error("Accessibility profile not found: {0}")]
    ProfileNotFound(uuid::Uuid),

    #[error("Incompatible features: {0}")]
    IncompatibleFeatures(String),

    #[error("Compliance level insufficient: required {required:?}, actual {actual:?}")]
    ComplianceInsufficient { required: super::types::ComplianceLevel, actual: super::types::ComplianceLevel },
}
