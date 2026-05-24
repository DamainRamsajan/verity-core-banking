#[derive(Debug, thiserror::Error)]
pub enum EdgeError {
    #[error("Offline limit exceeded: limit {limit}, attempted {attempted}, remaining {remaining}")]
    OfflineLimitExceeded { limit: rust_decimal::Decimal, attempted: rust_decimal::Decimal, remaining: rust_decimal::Decimal },
}
