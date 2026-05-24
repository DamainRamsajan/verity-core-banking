#[cfg(test)]
mod tests {
    use vcbp_payments::*;

    #[tokio::test]
    async fn test_payment_engine() {
        let engine = engine::PaymentEngine::new();
        let fednow = Arc::new(rails::FedNowRail::new());
        engine.register_rail(fednow).await.unwrap();

        let payment = rail::Payment {
            id: uuid::Uuid::new_v4(),
            from_account: uuid::Uuid::new_v4(),
            to_account: "123456789".into(),
            amount: rust_decimal::Decimal::new(500, 0),
            currency: "USD".into(),
            rail_type: rail::RailType::FedNow,
            priority: rail::PaymentPriority::High,
            capability_token: vaos_core::types::CapabilityToken::test_token(),
            metadata: serde_json::Value::Null,
        };

        let result = engine.send(&payment).await;
        assert!(result.is_ok());
    }
}
