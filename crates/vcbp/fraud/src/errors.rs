#[derive(Debug, thiserror::Error)]
pub enum FraudError {
    #[error("Model inference failed: {0}")]
    InferenceFailed(String),
    #[error("Graph construction failed")]
    GraphConstructionFailed,
}
