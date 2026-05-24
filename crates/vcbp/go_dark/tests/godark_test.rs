#[cfg(test)]
mod tests {
    use vcbp_go_dark::*;

    #[tokio::test]
    async fn test_execute_trade_with_zk_proof() {
        let engine = engine::GoDarkEngine::new(engine::GoDarkConfig::default());
        let intent = types::TradeIntent {
            trade_id: uuid::Uuid::new_v4(),
            asset_pair: "BTC/USD".into(),
            side: types::TradeSide::Buy,
            quantity: rust_decimal::Decimal::new(5, 0),
            limit_price: None,
            institution_id: uuid::Uuid::new_v4(),
            compliance_checks: vec![
                types::ComplianceCheck { check_type: "sanctions".into(), passed: true, details: None },
                types::ComplianceCheck { check_type: "capital".into(), passed: true, details: None },
            ],
        };
        let proof = engine.execute_trade(&intent).await.unwrap();
        assert!(proof.verified);
        assert!(!proof.proof_bytes.is_empty());
    }
}
