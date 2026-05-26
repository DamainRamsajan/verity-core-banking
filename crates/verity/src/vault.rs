//! Vault secrets provider – retrieves runtime secrets.
//! Source: ARC42 v22 ADR‑025
//! Feature‑gated: `verity --features production`

/// Retrieve a secret from Vault.
#[cfg(feature = "vault_client")]
#[allow(dead_code)]
pub async fn get_secret(key: &str) -> anyhow::Result<String> {
    let vault_addr = std::env::var("VAULT_ADDR")
        .context("VAULT_ADDR not set")?;
    let role_id = std::env::var("VAULT_ROLE_ID")
        .context("VAULT_ROLE_ID not set")?;
    let secret_id = std::env::var("VAULT_SECRET_ID")
        .context("VAULT_SECRET_ID not set")?;

    // Authenticate with Vault
    let client = vault_client::VaultClient::new(&vault_addr)?;
    let token = client.login_approle(&role_id, &secret_id).await?;

    // Read the secret
    let secret = client.read_secret(&token, key).await?;
    Ok(secret)
}

/// Stub for non‑production builds.
#[cfg(not(feature = "vault_client"))]
#[allow(dead_code)]
pub async fn get_secret(key: &str) -> anyhow::Result<String> {
    // In pilot mode, read from environment variable
    std::env::var(key)
        .with_context(|| format!("Secret '{}' not found in environment (Vault not enabled)", key))
}

use anyhow::Context;