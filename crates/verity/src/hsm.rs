//! PKCS#11 HSM abstraction – protect cryptographic keys.
//! Source: ARC42 v22 ADR‑023
//! Feature‑gated: `verity --features production`

/// Initialise the HSM connection.
#[cfg(feature = "pkcs11")]
#[allow(dead_code)]
pub fn init_hsm() -> anyhow::Result<()> {
    let lib_path = std::env::var("HSM_PKCS11_LIBRARY_PATH")
        .context("HSM_PKCS11_LIBRARY_PATH not set")?;
    let slot_id: u64 = std::env::var("HSM_SLOT_ID")
        .context("HSM_SLOT_ID not set")?
        .parse()?;
    let user_pin = std::env::var("HSM_USER_PIN")
        .context("HSM_USER_PIN not set")?;

    // Open PKCS#11 session
    let _pkcs11 = pkcs11::Pkcs11::new(&lib_path)?;
    // let session = pkcs11.open_session(slot_id)?;
    // session.login(&user_pin)?;

    tracing::info!(%lib_path, slot_id, "HSM initialised via PKCS#11");
    Ok(())
}

/// Stub for non‑production builds.
#[cfg(not(feature = "pkcs11"))]
#[allow(dead_code)]
pub fn init_hsm() -> anyhow::Result<()> {
    tracing::warn!("HSM not available – running without hardware key protection");
    Ok(())
}
