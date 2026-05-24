//! eIDAS 2.0 digital identity wallet bridge.
//!
//! Source: eIDAS 2.0 regulation — Member States must issue EUDI Wallets by
//! December 2026; banks must accept them for Strong Customer Authentication
//! by December 2027.

/// Bridge to eIDAS 2.0 EUDI Wallets.
#[derive(Debug)]
pub struct EidasBridge {
    enabled: bool,
}

impl EidasBridge {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}
