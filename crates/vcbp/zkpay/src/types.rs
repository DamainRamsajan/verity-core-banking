use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A payment intent with ZK‑compliance proofs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentIntent {
    pub intent_id: Uuid,
    pub payer_agent_id: Uuid,
    pub payee_agent_id: Uuid,
    pub amount_sats: u64,
    pub currency: String,
    pub stealth_address: Option<String>,   // for unlinkability
    pub compliance_proof: ZkPaymentProof,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// ZK proof of compliance (sanctions, KYA, amount range).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkPaymentProof {
    pub proof_data: Vec<u8>,               // serialised Groth16/PLONK proof
    pub public_inputs: Vec<String>,
    pub pqc_signature: Option<super::engine::PqcSignature>,
}
