use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationPhase { Inventory, Hybrid, PqcOnly, Complete }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PqcAlgorithm { MlDsa44, MlDsa65, MlDsa87 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSignature {
    pub classical: Vec<u8>,
    pub pqc: Vec<u8>,
    pub algorithm: PqcAlgorithm,
    pub signed_at: chrono::DateTime<chrono::Utc>,
}
