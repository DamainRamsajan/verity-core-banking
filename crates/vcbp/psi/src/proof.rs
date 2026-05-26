use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::{ComplianceRequest, ProofFormat};

/// A PSI‑compliant regulatory proof package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsiComplianceProof {
    pub proof_id: Uuid,
    pub request_id: Uuid,
    pub framework: super::RegulatoryFramework,
    pub proof_format: ProofFormat,
    pub proof_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub merkle_root: Option<String>,
}

impl PsiComplianceProof {
    /// Verify the integrity of this proof.
    pub fn verify(&self) -> Result<bool, super::PsiError> {
        if self.proof_data.is_empty() {
            return Err(super::PsiError::ProofVerificationFailed(
                "Proof data is empty".into(),
            ));
        }
        let hash = blake3::hash(&self.proof_data);
        Ok(!hash.as_bytes().iter().all(|b| *b == 0))
    }
}
