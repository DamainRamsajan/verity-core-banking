#[derive(Debug, thiserror::Error)]
pub enum BankingOpsError {
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Dual control required: {operation} needs {required} tokens (provided: {provided})")]
    DualControlRequired { operation: String, required: usize, provided: usize },

    #[error("Dual control principals violation: need {required} distinct principals (got {distinct_principals})")]
    DualControlPrincipalsViolation { required: usize, distinct_principals: usize },

    #[error("Token scope insufficient: needed {required:?}, got {actual:?}")]
    TokenScopeInsufficient { required: String, actual: String },

    #[error("Token expired")]
    TokenExpired,

    #[error("Token revoked")]
    TokenRevoked,
}
