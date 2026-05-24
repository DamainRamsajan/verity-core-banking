use std::collections::{HashSet, HashMap};
use uuid::Uuid;

use super::types::{FinancialNetwork, ContagionResult, RiskChannel};
use super::errors::RiskError;

/// Gai-Kapadia default cascade simulator with IMF/ECB extensions.
///
/// Implements the five-channel multilayer propagation model.
pub struct GaiKapadiaSimulator {
    loss_given_default: f64,
    fire_sale_discount: f64,
    funding_rollover_probability: f64,
}

impl GaiKapadiaSimulator {
    pub fn new(lgd: f64, fire_sale: f64, rollover: f64) -> Self {
        Self { loss_given_default: lgd, fire_sale_discount: fire_sale, funding_rollover_probability: rollover }
    }

    /// Run the cascade simulation from an initial default.
    pub fn run(
        &self,
        network: &FinancialNetwork,
        initial_shock: Uuid,
    ) -> Result<ContagionResult, RiskError> {
        let mut defaulted: HashSet<Uuid> = HashSet::new();
        let mut newly_defaulted: Vec<Uuid> = vec![initial_shock];
        let mut total_losses = rust_decimal::Decimal::ZERO;
        let mut cascade_rounds = 0;
        let max_rounds = 100;

        // Capital buffer tracking
        let mut capital: HashMap<Uuid, rust_decimal::Decimal> = network.nodes
            .iter()
            .map(|n| (n.id, n.tier1_capital))
            .collect();

        while !newly_defaulted.is_empty() && cascade_rounds < max_rounds {
            defaulted.extend(newly_defaulted.drain(..));
            cascade_rounds += 1;

            // Propagate losses through all five channels
            for edge in &network.edges {
                if defaulted.contains(&edge.source) && !defaulted.contains(&edge.target) {
                    let loss = self.compute_loss(edge);
                    total_losses += loss;

                    let remaining = capital.entry(edge.target).or_default();
                    if loss > *remaining {
                        *remaining = rust_decimal::Decimal::ZERO;
                        newly_defaulted.push(edge.target);
                    } else {
                        *remaining -= loss;
                    }
                }
            }
        }

        let systemic_risk_score = defaulted.len() as f64 / network.nodes.len().max(1) as f64;

        Ok(ContagionResult {
            initial_shock,
            defaulted_institutions: defaulted.into_iter().collect(),
            total_losses,
            cascade_rounds,
            capital_depletion_pct: systemic_risk_score * 100.0,
            systemic_risk_score,
        })
    }

    fn compute_loss(&self, edge: &super::types::ExposureEdge) -> rust_decimal::Decimal {
        let base_loss = edge.amount * rust_decimal::Decimal::from_f64_retain(self.loss_given_default).unwrap_or(rust_decimal::Decimal::ZERO);
        match edge.channel {
            RiskChannel::FireSale => base_loss * rust_decimal::Decimal::from_f64_retain(1.0 + self.fire_sale_discount).unwrap_or(base_loss),
            RiskChannel::FundingRollover => {
                if rand::random::<f64>() < self.funding_rollover_probability { base_loss } else { rust_decimal::Decimal::ZERO }
            }
            RiskChannel::NbfiAmplification => base_loss * rust_decimal::Decimal::new(15, 1), // 1.5× multiplier
            _ => base_loss,
        }
    }
}
