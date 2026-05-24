
use tokio::sync::RwLock;
use super::types::{TransactionGraph, FraudScore};
use super::errors::FraudError;

pub struct GnnFraudEngine {
    stats: RwLock<FraudStats>,
}

#[derive(Debug, Default, Clone)]
pub struct FraudStats {
    pub graphs_processed: u64,
    pub alerts_generated: u64,
}

impl GnnFraudEngine {
    pub fn new() -> Self { Self { stats: RwLock::new(FraudStats::default()) } }

    pub async fn score_graph(&self, graph: &TransactionGraph) -> Result<FraudScore, FraudError> {
        let mut stats = self.stats.write().await;
        stats.graphs_processed += 1;
        let mut score: f64 = 0.0;
        let mut flags = Vec::new();
        for edge in &graph.edges {
            if edge.amount > 10_000.0 { score += 0.3; flags.push("large_amount".into()); }
        }
        if score > 0.7 { stats.alerts_generated += 1; }
        Ok(FraudScore { transaction_id: uuid::Uuid::new_v4(), score: score.min(1.0_f64), flags })
    }
}
