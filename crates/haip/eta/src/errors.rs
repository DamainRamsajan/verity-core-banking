#[derive(Debug, thiserror::Error)]
pub enum EtaError { #[error("Classification failed")] ClassificationFailed }
