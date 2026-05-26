use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::{PaymentIntent, ProofOfCompliance};

/// A complete ZK payment proof package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkPaymentProof {
    pub proof_id: Uuid,
    pub intent_id: Uuid,
    pub compliance: ProofOfCompliance,
    pub lightning_preimage: Option<Vec<u8>>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

impl ZkPaymentProof {
    /// Verify that all compliance checks passed.
    pub fn all_compliant(&self) -> bool {
        self.compliance.sanctions_ok
            && self.compliance.kya_ok
            && self.compliance.amount_in_range
    }
}
