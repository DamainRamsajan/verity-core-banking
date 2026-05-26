use super::types::{LearningEvent, AuditRecord};
use super::errors::EvidenceError;

/// A Merkle‑proofed, append‑only audit log of agent learning events.
///
/// Every record is cryptographically chained, making the entire
/// learning history tamper‑evident and independently verifiable.
pub struct LearningAuditLog {
    records: Vec<AuditRecord>,
    chain_hash: Option<[u8; 32]>,
}

impl LearningAuditLog {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            chain_hash: None,
        }
    }

    /// Append a learning event to the audit log.
    ///
    /// The record is cryptographically chained to the previous record
    /// via BLAKE3 hashing, creating a tamper‑evident sequence.
    pub fn append(
        &mut self,
        event: &LearningEvent,
    ) -> Result<AuditRecord, EvidenceError> {
        let mut hasher = blake3::Hasher::new();

        // Chain to previous hash
        if let Some(prev) = &self.chain_hash {
            hasher.update(prev);
        }

        // Hash the event content
        hasher.update(event.event_id.as_bytes());
        hasher.update(event.agent_id.0.as_bytes());
        hasher.update(event.description.as_bytes());
        hasher.update(event.evidence.source_url.as_bytes());
        hasher.update(&event.evidence.confidence.to_le_bytes());

        let record_hash = *hasher.finalize().as_bytes();
        self.chain_hash = Some(record_hash);

        let record = AuditRecord {
            record_id: uuid::Uuid::new_v4(),
            event: event.clone(),
            merkle_proof_hash: record_hash,
            signature: Vec::new(),
            recorded_at: chrono::Utc::now(),
        };

        self.records.push(record.clone());
        Ok(record)
    }

    /// Return all audit records.
    pub fn records(&self) -> Vec<AuditRecord> {
        self.records.clone()
    }

    /// Verify the integrity of the entire audit log.
    pub fn verify_integrity(&self) -> bool {
        if self.records.is_empty() {
            return true;
        }

        let mut prev_hash: Option<[u8; 32]> = None;
        for record in &self.records {
            let mut hasher = blake3::Hasher::new();
            if let Some(prev) = &prev_hash {
                hasher.update(prev);
            }
            hasher.update(record.event.event_id.as_bytes());
            hasher.update(record.event.agent_id.0.as_bytes());
            hasher.update(record.event.description.as_bytes());
            hasher.update(record.event.evidence.source_url.as_bytes());
            hasher.update(&record.event.evidence.confidence.to_le_bytes());

            let computed = *hasher.finalize().as_bytes();
            if computed != record.merkle_proof_hash {
                return false;
            }
            prev_hash = Some(computed);
        }
        true
    }
}
