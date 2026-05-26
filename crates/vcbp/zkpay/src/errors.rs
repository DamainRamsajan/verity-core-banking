use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZkPayError {
    #[error("Payment rejected: compliance proof invalid")]
    InvalidComplianceProof,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Payment intent expired")]
    ExpiredIntent,
    #[error("Invalid payment intent")]
    InvalidIntent,
}
