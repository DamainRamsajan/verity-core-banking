#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Signing failed: {0}")]
    SigningFailed(String),
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),
    #[error("Algorithm not supported: {0:?}")]
    AlgorithmNotSupported(super::types::KeyAlgorithm),
}
