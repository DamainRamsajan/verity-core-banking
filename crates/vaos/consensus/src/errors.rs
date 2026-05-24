//! Error types for ORCHID consensus.

#[derive(Debug, thiserror::Error)]
pub enum ConsensusError {
    #[error("Insufficient nodes: {current} (required: {required})")]
    InsufficientNodes { current: usize, required: usize },

    #[error("QSS proof invalid")]
    QssProofInvalid,

    #[error("Binding threshold not reached: r={r}, θ_b={threshold}")]
    BindingThresholdNotReached { r: f64, threshold: f64 },
}
