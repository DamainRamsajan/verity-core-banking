use std::sync::Arc;
use tokio::sync::RwLock;
use super::dsfl::DsflAggregator;
use super::defenses::FedSurrogate;
use super::ensemble::EnsembleBridge;
use super::errors::FlError;

pub struct FlMesh {
    dsfl: Arc<DsflAggregator>,
    fed_surrogate: Arc<FedSurrogate>,
    ensemble: Arc<EnsembleBridge>,
    stats: RwLock<FlStats>,
}

#[derive(Debug, Default, Clone)]
pub struct FlStats {
    pub rounds_completed: u64,
    pub backdoor_attempts_blocked: u64,
    pub models_unlearned: u64,
}

impl FlMesh {
    pub fn new(participant_count: usize) -> Self {
        Self {
            dsfl: Arc::new(DsflAggregator::new(participant_count)),
            fed_surrogate: Arc::new(FedSurrogate::new()),
            ensemble: Arc::new(EnsembleBridge::new()),
            stats: RwLock::new(FlStats::default()),
        }
    }

    pub async fn start_round(&self) -> Result<(), FlError> {
        let mut stats = self.stats.write().await;
        stats.rounds_completed += 1;
        tracing::info!(round = stats.rounds_completed, "FL round starting");
        Ok(())
    }
}
