#[cfg(test)]
mod tests {
    use vcbp_fraud::*;
    use vcbp_federated::*;
    use vcbp_quantum::*;
    use vcbp_edge::*;
    use vcbp_migration::*;
    use vcbp_marketplace::*;

    #[tokio::test]
    async fn test_fraud_scoring() {
        let engine = engine::GnnFraudEngine::new();
        let graph = types::TransactionGraph {
            nodes: vec![],
            edges: vec![types::GraphEdge {
                source: 0, target: 1, amount: 50_000.0, currency: "USD".into(),
                timestamp: chrono::Utc::now(),
            }],
            snapshot_at: chrono::Utc::now(),
        };
        let score = engine.score_graph(&graph).await.unwrap();
        assert!(score.score > 0.0);
    }

    #[tokio::test]
    async fn test_federated_round() {
        let mesh = mesh::FlMesh::new(4);
        mesh.start_round().await.unwrap();
    }

    #[tokio::test]
    async fn test_quantum_optimization() {
        let engine = engine::QuantumEngine::new();
        let portfolio = types::Portfolio {
            id: uuid::Uuid::new_v4(),
            assets: vec![types::Asset { symbol: "AAPL".into(), expected_return: 0.12, volatility: 0.20, weight_min: 0.0, weight_max: 0.4 }],
        };
        let result = engine.optimize(&portfolio).await.unwrap();
        assert!(!result.weights.is_empty());
    }

    #[tokio::test]
    async fn test_edge_offline_transaction() {
        let config = types::EdgeConfig::default();
        let runtime = runtime::EdgeRuntime::new(config);
        let tx = types::OfflineTransaction {
            id: uuid::Uuid::new_v4(),
            from_account: uuid::Uuid::new_v4(),
            to_account: "recipient".into(),
            amount: rust_decimal::Decimal::new(500, 0),
            currency: "USD".into(),
            timestamp: chrono::Utc::now(),
            signature: vec![],
            synced: false,
        };
        runtime.process_transaction(tx).await.unwrap();
    }

    #[tokio::test]
    async fn test_migration_engine() {
        let engine = engine::MigrationEngine::new(types::MigrationConfig::default());
        let report = engine.start_migration("Test Bank", "Fiserv Premier").await.unwrap();
        assert_eq!(report.phase, types::MigrationPhase::Discovery);
    }

    #[tokio::test]
    async fn test_marketplace_listing() {
        let registry = registry::TokenCuratedRegistry::new(registry::RegistryConfig::default());
        let listing = types::AgentListing {
            listing_id: uuid::Uuid::new_v4(),
            agent_id: vaos_core::types::AgentId::new(),
            name: "Fraud Agent".into(),
            description: "Detects fraud".into(),
            capabilities: vec!["fraud_detection".into()],
            stake_amount: rust_decimal::Decimal::new(2_000, 0),
            status: types::ListingStatus::Pending,
            reputation: types::ReputationScore::new(),
            listed_at: chrono::Utc::now(),
        };
        let result = registry.apply_listing(listing).await.unwrap();
        assert_eq!(result.status, types::ListingStatus::Pending);
    }
}
