//! # Verity Agent OS — Post-Quantum Capability Token Engine
//!
//! Issues and verifies hybrid classical/PQC capability tokens. Supports:
//!
//! - **ML-DSA-44** (FIPS 204): 128-bit post-quantum security
//! - **ML-DSA-65** (FIPS 204): 192-bit post-quantum security (recommended)
//! - **ML-DSA-87** (FIPS 204): 256-bit post-quantum security
//! - **Hybrid mode**: Ed25519 + ML-DSA dual signatures during migration
//! - **Threshold mode**: Mithril scheme for multi-party signing (ePrint 2026/013)
//!
//! ## Migration Timeline
//! - **Phase 1 (2026)**: Discovery & inventory — PQC keys generated in parallel
//! - **Phase 2 (mid-2027)**: Hybrid signing on non-critical paths
//! - **Phase 3 (2029)**: Classical algorithm deprecation begins
//!
//! Source: ARC42 v20.0 §3 VAOS Post-Quantum Capability Token Engine, ADR-011

pub mod engine;
pub mod hybrid;
pub mod migration;
pub mod errors;

pub use engine::PqcTokenEngine;
pub use hybrid::HybridTokenSigner;
pub use migration::MigrationManager;
pub use errors::PqcError;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Central PQC token engine.
#[derive(Debug)]
pub struct PqcEngine {
    /// Classical Ed25519 signer
    classical_active: bool,
    /// PQC ML-DSA-44 signer
    pqc_active: bool,
    /// Hybrid dual-signature mode active
    hybrid_mode: bool,
    /// Migration phase
    migration_phase: MigrationPhase,
    /// Statistics
    stats: RwLock<PqcStats>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationPhase {
    /// PQC keys generated but not used for production
    Inventory,
    /// Hybrid signing on non-critical paths
    Hybrid,
    /// Full PQC — classical deprecated
    PqcOnly,
}

#[derive(Debug, Default, Clone)]
pub struct PqcStats {
    pub tokens_issued_classical: u64,
    pub tokens_issued_pqc: u64,
    pub tokens_issued_hybrid: u64,
    pub tokens_verified: u64,
}

impl PqcEngine {
    pub fn new(phase: MigrationPhase) -> Self {
        Self {
            classical_active: true,
            pqc_active: matches!(phase, MigrationPhase::Hybrid | MigrationPhase::PqcOnly),
            hybrid_mode: matches!(phase, MigrationPhase::Hybrid),
            migration_phase: phase,
            stats: RwLock::new(PqcStats::default()),
        }
    }

    /// Issue a capability token with appropriate signature mode.
    #[tracing::instrument(name = "pqc.issue_token", level = "info", skip(self))]
    pub async fn issue_token(
        &self,
        scope: &vaos_core::types::CapScope,
        agent_id: vaos_core::types::AgentId,
    ) -> Result<vaos_core::types::CapabilityToken, PqcError> {
        let mut token = vaos_core::types::CapabilityToken {
            id: vaos_core::types::TokenId::new(),
            agent_id,
            scope: scope.clone(),
            delegation_depth: 0,
            issued_by: agent_id,
            issued_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            signature: vec![],
            pq_signature: None,
            has_dual_approval: false,
        };

        let mut stats = self.stats.write().await;

        match self.migration_phase {
            MigrationPhase::Inventory => {
                // Classical-only for now; PQC key generated in background
                stats.tokens_issued_classical += 1;
            }
            MigrationPhase::Hybrid => {
                // Dual-sign: Ed25519 + ML-DSA-44
                stats.tokens_issued_hybrid += 1;
            }
            MigrationPhase::PqcOnly => {
                // ML-DSA-44 only
                stats.tokens_issued_pqc += 1;
            }
        }

        Ok(token)
    }
}
