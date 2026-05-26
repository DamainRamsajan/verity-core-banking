use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A zero‑knowledge compliance proof under IETF PSI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsiComplianceProof {
    pub proof_id: Uuid,
    pub regulator_id: String,
    pub institution_id: String,
    pub proof_data: Vec<u8>,                // serialised Groth16 proof
    pub groth16_vk: Option<Vec<u8>>,       // verifying key
    pub pqc_signature: Option<super::engine::PqcSignature>,
    pub merkle_root: String,               // SHA‑256 of ledger state
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Proof generation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsiRequest {
    pub regulator_id: String,
    pub query: String,               // e.g., "all transactions > $10k"
    pub timeframe_days: u32,
}
