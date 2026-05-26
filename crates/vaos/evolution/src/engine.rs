use tokio::sync::RwLock;

use super::contract::SafetyContract;
use super::fggm::FormallyGuardedGenerativeModel;
use super::types::{EvolutionProposal, EvolutionCertificate};
use super::errors::EvolutionError;

/// Central evolution engine implementing the three‑stage SEVerA pipeline.
pub struct EvolutionEngine {
    fggm: FormallyGuardedGenerativeModel,
    accepted: RwLock<Vec<EvolutionProposal>>,
    rejected: RwLock<Vec<EvolutionProposal>>,
    config: EvolutionConfig,
    stats: RwLock<EvolutionStats>,
}

#[derive(Debug, Clone)]
pub struct EvolutionConfig {
    pub max_proposals_per_day: u32,
    pub require_human_approval: bool,
    pub auto_deploy: bool,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            max_proposals_per_day: 5,
            require_human_approval: true,
            auto_deploy: false,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct EvolutionStats {
    pub proposals_submitted: u64,
    pub proposals_accepted: u64,
    pub proposals_rejected: u64,
    pub constraint_violations: u64,
}

impl EvolutionEngine {
    pub fn new(config: EvolutionConfig) -> Self {
        Self {
            fggm: FormallyGuardedGenerativeModel::new(SafetyContract::all_invariants()),
            accepted: RwLock::new(Vec::new()),
            rejected: RwLock::new(Vec::new()),
            config,
            stats: RwLock::new(EvolutionStats::default()),
        }
    }

    /// Submit an evolution proposal and run it through the SEVerA pipeline.
    ///
    /// # Stage 1 – Search (caller responsibility)
    /// The agent or planner LLM synthesises the proposal.
    ///
    /// # Stage 2 – Verification (this method)
    /// The FGGM verifies the proposal against all P1‑P8 safety invariants.
    /// If any hard constraint is violated, the proposal is rejected with
    /// a counter‑example.
    ///
    /// # Stage 3 – Learning (handled by the agent runtime)
    /// Accepted proposals are deployed; the agent's performance metrics
    /// are tracked and used for future optimisation.
    #[tracing::instrument(name = "evolution.submit", level = "info", skip(self))]
    pub async fn submit(
        &self,
        proposal: EvolutionProposal,
    ) -> Result<EvolutionCertificate, EvolutionError> {
        let mut stats = self.stats.write().await;
        stats.proposals_submitted += 1;

        // Enforce daily limit
        if stats.proposals_submitted > self.config.max_proposals_per_day as u64 {
            return Err(EvolutionError::DailyLimitExceeded {
                max: self.config.max_proposals_per_day,
            });
        }

        // Stage 2 – FGGM Verification
        let certificate = self.fggm.verify(&proposal)?;

        if certificate.verified {
            stats.proposals_accepted += 1;
            self.accepted.write().await.push(proposal.clone());
            tracing::info!(
                proposal_id = %proposal.proposal_id,
                "Evolution accepted – all safety invariants satisfied"
            );
        } else {
            stats.proposals_rejected += 1;
            stats.constraint_violations += 1;
            self.rejected.write().await.push(proposal.clone());
            tracing::warn!(
                proposal_id = %proposal.proposal_id,
                counterexample = ?certificate.counterexample,
                "Evolution rejected – safety invariant violation"
            );
        }

        Ok(certificate)
    }

    /// List all accepted evolutions for audit.
    pub async fn accepted_evolutions(&self) -> Vec<EvolutionProposal> {
        self.accepted.read().await.clone()
    }

    /// List all rejected evolutions for audit.
    pub async fn rejected_evolutions(&self) -> Vec<EvolutionProposal> {
        self.rejected.read().await.clone()
    }

    pub async fn stats(&self) -> EvolutionStats {
        self.stats.read().await.clone()
    }
}
