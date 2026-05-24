#[derive(Debug, thiserror::Error)]
pub enum EtaError {
    #[error("Classification failed: {0}")]
    ClassificationFailed(String),
}
