//! Verified invariants from the TLA+ specification.

/// An invariant verified by the TLA+ model checker.
#[derive(Debug, Clone)]
pub struct VerifiedInvariant {
    pub name: String,
    pub description: String,
    pub tla_expression: String,
    pub verified: bool,
}

impl VerifiedInvariant {
    /// Conservation of Value: the sum of all transaction entries must be zero.
    pub fn conservation_of_value() -> Self {
        Self {
            name: "ConservationOfValue".into(),
            description: "Σ entries = 0 for all transactions".into(),
            tla_expression: "∀ tx ∈ transactions: Σ tx.entries = 0".into(),
            verified: true,
        }
    }

    /// Merkle root consistency.
    pub fn merkle_root_consistency() -> Self {
        Self {
            name: "MerkleRootConsistency".into(),
            description: "Merkle root correctly reflects all transaction entries".into(),
            tla_expression: "root = MerkleHash(entries)".into(),
            verified: true,
        }
    }
}
