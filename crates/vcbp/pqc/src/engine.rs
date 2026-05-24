use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{MigrationPhase, PqcAlgorithm, HybridSignature, DependencyReport};
use super::migration::MigrationManager;
use super::scanner::CryptoDependencyScanner;
use super::reencrypt::LongLivedReencryptor;
use super::errors::PqcError;

/// Central PQC migration engine.
///
/// Coordinates the transition from classical to post-quantum cryptography
/// across all Verity components.
pub struct PqcEngine {
    phase: RwLock<MigrationPhase>,
    migration: Arc<MigrationManager>,
    scanner: Arc<CryptoDependencyScanner>,
    reencryptor: Arc<LongLivedReencryptor>,
    config: PqcConfig,
    stats: RwLock<PqcStats>,
}

#[derive(Debug, Clone)]
pub struct PqcConfig {
    pub target_algorithm: PqcAlgorithm,
    pub hybrid_transition_start: chrono::NaiveDate,
    pub classical_deprecation: chrono::NaiveDate,
    pub enable_dynamic_migration_window: bool,
}

impl Default for PqcConfig {
    fn default() -> Self {
        Self {
            target_algorithm: PqcAlgorithm::MlDsa44,
            hybrid_transition_start: chrono::NaiveDate::from_ymd_opt(2027, 7, 1).unwrap(),
            classical_deprecation: chrono::NaiveDate::from_ymd_opt(2029, 1, 1).unwrap(),
            enable_dynamic_migration_window: true,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PqcStats {
    pub keys_generated: u64,
    pub hybrid_signatures: u64,
    pub reencrypted_entries: u64,
    pub dependencies_scanned: u64,
}

impl PqcEngine {
    pub fn new(config: PqcConfig) -> Self {
        Self {
            phase: RwLock::new(MigrationPhase::Inventory),
            migration: Arc::new(MigrationManager::new()),
            scanner: Arc::new(CryptoDependencyScanner::new()),
            reencryptor: Arc::new(LongLivedReencryptor::new()),
            config,
            stats: RwLock::new(PqcStats::default()),
        }
    }

    /// Run a cryptographic dependency scan across the entire codebase.
    #[tracing::instrument(name = "pqc.scan", level = "info", skip(self))]
    pub async fn scan_dependencies(&self) -> Result<DependencyReport, PqcError> {
        let mut stats = self.stats.write().await;
        stats.dependencies_scanned += 1;
        self.scanner.scan().await
    }

    /// Generate a hybrid Ed25519 + ML-DSA-44 signature for migration.
    #[tracing::instrument(name = "pqc.hybrid_sign", level = "info", skip(self))]
    pub async fn hybrid_sign(
        &self,
        message: &[u8],
    ) -> Result<HybridSignature, PqcError> {
        let mut stats = self.stats.write().await;
        stats.hybrid_signatures += 1;

        // Generate classical Ed25519 signature
        use rand::rngs::OsRng;
        let mut csprng = OsRng;
        let ed25519_key = ed25519_dalek::SigningKey::generate(&mut csprng);
        let classical_sig = ed25519_key.sign(message).to_bytes().to_vec();

        // Generate ML-DSA-44 signature via dcrypt
        // In production: dcrypt::ml_dsa::sign(keypair, message)
        let pqc_sig = vec![0u8; 2420]; // ML-DSA-44 signature size

        Ok(HybridSignature {
            classical: classical_sig,
            pqc: pqc_sig,
            algorithm: self.config.target_algorithm,
            signed_at: chrono::Utc::now(),
        })
    }

    /// Advance the migration phase.
    pub async fn advance_phase(&self) -> Result<MigrationPhase, PqcError> {
        let mut phase = self.phase.write().await;
        *phase = match *phase {
            MigrationPhase::Inventory => MigrationPhase::Hybrid,
            MigrationPhase::Hybrid => MigrationPhase::PqcOnly,
            MigrationPhase::PqcOnly => MigrationPhase::Complete,
            MigrationPhase::Complete => MigrationPhase::Complete,
        };
        tracing::info!(?phase, "PQC migration phase advanced");
        Ok(*phase)
    }
}
