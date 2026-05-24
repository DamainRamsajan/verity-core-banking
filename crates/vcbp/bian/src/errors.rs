#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Domain not found: {0}")] DomainNotFound(String),
    #[error("Domain already registered: {0}")] DomainAlreadyRegistered(String),
    #[error("Unsupported operation in domain {domain}: {operation}")] UnsupportedOperation { domain: String, operation: String },
}
