use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::types::{AgentBoundary, ActivityEvent, OverrideAction};
use super::policy::DelegationPolicy;
use super::activity::ActivityFeed;
use super::session::SessionBridge;
use super::errors::DashboardError;

/// Central delegative governance dashboard engine.
pub struct DashboardEngine {
    policies: RwLock<HashMap<vaos_core::types::AgentId, AgentBoundary>>,
    feed: ActivityFeed,
    session_bridge: SessionBridge,
}

impl DashboardEngine {
    pub fn new() -> Self {
        Self {
            policies: RwLock::new(HashMap::new()),
            feed: ActivityFeed::new(),
            session_bridge: SessionBridge::new(),
        }
    }

    /// Set boundaries for an agent.
    #[tracing::instrument(name = "dashboard.set_boundaries", level = "info", skip(self))]
    pub async fn set_boundaries(
        &self,
        agent_id: vaos_core::types::AgentId,
        boundary: AgentBoundary,
    ) -> Result<(), DashboardError> {
        let mut policies = self.policies.write().await;
        policies.insert(agent_id, boundary);
        tracing::info!(?agent_id, "Agent boundaries updated");
        Ok(())
    }

    /// Check if an agent action is within its delegated boundaries.
    #[tracing::instrument(name = "dashboard.check_action", level = "debug", skip(self))]
    pub async fn check_action(
        &self,
        agent_id: vaos_core::types::AgentId,
        action: &str,
        amount: Option<rust_decimal::Decimal>,
        counterparty: Option<&str>,
    ) -> Result<bool, DashboardError> {
        let policies = self.policies.read().await;
        let boundary = policies.get(&agent_id)
            .ok_or(DashboardError::AgentNotConfigured(agent_id))?;

        // Check operation allowed
        if !boundary.allowed_operations.iter().any(|op| op == action) {
            return Ok(false);
        }

        // Check spending limit
        if let (Some(amt), limit) = (amount, boundary.spending_limit) {
            if amt > limit { return Ok(false); }
        }

        // Check counterparty
        if let Some(cpty) = counterparty {
            if !boundary.counterparty_allowlist.is_empty() && !boundary.counterparty_allowlist.contains(&cpty.to_string()) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Record an activity event and feed it to the dashboard.
    pub async fn record_activity(&self, event: ActivityEvent) {
        self.feed.push(event).await;
    }

    /// Execute a human override on an agent action.
    #[tracing::instrument(name = "dashboard.override", level = "warn", skip(self))]
    pub async fn execute_override(
        &self,
        event_id: Uuid,
        action: OverrideAction,
    ) -> Result<(), DashboardError> {
        tracing::warn!(%event_id, ?action, "Human override executed");
        // In production: revoke capability token, suspend agent, etc.
        Ok(())
    }
}
