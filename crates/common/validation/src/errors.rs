#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid currency code: {0}")]
    InvalidCurrency(String),
    #[error("Invalid BIAN domain: {0}")]
    InvalidBianDomain(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Invalid account ID: {0}")]
    InvalidAccountId(String),
    #[error("Regulatory constraint violated: {regulation} — {detail}")]
    RegulatoryConstraintViolated { regulation: String, detail: String },
}
