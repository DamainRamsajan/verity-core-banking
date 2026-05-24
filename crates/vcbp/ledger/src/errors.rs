#[derive(Debug, thiserror::Error)]
pub enum LedgerError {
    #[error("Transaction unbalanced: sum = {0}")]
    UnbalancedTransaction(rust_decimal::Decimal),
    #[error("Account not found: {0:?}")]
    AccountNotFound(uuid::Uuid),
    #[error("Merkle tree empty")]
    MerkleTreeEmpty,
    #[error("TLA+ invariant violation")]
    TlaInvariantViolation,
    #[error("Financial invariant violation: {0}")]
    FimViolation(String),
}
