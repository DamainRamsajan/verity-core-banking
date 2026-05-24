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
