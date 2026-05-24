use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vaos_core::types::AgentId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBoundary {
    pub agent_id: AgentId,
    pub spending_limit: rust_decimal::Decimal,
    pub approval_threshold: rust_decimal::Decimal,
    pub allowed_operations: Vec<String>,
}

impl Default for AgentBoundary {
    fn default() -> Self {
        Self {
            agent_id: AgentId::new(),
            spending_limit: rust_decimal::Decimal::new(1000, 0),
            approval_threshold: rust_decimal::Decimal::new(500, 0),
            allowed_operations: vec!["balance_inquiry".into()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEvent {
    pub event_id: Uuid,
    pub agent_id: AgentId,
    pub action: String,
    pub amount: Option<rust_decimal::Decimal>,
    pub risk_score: f64,
    pub within_boundary: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverrideAction { Approve, Reject, RevokeToken, SuspendAgent, TerminateAgent }
