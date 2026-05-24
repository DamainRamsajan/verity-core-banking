use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use super::rail::{PaymentRail, Payment, PaymentReceipt, RailType};
use super::router::SmartRouter;
use super::errors::PaymentError;

pub struct PaymentEngine {
    rails: RwLock<HashMap<RailType, Arc<dyn PaymentRail>>>,
    router: SmartRouter,
    stats: RwLock<PaymentStats>,
}

#[derive(Debug, Default, Clone)]
pub struct PaymentStats {
    pub payments_sent: u64,
    pub payments_settled: u64,
    pub payments_rejected: u64,
    pub rail_failovers: u64,
}

impl PaymentEngine {
    pub fn new() -> Self {
        Self { rails: RwLock::new(HashMap::new()), router: SmartRouter::new(), stats: RwLock::new(PaymentStats::default()) }
    }

    pub async fn register_rail(&self, rail: Arc<dyn PaymentRail>) -> Result<(), PaymentError> {
        let mut rails = self.rails.write().await;
        rails.insert(rail.rail_type(), rail);
        Ok(())
    }

    pub async fn send(&self, payment: &Payment) -> Result<PaymentReceipt, PaymentError> {
        let rails = self.rails.read().await;
        let mut stats = self.stats.write().await;
        let rail_type = self.router.select_rail(payment.currency.as_str(), payment.amount, payment.priority, &rails)?;
        let rail = rails.get(&rail_type).ok_or(PaymentError::RailNotFound(rail_type))?;
        match rail.send(payment).await {
            Ok(receipt) => { stats.payments_sent += 1; Ok(receipt) }
            Err(e) => { stats.payments_rejected += 1; Err(e) }
        }
    }
}
