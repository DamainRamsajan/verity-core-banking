use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportResponse {
    pub report_id: Uuid,
    pub report_type: String,
    pub period_end: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofRequest {
    pub report_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofResponse {
    pub report_id: Uuid,
    pub proof_bytes: String,
    pub verified_at: chrono::DateTime<chrono::Utc>,
}
