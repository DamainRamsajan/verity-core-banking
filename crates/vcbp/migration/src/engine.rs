use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{MigrationConfig, MigrationPhase, MigrationReport};
use super::cobol::CobolParser;
use super::parallel_run::ParallelRunSimulator;
use super::documentation::DocumentationPipeline;
use super::errors::MigrationError;

/// Central migration engine.
pub struct MigrationEngine {
    config: MigrationConfig,
    cobol: CobolParser,
    parallel_run: Arc<ParallelRunSimulator>,
    documentation: DocumentationPipeline,
    phase: RwLock<MigrationPhase>,
    stats: RwLock<MigrationStats>,
}

#[derive(Debug, Default, Clone)]
pub struct MigrationStats {
    pub files_analyzed: u64,
    pub business_rules_extracted: u64,
    pub lines_processed: u64,
}

impl MigrationEngine {
    pub fn new(config: MigrationConfig) -> Self {
        Self {
            cobol: CobolParser::new(),
            parallel_run: Arc::new(ParallelRunSimulator::new(config.parallel_run_days)),
            documentation: DocumentationPipeline::new(),
            phase: RwLock::new(MigrationPhase::Discovery),
            stats: RwLock::new(MigrationStats::default()),
            config,
        }
    }

    /// Start the migration with a COBOL source file.
    #[tracing::instrument(name = "migration.start", level = "info", skip(self))]
    pub async fn start_migration(
        &self,
        institution_name: &str,
        source_system: &str,
    ) -> Result<MigrationReport, MigrationError> {
        let report = MigrationReport {
            report_id: uuid::Uuid::new_v4(),
            institution_name: institution_name.to_string(),
            source_system: source_system.to_string(),
            start_date: chrono::Utc::now(),
            completion_date: None,
            total_transactions_migrated: 0,
            total_mismatches: 0,
            total_rollbacks: 0,
            phase: MigrationPhase::Discovery,
            evidence_package_hash: None,
        };

        tracing::info!(institution = institution_name, "Migration started");
        Ok(report)
    }

    /// Advance to the next migration phase.
    pub async fn advance_phase(&self) -> Result<MigrationPhase, MigrationError> {
        let mut phase = self.phase.write().await;
        *phase = match *phase {
            MigrationPhase::Discovery => MigrationPhase::RuleExtraction,
            MigrationPhase::RuleExtraction => MigrationPhase::Validation,
            MigrationPhase::Validation => MigrationPhase::ParallelRun,
            MigrationPhase::ParallelRun => MigrationPhase::Cutover,
            MigrationPhase::Cutover => MigrationPhase::Complete,
            MigrationPhase::Complete => MigrationPhase::Complete,
        };
        Ok(*phase)
    }
}
