#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub transaction_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub proof_hashes: Vec<[u8; 32]>,
    pub proof_index: u64,
}
