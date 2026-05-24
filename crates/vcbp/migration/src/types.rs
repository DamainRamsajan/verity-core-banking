use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Migration configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    pub parallel_run_days: u32,
    pub require_zero_mismatches: bool,
    pub auto_cutover: bool,
    pub claude_api_enabled: bool,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self { parallel_run_days: 90, require_zero_mismatches: true, auto_cutover: false, claude_api_enabled: false }
    }
}

/// Migration phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationPhase {
    Discovery,
    RuleExtraction,
    Validation,
    ParallelRun,
    Cutover,
    Complete,
}

/// A migration report for regulatory submission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationReport {
    pub report_id: Uuid,
    pub institution_name: String,
    pub source_system: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub completion_date: Option<chrono::DateTime<chrono::Utc>>,
    pub total_transactions_migrated: u64,
    pub total_mismatches: u64,
    pub total_rollbacks: u64,
    pub phase: MigrationPhase,
    pub evidence_package_hash: Option<[u8; 32]>,
}
