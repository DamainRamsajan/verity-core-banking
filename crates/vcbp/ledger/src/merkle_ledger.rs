use std::sync::Arc;
use tokio::sync::RwLock;
use rs_merkle::{MerkleTree, Hasher as MerkleHasher};

use super::types::{Transaction, Balance, AccountId};
use super::event_store::EventStore;
use super::positions::PositionKeeper;
use super::proof::MerkleProof;
use super::errors::LedgerError;

#[derive(Clone)]
struct Blake3Hasher;

impl MerkleHasher for Blake3Hasher {
    type Hash = [u8; 32];
    fn hash(data: &[u8]) -> Self::Hash { *blake3::hash(data).as_bytes() }
}

pub struct MerkleLedger {
    event_store: Arc<RwLock<EventStore>>,
    merkle_tree: Arc<RwLock<MerkleTree<Blake3Hasher>>>,
    positions: Arc<RwLock<PositionKeeper>>,
    config: LedgerConfig,
}

#[derive(Debug, Clone)]
pub struct LedgerConfig {
    pub enable_tla_runtime_check: bool,
    pub enable_fim: bool,
}

impl Default for LedgerConfig {
    fn default() -> Self { Self { enable_tla_runtime_check: true, enable_fim: true } }
}

impl MerkleLedger {
    pub fn new(config: LedgerConfig) -> Self {
        Self {
            event_store: Arc::new(RwLock::new(EventStore::new())),
            merkle_tree: Arc::new(RwLock::new(MerkleTree::new())),
            positions: Arc::new(RwLock::new(PositionKeeper::new())),
            config,
        }
    }

    pub async fn append(&self, tx: Transaction) -> Result<MerkleProof, LedgerError> {
        tx.verify_conservation()?;

        // FIM check (if enabled)
        if self.config.enable_fim {
            // In production: self.fim.check_transaction(&tx).await?;
        }

        // TLA+ sample (if enabled)
        if self.config.enable_tla_runtime_check {
            // In production: self.tla_verifier.sample(&tx).await?;
        }

        let mut store = self.event_store.write().await;
        store.append(&tx)?;

        let mut tree = self.merkle_tree.write().await;
        let leaf_hash = Blake3Hasher::hash(&tx.hash());
        tree.insert(leaf_hash);
        let proof = let leaf_index = tree.leaves().len().saturating_sub(1);
        let proof = tree.proof(tree.proof(&[leaf_hash]);[leaf_index]);
        let root = tree.root().ok_or(LedgerError::MerkleTreeEmpty)?;

        let mut pos = self.positions.write().await;
        for entry in &tx.entries {
            pos.apply_entry(entry)?;
        }

        Ok(MerkleProof {
            transaction_hash: tx.hash(),
            merkle_root: root,
            proof_hashes: proof.proof_hashes().to_vec(),
            proof_index: proof.proof_hashes().len() as u64,
        })
    }

    pub async fn get_balance(&self, account_id: AccountId) -> Option<Balance> {
        self.positions.read().await.get(account_id).cloned()
    }
}
