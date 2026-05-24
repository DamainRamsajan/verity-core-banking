#[derive(Debug, thiserror::Error)]
pub enum FlError { #[error("Aggregation failed")] AggregationFailed }
