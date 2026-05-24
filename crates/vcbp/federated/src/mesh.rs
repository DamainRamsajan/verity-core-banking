use tokio::sync::RwLock;
use super::errors::FlError;

#[allow(dead_code)]
pub struct FlMesh {
    participant_count: usize,
    stats: RwLock<FlStats>,
}

#[derive(Debug, Default, Clone)]
pub struct FlStats { pub rounds_completed: u64 }

impl FlMesh {
    pub fn new(participant_count: usize) -> Self {
        Self { participant_count, stats: RwLock::new(FlStats::default()) }
    }

    pub async fn start_round(&self) -> Result<(), FlError> {
        let mut stats = self.stats.write().await;
        stats.rounds_completed += 1;
        Ok(())
    }
}
