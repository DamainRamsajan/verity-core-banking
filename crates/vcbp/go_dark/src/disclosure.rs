use super::types::{DisclosureLevel, TradeIntent};

/// Selective disclosure engine — reveals only what is required.
pub struct SelectiveDisclosure;

impl SelectiveDisclosure {
    pub fn new() -> Self { Self }

    /// Disclose trade information at the specified level.
    pub fn disclose(
        &self,
        intent: &TradeIntent,
        level: DisclosureLevel,
    ) -> serde_json::Value {
        match level {
            DisclosureLevel::ProofOnly => serde_json::json!({
                "trade_id": intent.trade_id,
                "status": "compliant",
                "proof_type": "zk_snark"
            }),
            DisclosureLevel::AggregateOnly => serde_json::json!({
                "trade_id": intent.trade_id,
                "asset_pair": intent.asset_pair,
                "side": intent.side,
                "status": "compliant"
            }),
            DisclosureLevel::FullDisclosure => serde_json::json!({
                "trade_id": intent.trade_id,
                "asset_pair": intent.asset_pair,
                "side": intent.side,
                "quantity": intent.quantity,
                "institution_id": intent.institution_id,
                "compliance_checks": intent.compliance_checks,
                "status": "compliant"
            }),
        }
    }
}
