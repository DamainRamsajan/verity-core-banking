//! Error types for containment verification.

#[derive(Debug, thiserror::Error)]
pub enum ContainmentError {
    #[error("Containment breach: action {action}: {reason}")]
    ContainmentBreach { action: uuid::Uuid, reason: String },

    #[error("Amount ${amount} exceeds limit ${limit}")]
    AmountExceedsLimit { amount: rust_decimal::Decimal, limit: rust_decimal::Decimal },

    #[error("Counterparty '{counterparty}' not in allowlist")]
    CounterpartyNotAllowed { counterparty: String },

    #[error("Havoc oracle exhausted action space ({0} actions)")]
    OracleExhausted(usize),
}
