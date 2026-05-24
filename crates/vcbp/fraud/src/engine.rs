use std::sync::Arc;
use tokio::sync::RwLock;
use super::types::{TransactionGraph, FraudScore, FraudAlert};
use super::models::ScafdsModel;
use super::trilemma::TrilemmaDetector;
use super::errors::FraudError;

pub struct GnnFraudEngine {
    scafds: ScafdsModel,
    trilemma: TrilemmaDetector,
    stats: RwLock<FraudStats>,
}

#[derive(Debug, Default, Clone)]
pub struct FraudStats {
    pub graphs_processed: u64,
    pub alerts_generated: u64,
    pub avg_inference_ms: f64,
}

impl GnnFraudEngine {
    pub fn new() -> Self {
        Self {
            scafds: ScafdsModel::new(),
            trilemma: TrilemmaDetector::new(),
            stats: RwLock::new(FraudStats::default()),
        }
    }

    #[tracing::instrument(name = "fraud.score", level = "info", skip(self))]
    pub async fn score_graph(&self, graph: &TransactionGraph) -> Result<FraudScore, FraudError> {
        let mut stats = self.stats.write().await;
        stats.graphs_processed += 1;

        let scafds_score = self.scafds.predict(graph)?;
        let trilemma_hit = self.trilemma.detect_centralized_cashout(graph)?;

        let mut model_scores = std::collections::HashMap::new();
        model_scores.insert("scafds".into(), scafds_score);

        let score = if trilemma_hit { 0.99 } else { scafds_score };

        Ok(FraudScore {
            transaction_id: uuid::Uuid::new_v4(),
            score,
            model_scores,
            flags: if trilemma_hit { vec!["centralized_cashout".into()] } else { vec![] },
        })
    }
}
