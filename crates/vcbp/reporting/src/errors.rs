#[derive(Debug, thiserror::Error)]
pub enum ReportError {
    #[error("Insufficient data for report")]
    InsufficientData,

    #[error("Report generation failed: {0}")]
    GenerationFailed(String),

    #[error("ZK‑proof generation failed: {0}")]
    ZkProofGenerationFailed(String),
}
