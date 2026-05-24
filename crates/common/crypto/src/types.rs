use serde::{Deserialize, Serialize};

/// A cryptographic key pair (Ed25519 + optional ML-DSA-44).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub algorithm: KeyAlgorithm,
    pub public_key: Vec<u8>,
    pub private_key_hash: [u8; 32],
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyAlgorithm {
    Ed25519,
    MlDsa44,
    HybridEd25519MlDsa44,
}

impl KeyPair {
    pub fn is_expired(&self) -> bool {
        self.expires_at.map(|e| chrono::Utc::now() > e).unwrap_or(false)
    }
}
