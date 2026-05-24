#[derive(Debug, thiserror::Error)]
pub enum LineageError {
    #[error("Memory entry not found: {0}")]
    EntryNotFound(uuid::Uuid),
    #[error("Memory entry quarantined: {0}")]
    EntryQuarantined(uuid::Uuid),
    #[error("Parent entry quarantined: {0}")]
    ParentQuarantined(uuid::Uuid),
    #[error("Merkle proof verification failed")]
    MerkleVerificationFailed,
    #[error("Provenance score below threshold")]
    ProvenanceBelowThreshold,
}
