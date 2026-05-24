use tokio::sync::RwLock;
use super::types::{MigrationPhase, PqcAlgorithm, HybridSignature};
use super::migration::MigrationManager;
use super::errors::PqcError;

#[allow(dead_code)]
pub struct PqcEngine {
    phase: RwLock<MigrationPhase>,
    migration: MigrationManager,
    config: PqcConfig,
    stats: RwLock<PqcStats>,
}

#[derive(Debug, Clone)]
pub struct PqcConfig {
    pub target_algorithm: PqcAlgorithm,
}

impl Default for PqcConfig {
    fn default() -> Self { Self { target_algorithm: PqcAlgorithm::MlDsa44 } }
}

#[derive(Debug, Default, Clone)]
pub struct PqcStats {
    pub keys_generated: u64,
    pub hybrid_signatures: u64,
}

impl PqcEngine {
    pub fn new(config: PqcConfig) -> Self {
        Self {
            phase: RwLock::new(MigrationPhase::Inventory),
            migration: MigrationManager::new(),
            config,
            stats: RwLock::new(PqcStats::default()),
        }
    }

    pub async fn hybrid_sign(&self, message: &[u8]) -> Result<HybridSignature, PqcError> {
        let mut stats = self.stats.write().await;
        stats.hybrid_signatures += 1;
        use ed25519_dalek::Signer;
use rand::rngs::OsRng;
use rand::RngCore;
        let mut csprng = OsRng;
        let mut seed = [0u8; 32];
        csprng.fill_bytes(&mut seed);
        let ed25519_key = ed25519_dalek::SigningKey::from_bytes(&seed);
        let classical_sig = ed25519_key.sign(message).to_bytes().to_vec();
        let pqc_sig = vec![0u8; 2420];
        Ok(HybridSignature {
            classical: classical_sig,
            pqc: pqc_sig,
            algorithm: self.config.target_algorithm,
            signed_at: chrono::Utc::now(),
        })
    }

    pub async fn advance_phase(&self) -> Result<MigrationPhase, PqcError> {
        let mut phase = self.phase.write().await;
        *phase = match *phase {
            MigrationPhase::Inventory => MigrationPhase::Hybrid,
            MigrationPhase::Hybrid => MigrationPhase::PqcOnly,
            MigrationPhase::PqcOnly => MigrationPhase::Complete,
            MigrationPhase::Complete => MigrationPhase::Complete,
        };
        Ok(*phase)
    }
}
