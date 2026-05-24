#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("No rail available for {currency} at amount {amount}")]
    NoRailAvailable { currency: String, amount: rust_decimal::Decimal },

    #[error("Rail not found: {0:?}")]
    RailNotFound(super::rail::RailType),

    #[error("Circuit breaker open")]
    CircuitOpen,

    #[error("Risk threshold exceeded: {0:.2}")]
    RiskThresholdExceeded(f64),

    #[error("Payment rejected: {0}")]
    PaymentRejected(String),
}
