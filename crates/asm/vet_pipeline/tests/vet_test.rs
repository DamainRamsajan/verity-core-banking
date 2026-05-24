#[cfg(test)]
mod tests {
    use asm_vet_pipeline::*;

    #[tokio::test]
    async fn test_vet_benign_skill() {
        let pipeline = engine::VetPipeline::new(engine::VetConfig::default());
        let submission = types::SkillSubmission {
            submission_id: uuid::Uuid::new_v4(),
            agent_id: vaos_core::types::AgentId::new(),
            name: "Test Skill".into(),
            description: "A benign test skill".into(),
            skill_md: "".into(),
            executable_payload: vec![],
            submitted_at: chrono::Utc::now(),
        };
        let result = pipeline.vet(&submission).await.unwrap();
        assert_eq!(result.overall_status, types::StageStatus::Passed);
    }
}
