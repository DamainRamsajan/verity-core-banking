use std::collections::HashMap;
use tokio::sync::RwLock;
use vaos_core::types::AgentId;

use super::types::{DriftStatus, BehavioralBaseline, AnomalyReport};
use super::errors::DriftError;

pub struct DriftMonitor {
    baselines: RwLock<HashMap<AgentId, BehavioralBaseline>>,
    config: DriftConfig,
    stats: RwLock<DriftStats>,
}

#[derive(Debug, Clone)]
pub struct DriftConfig {
    pub drift_threshold: f64,
    pub baseline_window_days: u32,
    pub anomaly_min_severity: u8,
}

impl Default for DriftConfig {
    fn default() -> Self { Self { drift_threshold: 0.85, baseline_window_days: 30, anomaly_min_severity: 5 } }
}

#[derive(Debug, Default, Clone)]
pub struct DriftStats { pub actions_monitored: u64, pub anomalies_detected: u64, pub silent_overrides_blocked: u64 }

impl DriftMonitor {
    pub fn new(config: DriftConfig) -> Self {
        Self { baselines: RwLock::new(HashMap::new()), config, stats: RwLock::new(DriftStats::default()) }
    }

    pub async fn evaluate(&self, agent_id: AgentId, action: &serde_json::Value) -> Result<DriftStatus, DriftError> {
        let mut stats = self.stats.write().await;
        stats.actions_monitored += 1;
        let baselines = self.baselines.read().await;
        if let Some(baseline) = baselines.get(&agent_id) {
            let deviation = baseline.compute_deviation(action);
            if deviation > self.config.drift_threshold {
                stats.anomalies_detected += 1;
                if action.get("parameter_mutation").and_then(|v| v.as_bool()).unwrap_or(false) {
                    stats.silent_overrides_blocked += 1;
                }
                return Ok(DriftStatus::Anomalous(AnomalyReport { agent_id, deviation, severity: (deviation * 10.0) as u8, timestamp: chrono::Utc::now() }));
            }
        }
        Ok(DriftStatus::WithinBounds)
    }
}
