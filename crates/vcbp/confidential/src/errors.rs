use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfidentialError {
    #[error("TFHE encryption failed: {0}")]
    EncryptionError(String),
    #[error("TFHE decryption failed: {0}")]
    DecryptionError(String),
    #[error("Feature not available (requires 'confidential-mode')")]
    FeatureNotAvailable,
    #[error("Multi‑key mode requires at least two public keys")]
    MultiKeyError,
}
