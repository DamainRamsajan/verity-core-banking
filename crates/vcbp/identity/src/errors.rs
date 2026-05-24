#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("Agent already registered")]
    AlreadyRegistered,
    #[error("Spending limit exceeded")]
    SpendingLimitExceeded,
}
