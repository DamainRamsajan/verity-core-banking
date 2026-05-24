//! Spera Certificate — cryptographic proof of compositional safety.
//!
//! Source: ARC42 v20.0 ADR-019

use blake3::Hasher;
use ed25519_dalek::{SigningKey, Signature, Signer, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use vaos_core::types::AgentId;

use super::hypergraph::ClosureResult;
use super::errors::LatticeError;

/// A Spera Certificate is a cryptographically signed attestation that
/// a full conjunctive capability hypergraph closure was computed for a
/// specific agent composition and no forbidden states were reachable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SperaCertificate {
    /// Agent composition covered by this certificate
    pub agent_ids: Vec<AgentId>,
    /// Total number of capabilities in the closure
    pub closure_size: usize,
    /// Number of hyperedges evaluated
    pub hyperedges_evaluated: usize,
    /// Whether the composition was found safe
    pub safe: bool,
    /// Any forbidden states that were reached (empty = safe)
    pub forbidden_states_reached: Vec<String>,
    /// Ed25519 signature over the certificate content
    pub signature: Vec<u8>,
    /// Timestamp of certification
    pub certified_at: chrono::DateTime<chrono::Utc>,
    /// BLAKE3 hash of the certificate
    content_hash: [u8; 32],
}

impl SperaCertificate {
    pub fn new(
        agent_ids: &[AgentId],
        closure: &ClosureResult,
        forbidden: &[super::hypergraph::ForbiddenState],
    ) -> Self {
        let mut cert = Self {
            agent_ids: agent_ids.to_vec(),
            closure_size: closure.total_capabilities,
            hyperedges_evaluated: 0,
            safe: forbidden.is_empty(),
            forbidden_states_reached: forbidden.iter().map(|f| f.reason.clone()).collect(),
            signature: Vec::new(),
            certified_at: chrono::Utc::now(),
            content_hash: [0u8; 32],
        };
        cert.content_hash = cert.compute_hash();
        cert
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        for aid in &self.agent_ids {
            hasher.update(aid.0.as_bytes());
        }
        hasher.update(&self.closure_size.to_le_bytes());
        hasher.update(&[self.safe as u8]);
        hasher.update(self.certified_at.to_string().as_bytes());
        *hasher.finalize().as_bytes()
    }

    pub fn hash(&self) -> [u8; 32] {
        self.content_hash
    }

    /// Sign the certificate with an Ed25519 signing key.
    pub fn sign(&mut self, signing_key: &SigningKey) {
        let signature = signing_key.sign(&self.content_hash);
        self.signature = signature.to_bytes().to_vec();
    }

    /// Verify the certificate's Ed25519 signature.
    pub fn verify(&self, verifying_key: &VerifyingKey) -> Result<(), LatticeError> {
        let signature = Signature::from_slice(&self.signature)
            .map_err(|_| LatticeError::CertificateVerificationFailed)?;
        verifying_key.verify(&self.content_hash, &signature)
            .map_err(|_| LatticeError::CertificateVerificationFailed)
    }
}
