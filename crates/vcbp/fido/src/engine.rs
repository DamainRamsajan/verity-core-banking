use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use ed25519_dalek::{SigningKey, Signer, VerifyingKey, Signature, Verifier};
use super::types::{AgentCredential, Ap2Mandate, MandateScope, PqcSignature};
use super::errors::FidoError;

#[derive(Debug)]
pub struct FidoEngineConfig {
    pub max_credentials_per_agent: usize,
}

impl Default for FidoEngineConfig {
    fn default() -> Self {
        Self {
            max_credentials_per_agent: 100,
        }
    }
}

#[derive(Debug, Default)]
pub struct FidoEngineStats {
    pub credentials_issued: u64,
    pub mandates_verified: u64,
    pub failed_verifications: u64,
}

pub struct FidoEngine {
    credentials: Arc<RwLock<HashMap<Uuid, AgentCredential>>>,
    mandates: Arc<RwLock<HashMap<Uuid, Ap2Mandate>>>,
    config: FidoEngineConfig,
    stats: Arc<RwLock<FidoEngineStats>>,
}

impl FidoEngine {
    pub fn new(config: FidoEngineConfig) -> Self {
        Self {
            credentials: Arc::new(RwLock::new(HashMap::new())),
            mandates: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(FidoEngineStats::default())),
        }
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn issue_credential(
        &self,
        agent_id: Uuid,
        public_key: Vec<u8>,
        expiry_days: u32,
    ) -> Result<AgentCredential, FidoError> {
        let mut creds = self.credentials.write().await;
        if creds.len() >= self.config.max_credentials_per_agent {
            return Err(FidoError::DuplicateCredential);
        }

        let credential_id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let credential = AgentCredential {
            credential_id,
            agent_id,
            public_key,
            pqc_signature: Some(PqcSignature {
                classical: vec![],
                pqc: None,
            }),
            tee_attestation: None, // populated by TEE verifier later
            issued_at: now,
            expires_at: now + chrono::Duration::days(expiry_days as i64),
            issuer: "Verity FIDO Authority".into(),
        };

        creds.insert(credential_id, credential.clone());
        let mut stats = self.stats.write().await;
        stats.credentials_issued += 1;
        Ok(credential)
    }

    #[tracing::instrument(skip(self, mandate), level = "info")]
    pub async fn verify_payment(
        &self,
        mandate: &Ap2Mandate,
    ) -> Result<(), FidoError> {
        let creds = self.credentials.read().await;
        let credential = creds.get(&mandate.credential_id)
            .ok_or(FidoError::CredentialNotFound(mandate.credential_id))?;

        // Verify the Ed25519 signature
        let payload = serde_json::to_vec(&(&mandate.mandate_id, &mandate.scope))
            .map_err(|_| FidoError::InvalidSignature)?;

        if mandate.signed_payload.len() <= 64 {
            return Err(FidoError::InvalidSignature);
        }
        let sig_bytes = &mandate.signed_payload[mandate.signed_payload.len() - 64..];
        let signature = Signature::from_slice(sig_bytes)
            .map_err(|_| FidoError::InvalidSignature)?;

        let verifying_key = VerifyingKey::from_bytes(
            &credential.public_key[..32].try_into().map_err(|_| FidoError::InvalidSignature)?
        ).map_err(|_| FidoError::InvalidSignature)?;

        verifying_key.verify(&payload, &signature)
            .map_err(|_| FidoError::InvalidSignature)?;

        // Check PQC signature if present
        if let Some(pqc) = &mandate.pqc_signature {
            // Stub: when PQC stack is integrated, verify ML‑DSA‑44 here
            let _ = pqc;
        }

        let mut stats = self.stats.write().await;
        stats.mandates_verified += 1;
        Ok(())
    }

    pub async fn get_stats(&self) -> FidoEngineStats {
        self.stats.read().await.clone()
    }
}
