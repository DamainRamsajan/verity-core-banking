#[cfg(test)]
mod tests {
    use vcbp_pqc::*;

    #[tokio::test]
    async fn test_scan_dependencies() {
        let engine = engine::PqcEngine::new(engine::PqcConfig::default());
        let report = engine.scan_dependencies().await.unwrap();
        assert!(report.total_dependencies > 0);
        assert!(!report.classical_crypto_instances.is_empty());
    }

    #[tokio::test]
    async fn test_hybrid_sign() {
        let engine = engine::PqcEngine::new(engine::PqcConfig::default());
        let sig = engine.hybrid_sign(b"test message").await.unwrap();
        assert!(!sig.classical.is_empty());
    }
}
