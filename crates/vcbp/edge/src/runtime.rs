use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{EdgeConfig, OfflineTransaction, SyncStatus};
use super::reservation::ReservationPool;
use super::mesh::MeshSync;
use super::errors::EdgeError;

/// Lightweight edge banking runtime.
///
/// Processes transactions locally during connectivity loss, using
/// pre‑reserved liquidity. Syncs via cryptographic mesh on reconnection.
pub struct EdgeRuntime {
    config: EdgeConfig,
    reservation: Arc<RwLock<ReservationPool>>,
    mesh: Arc<MeshSync>,
    offline_tx_log: RwLock<Vec<OfflineTransaction>>,
    status: RwLock<SyncStatus>,
    stats: RwLock<EdgeStats>,
}

#[derive(Debug, Default, Clone)]
pub struct EdgeStats {
    pub offline_transactions: u64,
    pub syncs_completed: u64,
    pub conflicts_resolved: u64,
    pub total_offline_value: rust_decimal::Decimal,
}

impl EdgeRuntime {
    pub fn new(config: EdgeConfig) -> Self {
        Self {
            reservation: Arc::new(RwLock::new(ReservationPool::new(config.reservation_limit))),
            mesh: Arc::new(MeshSync::new()),
            offline_tx_log: RwLock::new(Vec::new()),
            status: RwLock::new(SyncStatus::Online),
            stats: RwLock::new(EdgeStats::default()),
            config,
        }
    }

    /// Process a transaction while potentially offline.
    #[tracing::instrument(name = "edge.process", level = "info", skip(self))]
    pub async fn process_transaction(
        &self,
        tx: OfflineTransaction,
    ) -> Result<(), EdgeError> {
        let mut reservation = self.reservation.write().await;
        let mut stats = self.stats.write().await;

        // Check against reservation balance (Crunchfish pattern)
        reservation.consume(tx.amount)?;

        // Log for later sync
        self.offline_tx_log.write().await.push(tx.clone());

        stats.offline_transactions += 1;
        stats.total_offline_value += tx.amount;

        tracing::info!(
            tx_id = %tx.id,
            amount = ?tx.amount,
            "Offline transaction processed"
        );

        Ok(())
    }

    /// Trigger mesh synchronisation with central ledger.
    pub async fn sync(&self) -> Result<(), EdgeError> {
        *self.status.write().await = SyncStatus::Syncing;

        let txs = self.offline_tx_log.read().await.clone();
        self.mesh.sync_transactions(&txs).await?;

        let mut stats = self.stats.write().await;
        stats.syncs_completed += 1;

        *self.status.write().await = SyncStatus::Online;
        tracing::info!(txs = txs.len(), "Mesh sync completed");

        Ok(())
    }

    pub async fn status(&self) -> SyncStatus { *self.status.read().await }
}
