use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

/// A FIDO‑verifiable credential issued to an AI agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCredential {
    pub credential_id: Uuid,
    pub agent_id: Uuid,
    pub public_key: Vec<u8>,                // Ed25519 public key
    pub pqc_signature: Option<PqcSignature>, // post‑quantum signature placeholder
    pub tee_attestation: Option<String>,    // TEE attestation JWT (Intel TDX / AMD SEV‑SNP)
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub issuer: String,
}

/// A PQC‑hybrid signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcSignature {
    pub classical: Vec<u8>,   // Ed25519
    pub pqc: Option<Vec<u8>>, // ML‑DSA‑44 (future)
}

/// AP2 mandate: a cryptographically signed authorisation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ap2Mandate {
    pub mandate_id: Uuid,
    pub credential_id: Uuid,
    pub scope: MandateScope,
    pub signed_payload: Vec<u8>, // serialised mandate (w/o signature) + signature
    pub pqc_signature: Option<PqcSignature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandateScope {
    pub max_amount: Decimal,
    pub currency: String,
    pub counterparty_allowlist: Vec<String>,
    pub frequency_limit: Option<u32>, // per hour
    pub action_types: Vec<String>,    // e.g., "transfer", "invoice"
}
