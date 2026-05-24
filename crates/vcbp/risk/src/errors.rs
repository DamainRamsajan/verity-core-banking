#[derive(Debug, thiserror::Error)]
pub enum RiskError {
    #[error("Institution not found in network")]
    InstitutionNotFound,
}
