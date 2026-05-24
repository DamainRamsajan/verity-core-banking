//! Provenance infrastructure — TraceCaps, Merkle chains, SCITT anchoring.
//!
//! Source: ARC42 v20.0 §3 Cortex ProvenanceEngine, P6 (ASL spec)
//!   TraceCaps (ICSE 2026), VAP-LAP Framework (IETF March 2026)

use serde::{Deserialize, Serialize};

/// An inline provenance capsule per the TraceCaps (ICSE 2026) pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceCaps {
    /// Monotone risk score that gates tool actions inline
    pub risk_score: f64,
    /// Ed25519 signature over the capsule content
    pub signature: Vec<u8>,
    /// BLAKE3 hash of this capsule
    pub capsule_hash: [u8; 32],
    /// Parent capsule hashes forming the Merkle chain
    pub parent_hashes: Vec<[u8; 32]>,
    /// VAP conformance level
    pub vap_level: VapLevel,
}

/// VAP (Verifiable Audit Protocol) conformance levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VapLevel {
    /// Basic audit — action recorded
    Bronze,
    /// Enhanced audit — action recorded + signed
    Silver,
    /// Full audit — action recorded + signed + externally anchored
    Gold,
}

impl TraceCaps {
    /// Create a new TraceCaps capsule.
    /// Risk accumulation is monotone: risk = max(parent_risks) + delta(step).
    pub fn new(
        risk_delta: f64,
        parent_risks: &[f64],
        vap_level: VapLevel,
    ) -> Self {
        let parent_max = parent_risks.iter().cloned().fold(0.0, f64::max);
        Self {
            risk_score: parent_max + risk_delta,
            signature: Vec::new(),
            capsule_hash: [0u8; 32],
            parent_hashes: Vec::new(),
            vap_level,
        }
    }

    /// Whether the risk score exceeds the block threshold.
    pub fn should_block(&self, threshold: f64) -> bool {
        self.risk_score >= threshold
    }

    /// Whether the risk score exceeds the warn threshold.
    pub fn should_warn(&self, threshold: f64) -> bool {
        self.risk_score >= threshold && !self.should_block(threshold)
    }
}
