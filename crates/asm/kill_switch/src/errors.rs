#[derive(Debug, thiserror::Error)]
pub enum KillSwitchError { #[error("Agent not found: {0:?}")] AgentNotFound(vaos_core::types::AgentId), #[error("Forensic snapshot failed")] SnapshotFailed, #[error("NMI trigger failed")] NmiTriggerFailed }
