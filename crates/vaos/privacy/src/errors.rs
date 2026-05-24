//! Error types for privacy services.

#[derive(Debug, thiserror::Error)]
pub enum PrivacyError {
    #[error("Privacy budget exhausted: {remaining:.6} ε remaining, {requested:.6} ε requested")]
    DpBudgetExhausted { remaining: f64, requested: f64 },

    #[error("MPC party count exceeded: {requested} requested (max {max})")]
    MpcPartyCountExceeded { requested: usize, max: usize },

    #[error("SMPC abort: participant failed")]
    SmpcAbort,

    #[error("FHE ciphertext integrity violation")]
    FheIntegrityViolation,

    #[error("Service not initialized: {0}")]
    ServiceNotInitialized(String),
}
