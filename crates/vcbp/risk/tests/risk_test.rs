#[cfg(test)]
mod tests {
    use vcbp_risk::*;

    #[tokio::test]
    async fn test_cascade_simulation() {
        let engine = engine::SystemicRiskEngine::new(engine::RiskConfig::default());
        let node_a = uuid::Uuid::new_v4();
        let node_b = uuid::Uuid::new_v4();
        let network = types::FinancialNetwork {
            nodes: vec![
                types::Institution {
                    id: node_a, name: "Bank A".into(), total_assets: rust_decimal::Decimal::new(1_000_000, 0),
                    tier1_capital: rust_decimal::Decimal::new(100_000, 0), leverage_ratio: 10.0, is_sib: true,
                },
                types::Institution {
                    id: node_b, name: "Bank B".into(), total_assets: rust_decimal::Decimal::new(500_000, 0),
                    tier1_capital: rust_decimal::Decimal::new(50_000, 0), leverage_ratio: 10.0, is_sib: false,
                },
            ],
            edges: vec![
                types::ExposureEdge { source: node_a, target: node_b, amount: rust_decimal::Decimal::new(80_000, 0), channel: types::RiskChannel::Counterparty },
            ],
            snapshot_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        };

        let result = engine.simulate_cascade(&network, node_a).await.unwrap();
        assert!(result.defaulted_institutions.len() >= 1);
    }
}
