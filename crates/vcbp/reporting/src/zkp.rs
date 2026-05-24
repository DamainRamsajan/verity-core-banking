use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofAuditPackage {
    pub report_id: uuid::Uuid,
    pub proof_bytes: Vec<u8>,
    pub verified_at: chrono::DateTime<chrono::Utc>,
    pub proof_system: String,
}
