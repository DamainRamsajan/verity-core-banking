//! Core traits for the Verity Agent OS microkernel.
//!
//! Source: ARC42 v20.0 §3 VAOS (all component contracts)

use async_trait::async_trait;

use crate::types::{
    CapabilityToken, CapScope, AgentId, AgentAction, SessionId,
    ProvenanceCapsule, ClosureResult,
};
use crate::errors::VaosError;

/// Validates and manages capability tokens.
///
/// # Contract
/// - Pre: Token must be PASETO v4 signed with valid delegation chain
/// - Post: Either a ValidationResult is returned or VaosError
/// - Inv: Tokens are unforgeable; privilege escalation is impossible
#[async_trait]
pub trait CapabilityValidator: Send + Sync {
    async fn validate(&self, token: &CapabilityToken) -> Result<(), VaosError>;
    async fn revoke(&self, token_id: &crate::types::TokenId) -> Result<(), VaosError>;
    async fn delegate(
        &self,
        token: &CapabilityToken,
        scope: &CapScope,
    ) -> Result<CapabilityToken, VaosError>;
}

/// Manages communication sessions between agents.
#[async_trait]
pub trait SessionManager: Send + Sync {
    async fn establish(
        &self,
        agent: &AgentId,
        protocol: &str,
    ) -> Result<SessionId, VaosError>;
    async fn check(&self, session: &SessionId, action_type: &str) -> Result<(), VaosError>;
    async fn terminate(&self, session: &SessionId) -> Result<(), VaosError>;
}

/// Evaluates the trust lattice for compositional safety.
///
/// Implements Spera Theorem 9.2 (March 2026): safety is non-compositional
/// in the presence of conjunctive capability dependencies.
#[async_trait]
pub trait TrustLatticeEvaluator: Send + Sync {
    /// Compute conjunctive capability hypergraph closure.
    /// O(n + m·k) worklist algorithm (Datalog equivalence).
    async fn compute_closure(&self, agents: &[AgentId]) -> Result<ClosureResult, VaosError>;
}

/// Verifies containment under havoc oracle semantics.
///
/// Source: Moon et al. (May 2026) — first deductive formal verification
/// of an agentic framework, treating the AI as an unconstrained oracle.
#[async_trait]
pub trait ContainmentVerifier: Send + Sync {
    /// Verify that an agent action respects the boundary policy.
    /// The AI model is treated as a "havoc oracle" — any output is possible.
    async fn verify_boundary(
        &self,
        action: &AgentAction,
        closure: &ClosureResult,
    ) -> Result<(), VaosError>;
}
