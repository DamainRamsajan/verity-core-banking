use rs_merkle::{MerkleTree, Hasher};
use uuid::Uuid;
use super::errors::LineageError;

#[derive(Clone)]
struct Blake3Hasher;

impl Hasher for Blake3Hasher {
    type Hash = [u8; 32];
    fn hash(data: &[u8]) -> Self::Hash { *blake3::hash(data).as_bytes() }
}

pub struct MerkleLog {
    tree: MerkleTree<Blake3Hasher>,
    entries: Vec<Uuid>,
}

impl MerkleLog {
    pub fn new() -> Self { Self { tree: MerkleTree::new(), entries: Vec::new() } }

    pub fn insert(&mut self, entry_id: Uuid) -> Result<(), LineageError> {
        let hash = Blake3Hasher::hash(entry_id.as_bytes());
        self.tree.insert(hash);
        self.entries.push(entry_id);
        Ok(())
    }
}
