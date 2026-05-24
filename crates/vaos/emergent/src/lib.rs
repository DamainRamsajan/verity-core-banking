//! # Verity Agent OS — Emergent Protocol Learner
//!
//! Enables agents to negotiate task-specific communication protocols while
//! respecting the session-type safety envelope. Based on the **MARL-CPC**
//! framework: collective predictive coding enables decentralized multi-agent
//! communication without parameter sharing, supporting non-cooperative and
//! reward-independent settings.
//!
//! ## Key Insight
//! Traditional MARL treats messages as part of the action space under
//! cooperation assumptions. MARL-CPC links messages to state inference,
//! enabling communication even when agents are independent and have
//! different reward functions. This is essential for cross-institutional
//! banking agents that cannot share model parameters.
//!
//! Source: ARC42 v20.0 §3 VAOS Emergent Protocol Learner

pub mod learner;
pub mod validator;
pub mod errors;

use std::sync::Arc;
use tokio::sync::RwLock;

pub use learner::EmergentLearner;
pub use validator::SafetyEnvelopeValidator;
pub use errors::EmergentError;

/// A learned communication protocol.
#[derive(Debug, Clone)]
pub struct LearnedProtocol {
    pub id: uuid::Uuid,
    pub agents: Vec<vaos_core::types::AgentId>,
    pub protocol_spec: String,
    pub verified_safe: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
