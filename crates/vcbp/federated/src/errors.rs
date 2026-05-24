#[derive(Debug, thiserror::Error)]
pub enum FlError {
    #[error("Aggregation failed: {0}")]
    AggregationFailed(String),
    #[error("Poisoning detected")]
    PoisoningDetected,
}
