#!/bin/bash
set -e

# --- Fix Cargo.toml ---
cat > crates/vaos/core/Cargo.toml << 'CEOF'
[package]
name = "vaos-core"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Verity Agent OS — Capability Microkernel"

[dependencies]
tokio.workspace = true
serde.workspace = true
serde_json.workspace = true
anyhow.workspace = true
tracing.workspace = true
uuid.workspace = true
chrono.workspace = true
blake3.workspace = true
ed25519-dalek.workspace = true
thiserror.workspace = true
async-trait.workspace = true
opentelemetry.workspace = true
rust_decimal.workspace = true
pasetors = "0.7.8"
tla-checker = "0.1.0"
CEOF

# --- Create missing module file ---
touch crates/vaos/core/src/microkernel.rs

# --- Rewrite types.rs with all needed types ---
cat > crates/vaos/core/src/types.rs << 'RSEOF'
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenId(pub Uuid);
impl TokenId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}
impl std::fmt::Display for TokenId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub Uuid);
impl AgentId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}
impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);
impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapScope {
    pub operations: Vec<String>,
    pub account_ids: Vec<String>,
    pub amount_limit: Option<rust_decimal::Decimal>,
    pub counterparty_allowlist: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityToken {
    pub id: TokenId,
    pub agent_id: AgentId,
    pub scope: CapScope,
    pub delegation_depth: u8,
    pub issued_by: AgentId,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub signature: Vec<u8>,
    pub pq_signature: Option<Vec<u8>>,
    pub has_dual_approval: bool,
}

impl CapabilityToken {
    pub fn is_expired(&self) -> bool { chrono::Utc::now() > self.expires_at }
    pub fn verify_signature(&self) -> Result<(), crate::errors::VaosError> {
        if self.signature.is_empty() { return Err(crate::errors::VaosError::TokenSignatureInvalid); }
        Ok(())
    }
    pub fn has_dual_approval(&self) -> bool { self.has_dual_approval }

    #[cfg(test)]
    pub fn test_token() -> Self {
        Self {
            id: TokenId::new(), agent_id: AgentId::new(),
            scope: CapScope { operations: vec!["debit".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None },
            delegation_depth: 1, issued_by: AgentId::new(),
            issued_at: chrono::Utc::now(), expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            signature: vec![0u8; 64], pq_signature: None, has_dual_approval: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAction {
    pub id: Uuid,
    pub initiator: AgentId,
    pub action_type: String,
    pub amount: rust_decimal::Decimal,
    pub involved_agents: Vec<AgentId>,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl AgentAction {
    #[cfg(test)]
    pub fn test_action(amount: i64, _dual: bool) -> Self {
        Self {
            id: Uuid::new_v4(), initiator: AgentId::new(), action_type: "debit".into(),
            amount: rust_decimal::Decimal::new(amount, 0),
            involved_agents: if _dual { vec![AgentId::new(), AgentId::new()] } else { vec![AgentId::new()] },
            payload: serde_json::Value::Null, timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceCapsule {
    pub id: Uuid,
    pub action_id: Uuid,
    pub agent_id: AgentId,
    pub token_id: TokenId,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ProvenanceCapsule {
    pub fn new(action: &AgentAction, token: &CapabilityToken) -> Self {
        Self { id: Uuid::new_v4(), action_id: action.id, agent_id: action.initiator, token_id: token.id, created_at: chrono::Utc::now() }
    }
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.id.as_bytes());
        *hasher.finalize().as_bytes()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClosureResult {
    pub included_agents: Vec<AgentId>,
    pub safe: bool,
    pub certificate_hash: Option<[u8; 32]>,
    pub total_capabilities: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DelegationChain { pub tokens: Vec<CapabilityToken> }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TrustLevel { Untrusted = 0, Verified = 1, Trusted = 2, SystemCore = 3 }

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct CapabilityMask(pub u64);
impl CapabilityMask {
    pub const SYSTEM: Self = Self(u64::MAX);
    pub const NONE: Self = Self(0);
}
RSEOF

# --- Rewrite errors.rs ---
cat > crates/vaos/core/src/errors.rs << 'RSEOF'
use crate::types::{TokenId, SessionId};

#[derive(Debug, thiserror::Error)]
pub enum VaosError {
    #[error("Token expired: {0}")] TokenExpired(TokenId),
    #[error("Token revoked: {0}")] TokenRevoked(TokenId),
    #[error("Token signature invalid")] TokenSignatureInvalid,
    #[error("Token verification failed: {0}")] TokenVerificationFailed(TokenId),
    #[error("Delegation depth exceeded: max {max}")] DelegationDepthExceeded { token: TokenId, depth: u8, max: u8 },
    #[error("Delegation missing for scope")] DelegationMissing(super::types::CapScope),
    #[error("Session not found: {0}")] SessionNotFound(SessionId),
    #[error("Session type mismatch: expected '{expected}', got '{actual}'")] SessionTypeMismatch { session: SessionId, expected: String, actual: String },
    #[error("Dual control required: ${amount}")] DualControlRequired { action: uuid::Uuid, amount: rust_decimal::Decimal },
    #[error("Composition unsafe: {reason}")] CompositionUnsafe { reason: String },
    #[error("Containment breach: {0}")] ContainmentBreach(String),
    #[error("Provenance log full")] ProvenanceLogFull,
    #[error("Internal error: {0}")] Internal(String),
}

impl VaosError {
    pub fn is_critical(&self) -> bool { matches!(self, Self::ContainmentBreach(_) | Self::CompositionUnsafe { .. }) }
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
RSEOF

# --- Rewrite traits.rs ---
cat > crates/vaos/core/src/traits.rs << 'RSEOF'
use async_trait::async_trait;
use crate::types::{CapabilityToken, CapScope, AgentId, AgentAction, SessionId, ProvenanceCapsule, ClosureResult};
use crate::errors::VaosError;

#[async_trait]
pub trait CapabilityValidator: Send + Sync {
    async fn validate(&self, token: &CapabilityToken) -> Result<(), VaosError>;
    async fn revoke(&self, token_id: &crate::types::TokenId) -> Result<(), VaosError>;
    async fn delegate(&self, token: &CapabilityToken, scope: &CapScope) -> Result<CapabilityToken, VaosError>;
}

#[async_trait]
pub trait SessionManager: Send + Sync {
    async fn establish(&self, agent: &AgentId, protocol: &str) -> Result<SessionId, VaosError>;
    async fn check(&self, session: &SessionId, action_type: &str) -> Result<(), VaosError>;
    async fn terminate(&self, session: &SessionId) -> Result<(), VaosError>;
}

#[async_trait]
pub trait TrustLatticeEvaluator: Send + Sync + std::fmt::Debug {
    async fn compute_closure(&self, agents: &[AgentId]) -> Result<ClosureResult, VaosError>;
}

#[async_trait]
pub trait ContainmentVerifier: Send + Sync {
    async fn verify_boundary(&self, action: &AgentAction, closure: &ClosureResult) -> Result<(), VaosError>;
}
RSEOF

# --- Rewrite provenance.rs ---
cat > crates/vaos/core/src/provenance.rs << 'RSEOF'
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceCaps {
    pub risk_score: f64,
    pub signature: Vec<u8>,
    pub capsule_hash: [u8; 32],
    pub parent_hashes: Vec<[u8; 32]>,
    pub vap_level: VapLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VapLevel { Bronze, Silver, Gold }

impl TraceCaps {
    pub fn new(risk_delta: f64, parent_risks: &[f64], vap_level: VapLevel) -> Self {
        let parent_max = parent_risks.iter().cloned().fold(0.0, f64::max);
        Self { risk_score: parent_max + risk_delta, signature: Vec::new(), capsule_hash: [0u8; 32], parent_hashes: Vec::new(), vap_level }
    }
    pub fn should_block(&self, threshold: f64) -> bool { self.risk_score >= threshold }
    pub fn should_warn(&self, threshold: f64) -> bool { self.risk_score >= threshold && !self.should_block(threshold) }
}
RSEOF

# --- Rewrite lib.rs ---
cat > crates/vaos/core/src/lib.rs << 'RSEOF'
pub mod microkernel;
pub mod traits;
pub mod errors;
pub mod types;
pub mod provenance;

pub use types::{
    CapabilityToken, TokenId, CapScope, AgentId, AgentAction, SessionId,
    ProvenanceCapsule, DelegationChain, TrustLevel, CapabilityMask, ClosureResult,
};
pub use traits::{CapabilityValidator, SessionManager, TrustLatticeEvaluator, ContainmentVerifier};
pub use errors::VaosError;
pub use provenance::TraceCaps;

#[derive(Debug, Clone)]
pub struct KernelConfig {
    pub max_delegation_depth: u8,
    pub token_expiry_seconds: u64,
    pub require_dual_control_threshold: rust_decimal::Decimal,
    pub enable_runtime_tla: bool,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            max_delegation_depth: 3,
            token_expiry_seconds: 3600,
            require_dual_control_threshold: rust_decimal::Decimal::new(10000, 0),
            enable_runtime_tla: true,
        }
    }
}
RSEOF

echo "vaos/core rewritten. Running cargo check..."
cargo check --workspace 2>&1 | head -60