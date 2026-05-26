use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{EvidenceSpan, LearningEvent, AuditRecord};
use super::audit::LearningAuditLog;
use super::errors::EvidenceError;

/// Central evidence engine with contribution measurement.
pub struct EvidenceEngine {
    audit_log: Arc<RwLock<LearningAuditLog>>,
    config: EvidenceConfig,
    stats: RwLock<EvidenceStats>,
}

#[derive(Debug, Clone)]
pub struct EvidenceConfig {
    pub require_evidence: bool,
    pub min_confidence: f64,
    pub auto_deploy: bool,
}

impl Default for EvidenceConfig {
    fn default() -> Self {
        Self {
            require_evidence: true,
            min_confidence: 0.7,
            auto_deploy: false,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct EvidenceStats {
    pub events_recorded: u64,
    pub events_deployed: u64,
    pub events_rejected: u64,
    pub average_confidence: f64,
}

impl EvidenceEngine {
    pub fn new(config: EvidenceConfig) -> Self {
        Self {
            audit_log: Arc::new(RwLock::new(LearningAuditLog::new())),
            config,
            stats: RwLock::new(EvidenceStats::default()),
        }
    }

    /// Record a learning event with contribution‑measured evidence.
    #[tracing::instrument(name = "evidence.record", level = "info", skip(self))]
    pub async fn record(
        &self,
        agent_id: vaos_core::types::AgentId,
        description: &str,
        mut evidence: EvidenceSpan,
    ) -> Result<AuditRecord, EvidenceError> {
        // Automatically compute contribution score based on confidence and verification
        evidence.contribution_score = if evidence.verified {
            (evidence.confidence * 100.0).round() / 100.0
        } else {
            0.0
        };

        let mut stats = self.stats.write().await;
        stats.events_recorded += 1;

        let deployed = evidence.confidence >= self.config.min_confidence
            && evidence.verified;

        let event = LearningEvent {
            event_id: uuid::Uuid::new_v4(),
            agent_id,
            description: description.to_string(),
            evidence: evidence.clone(),
            learned_at: chrono::Utc::now(),
            deployed,
        };

        if deployed {
            stats.events_deployed += 1;
        } else {
            stats.events_rejected += 1;
        }

        stats.average_confidence = (stats.average_confidence
            * (stats.events_recorded - 1) as f64
            + evidence.confidence)
            / stats.events_recorded as f64;

        let record = self.audit_log.write().await.append(&event)?;

        tracing::info!(
            event_id = %event.event_id,
            agent_id = %agent_id,
            confidence = evidence.confidence,
            contribution = evidence.contribution_score,
            deployed,
            "Learning event recorded with contribution measurement"
        );

        Ok(record)
    }

    pub async fn audit_log(&self) -> Vec<AuditRecord> {
        self.audit_log.read().await.records()
    }

    pub async fn stats(&self) -> EvidenceStats {
        self.stats.read().await.clone()
    }
}
