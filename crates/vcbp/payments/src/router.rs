use std::collections::HashMap;
use std::sync::Arc;
use super::rail::{PaymentRail, RailType, PaymentPriority};
use super::errors::PaymentError;

pub struct SmartRouter;

impl SmartRouter {
    pub fn new() -> Self { Self }

    pub fn select_rail(
        &self,
        currency: &str,
        amount: rust_decimal::Decimal,
        priority: PaymentPriority,
        rails: &HashMap<RailType, Arc<dyn PaymentRail>>,
    ) -> Result<RailType, PaymentError> {
        let available: Vec<&RailType> = rails.iter()
            .filter(|(_, r)| r.is_available() && r.supports(currency, amount))
            .map(|(t, _)| t).collect();

        if available.is_empty() {
            return Err(PaymentError::NoRailAvailable { currency: currency.to_string(), amount });
        }

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
            _ => Ok(*available[0]),
        }
    }
}
