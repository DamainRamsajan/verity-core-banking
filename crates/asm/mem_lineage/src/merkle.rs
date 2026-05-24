use rs_merkle::{MerkleTree, algorithms::Sha256};
use uuid::Uuid;
use super::types::DerivationEdge;
use super::errors::LineageError;

pub struct MerkleLog { tree: MerkleTree<Sha256>, entries: Vec<[u8; 32]> }

pub struct MerkleProofResult { pub leaf_hash: [u8; 32], pub proof_hashes: Vec<[u8; 32]> }

impl MerkleLog {
    pub fn new() -> Self { Self { tree: MerkleTree::new(), entries: Vec::new() } }

    pub fn insert(&mut self, entry_id: Uuid, content: &serde_json::Value, edges: &[DerivationEdge]) -> Result<MerkleProofResult, LineageError> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(entry_id.as_bytes());
        hasher.update(&serde_json::to_vec(content).unwrap_or_default());
        for edge in edges {
            hasher.update(edge.parent_entry_id.as_bytes());
        }
        let hash = *hasher.finalize().as_bytes();
        self.entries.push(hash);
        self.tree.insert(Sha256::hash(&hash));
        let proof = self.tree.proof(&[Sha256::hash(&hash)]);
        Ok(MerkleProofResult {
            leaf_hash: hash,
            proof_hashes: proof.proof_hashes().iter().map(|h| *h).collect(),
        })
    }
}
