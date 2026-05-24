//! Intel TDX Hardware Trust Interface
//! Source: ARC42 v20.0 §3 VAOS HTI

use super::{HtiTrait, TeeAttestationReport, TeePlatform, SealedKey, HtiError};
use async_trait::async_trait;

pub struct IntelTdxHti;

impl IntelTdxHti {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl HtiTrait for IntelTdxHti {
    async fn attest(&self) -> Result<TeeAttestationReport, HtiError> {
        Ok(TeeAttestationReport {
            platform: TeePlatform::IntelTdx,
            measurement: vec![0u8; 64],
            signature: vec![],
            timestamp: chrono::Utc::now(),
            is_healthy: true,
        })
    }

    async fn seal(&self, _data: &[u8]) -> Result<SealedKey, HtiError> {
        Ok(SealedKey { platform: TeePlatform::IntelTdx, encrypted_blob: vec![] })
    }

    async fn unseal(&self, _key: &SealedKey) -> Result<Vec<u8>, HtiError> {
        Ok(vec![])
    }

    fn arm_nmi(&self) -> Result<(), HtiError> { Ok(()) }
    fn nmi_triggered(&self) -> bool { false }
}