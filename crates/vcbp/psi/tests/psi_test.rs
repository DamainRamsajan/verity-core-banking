#[cfg(test)]
mod tests {
    use vcbp_psi::*;

    #[tokio::test]
    async fn test_generate_and_verify_psi_proof() {
        let engine = engine::PsiEngine::new(engine::PsiConfig::default());

        let request = types::ComplianceRequest {
            request_id: uuid::Uuid::new_v4(),
            regulator: "ECB".into(),
            framework: types::RegulatoryFramework::Dora,
            scope: vec!["ICT_risk_management".into()],
            requested_at: chrono::Utc::now(),
        };

        let proof = engine.generate(&request).await.unwrap();
        assert!(proof.merkle_root.is_some());

        let verified = engine.verify(&proof).await.unwrap();
        assert!(verified);
    }
}
