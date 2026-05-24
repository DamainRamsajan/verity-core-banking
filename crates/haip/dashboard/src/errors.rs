#[derive(Debug, thiserror::Error)]
pub enum DashboardError {
    #[error("Agent not configured: {0:?}")]
    AgentNotConfigured(vaos_core::types::AgentId),
}
