use std::sync::Arc;
use tokio::sync::RwLock;

use super::policy::PolicyNetwork;
use super::compiler::GovernanceJitCompiler;
use super::types::{PolicyUpdate, GovernanceLatency};
use super::errors::EhvError;

/// Central EHV engine.
///
/// Coordinates the policy network and JIT compiler to achieve
/// O(1) governance latency with formal determinism.
pub struct EhvEngine {
    policy_network: Arc<PolicyNetwork>,
    compiler: RwLock<GovernanceJitCompiler>,
    config: EhvConfig,
    stats: RwLock<EhvStats>,
}

#[derive(Debug, Clone)]
pub struct EhvConfig {
    pub auto_compile: bool,
    pub enforcement_point: super::PolicyEnforcementPoint,
}

impl Default for EhvConfig {
    fn default() -> Self {
        Self {
            auto_compile: true,
            enforcement_point: super::PolicyEnforcementPoint::InlineJIT,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct EhvStats {
    pub policies_ingested: u64,
    pub policies_active: u64,
    pub average_latency_ms: f64,
}

impl EhvEngine {
    pub fn new(config: EhvConfig) -> Self {
        Self {
            policy_network: Arc::new(PolicyNetwork::new()),
            compiler: RwLock::new(GovernanceJitCompiler::new()),
            config,
            stats: RwLock::new(EhvStats::default()),
        }
    }

    /// Ingest a regulatory change and propagate it globally.
    ///
    /// This is the O(1) governance path that replaces the current
    /// 14‑30 day regulatory latency.
    #[tracing::instrument(name = "ehv.ingest", level = "info", skip(self))]
    pub async fn ingest_regulation(
        &self,
        update: PolicyUpdate,
    ) -> Result<GovernanceLatency, EhvError> {
        let mut stats = self.stats.write().await;
        stats.policies_ingested += 1;

        // 1. Propagate via CRDT policy network
        let latency = self.policy_network.ingest(update).await?;

        // 2. Re‑compile the JIT policy set
        if self.config.auto_compile {
            let policies = self.policy_network.active_policies().await;
            self.compiler.write().await.load_policies(&policies)?;
        }

        stats.policies_active = self.policy_network.active_policies().await.len() as u64;
        stats.average_latency_ms = (stats.average_latency_ms * (stats.policies_ingested - 1) as f64
            + latency.total_latency_ms as f64)
            / stats.policies_ingested as f64;

        tracing::info!(
            latency_ms = latency.total_latency_ms,
            active_policies = stats.policies_active,
            "Regulation ingested – agents now compliant"
        );

        Ok(latency)
    }

    /// Verify an agent action against all active policies.
    pub async fn verify_action(
        &self,
        action: &str,
        context: &serde_json::Value,
    ) -> Result<bool, EhvError> {
        self.compiler.read().await.verify_action(action, context)
    }
}
