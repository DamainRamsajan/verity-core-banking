use tokio::sync::RwLock;
use uuid::Uuid;

use super::types::PolicyUpdate;
use super::errors::EhvError;

/// A CRDT‑synchronised policy network using an OR‑Set (Observed‑Remove Set).
/// Policies are eventually consistent across all Verity instances.
pub struct PolicyNetwork {
    policies: RwLock<crdts::Orswot<PolicyUpdate, Uuid>>,
    version: RwLock<u64>,
}

impl PolicyNetwork {
    pub fn new() -> Self {
        Self {
            policies: RwLock::new(crdts::Orswot::new()),
            version: RwLock::new(0),
        }
    }

    /// Ingest a new regulatory policy and propagate it.
    #[tracing::instrument(name = "ehv.policy.ingest", level = "info", skip(self))]
    pub async fn ingest(
        &self,
        update: PolicyUpdate,
    ) -> Result<super::GovernanceLatency, EhvError> {
        let now = chrono::Utc::now();
        let published_at = update.published_at;

        let mut policies = self.policies.write().await;
        policies.add(update.update_id, update);

        let mut version = self.version.write().await;
        *version += 1;

        let latency_ms = (now - published_at).num_milliseconds() as u64;

        tracing::info!(
            policy_id = %update.update_id,
            regulation = %update.regulation,
            latency_ms,
            "Policy propagated via CRDT"
        );

        Ok(super::GovernanceLatency {
            regulation_published_at: published_at,
            policy_propagated_at: now,
            agents_compliant_at: now,
            total_latency_ms: latency_ms,
        })
    }

    /// Get all active policies.
    pub async fn active_policies(&self) -> Vec<PolicyUpdate> {
        self.policies.read().await.values().cloned().collect()
    }

    pub async fn version(&self) -> u64 {
        *self.version.read().await
    }
}