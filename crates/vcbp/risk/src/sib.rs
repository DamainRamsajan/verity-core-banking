use uuid::Uuid;
use super::types::FinancialNetwork;
use super::errors::RiskError;

/// Systemically Important Bank (SIB) identifier.
///
/// Uses network centrality and exposure analysis to identify
/// institutions whose failure would trigger systemic contagion.
pub struct SibIdentifier {
    threshold_bps: f64,
}

impl SibIdentifier {
    pub fn new(threshold_bps: f64) -> Self { Self { threshold_bps } }

    /// Identify SIBs based on network position and exposures.
    pub fn identify(&self, network: &FinancialNetwork) -> Result<Vec<Uuid>, RiskError> {
        let mut scores: Vec<(Uuid, f64)> = network.nodes.iter().map(|n| {
            let total_exposure: f64 = network.edges.iter()
                .filter(|e| e.source == n.id || e.target == n.id)
                .map(|e| e.amount.to_f64().unwrap_or(0.0))
                .sum();
            (n.id, total_exposure)
        }).collect();

        // Threshold: institutions with exposure score > threshold_bps of total
        let total: f64 = scores.iter().map(|(_, s)| s).sum();
        let threshold = total * (self.threshold_bps / 10_000.0);

        scores.retain(|(_, score)| *score > threshold);
        Ok(scores.into_iter().map(|(id, _)| id).collect())
    }
}
