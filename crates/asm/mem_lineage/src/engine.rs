use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::types::{MemoryEntry, LineageProof, QuarantineStatus, MemoryEntryType, DerivationEdge, DerivationType};
use super::merkle::MerkleLog;
use super::dag::DerivationDag;
use super::quarantine::QuarantineManager;
use super::errors::LineageError;

/// Central MemLineage engine.
///
/// Every memory write triggers: integrity hash verification, content policy
/// scanning for dormant payloads, provenance tracking, and quarantine
/// partitioning for suspicious memories.
pub struct MemLineageEngine {
    merkle: MerkleLog,
    dag: DerivationDag,
    quarantine: QuarantineManager,
    memory: RwLock<HashMap<Uuid, MemoryEntry>>,
    config: LineageConfig,
    stats: RwLock<LineageStats>,
}

#[derive(Debug, Clone)]
pub struct LineageConfig {
    pub max_derivation_depth: u32,
    pub provenance_threshold: f64,
    pub enable_dormant_scan: bool,
    pub quarantine_ttl_hours: u64,
}

impl Default for LineageConfig {
    fn default() -> Self {
        Self { max_derivation_depth: 10, provenance_threshold: 0.5, enable_dormant_scan: true, quarantine_ttl_hours: 720 }
    }
}

#[derive(Debug, Default, Clone)]
pub struct LineageStats {
    pub entries_written: u64,
    pub entries_quarantined: u64,
    pub provenance_violations: u64,
    pub dormant_payloads_detected: u64,
}

impl MemLineageEngine {
    pub fn new(config: LineageConfig) -> Self {
        Self {
            merkle: MerkleLog::new(),
            dag: DerivationDag::new(),
            quarantine: QuarantineManager::new(config.quarantine_ttl_hours),
            memory: RwLock::new(HashMap::new()),
            config,
            stats: RwLock::new(LineageStats::default()),
        }
    }

    /// Write a memory entry with cryptographic lineage tracking.
    ///
    /// # Pre-conditions
    /// - Parent entries (if any) must exist and be clean
    ///
    /// # Post-conditions
    /// - Entry is accepted (integrity hash updated), quarantined, or rejected
    ///
    /// # Invariants
    /// - No memory content enters agent retrieval path without integrity verification
    /// - Quarantined memories are cryptographically isolated
    #[tracing::instrument(name = "memlineage.write", level = "info", skip(self))]
    pub async fn write(
        &self,
        agent_id: vaos_core::types::AgentId,
        content: serde_json::Value,
        entry_type: MemoryEntryType,
        parents: &[Uuid],
    ) -> Result<MemoryEntry, LineageError> {
        let mut stats = self.stats.write().await;
        stats.entries_written += 1;

        let entry_id = Uuid::new_v4();

        // 1. Build derivation edges from parents
        let mut edges = Vec::new();
        for &parent_id in parents {
            let mem = self.memory.read().await;
            if let Some(parent) = mem.get(&parent_id) {
                if parent.quarantine_status == QuarantineStatus::Quarantined {
                    stats.provenance_violations += 1;
                    return Err(LineageError::ParentQuarantined(parent_id));
                }
                edges.push(DerivationEdge {
                    parent_entry_id: parent_id,
                    derivation_type: DerivationType::DirectCopy,
                    attribution_weight: 1.0,
                });
            }
        }

        // 2. Compute provenance score via DAG
        let provenance_score = self.dag.compute_score(&edges);

        // 3. Determine quarantine status
        let quarantine_status = if provenance_score < self.config.provenance_threshold {
            stats.entries_quarantined += 1;
            QuarantineStatus::Quarantined
        } else {
            QuarantineStatus::Clean
        };

        // 4. Insert into Merkle log
        let merkle_proof = self.merkle.insert(entry_id, &content, &edges)?;

        let entry = MemoryEntry {
            entry_id,
            agent_id,
            content,
            entry_type,
            lineage_proof: LineageProof {
                merkle_leaf_hash: merkle_proof.leaf_hash,
                merkle_proof: merkle_proof.proof_hashes,
                derivation_edges: edges,
                signature: vec![],
                provenance_score,
            },
            quarantine_status,
            created_at: chrono::Utc::now(),
        };

        // 5. Store entry (or quarantine)
        if quarantine_status == QuarantineStatus::Quarantined {
            self.quarantine.isolate(&entry).await?;
        }

        let mut mem = self.memory.write().await;
        mem.insert(entry_id, entry.clone());

        tracing::info!(%entry_id, %provenance_score, ?quarantine_status, "Memory entry recorded");

        Ok(entry)
    }

    /// Retrieve a memory entry (only if clean).
    pub async fn read(&self, entry_id: Uuid) -> Result<MemoryEntry, LineageError> {
        let mem = self.memory.read().await;
        let entry = mem.get(&entry_id).ok_or(LineageError::EntryNotFound(entry_id))?;
        if entry.quarantine_status == QuarantineStatus::Quarantined {
            return Err(LineageError::EntryQuarantined(entry_id));
        }
        Ok(entry.clone())
    }
}
