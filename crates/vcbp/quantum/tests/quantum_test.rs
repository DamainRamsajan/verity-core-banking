#[cfg(test)]
mod tests {
    use vcbp_quantum::*;

    #[tokio::test]
    async fn test_portfolio_optimization() {
        let engine = engine::QuantumEngine::new(engine::QuantumConfig::default());
        let portfolio = types::Portfolio {
            id: uuid::Uuid::new_v4(),
            assets: vec![
                types::Asset { symbol: "AAPL".into(), expected_return: 0.12, volatility: 0.20, weight_min: 0.0, weight_max: 0.4 },
                types::Asset { symbol: "MSFT".into(), expected_return: 0.10, volatility: 0.18, weight_min: 0.0, weight_max: 0.4 },
                types::Asset { symbol: "GOOG".into(), expected_return: 0.14, volatility: 0.22, weight_min: 0.0, weight_max: 0.4 },
            ],
            constraints: vec![],
            objective: types::OptimizationObjective::MaxSharpeRatio,
        };
        let result = engine.optimize(&portfolio).await.unwrap();
        assert!(result.weights.len() == 3);
        let sum: f64 = result.weights.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }
}
