use thiserror::Error;

#[derive(Error, Debug)]
pub enum PsiError {
    #[error("Proof generation failed: {0}")]
    ProofGenerationError(String),
    #[error("Proof verification failed: {0}")]
    ProofVerificationError(String),
    #[error("Invalid request")]
    InvalidRequest,
}
