#[derive(Debug, thiserror::Error)]
pub enum GoDarkError {
    #[error("Compliance check failed: {0}")]
    ComplianceCheckFailed(String),

    #[error("ZK proof generation failed: {0}")]
    ProofGenerationFailed(String),

    #[error("ZK proof verification failed: {0}")]
    ProofVerificationFailed(String),

    #[error("Trade value below minimum: {value} < {minimum}")]
    TradeValueBelowMinimum { value: rust_decimal::Decimal, minimum: rust_decimal::Decimal },
}
