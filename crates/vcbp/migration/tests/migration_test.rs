#[cfg(test)]
mod tests {
    use vcbp_migration::*;

    #[tokio::test]
    async fn test_migration_engine() {
        let config = types::MigrationConfig::default();
        let engine = engine::MigrationEngine::new(config);
        let report = engine.start_migration("Test Bank", "Fiserv Premier").await.unwrap();
        assert_eq!(report.phase, types::MigrationPhase::Discovery);
    }

    #[test]
    fn test_parallel_run() {
        let mut sim = parallel_run::ParallelRunSimulator::new(90);
        let legacy = vec![(uuid::Uuid::new_v4(), "balance".into(), "100.00".into())];
        let verity = vec![(uuid::Uuid::new_v4(), "balance".into(), "100.00".into())];
        let mismatches = sim.compare_batch(&legacy, &verity).unwrap();
        assert!(mismatches.is_empty());
    }
}
