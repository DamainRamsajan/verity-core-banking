use async_trait::async_trait;
use super::super::rail::{PaymentRail, Payment, PaymentReceipt, PaymentStatus, RailType};
use super::super::errors::PaymentError;

pub struct SwiftBlockchainRail;

impl SwiftBlockchainRail { pub fn new() -> Self { Self } }

#[async_trait]
impl PaymentRail for SwiftBlockchainRail {
    async fn send(&self, payment: &Payment) -> Result<PaymentReceipt, PaymentError> {
        Ok(PaymentReceipt {
            payment_id: payment.id,
            rail_reference: format!("SWIFT-{}", uuid::Uuid::new_v4()),
            status: PaymentStatus::Accepted,
            timestamp: chrono::Utc::now(),
            fee: None,
        })
    }
    fn rail_type(&self) -> RailType { RailType::Swift }
    fn is_available(&self) -> bool { true }
    fn supports(&self, _c: &str, _a: rust_decimal::Decimal) -> bool { true }
}
