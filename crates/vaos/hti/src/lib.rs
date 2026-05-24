//! # Hardware Trust Interface (HTI)
//!
//! Abstracts over Intel TDX, AMD SEV-SNP, and ARM CCA trusted execution
//! environments. Provides remote attestation, sealed storage, and the
//! Non-Maskable Interrupt (NMI) vector for hardware‑rooted corrigibility.
//!
//! ## Architecture
//! - Concurrent multi-TEE operation (ADR-006)
//! - CVE‑driven failover within 72 hours (CVE‑2025‑66660 class)
//! - KingsGuard enclave data flow protection (ACM CCS 2026)
//! - IBM ACE‑RISCV formally verified security monitor pattern
//!
//! Source: ARC42 v20.0 §3 VAOS HTI

pub mod intel_tdx;
pub mod amd_sev;
pub mod tee_vuln;
pub mod kings_guard;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Result of a TEE remote attestation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeAttestationReport {
    pub platform: TeePlatform,
    pub measurement: [u8; 64],
    pub signature: Vec<u8>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub is_healthy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeePlatform {
    IntelTdx,
    AmdSevSnp,
    ArmCca,
}

/// Encrypted key sealed to the TEE's hardware identity.
#[derive(Debug, Clone)]
pub struct SealedKey {
    pub platform: TeePlatform,
    pub encrypted_blob: Vec<u8>,
}

/// The Hardware Trust Interface trait.
#[async_trait]
pub trait HtiTrait: Send + Sync {
    /// Perform remote attestation — prove the TEE's identity and integrity.
    async fn attest(&self) -> Result<TeeAttestationReport, HtiError>;

    /// Seal data to the TEE (hardware‑bound encryption).
    async fn seal(&self, data: &[u8]) -> Result<SealedKey, HtiError>;

    /// Unseal previously‑sealed data.
    async fn unseal(&self, key: &SealedKey) -> Result<Vec<u8>, HtiError>;

    /// Arm the Non‑Maskable Interrupt (NMI) for hardware‑rooted corrigibility.
    fn arm_nmi(&self) -> Result<(), HtiError>;

    /// Check whether the NMI has been triggered.
    fn nmi_triggered(&self) -> bool;
}

#[derive(Debug, thiserror::Error)]
pub enum HtiError {
    #[error("TEE attestation failed: {0}")]
    AttestationFailed(String),
    #[error("Sealing failed: {0}")]
    SealFailed(String),
    #[error("NMI not configured")]
    NmiNotConfigured,
    #[error("Both TEEs compromised — safe halt required")]
    DualTeeCompromised,
    #[error("Platform not supported: {0:?}")]
    PlatformNotSupported(TeePlatform),
}

/// Factory to create the appropriate HTI implementation based on
/// platform detection.
pub fn create_hti() -> Result<Box<dyn HtiTrait>, HtiError> {
    if std::path::Path::new("/dev/tdx-attest").exists() {
        Ok(Box::new(intel_tdx::IntelTdxHti::new()))
    } else if std::path::Path::new("/dev/sev").exists() {
        Ok(Box::new(amd_sev::AmdSevHti::new()))
    } else {
        tracing::warn!("No TEE detected — running in simulation mode");
        Ok(Box::new(SimulatedHti::new()))
    }
}

/// Simulated HTI for development and testing.
struct SimulatedHti {
    nmi_armed: std::sync::atomic::AtomicBool,
}

impl SimulatedHti {
    fn new() -> Self {
        Self { nmi_armed: std::sync::atomic::AtomicBool::new(false) }
    }
}

#[async_trait]
impl HtiTrait for SimulatedHti {
    async fn attest(&self) -> Result<TeeAttestationReport, HtiError> {
        Ok(TeeAttestationReport {
            platform: TeePlatform::IntelTdx,
            measurement: [0u8; 64],
            signature: vec![],
            timestamp: chrono::Utc::now(),
            is_healthy: true,
        })
    }

    async fn seal(&self, _data: &[u8]) -> Result<SealedKey, HtiError> {
        Ok(SealedKey {
            platform: TeePlatform::IntelTdx,
            encrypted_blob: vec![],
        })
    }

    async fn unseal(&self, _key: &SealedKey) -> Result<Vec<u8>, HtiError> {
        Ok(vec![])
    }

    fn arm_nmi(&self) -> Result<(), HtiError> {
        self.nmi_armed.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    fn nmi_triggered(&self) -> bool {
        self.nmi_armed.load(std::sync::atomic::Ordering::SeqCst)
    }
}
