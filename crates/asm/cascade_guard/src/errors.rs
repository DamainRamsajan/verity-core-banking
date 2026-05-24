#[derive(Debug, thiserror::Error)]
pub enum CascadeError { #[error("Circuit open on channel {0}")] CircuitOpen(super::types::ChannelId), #[error("Channel not found")] ChannelNotFound }
