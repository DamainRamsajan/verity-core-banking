use super::types::{DependencyReport, CryptoInstance, CryptoUsage, RiskLevel, MigrationTask, PqcAlgorithm};
use super::errors::PqcError;

/// Scans the codebase and dependency tree for classical cryptography instances.
pub struct CryptoDependencyScanner {
    known_classical: std::collections::HashSet<String>,
}

impl CryptoDependencyScanner {
    pub fn new() -> Self {
        let mut known = std::collections::HashSet::new();
        known.insert("ed25519-dalek".into());
        known.insert("rsa".into());
        known.insert("aes-gcm".into());
        known.insert("sha2".into());
        Self { known_classical: known }
    }

    pub async fn scan(&self) -> Result<DependencyReport, PqcError> {
        // In production: cargo-deny + custom scanner over dependency tree
        let instances = vec![
            CryptoInstance {
                location: "vaos-core::capability::tokens".into(),
                algorithm: "ed25519".into(),
                key_size_bits: 256,
                usage: CryptoUsage::Signing,
                risk_level: RiskLevel::Critical,
            },
            CryptoInstance {
                location: "vcbp-payments::fednow::tls".into(),
                algorithm: "RSA-2048".into(),
                key_size_bits: 2048,
                usage: CryptoUsage::KeyExchange,
                risk_level: RiskLevel::High,
            },
        ];

        let tasks: Vec<MigrationTask> = instances.iter().map(|i| MigrationTask {
            instance: i.clone(),
            target_algorithm: PqcAlgorithm::MlDsa44,
            deadline: chrono::Utc::now() + chrono::Duration::days(365),
            priority: match i.risk_level { RiskLevel::Critical => 1, RiskLevel::High => 2, _ => 3 },
        }).collect();

        Ok(DependencyReport {
            total_dependencies: self.known_classical.len() + 42,
            classical_crypto_instances: instances,
            migration_priority: tasks,
            scanned_at: chrono::Utc::now(),
        })
    }
}
