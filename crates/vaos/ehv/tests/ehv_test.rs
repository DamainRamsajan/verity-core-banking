#[cfg(test)]
mod tests {
    use vaos_ehv::*;

    #[tokio::test]
    async fn test_policy_ingestion_and_propagation() {
        let engine = engine::EhvEngine::new(engine::EhvConfig::default());

        let update = types::PolicyUpdate {
            update_id: uuid::Uuid::new_v4(),
            regulation: "CFPB ECOA Final Rule".into(),
            description: "Adverse action explanations must be in plain language".into(),
            formal_rule: "∀ action ∈ adverse_actions · plain_language(action.explanation)".into(),
            published_at: chrono::Utc::now(),
            effective_at: chrono::Utc::now() + chrono::Duration::days(30),
            propagated_at: None,
        };

        let latency = engine.ingest_regulation(update).await.unwrap();
        assert!(latency.total_latency_ms < 1000); // sub‑second propagation
    }

    #[tokio::test]
    async fn test_compliance_violation_detected() {
        let engine = engine::EhvEngine::new(engine::EhvConfig::default());

        let update = types::PolicyUpdate {
            update_id: uuid::Uuid::new_v4(),
            regulation: "Anti‑Fraud Directive".into(),
            description: "No unauthorised transfers".into(),
            formal_rule: "∀ transfer · authorised(transfer)".into(),
            published_at: chrono::Utc::now(),
            effective_at: chrono::Utc::now(),
            propagated_at: None,
        };

        engine.ingest_regulation(update).await.unwrap();

        let ok = engine.verify_action("payment", &serde_json::json!({})).await.unwrap();
        assert!(ok);

        let err = engine.verify_action("unauthorised transfer", &serde_json::json!({})).await;
        assert!(err.is_err());
    }
}
