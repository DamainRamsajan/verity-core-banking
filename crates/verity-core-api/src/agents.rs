use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub agent_id: Uuid,
    pub name: String,
    pub agent_type: String,
    pub status: String,
    pub trust_level: String,
    pub capability_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBoundaryRequest {
    pub spending_limit: Option<rust_decimal::Decimal>,
    pub approval_threshold: Option<rust_decimal::Decimal>,
    pub allowed_operations: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentActivityResponse {
    pub event_id: Uuid,
    pub agent_id: Uuid,
    pub action: String,
    pub amount: Option<rust_decimal::Decimal>,
    pub risk_score: f64,
    pub within_boundary: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
