#[derive(Debug, thiserror::Error)]
pub enum RiskError {
    #[error("Cascade simulation failed: {0}")]
    SimulationFailed(String),
    #[error("Network too small for systemic analysis")]
    NetworkTooSmall,
    #[error("Institution not found in network: {0}")]
    InstitutionNotFound(uuid::Uuid),
}
