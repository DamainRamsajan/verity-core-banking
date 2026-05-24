use tokio::sync::RwLock;
use super::types::{KillLevel, KillSwitchAction, ForensicSnapshot};
use super::errors::KillSwitchError;

pub struct KillSwitchEngine { config: KillSwitchConfig, stats: RwLock<KillSwitchStats> }

#[derive(Debug, Clone)]
pub struct KillSwitchConfig { pub enable_hardware_nmi: bool, pub forensic_snapshot_enabled: bool, pub auto_escalate_after_ms: u64 }

impl Default for KillSwitchConfig {
    fn default() -> Self { Self { enable_hardware_nmi: true, forensic_snapshot_enabled: true, auto_escalate_after_ms: 30_000 } }
}

#[derive(Debug, Default, Clone)]
pub struct KillSwitchStats { pub pause_events: u64, pub suspend_events: u64, pub terminate_events: u64, pub nmi_events: u64 }

impl KillSwitchEngine {
    pub fn new(config: KillSwitchConfig) -> Self { Self { config, stats: RwLock::new(KillSwitchStats::default()) } }

    pub async fn execute(&self, agent_id: vaos_core::types::AgentId, level: KillLevel, reason: &str) -> Result<KillSwitchAction, KillSwitchError> {
        let mut stats = self.stats.write().await;
        match level {
            KillLevel::Pause => { stats.pause_events += 1; tracing::warn!(?agent_id, "Agent PAUSED"); }
            KillLevel::Suspend => { stats.suspend_events += 1; tracing::warn!(?agent_id, "Agent SUSPENDED"); }
            KillLevel::Terminate => { stats.terminate_events += 1; tracing::error!(?agent_id, "Agent TERMINATED"); }
        }
        let snapshot = if self.config.forensic_snapshot_enabled && level == KillLevel::Terminate {
            Some(ForensicSnapshot { agent_id, snapshot_hash: [0u8; 32], captured_at: chrono::Utc::now(), memory_size_bytes: 0 })
        } else { None };
        Ok(KillSwitchAction { agent_id, level: level.clone(), reason: reason.to_string(), timestamp: chrono::Utc::now(), snapshot })
    }
}
