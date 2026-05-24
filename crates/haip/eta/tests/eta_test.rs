#[cfg(test)]
mod tests {
    use haip_eta::*;

    #[tokio::test]
    async fn test_stress_detection() {
        let engine = engine::EtaEngine::new();
        let user = uuid::Uuid::new_v4();
        let cal = engine.adapt(user, "overdraft", None).await.unwrap();
        assert_eq!(cal.recommended_tone, types::InteractionTone::Supportive);
        assert_eq!(cal.explanation_detail, types::ExplanationDetail::Balanced);
    }

    #[tokio::test]
    async fn test_knowledge_level_adaptation() {
        let engine = engine::EtaEngine::new();
        let user = uuid::Uuid::new_v4();
        engine.update_knowledge_level(user, types::KnowledgeLevel::Low).await;
        let cal = engine.adapt(user, "balance_inquiry", None).await.unwrap();
        assert_eq!(cal.explanation_detail, types::ExplanationDetail::Concrete);
    }
}
