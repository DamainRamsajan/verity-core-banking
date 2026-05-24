//! Error types for the VAOS capability microkernel.
//!
//! Source: ARC42 v20.0 §3 VAOS (all component contracts)

use crate::types::{TokenId, AgentId, SessionId};

#[derive(Debug, thiserror::Error)]
pub enum VaosError {
    #[error("Token expired: {0:?}")]
    TokenExpired(TokenId),

    #[error("Token revoked: {0:?}")]
    TokenRevoked(TokenId),

    #[error("Token signature invalid")]
    TokenSignatureInvalid,

    #[error("Token verification failed: {0:?}")]
    TokenVerificationFailed(TokenId),

    #[error("Delegation depth exceeded: token {token:?} at depth {depth} (max {max})")]
    DelegationDepthExceeded { token: TokenId, depth: u8, max: u8 },

    #[error("Delegation missing for scope: {0:?}")]
    DelegationMissing(crate::types::CapScope),

    #[error("Session not found: {0:?}")]
    SessionNotFound(SessionId),

    #[error("Session type mismatch: expected '{expected}', got '{actual}'")]
    SessionTypeMismatch {
        session: SessionId,
        expected: String,
        actual: String,
    },

    #[error("Dual control required: action {action:?} for ${amount}")]
    DualControlRequired { action: uuid::Uuid, amount: rust_decimal::Decimal },

    #[error("Composition unsafe: {reason}")]
    CompositionUnsafe { reason: String },

    #[error("Containment breach: {0}")]
    ContainmentBreach(String),

    #[error("Provenance log full")]
    ProvenanceLogFull,

    #[error("Internal error: {0}")]
    Internal(String),
}

impl VaosError {
    /// Whether this error should trigger the Kill Switch Protocol.
    pub fn is_critical(&self) -> bool {
        matches!(self, Self::ContainmentBreach(_) | Self::CompositionUnsafe { .. })
    }

    /// OWASP Agentic Top 10 category for this error.
    pub fn owasp_category(&self) -> &'static str {
        match self {
            Self::TokenExpired(_) | Self::TokenRevoked(_) => "ASI03",
            Self::TokenSignatureInvalid | Self::TokenVerificationFailed(_) => "ASI03",
            Self::DelegationDepthExceeded { .. } | Self::DelegationMissing(_) => "ASI03",
            Self::SessionNotFound(_) | Self::SessionTypeMismatch { .. } => "ASI07",
            Self::DualControlRequired { .. } => "ASI10",
            Self::CompositionUnsafe { .. } => "ASI08",
            Self::ContainmentBreach(_) => "ASI05",
            _ => "ASI01",
        }
    }
}
