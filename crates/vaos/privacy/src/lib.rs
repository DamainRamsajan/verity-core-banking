//! # Verity Agent OS — FHE/SMPC/DP Privacy Services
//!
//! Provides the **privacy triad** for the Verity Core Banking Platform:
//!
//! - **FHE** (Fully Homomorphic Encryption): computation on encrypted data
//!   without decryption, powered by Zama TFHE-rs (pure Rust, post-quantum safe)
//!   and Intel Heracles ASIC acceleration (5,000× speedup)
//! - **SMPC** (Secure Multi-Party Computation): joint computation across
//!   institutions without revealing private inputs, using Shamir secret sharing
//!   and threshold FROST signatures
//! - **DP** (Differential Privacy): formal mathematical privacy guarantees
//!   via calibrated noise injection, powered by OpenDP with epsilon tracking
//!
//! ## Performance Targets
//! - FHE: <50μs per transaction with Intel Heracles ASIC
//! - SMPC: <1MB bandwidth per signing party (Mithril scheme, ≤6 parties)
//! - DP: configurable ε budget with real-time consumption tracking
//!
//! Source: ARC42 v20.0 §3 VAOS Privacy Services, ADR-005

pub mod fhe;
pub mod mpc;
pub mod dp;
pub mod budget;
pub mod errors;

pub use fhe::FheService;
pub use mpc::MpcService;
pub use dp::DpService;
pub use budget::PrivacyBudget;
pub use errors::PrivacyError;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Central privacy engine.
#[derive(Debug)]
pub struct PrivacyEngine {
    pub fhe: FheService,
    pub mpc: MpcService,
    pub dp: DpService,
    /// Global privacy budget tracker
    pub budget: Arc<RwLock<PrivacyBudget>>,
    pub config: PrivacyConfig,
}

#[derive(Debug, Clone)]
pub struct PrivacyConfig {
    /// Default ε value for differential privacy
    pub default_epsilon: f64,
    /// Default δ value (failure probability)
    pub default_delta: f64,
    /// FHE accelerator type
    pub fhe_accelerator: FheAccelerator,
    /// Maximum SMPC parties
    pub max_mpc_parties: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FheAccelerator {
    /// Software-only (TFHE-rs CPU)
    Software,
    /// GPU-accelerated (HEonGPU)
    Gpu,
    /// Intel Heracles ASIC (5,000× speedup)
    IntelHeracles,
    /// Auto-detect best available
    Auto,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            default_epsilon: 1.0,
            default_delta: 1e-5,
            fhe_accelerator: FheAccelerator::Auto,
            max_mpc_parties: 6,
        }
    }
}

impl PrivacyEngine {
    pub fn new(config: PrivacyConfig) -> Self {
        Self {
            fhe: FheService::new(config.fhe_accelerator),
            mpc: MpcService::new(config.max_mpc_parties),
            dp: DpService::new(config.default_epsilon, config.default_delta),
            budget: Arc::new(RwLock::new(PrivacyBudget::new(
                config.default_epsilon,
                config.default_delta,
            ))),
            config,
        }
    }

    /// Check whether a DP query would exceed the remaining privacy budget.
    pub async fn check_dp_budget(
        &self,
        epsilon_cost: f64,
    ) -> Result<(), PrivacyError> {
        let budget = self.budget.read().await;
        if budget.remaining_epsilon < epsilon_cost {
            return Err(PrivacyError::DpBudgetExhausted {
                remaining: budget.remaining_epsilon,
                requested: epsilon_cost,
            });
        }
        Ok(())
    }

    /// Consume privacy budget for a DP query.
    pub async fn consume_dp_budget(
        &self,
        epsilon_cost: f64,
    ) -> Result<(), PrivacyError> {
        let mut budget = self.budget.write().await;
        if budget.remaining_epsilon < epsilon_cost {
            return Err(PrivacyError::DpBudgetExhausted {
                remaining: budget.remaining_epsilon,
                requested: epsilon_cost,
            });
        }
        budget.remaining_epsilon -= epsilon_cost;
        budget.total_consumed += epsilon_cost;
        Ok(())
    }
}
