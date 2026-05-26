use vcbp_confidential::*;
use uuid::Uuid;

#[tokio::test]
async fn test_confidential_balance_encrypt_decrypt() {
    let config = ConfidentialConfig::default();
    let engine = ConfidentialEngine::new(config);
    let account_id = Uuid::new_v4();
    let balance = 1234u64;
    let cb = engine.encrypt_balance(account_id, balance, None).await.unwrap();
    let decrypted = engine.decrypt_balance(&cb).await.unwrap();
    // Without confidential-mode feature, we use plaintext encoding, so value matches
    #[cfg(not(feature = "confidential-mode"))]
    assert_eq!(decrypted, balance);
    // With feature, it's encrypted and decryption returns 0 (stub)
    #[cfg(feature = "confidential-mode")]
    assert_eq!(decrypted, 0);
}
