use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::CredentialStatus;

/// A FIDO‑verifiable agent credential.
///
/// Issued by the bank's FIDO infrastructure and cryptographically
/// bound to the agent's zkVM binary‑hash identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCredential {
    pub credential_id: Uuid,
    pub agent_id: vaos_core::types::AgentId,
    pub principal_id: String,
    pub fido_attestation: Vec<u8>,
    pub public_key: Vec<u8>,
    pub status: CredentialStatus,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl AgentCredential {
    /// Verify that the credential is valid and not expired.
    pub fn is_valid(&self, now: chrono::DateTime<chrono::Utc>) -> bool {
        self.status == CredentialStatus::Active && now < self.expires_at
    }
}
