//! KYA credential and Verifiable Credential types.

use serde::{Deserialize, Serialize};

/// A Know Your Agent (KYA) credential.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KyaCredential {
    pub id: uuid::Uuid,
    pub agent_id: vaos_core::types::AgentId,
    pub level: super::KyaLevel,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub signature: Vec<u8>,
}

/// A W3C Verifiable Credential.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableCredential {
    pub context: Vec<String>,
    pub id: String,
    pub credential_type: Vec<String>,
    pub issuer: String,
    pub issuance_date: chrono::DateTime<chrono::Utc>,
    pub credential_subject: serde_json::Value,
    pub proof: Option<VcProof>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VcProof {
    pub proof_type: String,
    pub created: chrono::DateTime<chrono::Utc>,
    pub proof_value: String,
}
