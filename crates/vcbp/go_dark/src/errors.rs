#[derive(Debug, thiserror::Error)]
pub enum GoDarkError {
    #[error("Proof verification failed")]
    ProofVerificationFailed,
}
