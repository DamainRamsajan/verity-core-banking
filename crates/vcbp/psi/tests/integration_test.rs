use vcbp_psi::*;
use uuid::Uuid;

#[tokio::test]
async fn test_psi_proof_generation_and_verification() {
    let engine = PsiEngine::new(PsiEngineConfig::default());
    let request = PsiRequest {
        regulator_id: "REG-001".into(),
        query: "all_tx_above_10k".into(),
        timeframe_days: 30,
    };
    let proof = engine.generate_compliance_proof(&request, "BANK-001").await.unwrap();
    let valid = engine.verify_proof(&proof).unwrap();
    assert!(valid);
    let stats = engine.get_stats().await;
    assert_eq!(stats.proofs_generated, 1);
    assert_eq!(stats.proofs_verified, 1);
}
