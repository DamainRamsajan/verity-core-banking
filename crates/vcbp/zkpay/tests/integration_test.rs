use vcbp_zkpay::*;
use uuid::Uuid;

#[tokio::test]
async fn test_zkpay_flow() {
    let engine = ZkPayEngine::new(ZkPayEngineConfig::default());
    let intent = engine.generate_payment_intent(
        Uuid::new_v4(),
        Uuid::new_v4(),
        1000,
    ).await.unwrap();
    assert!(intent.stealth_address.is_some());
    engine.process_payment(&intent).await.unwrap();
    let stats = engine.get_stats().await;
    assert_eq!(stats.payments_completed, 1);
}
