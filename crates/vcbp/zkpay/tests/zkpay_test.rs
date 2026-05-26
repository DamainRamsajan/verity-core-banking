#[cfg(test)]
mod tests {
    use vcbp_zkpay::*;

    #[tokio::test]
    async fn test_process_valid_payment() {
        let engine = engine::ZkPayEngine::new(engine::ZkPayConfig::default());

        let intent = types::PaymentIntent {
            intent_id: uuid::Uuid::new_v4(),
            payer_agent: vaos_core::types::AgentId::new(),
            payee_agent: vaos_core::types::AgentId::new(),
            amount_sats: 1000,
            description: "Test payment".into(),
            created_at: chrono::Utc::now(),
        };

        let proof = engine.process(&intent).await.unwrap();
        assert!(proof.all_compliant());
    }
}
