//! # Verity Agent OS — Assume-Guarantee Contract Monitor
//!
//! Continuously monitors the three-layer assume-guarantee contract:
//!
//! **Layer 1 (ASL)**: ASSUMES the kernel enforces capability discipline
//! **Layer 2 (Kernel)**: GUARANTEES to VeriChain that all state transitions
//!   are capability-valid
//! **Layer 3 (VeriChain)**: GUARANTEES to the world that the audit trail
//!   is tamper-evident
//!
//! ## Architecture
//! - **Formal Policy Enforcement** (May 8, 2026): aspect-oriented programming
//!   with assume/guarantee contracts and reference monitor
//! - **modelator** v0.2.1: runs system under test against TLA+ traces
//! - **GR(1)**: liveness properties as implication of conjoined recurrence
//!
//! Source: ARC42 v20.0 §3 VAOS Assume-Guarantee Contract Monitor

pub mod contract;
pub mod monitor;
pub mod errors;

pub use contract::LayerContract;
pub use monitor::ContractMonitor;
pub use errors::ContractError;

/// The central contract monitoring engine.
#[derive(Debug)]
pub struct AssumeGuaranteeEngine {
    contracts: Vec<LayerContract>,
    monitor: ContractMonitor,
    stats: ContractStats,
}

#[derive(Debug, Default)]
pub struct ContractStats {
    pub checks_performed: u64,
    pub violations_detected: u64,
    pub last_violation: Option<chrono::DateTime<chrono::Utc>>,
}

impl AssumeGuaranteeEngine {
    pub fn new() -> Self {
        let contracts = vec![
            LayerContract::asl_layer(),
            LayerContract::kernel_layer(),
            LayerContract::verichain_layer(),
        ];

        Self {
            monitor: ContractMonitor::new(),
            contracts,
            stats: ContractStats::default(),
        }
    }

    /// Monitor that all three layer contracts are being satisfied.
    /// Returns Ok if all layers are consistent, or ContractBreach if
    /// any layer's assumptions are violated.
    #[tracing::instrument(name = "ag.monitor", level = "debug", skip(self))]
    pub async fn check_all(&mut self) -> Result<(), ContractError> {
        self.stats.checks_performed += 1;

        for contract in &self.contracts {
            self.monitor.check_contract(contract)?;
        }

        Ok(())
    }

    pub fn stats(&self) -> &ContractStats {
        &self.stats
    }
}

impl Default for AssumeGuaranteeEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_contracts_initialized() {
        let engine = AssumeGuaranteeEngine::new();
        assert_eq!(engine.contracts.len(), 3);
    }

    #[tokio::test]
    async fn test_monitor_initial_check() {
        let mut engine = AssumeGuaranteeEngine::new();
        let result = engine.check_all().await;
        assert!(result.is_ok());
    }
}
