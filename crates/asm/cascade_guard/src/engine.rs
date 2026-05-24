use std::collections::HashMap;
use tokio::sync::RwLock;

use super::types::{CircuitState, ChannelId, CircuitConfig};
use super::errors::CascadeError;

pub struct CascadeGuard {
    channels: RwLock<HashMap<ChannelId, ChannelState>>,
    config: CircuitConfig,
}

#[derive(Debug, Clone)]
struct ChannelState {
    state: CircuitState,
    failure_count: u32,
    last_failure: Option<chrono::DateTime<chrono::Utc>>,
}

impl CascadeGuard {
    pub fn new(config: CircuitConfig) -> Self {
        Self { channels: RwLock::new(HashMap::new()), config }
    }

    pub async fn check(&self, channel_id: ChannelId) -> Result<(), CascadeError> {
        let channels = self.channels.read().await;
        if let Some(ch) = channels.get(&channel_id) {
            if ch.state == CircuitState::Open {
                if let Some(last) = ch.last_failure {
                    let elapsed = (chrono::Utc::now() - last).num_seconds() as u64;
                    if elapsed < self.config.recovery_timeout_secs {
                        return Err(CascadeError::CircuitOpen(channel_id));
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn record_failure(&self, channel_id: ChannelId) {
        let mut channels = self.channels.write().await;
        let ch = channels.entry(channel_id).or_insert(ChannelState { state: CircuitState::Closed, failure_count: 0, last_failure: None });
        ch.failure_count += 1;
        ch.last_failure = Some(chrono::Utc::now());
        if ch.failure_count >= self.config.failure_threshold {
            ch.state = CircuitState::Open;
            tracing::warn!(%channel_id, "Circuit OPEN");
        }
    }

    pub async fn record_success(&self, channel_id: ChannelId) {
        let mut channels = self.channels.write().await;
        if let Some(ch) = channels.get_mut(&channel_id) {
            ch.failure_count = 0;
            ch.state = CircuitState::Closed;
        }
    }
}
