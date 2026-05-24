//! Error types for the Non-Human Identity Manager.

#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("Agent already registered: {0:?}")]
    AgentAlreadyRegistered([u8; 32]),

    #[error("ZKP verification failed: {0}")]
    ZkpVerificationFailed(String),

    #[error("KYA credential expired")]
    KyaCredentialExpired,

    #[error("eIDAS wallet verification failed: {0}")]
    EidasVerificationFailed(String),

    #[error("Smart account spending limit exceeded")]
    SpendingLimitExceeded,

    #[error("Agent identity revoked")]
    IdentityRevoked,
}
