use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vaos_core::types::AgentId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentListing {
    pub listing_id: Uuid,
    pub agent_id: AgentId,
    pub name: String,
    pub description: String,
    pub capabilities: Vec<String>,
    pub stake_amount: rust_decimal::Decimal,
    pub status: ListingStatus,
    pub reputation: ReputationScore,
    pub listed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListingStatus { Pending, Active, Challenged, Rejected, Slashed, Delisted }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ReputationScore {
    pub mean: f64,
    pub variance: f64,
}

impl ReputationScore {
    pub fn new() -> Self { Self { mean: 0.5, variance: 0.083 } }
}
