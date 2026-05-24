use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vaos_core::types::AgentId;

/// An agent listing in the marketplace.
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
    pub challenges: Vec<Challenge>,
}

/// Current status of a listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ListingStatus {
    Pending,
    Active,
    Challenged,
    Rejected,
    Slashed,
    Delisted,
}

/// A challenge against an agent listing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub challenge_id: Uuid,
    pub challenger: AgentId,
    pub reason: String,
    pub evidence: Option<String>,
    pub resolved: bool,
    pub outcome: Option<ChallengeOutcome>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChallengeOutcome {
    Upheld,
    Rejected,
}

/// Cryptographic reputation score (Bayesian).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ReputationScore {
    pub alpha: f64,   // successes
    pub beta: f64,    // failures
    pub mean: f64,
    pub variance: f64,
}

impl ReputationScore {
    pub fn new() -> Self {
        Self { alpha: 1.0, beta: 1.0, mean: 0.5, variance: 0.083 }
    }

    /// Update via Bayesian conjugate prior (Beta‑Binomial).
    pub fn update(&mut self, success: bool) {
        if success { self.alpha += 1.0; } else { self.beta += 1.0; }
        self.mean = self.alpha / (self.alpha + self.beta);
        self.variance = (self.alpha * self.beta) / ((self.alpha + self.beta).powi(2) * (self.alpha + self.beta + 1.0));
    }
}
