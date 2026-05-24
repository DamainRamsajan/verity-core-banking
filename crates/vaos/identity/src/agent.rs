//! Agent identity record.

use serde::{Deserialize, Serialize};

use super::ZkpIdentityProof;

/// An agent's cryptographically verifiable identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub agent_id: vaos_core::types::AgentId,
    /// Content hash of the compiled agent binary (P4)
    pub binary_hash: [u8; 32],
    /// zkVM proof attesting to the binary hash
    pub zk_proof: ZkpIdentityProof,
    /// W3C Decentralized Identifier
    pub did: String,
    /// On-chain identity address (VeriChain ERC-8004)
    pub verichain_address: String,
    /// KYA credential ID (if issued)
    pub kya_credential_id: Option<uuid::Uuid>,
    /// eIDAS 2.0 wallet identifier (if linked)
    pub eidas_wallet_id: Option<String>,
    /// Capability-governed smart account (1A1A)
    pub smart_account: super::smart_account::SmartAccount,
    /// When the identity was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the identity was revoked (if ever)
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl AgentIdentity {
    /// Whether this identity is currently active.
    pub fn is_active(&self) -> bool {
        self.revoked_at.is_none()
    }

    /// Revoke this identity.
    pub fn revoke(&mut self) {
        self.revoked_at = Some(chrono::Utc::now());
    }
}
