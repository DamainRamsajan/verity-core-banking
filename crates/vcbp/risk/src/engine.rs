use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{FinancialNetwork, ContagionResult, RiskChannel};
use super::cascade::GaiKapadiaSimulator;
use super::sib::SibIdentifier;
use super::errors::RiskError;

/// Central systemic risk engine.
///
/// Integrates the IMF/ECB multilayer contagion model with the
/// Gai-Kapadia cascade simulation framework.
pub struct SystemicRiskEngine {
    simulator: GaiKapadiaSimulator,
    sib_identifier: SibIdentifier,
    config: RiskConfig,
    stats: RwLock<RiskStats>,
}

#[derive(Debug, Clone)]
pub struct RiskConfig {
    pub loss_given_default: f64,
    pub fire_sale_discount: f64,
    pub funding_rollover_probability: f64,
    pub sib_threshold_bps: f64,
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self { loss_given_default: 0.60, fire_sale_discount: 0.30, funding_rollover_probability: 0.15, sib_threshold_bps: 100.0 }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RiskStats {
    pub simulations_run: u64,
    pub sibs_identified: u64,
    pub worst_case_losses: rust_decimal::Decimal,
}

impl SystemicRiskEngine {
    pub fn new(config: RiskConfig) -> Self {
        Self {
            simulator: GaiKapadiaSimulator::new(config.loss_given_default, config.fire_sale_discount, config.funding_rollover_probability),
            sib_identifier: SibIdentifier::new(config.sib_threshold_bps),
            config,
            stats: RwLock::new(RiskStats::default()),
        }
    }

    /// Simulate a default cascade triggered by an initial shock.
    #[tracing::instrument(name = "risk.simulate", level = "info", skip(self))]
    pub async fn simulate_cascade(
        &self,
        network: &FinancialNetwork,
        initial_shock: uuid::Uuid,
    ) -> Result<ContagionResult, RiskError> {
        let mut stats = self.stats.write().await;
        stats.simulations_run += 1;

        let result = self.simulator.run(network, initial_shock)?;
        if result.total_losses > stats.worst_case_losses {
            stats.worst_case_losses = result.total_losses;
        }

        tracing::info!(
            defaults = result.defaulted_institutions.len(),
            total_losses = ?result.total_losses,
            cascade_rounds = result.cascade_rounds,
            "Contagion simulation complete"
        );

        Ok(result)
    }

    /// Identify Systemically Important Banks (SIBs) in the network.
    #[tracing::instrument(name = "risk.identify_sibs", level = "info", skip(self))]
    pub async fn identify_sibs(
        &self,
        network: &FinancialNetwork,
    ) -> Result<Vec<uuid::Uuid>, RiskError> {
        let mut stats = self.stats.write().await;
        let sibs = self.sib_identifier.identify(network)?;
        stats.sibs_identified = sibs.len() as u64;
        Ok(sibs)
    }
}
