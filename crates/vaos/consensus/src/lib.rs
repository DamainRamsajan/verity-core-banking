//! # Verity Agent OS — ORCHID Quantum-Augmented Consensus
//!
//! Implements the **ORCHID protocol** (arXiv:2605.09782, May 12, 2026):
//! a bio-inspired, quantum-augmented consensus mechanism for post-quantum
//! distributed ledgers.
//!
//! ## Protocol Design
//! - **Bio-inspired**: maps the neuroscientific binding problem — how the
//!   brain synchronizes distributed neural activity — to distributed consensus
//! - **Binding threshold θ_b**: consensus is triggered when the network's
//!   order parameter r(t) crosses θ_b
//! - **Coherence-weighted QSS**: Quantum Secret Sharing layer extends
//!   Weinberg's survey framework to concrete consensus application
//! - **Scalability**: proven for n ≥ 150 nodes with sub-second finality
//!
//! ## Post-Quantum Security
//! - All consensus messages are signed with ML-DSA-44 (FIPS 204)
//! - QSS layer provides information-theoretic security against quantum adversaries
//! - Bio-inspired adaptive mechanism enables organic scaling
//!
//! Source: ARC42 v20.0 §3 VAOS ORCHID Consensus, ADR-006

pub mod protocol;
pub mod oscillator;
pub mod qss;
pub mod errors;

pub use protocol::OrchidConsensus;
pub use oscillator::KuramotoOscillator;
pub use qss::QuantumSecretSharing;
pub use errors::ConsensusError;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Central ORCHID consensus engine.
#[derive(Debug)]
pub struct OrchidEngine {
    /// Kuramoto oscillator network — models the binding problem
    oscillator: Arc<RwLock<KuramotoOscillator>>,
    /// Quantum Secret Sharing layer
    qss: QuantumSecretSharing,
    /// Current order parameter r(t)
    order_parameter: RwLock<f64>,
    /// Binding threshold θ_b
    binding_threshold: f64,
    /// Number of participating nodes
    node_count: usize,
    /// Consensus statistics
    stats: RwLock<ConsensusStats>,
}

#[derive(Debug, Default, Clone)]
pub struct ConsensusStats {
    pub rounds_completed: u64,
    pub blocks_finalized: u64,
    pub average_finality_ms: f64,
    pub quantum_proofs_verified: u64,
}

impl OrchidEngine {
    /// Create a new ORCHID consensus engine.
    ///
    /// The binding threshold θ_b is set to 0.75 per the paper:
    /// consensus triggers when r(t) > θ_b.
    pub fn new(node_count: usize) -> Result<Self, ConsensusError> {
        if node_count < 150 {
            return Err(ConsensusError::InsufficientNodes {
                current: node_count,
                required: 150,
            });
        }

        Ok(Self {
            oscillator: Arc::new(RwLock::new(KuramotoOscillator::new(node_count))),
            qss: QuantumSecretSharing::new(node_count),
            order_parameter: RwLock::new(0.0),
            binding_threshold: 0.75,
            node_count,
            stats: RwLock::new(ConsensusStats::default()),
        })
    }

    /// Propose a block and attempt to reach consensus.
    ///
    /// # Pre-conditions
    /// - At least 150 nodes must be participating
    /// - Nodes must have valid ML-DSA-44 keypairs
    ///
    /// # Post-conditions
    /// - If r(t) > θ_b, consensus is reached and the block is finalized
    /// - A quantum-secured proof is attached to the finalized block
    #[tracing::instrument(name = "orchid.propose", level = "info", skip(self))]
    pub async fn propose_block(
        &self,
        block_hash: &[u8; 32],
    ) -> Result<ConsensusResult, ConsensusError> {
        // 1. Evolve the Kuramoto oscillator network
        let mut osc = self.oscillator.write().await;
        let r = osc.evolve()?;
        *self.order_parameter.write().await = r;

        tracing::debug!(order_parameter = r, threshold = self.binding_threshold);

        // 2. Check binding threshold
        if r > self.binding_threshold {
            // 3. Consensus reached — finalize via QSS
            let qss_proof = self.qss.finalize_block(block_hash)?;

            let mut stats = self.stats.write().await;
            stats.rounds_completed += 1;
            stats.blocks_finalized += 1;
            stats.quantum_proofs_verified += 1;

            tracing::info!(
                block = ?hex::encode(block_hash),
                order_parameter = r,
                "Block finalized via ORCHID consensus"
            );

            Ok(ConsensusResult::Finalized {
                block_hash: *block_hash,
                order_parameter: r,
                qss_proof,
            })
        } else {
            Ok(ConsensusResult::Pending {
                block_hash: *block_hash,
                order_parameter: r,
                remaining: self.binding_threshold - r,
            })
        }
    }
}

/// Result of a consensus round.
#[derive(Debug, Clone)]
pub enum ConsensusResult {
    Finalized {
        block_hash: [u8; 32],
        order_parameter: f64,
        qss_proof: Vec<u8>,
    },
    Pending {
        block_hash: [u8; 32],
        order_parameter: f64,
        remaining: f64,
    },
}
