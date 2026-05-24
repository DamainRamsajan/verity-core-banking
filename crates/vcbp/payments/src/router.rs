use std::collections::HashMap;
use std::sync::Arc;
use super::rail::{PaymentRail, RailType, PaymentPriority};
use super::errors::PaymentError;

/// Smart router — selects the optimal payment rail based on
/// value, urgency, cost, and counterparty capability.
pub struct SmartRouter;

impl SmartRouter {
    pub fn new() -> Self { Self }

    /// Select the best available rail for a payment.
    pub fn select_rail(
        &self,
        currency: &str,
        amount: rust_decimal::Decimal,
        priority: PaymentPriority,
        rails: &HashMap<RailType, Arc<dyn PaymentRail>>,
    ) -> Result<RailType, PaymentError> {
        let available: Vec<&RailType> = rails
            .iter()
            .filter(|(_, r)| r.is_available() && r.supports(currency, amount))
            .map(|(t, _)| t)
            .collect();

        if available.is_empty() {
            return Err(PaymentError::NoRailAvailable {
                currency: currency.to_string(),
                amount,
            });
        }

        // Priority‑based selection
        match priority {
            PaymentPriority::Critical => {
                if available.contains(&&RailType::FedWire) { Ok(RailType::FedWire) }
                else { Ok(*available[0]) }
            }
            PaymentPriority::High => {
                if available.contains(&&RailType::FedNow) { Ok(RailType::FedNow) }
                else if available.contains(&&RailType::Rtp) { Ok(RailType::Rtp) }
                else { Ok(*available[0]) }
            }
            PaymentPriority::Normal | PaymentPriority::Low => {
                if amount < rust_decimal::Decimal::new(100_000, 0) && available.contains(&&RailType::Ach) {
                    Ok(RailType::Ach)
                } else {
                    Ok(*available[0])
                }
            }
        }
    }
}
