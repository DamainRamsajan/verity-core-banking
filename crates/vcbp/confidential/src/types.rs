use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Hardware backend for FHE operations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum HardwareBackend {
    Software,
    Gpu,
    HeraclesAsic,
}

/// Configuration for confidential banking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidentialConfig {
    pub multi_key_mode: bool,
    pub hardware_backend: HardwareBackend,
}

impl Default for ConfidentialConfig {
    fn default() -> Self {
        Self {
            multi_key_mode: false,
            hardware_backend: HardwareBackend::Software,
        }
    }
}

/// A confidential (FHE‑encrypted) balance entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidentialBalance {
    pub account_id: Uuid,
    pub encrypted_value: Vec<u8>,         // TFHE ciphertext
    pub encryption_key_hash: String,      // hash of the public key used
    pub pqc_signature: Option<super::engine::PqcSignature>,
}
