#[cfg(test)]
mod tests {
    use vcbp_fhe::*;
    use vcbp_pqc::*;
    use vcbp_risk::*;
    use vcbp_assets::*;
    use vcbp_go_dark::*;

    #[tokio::test]
    async fn test_fhe_encrypt_and_add() {
        let engine = engine::FheEngine::new(engine::FheConfig::default());
        let ct1 = engine.encrypt_balance(rust_decimal::Decimal::new(100, 0)).await.unwrap();
        let ct2 = engine.encrypt_balance(rust_decimal::Decimal::new(50, 0)).await.unwrap();
        let sum = engine.add_encrypted(&ct1, &ct2).await.unwrap();
        assert_eq!(sum.backend, ct1.backend);
    }

    #[tokio::test]
    async fn test_pqc_hybrid_sign() {
        let engine = engine::PqcEngine::new(engine::PqcConfig::default());
        let sig = engine.hybrid_sign(b"test message").await.unwrap();
        assert!(!sig.classical.is_empty());
    }

    #[tokio::test]
    async fn test_systemic_risk_simulation() {
        let engine = engine::SystemicRiskEngine::new();
        let node_a = uuid::Uuid::new_v4();
        let network = types::FinancialNetwork {
            nodes: vec![types::Institution {
                id: node_a, name: "Bank A".into(), total_assets: rust_decimal::Decimal::new(1_000_000, 0),
                tier1_capital: rust_decimal::Decimal::new(100_000, 0), leverage_ratio: 10.0, is_sib: true,
            }],
            edges: vec![],
            snapshot_date: chrono::NaiveDate::from_ymd_opt(2026, 3, 31).unwrap(),
        };
        let result = engine.simulate_cascade(&network, node_a).await.unwrap();
        assert!(!result.defaulted_institutions.is_empty());
    }

    #[tokio::test]
    async fn test_asset_position_update() {
        let engine = engine::MultiAssetEngine::new();
        let account = uuid::Uuid::new_v4();
        let pos = engine.update_position(account, "USD", rust_decimal::Decimal::new(1000, 0)).await.unwrap();
        assert_eq!(pos.currency_code, "USD");
    }

    #[tokio::test]
    async fn test_godark_trade() {
        let engine = engine::GoDarkEngine::new();
        let intent = types::TradeIntent {
            trade_id: uuid::Uuid::new_v4(),
            asset_pair: "BTC/USD".into(),
            side: types::TradeSide::Buy,
            quantity: rust_decimal::Decimal::new(5, 0),
            limit_price: None,
            institution_id: uuid::Uuid::new_v4(),
        };
        let proof = engine.execute_trade(&intent).await.unwrap();
        assert!(proof.verified);
    }
}
