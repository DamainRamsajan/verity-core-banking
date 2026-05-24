use serde::{Deserialize, Serialize};

/// PQC migration phases per G7 roadmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationPhase {
    Inventory,
    Hybrid,
    PqcOnly,
    Complete,
}

/// Supported PQC algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PqcAlgorithm {
    MlDsa44,
    MlDsa65,
    MlDsa87,
    MlKem512,
    MlKem768,
    MlKem1024,
    SlhDsa128s,
    SlhDsa128f,
}

/// A hybrid classical + PQC signature (dual-signing transition).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSignature {
    pub classical: Vec<u8>,
    pub pqc: Vec<u8>,
    pub algorithm: PqcAlgorithm,
    pub signed_at: chrono::DateTime<chrono::Utc>,
}

/// Result of cryptographic dependency scanning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyReport {
    pub total_dependencies: usize,
    pub classical_crypto_instances: Vec<CryptoInstance>,
    pub migration_priority: Vec<MigrationTask>,
    pub scanned_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoInstance {
    pub location: String,
    pub algorithm: String,
    pub key_size_bits: u32,
    pub usage: CryptoUsage,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CryptoUsage {
    Signing,
    Encryption,
    KeyExchange,
    Hashing,
    RandomGeneration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationTask {
    pub instance: CryptoInstance,
    pub target_algorithm: PqcAlgorithm,
    pub deadline: chrono::DateTime<chrono::Utc>,
    pub priority: u32,
}
