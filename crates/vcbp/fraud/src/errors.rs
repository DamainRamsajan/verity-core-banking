#[derive(Debug, thiserror::Error)]
pub enum FraudError {
    #[error("Graph construction failed")]
    GraphConstructionFailed,
}
