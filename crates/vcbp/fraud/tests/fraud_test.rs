#[cfg(test)]
mod tests {
    use vcbp_fraud::*;

    #[tokio::test]
    async fn test_engine_init() {
        let engine = engine::GnnFraudEngine::new();
        let graph = types::TransactionGraph {
            nodes: vec![],
            edges: vec![],
            snapshot_at: chrono::Utc::now(),
        };
        let score = engine.score_graph(&graph).await.unwrap();
        assert!(score.score >= 0.0 && score.score <= 1.0);
    }
}
