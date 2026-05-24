use super::types::MigrationPhase;

pub struct MigrationManager {
    pub phase: MigrationPhase,
}

impl MigrationManager {
    pub fn new() -> Self { Self { phase: MigrationPhase::Inventory } }
}
