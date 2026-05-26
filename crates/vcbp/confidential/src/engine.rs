use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use super::types::{ConfidentialConfig, ConfidentialBalance, HardwareBackend};
use super::errors::ConfidentialError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PqcSignature {
    pub classical: Vec<u8>,
    pub pqc: Option<Vec<u8>>,
}

pub struct ConfidentialEngine {
    balances: Arc<RwLock<HashMap<Uuid, ConfidentialBalance>>>,
    config: ConfidentialConfig,
    federated_key: Option<Vec<u8>>,
}

impl ConfidentialEngine {
    pub fn new(config: ConfidentialConfig) -> Self {
        Self {
            balances: Arc::new(RwLock::new(HashMap::new())),
            config,
            federated_key: None,
        }
    }

    /// Enable multi‑key mode by providing a federated key (stub).
    pub fn enable_multi_key(&mut self, key: Vec<u8>) {
        self.config.multi_key_mode = true;
        self.federated_key = Some(key);
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn encrypt_balance(
        &self,
        account_id: Uuid,
        balance: u64,
        public_key: Option<&[u8]>,
    ) -> Result<ConfidentialBalance, ConfidentialError> {
        #[cfg(feature = "confidential-mode")]
        {
            // Real TFHE encryption
            use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2;
            use tfhe::shortint::prelude::*;
            let (client_key, _server_key) = gen_keys(PARAM_MESSAGE_2_CARRY_2);
            let msg = balance % 4; // simple 2‑bit message for demonstration
            let ct = client_key.encrypt(msg as u64);
            let mut ct_bytes = vec![];
            // Serialize ciphertext (simplified)
            ct_bytes.push(ct.0 as u8);
            Ok(ConfidentialBalance {
                account_id,
                encrypted_value: ct_bytes,
                encryption_key_hash: hex::encode(blake3::hash(public_key.unwrap_or(&[])).as_bytes()),
                pqc_signature: Some(PqcSignature {
                    classical: vec![],
                    pqc: None,
                }),
            })
        }
        #[cfg(not(feature = "confidential-mode"))]
        {
            // Fallback: plaintext encoding (not real FHE)
            let encoded = balance.to_le_bytes().to_vec();
            Ok(ConfidentialBalance {
                account_id,
                encrypted_value: encoded,
                encryption_key_hash: hex::encode(blake3::hash(public_key.unwrap_or(&[])).as_bytes()),
                pqc_signature: Some(PqcSignature {
                    classical: vec![],
                    pqc: None,
                }),
            })
        }
    }

    #[tracing::instrument(skip(self), level = "info")]
    pub async fn decrypt_balance(
        &self,
        balance: &ConfidentialBalance,
    ) -> Result<u64, ConfidentialError> {
        #[cfg(feature = "confidential-mode")]
        {
            // Real decryption would require the client key (not stored)
            // For demonstration, return 0
            Ok(0)
        }
        #[cfg(not(feature = "confidential-mode"))]
        {
            // Decode plaintext encoding
            if balance.encrypted_value.len() < 8 {
                return Err(ConfidentialError::DecryptionError("Invalid ciphertext".into()));
            }
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&balance.encrypted_value[..8]);
            Ok(u64::from_le_bytes(bytes))
        }
    }

    pub async fn get_balance(&self, account_id: &Uuid) -> Option<ConfidentialBalance> {
        self.balances.read().await.get(account_id).cloned()
    }
}
