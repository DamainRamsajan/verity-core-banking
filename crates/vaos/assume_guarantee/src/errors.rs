//! Error types for the Assume-Guarantee Contract Monitor.

#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error("Contract breach in layer '{layer}': invariant '{invariant}' violated")]
    InvariantViolation { layer: String, invariant: String },

    #[error("Guarantee not satisfied in layer '{layer}': {guarantee}")]
    GuaranteeUnsatisfied { layer: String, guarantee: String },

    #[error("Cross-layer inconsistency: {0}")]
    CrossLayerInconsistency(String),

    #[error("TLA+ model check failed: {0}")]
    TlaModelCheckFailed(String),
}
