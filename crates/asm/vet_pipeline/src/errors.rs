#[derive(Debug, thiserror::Error)]
pub enum VetError {
    #[error("Static analysis failed: {0}")]
    StaticAnalysisFailed(String),
    #[error("Dynamic sandbox detected malicious behavior")]
    DynamicSandboxFailed,
    #[error("Semantic payload scan detected hidden instructions")]
    SemanticScanFailed,
    #[error("Human review rejected")]
    HumanReviewRejected,
}
