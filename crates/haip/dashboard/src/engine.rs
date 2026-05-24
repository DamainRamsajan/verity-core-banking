use std::collections::HashMap;
use tokio::sync::RwLock;
use vaos_core::types::AgentId;
use super::types::{AgentBoundary, ActivityEvent};
use super::errors::DashboardError;

pub struct DashboardEngine {
    policies: RwLock<HashMap<AgentId, AgentBoundary>>,
    activity_feed: RwLock<Vec<ActivityEvent>>,
}

impl DashboardEngine {
    pub fn new() -> Self { Self { policies: RwLock::new(HashMap::new()), activity_feed: RwLock::new(Vec::new()) } }

    pub async fn set_boundaries(&self, agent_id: AgentId, boundary: AgentBoundary) -> Result<(), DashboardError> {
        self.policies.write().await.insert(agent_id, boundary);
        Ok(())
    }

    pub async fn check_action(
        &self,
        agent_id: AgentId,
        action: &str,
        amount: Option<rust_decimal::Decimal>,
    ) -> Result<bool, DashboardError> {
        let policies = self.policies.read().await;
        let boundary = policies.get(&agent_id).ok_or(DashboardError::AgentNotConfigured(agent_id))?;
        if !boundary.allowed_operations.iter().any(|op| op == action) { return Ok(false); }
        if let (Some(amt), limit) = (amount, boundary.spending_limit) {
            if amt > limit { return Ok(false); }
        }
        Ok(true)
    }

    pub async fn record_activity(&self, event: ActivityEvent) {
        self.activity_feed.write().await.push(event);
    }
}
