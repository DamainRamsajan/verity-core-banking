use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleProofResponse {
    pub transaction_id: Uuid,
    pub merkle_root: String,
    pub proof_hashes: Vec<String>,
    pub verified: bool,
}
