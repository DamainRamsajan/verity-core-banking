use std::sync::Arc;
use tokio::sync::RwLock;
use super::types::{EdgeConfig, OfflineTransaction, SyncStatus};
use super::reservation::ReservationPool;
use super::errors::EdgeError;

#[allow(dead_code)]
pub struct EdgeRuntime {
    config: EdgeConfig,
    reservation: Arc<RwLock<ReservationPool>>,
    offline_tx_log: RwLock<Vec<OfflineTransaction>>,
    status: RwLock<SyncStatus>,
}

impl EdgeRuntime {
    pub fn new(config: EdgeConfig) -> Self {
        Self {
            reservation: Arc::new(RwLock::new(ReservationPool::new(config.reservation_limit))),
            offline_tx_log: RwLock::new(Vec::new()),
            status: RwLock::new(SyncStatus::Online),
            config,
        }
    }

    pub async fn process_transaction(&self, tx: OfflineTransaction) -> Result<(), EdgeError> {
        let mut reservation = self.reservation.write().await;
        reservation.consume(tx.amount)?;
        self.offline_tx_log.write().await.push(tx);
        Ok(())
    }

    pub async fn status(&self) -> SyncStatus { *self.status.read().await }
}
