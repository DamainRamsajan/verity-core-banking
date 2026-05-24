use tokio::sync::RwLock;
use uuid::Uuid;
use super::types::{FinancialNetwork, ContagionResult};
use super::errors::RiskError;

pub struct SystemicRiskEngine {
    stats: RwLock<RiskStats>,
}

#[derive(Debug, Default, Clone)]
pub struct RiskStats { pub simulations_run: u64 }

impl SystemicRiskEngine {
    pub fn new() -> Self { Self { stats: RwLock::new(RiskStats::default()) } }

    pub async fn simulate_cascade(
        &self,
        network: &FinancialNetwork,
        initial_shock: Uuid,
    ) -> Result<ContagionResult, RiskError> {
        let mut stats = self.stats.write().await;
        stats.simulations_run += 1;
        let defaulted = network.nodes.iter().filter(|n| n.id == initial_shock || n.is_sib).map(|n| n.id).collect();
        Ok(ContagionResult {
            initial_shock,
            defaulted_institutions: defaulted,
            total_losses: rust_decimal::Decimal::ZERO,
            cascade_rounds: 1,
            systemic_risk_score: 0.0,
        })
    }
}