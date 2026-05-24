use super::types::MigrationPhase;

/// Manages the PQC migration lifecycle per the G7 CEG roadmap.
pub struct MigrationManager {
    pub phase: MigrationPhase,
    pub tokens_migrated: u64,
    pub tokens_remaining: u64,
}

impl MigrationManager {
    pub fn new() -> Self {
        Self { phase: MigrationPhase::Inventory, tokens_migrated: 0, tokens_remaining: 0 }
    }

    /// Check the Fukuda-Matsuo migration liveness condition.
    /// Δeff ≥ ⌈4(1-ϵ)f⌉ must hold for safe migration.
    pub fn check_liveness(
        &self,
        effective_window: f64,
        epsilon: f64,
        fault_tolerance: u64,
    ) -> bool {
        let required = 4.0 * (1.0 - epsilon) * fault_tolerance as f64;
        effective_window >= required.ceil()
    }
}
