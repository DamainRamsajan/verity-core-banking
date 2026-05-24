//! PQC migration manager — tracks transition progress.
//!
//! Source: G7 CEG roadmap (Jan 2026), Google 2029 PQC target

/// Manages the PQC migration lifecycle.
#[derive(Debug)]
pub struct MigrationManager {
    pub phase: super::MigrationPhase,
    pub tokens_migrated: u64,
    pub tokens_remaining: u64,
}

impl MigrationManager {
    pub fn new() -> Self {
        Self {
            phase: super::MigrationPhase::Inventory,
            tokens_migrated: 0,
            tokens_remaining: 0,
        }
    }
}
