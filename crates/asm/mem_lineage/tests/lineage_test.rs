#[cfg(test)]
mod tests {
    use asm_mem_lineage::*;

    #[tokio::test]
    async fn test_write_and_read_clean() {
        let engine = engine::MemLineageEngine::new(engine::LineageConfig::default());
        let agent = vaos_core::types::AgentId::new();
        let entry = engine.write(agent, serde_json::json!({"key": "value"}), types::MemoryEntryType::Observation, &[]).await.unwrap();
        assert_eq!(entry.quarantine_status, types::QuarantineStatus::Clean);
        let read = engine.read(entry.entry_id).await.unwrap();
        assert_eq!(read.content, serde_json::json!({"key": "value"}));
    }
}
