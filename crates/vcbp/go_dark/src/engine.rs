use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::{TradeIntent, ZkTradeProof, DisclosureLevel};
use super::prover::ZkComplianceProver;
use super::errors::GoDarkError;

/// GoDark ZK institutional trading engine.
///
/// Provides confidential trading with zero-knowledge compliance proofs.
pub struct GoDarkEngine {
    prover: Arc<ZkComplianceProver>,
    config: GoDarkConfig,
    stats: RwLock<GoDarkStats>,
}

#[derive(Debug, Clone)]
pub struct GoDarkConfig {
    pub proof_system: String,
    pub default_disclosure: DisclosureLevel,
    pub min_trade_value: rust_decimal::Decimal,
}

impl Default for GoDarkConfig {
    fn default() -> Self {
        Self {
            proof_system: "groth16".into(),
            default_disclosure: DisclosureLevel::ProofOnly,
            min_trade_value: rust_decimal::Decimal::new(10_000, 0),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GoDarkStats {
    pub trades_executed: u64,
    pub zk_proofs_generated: u64,
    pub compliance_checks: u64,
}

impl GoDarkEngine {
    pub fn new(config: GoDarkConfig) -> Self {
        Self {
            prover: Arc::new(ZkComplianceProver::new()),
            config,
            stats: RwLock::new(GoDarkStats::default()),
        }
    }

    /// Execute a confidential institutional trade with ZK compliance proof.
    #[tracing::instrument(name = "godark.execute_trade", level = "info", skip(self))]
    pub async fn execute_trade(
        &self,
        intent: &TradeIntent,
    ) -> Result<ZkTradeProof, GoDarkError> {
        let mut stats = self.stats.write().await;
        stats.trades_executed += 1;
        stats.compliance_checks += intent.compliance_checks.len() as u64;

        // Verify all compliance checks passed
        for check in &intent.compliance_checks {
            if !check.passed {
                return Err(GoDarkError::ComplianceCheckFailed(check.check_type.clone()));
            }
        }

        // Generate ZK-SNARK proof of compliance
        let proof = self.prover.generate_proof(intent).await?;
        stats.zk_proofs_generated += 1;

        tracing::info!(
            trade_id = %intent.trade_id,
            asset_pair = %intent.asset_pair,
            "ZK trade proof generated"
        );

        Ok(proof)
    }

    /// Verify a ZK compliance proof without seeing the underlying trade.
    #[tracing::instrument(name = "godark.verify_proof", level = "debug", skip(self))]
    pub async fn verify_proof(
        &self,
        proof: &ZkTradeProof,
    ) -> Result<bool, GoDarkError> {
        self.prover.verify_proof(proof).await
    }
}
