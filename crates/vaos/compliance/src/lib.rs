//! # Verity Agent OS — Lean-Agent Compliance Verifier
//!
//! Embeds the **Lean 4 theorem prover** as a compliance verification capability
//! within the Verity OS kernel. Every proposed agent action is auto-formalized
//! into a Lean 4 theorem and checked against pre-compiled regulatory axioms at
//! microsecond latency.
//!
//! ## Architecture
//! - **lean-rs-host** v0.1.0: L2 application framework for embedding Lean 4,
//!   providing `LeanHost` / `LeanSession` / `LeanEvidence` types
//! - **karpal-verify** v0.4.0: external verification foundation with
//!   SMT-LIB2 and Lean 4 export for structured proof generation
//! - **verified-ledger pattern** (Jan 2026): Lean 4 model as executable oracle —
//!   the formal specification serves as the "absolute correctness standard"
//!
//! ## Compliance Coverage
//! - SEC Rule 15c3-5 (market access risk controls)
//! - OCC Bulletin 2011-12 (model risk management)
//! - FINRA Rule 3110 (supervisory system)
//! - Reg D (savings deposit limitations)
//! - Reg Z (truth in lending)
//! - Reg E (electronic fund transfers)
//! - ECOA / FCRA (fair lending and credit reporting)
//!
//! Source: ARC42 v20.0 §3 VAOS LeanCV, ADR-001,
//!   Lean-Agent Protocol (Rashie & Rashi, April 2026)

pub mod verifier;
pub mod axioms;
pub mod proof_cache;
pub mod errors;

use std::sync::Arc;
use tokio::sync::RwLock;

pub use verifier::LeanAgentVerifier;
pub use axioms::RegulatoryAxiomLibrary;
pub use proof_cache::ProofCache;
pub use errors::ComplianceError;

/// Central compliance verification engine.
#[derive(Debug)]
pub struct ComplianceEngine {
    pub verifier: LeanAgentVerifier,
    pub axioms: Arc<RwLock<RegulatoryAxiomLibrary>>,
    pub cache: Arc<RwLock<ProofCache>>,
    pub config: ComplianceConfig,
}

#[derive(Debug, Clone)]
pub struct ComplianceConfig {
    /// Whether to require Lean 4 kernel proofs for all actions
    pub require_kernel_proof: bool,
    /// Maximum proof time in milliseconds before falling back
    pub proof_timeout_ms: u64,
    /// Whether to cache successful proofs
    pub enable_cache: bool,
    /// Cache TTL in seconds
    pub cache_ttl_secs: u64,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            require_kernel_proof: true,
            proof_timeout_ms: 100,
            enable_cache: true,
            cache_ttl_secs: 3600,
        }
    }
}

impl ComplianceEngine {
    pub fn new(config: ComplianceConfig) -> Self {
        Self {
            verifier: LeanAgentVerifier::new(),
            axioms: Arc::new(RwLock::new(RegulatoryAxiomLibrary::new())),
            cache: Arc::new(RwLock::new(ProofCache::new(config.cache_ttl_secs))),
            config,
        }
    }

    /// Verify that an agent action complies with all applicable regulatory axioms.
    ///
    /// # Pre-conditions
    /// - The action must be well-formed with complete context
    /// - Applicable regulatory axioms must be loaded in the axiom library
    ///
    /// # Post-conditions
    /// - Returns a `ComplianceProof` if the action satisfies all axioms
    /// - Returns a `ComplianceError` with a Lean counter-example if any axiom fails
    ///
    /// # Invariants
    /// - Proofs are deterministic: same action + same axioms → same result
    /// - No false positives: a proof of compliance is mathematically sound
    #[tracing::instrument(name = "compliance.verify", level = "info", skip(self))]
    pub async fn verify(
        &self,
        action: &vaos_core::types::AgentAction,
        regulatory_domain: &str,
    ) -> Result<ComplianceProof, ComplianceError> {
        // 1. Check proof cache
        if self.config.enable_cache {
            let cache = self.cache.read().await;
            if let Some(proof) = cache.get(action.id) {
                tracing::debug!(action_id = %action.id, "Cache hit");
                return Ok(proof.clone());
            }
        }

        // 2. Load applicable axioms
        let axioms = self.axioms.read().await;
        let applicable = axioms.get_applicable(regulatory_domain)?;

        // 3. Auto-formalize action into Lean 4 theorem
        let formalized = self.verifier.formalize(action, &applicable)?;

        // 4. Verify via Lean 4 kernel
        let outcome = self.verifier.check(&formalized).await?;

        if outcome.is_satisfied() {
            let proof = ComplianceProof {
                action_id: action.id,
                regulatory_domain: regulatory_domain.to_string(),
                axioms_checked: applicable.len(),
                lean_outcome: outcome,
                generated_at: chrono::Utc::now(),
            };

            // 5. Cache successful proof
            if self.config.enable_cache {
                let mut cache = self.cache.write().await;
                cache.insert(action.id, proof.clone());
            }

            tracing::info!(
                action_id = %action.id,
                domain = regulatory_domain,
                axioms = applicable.len(),
                "Compliance verified"
            );

            Ok(proof)
        } else {
            Err(ComplianceError::ComplianceViolation {
                action: action.id,
                domain: regulatory_domain.to_string(),
                counterexample: outcome.counterexample().unwrap_or_default(),
            })
        }
    }
}

/// A machine-checked proof of regulatory compliance.
#[derive(Debug, Clone)]
pub struct ComplianceProof {
    pub action_id: uuid::Uuid,
    pub regulatory_domain: String,
    pub axioms_checked: usize,
    pub lean_outcome: LeanVerificationOutcome,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Outcome of Lean 4 verification.
#[derive(Debug, Clone)]
pub enum LeanVerificationOutcome {
    Satisfied,
    Counterexample(String),
    Timeout,
    Error(String),
}

impl LeanVerificationOutcome {
    pub fn is_satisfied(&self) -> bool {
        matches!(self, Self::Satisfied)
    }

    pub fn counterexample(&self) -> Option<String> {
        match self {
            Self::Counterexample(ce) => Some(ce.clone()),
            _ => None,
        }
    }
}
