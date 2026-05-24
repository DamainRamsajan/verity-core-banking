use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    pub parallel_run_days: u32,
    pub require_zero_mismatches: bool,
}

impl Default for MigrationConfig {
    fn default() -> Self { Self { parallel_run_days: 90, require_zero_mismatches: true } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationPhase { Discovery, RuleExtraction, Validation, ParallelRun, Cutover, Complete }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationReport {
    pub report_id: Uuid,
    pub institution_name: String,
    pub source_system: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub completion_date: Option<chrono::DateTime<chrono::Utc>>,
    pub total_transactions_migrated: u64,
    pub total_mismatches: u64,
    pub phase: MigrationPhase,
}
