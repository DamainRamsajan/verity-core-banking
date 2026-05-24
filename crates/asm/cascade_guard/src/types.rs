use serde::{Deserialize, Serialize};

pub type ChannelId = uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState { Closed, Open, HalfOpen }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitConfig {
    pub failure_threshold: u32,
    pub recovery_timeout_secs: u64,
    pub half_open_max_requests: u32,
}

impl Default for CircuitConfig {
    fn default() -> Self { Self { failure_threshold: 3, recovery_timeout_secs: 60, half_open_max_requests: 1 } }
}
