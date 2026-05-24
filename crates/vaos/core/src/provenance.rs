use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceCaps {
    pub risk_score: f64,
    pub signature: Vec<u8>,
    pub capsule_hash: [u8; 32],
    pub parent_hashes: Vec<[u8; 32]>,
    pub vap_level: VapLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VapLevel { Bronze, Silver, Gold }

impl TraceCaps {
    pub fn new(risk_delta: f64, parent_risks: &[f64], vap_level: VapLevel) -> Self {
        let parent_max = parent_risks.iter().cloned().fold(0.0, f64::max);
        Self { risk_score: parent_max + risk_delta, signature: Vec::new(), capsule_hash: [0u8; 32], parent_hashes: Vec::new(), vap_level }
    }
    pub fn should_block(&self, threshold: f64) -> bool { self.risk_score >= threshold }
    pub fn should_warn(&self, threshold: f64) -> bool { self.risk_score >= threshold && !self.should_block(threshold) }
}
