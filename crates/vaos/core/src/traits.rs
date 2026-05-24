use async_trait::async_trait;
use crate::errors::VaosError;
use crate::types::{CapabilityToken, CapScope, AgentId, AgentAction, SessionId, ClosureResult};

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
