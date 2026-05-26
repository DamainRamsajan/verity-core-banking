#[cfg(test)]
mod tests {
    use vaos_evidence::*;

    #[tokio::test]
    async fn test_record_and_audit() {
        let engine = engine::EvidenceEngine::new(engine::EvidenceConfig::default());

        let evidence = types::EvidenceSpan {
            span_id: uuid::Uuid::new_v4(),
            source_url: "https://finra.gov/rules/2026/anti-fraud-pattern-42".into(),
            source_text: "Pattern detected in 1,247 transactions across 3 institutions".into(),
            confidence: 0.92,
            verified: true,
        };

        let record = engine
            .record(
                vaos_core::types::AgentId::new(),
                "Learned new fraud pattern: cross‑border structuring below $10k",
                evidence,
            )
            .await
            .unwrap();

        assert!(record.event.deployed);
    }

    #[tokio::test]
    async fn test_audit_log_integrity() {
        let engine = engine::EvidenceEngine::new(engine::EvidenceConfig::default());

        for i in 0..5 {
            let evidence = types::EvidenceSpan {
                span_id: uuid::Uuid::new_v4(),
                source_url: format!("https://example.com/event-{}", i),
                source_text: format!("Evidence for event {}", i),
                confidence: 0.85,
                verified: true,
            };

            engine
                .record(vaos_core::types::AgentId::new(), &format!("Event {}", i), evidence)
                .await
                .unwrap();
        }

        let audit_log = engine.audit_log().await;
        assert_eq!(audit_log.len(), 5);

        // Each event should be unique
        let ids: Vec<_> = audit_log.iter().map(|r| r.event.event_id).collect();
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), 5);
    }
}
