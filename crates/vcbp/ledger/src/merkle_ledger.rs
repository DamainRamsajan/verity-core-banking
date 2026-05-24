use std::sync::Arc;
use tokio::sync::RwLock;
use rs_merkle::{MerkleTree, algorithms::Sha256};

use super::types::{Transaction, Balance, AccountId};
use super::event_store::EventStore;
use super::positions::PositionKeeper;
use super::proof::MerkleProof;
use super::tla_verifier::TlaVerifier;
use super::fim::FimIntegration;
use super::errors::LedgerError;

/// Central Merkle Double‑Entry Ledger
pub struct MerkleLedger {
    event_store: Arc<RwLock<EventStore>>,
    merkle_tree: Arc<RwLock<rs_merkle::MerkleTree<rs_merkle::algorithms::Sha256>>>,
    positions: Arc<RwLock<PositionKeeper>>,
    tla_verifier: TlaVerifier,
    fim: FimIntegration,
    config: LedgerConfig,
}

#[derive(Debug, Clone)]
pub struct LedgerConfig {
    pub enable_tla_runtime_check: bool,
    pub enable_fim: bool,
}

impl Default for LedgerConfig {
    fn default() -> Self {
        Self { enable_tla_runtime_check: true, enable_fim: true }
    }
}

impl MerkleLedger {
    pub fn new(config: LedgerConfig) -> Self {
        Self {
            event_store: Arc::new(RwLock::new(EventStore::new())),
            merkle_tree: Arc::new(RwLock::new(MerkleTree::new())),
            positions: Arc::new(RwLock::new(PositionKeeper::new())),
            tla_verifier: TlaVerifier::new(),
            fim: FimIntegration::new(),
            config,
        }
    }

    /// Append a transaction to the ledger.
    ///
    /// # Pre‑conditions
    /// - Transaction must balance (Σ entries = 0)
    /// - Runtime TLA+ checker samples the state space (if enabled)
    /// - FIM verifies no parameter mutation (if enabled)
    ///
    /// # Post‑conditions
    /// - Transaction appended to event store
    /// - Merkle proof returned
    /// - Positions updated in real‑time
    #[tracing::instrument(name = "ledger.append", level = "info", skip(self))]
    pub async fn append(&self, tx: Transaction) -> Result<MerkleProof, LedgerError> {
        // 1. Verify transaction balance
        tx.verify_conservation()?;

        // 2. Financial Invariants Monitor check
        if self.config.enable_fim {
            self.fim.check_transaction(&tx).await?;
        }

        // 3. TLA+ runtime sampling
        if self.config.enable_tla_runtime_check {
            self.tla_verifier.sample(&tx).await?;
        }

        // 4. Append to event store
        let mut store = self.event_store.write().await;
        store.append(&tx)?;

        // 5. Insert into Merkle tree
        let mut tree = self.merkle_tree.write().await;
        let leaf_hash = Sha256::hash(tx.hash().as_ref());
        tree.insert(leaf_hash);
        let proof = tree.proof(&[leaf_hash]);
        let merkle_root = tree.root().ok_or(LedgerError::MerkleTreeEmpty)?;

        // 6. Update positions
        let mut pos = self.positions.write().await;
        for entry in &tx.entries {
            pos.apply_entry(entry)?;
        }

        let merkle_proof = MerkleProof {
            transaction_hash: tx.hash(),
            merkle_root: merkle_root.try_into().unwrap_or([0u8; 32]),
            proof_hashes: proof.proof_hashes().iter().map(|h| *h).collect(),
            proof_index: proof.proof_hashes().len() as u64,
        };

        tracing::info!(
            tx_id = %tx.id,
            entries = tx.entries.len(),
            root = ?hex::encode(merkle_root),
            "Transaction appended"
        );

        Ok(merkle_proof)
    }

    /// Get the current balance of an account.
    pub async fn get_balance(&self, account_id: AccountId) -> Option<Balance> {
        let pos = self.positions.read().await;
        pos.get(account_id).cloned()
    }

    /// Prove inclusion of a transaction in the ledger.
    pub async fn prove(&self, tx_hash: &[u8; 32]) -> Option<MerkleProof> {
        let tree = self.merkle_tree.read().await;
        let leaf = Sha256::hash(tx_hash.as_ref());
        let proof = tree.proof(&[leaf]);
        let root = tree.root()?;
        Some(MerkleProof {
            transaction_hash: *tx_hash,
            merkle_root: root.try_into().unwrap_or([0u8; 32]),
            proof_hashes: proof.proof_hashes().iter().map(|h| *h).collect(),
            proof_index: proof.proof_hashes().len() as u64,
        })
    }
}
