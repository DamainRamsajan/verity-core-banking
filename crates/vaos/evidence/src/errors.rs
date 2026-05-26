#[derive(Debug, thiserror::Error)]
pub enum EvidenceError {
    #[error("Evidence confidence below minimum threshold: {confidence} < {minimum}")]
    ConfidenceBelowThreshold { confidence: f64, minimum: f64 },

    #[error("Evidence span not verified")]
    EvidenceNotVerified,

    #[error("Audit log integrity violation")]
    AuditIntegrityViolation,
}
