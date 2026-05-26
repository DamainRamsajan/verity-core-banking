use vcbp_fido::*;
use uuid::Uuid;
use ed25519_dalek::{SigningKey, Signer};

#[tokio::test]
async fn test_fido_credential_flow() {
    let engine = FidoEngine::new(FidoEngineConfig::default());
    let agent_id = Uuid::new_v4();
    let mut csprng = rand::thread_rng();
    let signing_key = SigningKey::generate(&mut csprng);
    let public_key = signing_key.verifying_key().to_bytes().to_vec();

    let cred = engine.issue_credential(agent_id, public_key.clone(), 30).await.unwrap();
    assert_eq!(cred.agent_id, agent_id);

    // Create mandate
    let scope = MandateScope {
        max_amount: rust_decimal::Decimal::new(1000, 0),
        currency: "USD".into(),
        counterparty_allowlist: vec![],
        frequency_limit: None,
        action_types: vec!["transfer".into()],
    };
    let payload = serde_json::to_vec(&(&Uuid::new_v4(), &scope)).unwrap();
    let signature = signing_key.sign(&payload);
    let mut signed_payload = payload.clone();
    signed_payload.extend_from_slice(&signature.to_bytes());

    let mandate = Ap2Mandate {
        mandate_id: Uuid::new_v4(),
        credential_id: cred.credential_id,
        scope,
        signed_payload,
        pqc_signature: None,
    };

    engine.verify_payment(&mandate).await.unwrap();
    let stats = engine.get_stats().await;
    assert_eq!(stats.mandates_verified, 1);
}
