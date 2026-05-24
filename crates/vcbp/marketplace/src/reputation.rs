use std::collections::HashMap;
use std::sync::RwLock;
use vaos_core::types::AgentId;
use super::types::ReputationScore;

/// Cryptographic reputation engine (Bayesian conjugate prior).
///
/// Follows AgentProof ERC‑8004 pattern: aggregates real activity across
/// ecosystems and converts it into transparent, verifiable trust scores.
pub struct ReputationEngine {
    scores: RwLock<HashMap<AgentId, ReputationScore>>,
}

impl ReputationEngine {
    pub fn new() -> Self { Self { scores: RwLock::new(HashMap::new()) } }

    /// Get or create a reputation score for an agent.
    pub fn get_score(&self, agent_id: AgentId) -> ReputationScore {
        self.scores.read().unwrap().get(&agent_id).copied().unwrap_or(ReputationScore::new())
    }

    /// Record a successful task completion.
    pub fn record_success(&self, agent_id: AgentId) {
        let mut scores = self.scores.write().unwrap();
        let score = scores.entry(agent_id).or_insert(ReputationScore::new());
        score.update(true);
        tracing::debug!(?agent_id, mean = score.mean, "Reputation: success recorded");
    }

    /// Record a failed or malicious task.
    pub fn record_failure(&self, agent_id: AgentId) {
        let mut scores = self.scores.write().unwrap();
        let score = scores.entry(agent_id).or_insert(ReputationScore::new());
        score.update(false);
        tracing::warn!(?agent_id, mean = score.mean, "Reputation: failure recorded");
    }
}
