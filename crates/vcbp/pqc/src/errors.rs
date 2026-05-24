#[derive(Debug, thiserror::Error)]
pub enum PqcError {
    #[error("PQC signature generation failed")]
    SignatureGenerationFailed,
}
