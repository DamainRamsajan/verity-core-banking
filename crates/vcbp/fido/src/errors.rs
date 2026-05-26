use thiserror::Error;

#[derive(Error, Debug)]
pub enum FidoError {
    #[error("Credential expired")]
    CredentialExpired,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Mandate scope exceeded")]
    ScopeExceeded,
    #[error("Credential not found: {0}")]
    CredentialNotFound(uuid::Uuid),
    #[error("Duplicate credential")]
    DuplicateCredential,
}
