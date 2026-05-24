use serde::{Deserialize, Serialize};

/// A zero‑knowledge proof audit package.
///
/// Enables regulators to verify that a report's underlying data
/// satisfies all regulatory requirements, without exposing the
/// raw transaction details. Uses the groth16 proof system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofAuditPackage {
    pub report_id: uuid::Uuid,
    pub proof_bytes: Vec<u8>,
    pub verified_at: chrono::DateTime<chrono::Utc>,
    pub proof_system: String,
}
