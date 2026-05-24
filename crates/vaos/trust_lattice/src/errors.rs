//! Error types for the Trust Lattice Engine.

use vaos_core::types::AgentId;

#[derive(Debug, thiserror::Error)]
pub enum LatticeError {
    #[error("Composition too large: {size} agents (max {max})")]
    CompositionTooLarge { size: usize, max: usize },

    #[error("Agent not registered: {0:?}")]
    AgentNotRegistered(AgentId),

    #[error("Composition unsafe: {agents:?} reaches forbidden states: {forbidden_states:?}")]
    CompositionUnsafe {
        agents: Vec<AgentId>,
        forbidden_states: Vec<super::hypergraph::ForbiddenState>,
    },

    #[error("Certificate verification failed")]
    CertificateVerificationFailed,

    #[error("Closure computation exceeded maximum iterations")]
    ClosureTimeout,

    #[error("Internal error: {0}")]
    Internal(String),
}
