#[derive(Debug, thiserror::Error)]
pub enum LedgerError {
    #[error("Unbalanced transaction: sum = {0}")]
    UnbalancedTransaction(rust_decimal::Decimal),
    #[error("Merkle tree empty")]
    MerkleTreeEmpty,
}
