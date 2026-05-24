//! Proof cache with TTL-based expiry.

use super::ComplianceProof;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ProofCache {
    entries: HashMap<Uuid, CacheEntry>,
    ttl_secs: u64,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    proof: ComplianceProof,
    inserted_at: chrono::DateTime<chrono::Utc>,
}

impl ProofCache {
    pub fn new(ttl_secs: u64) -> Self {
        Self { entries: HashMap::new(), ttl_secs }
    }

    pub fn get(&self, action_id: Uuid) -> Option<ComplianceProof> {
        self.entries.get(&action_id).and_then(|e| {
            let age = (chrono::Utc::now() - e.inserted_at).num_seconds() as u64;
            if age < self.ttl_secs { Some(e.proof.clone()) } else { None }
        })
    }

    pub fn insert(&mut self, action_id: Uuid, proof: ComplianceProof) {
        self.entries.insert(action_id, CacheEntry {
            proof,
            inserted_at: chrono::Utc::now(),
        });
    }

    pub fn flush_expired(&mut self) {
        let ttl = self.ttl_secs;
        self.entries.retain(|_, e| {
            (chrono::Utc::now() - e.inserted_at).num_seconds() as u64 < ttl
        });
    }
}
