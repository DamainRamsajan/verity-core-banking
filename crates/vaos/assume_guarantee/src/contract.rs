//! Layer contract definitions for the three-layer assume-guarantee contract.
//!
//! Source: ARC42 v20.0 §3 VAOS AGC

use serde::{Deserialize, Serialize};

/// A contract between architectural layers.
///
/// Each layer ASSUMES something from the layer below it,
/// and GUARANTEES something to the layer above it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerContract {
    pub name: String,
    pub layer: ContractLayer,
    pub assumes: Vec<String>,
    pub guarantees: Vec<String>,
    pub invariants: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractLayer {
    Asl,       // ASL compile-time safety
    Kernel,    // Capability microkernel
    VeriChain, // On-chain provenance
}

impl LayerContract {
    /// ASL layer: assumes kernel enforces capability discipline,
    /// guarantees compile-time safety invariants.
    pub fn asl_layer() -> Self {
        Self {
            name: "ASL Compile-Time Contract".into(),
            layer: ContractLayer::Asl,
            assumes: vec![
                "kernel_enforces_capability_discipline".into(),
                "kernel_prevents_privilege_escalation".into(),
            ],
            guarantees: vec![
                "asl_compile_time_safety_invariants".into(),
                "products_satisfy_regulatory_constraints".into(),
                "agents_are_corrigible".into(),
            ],
            invariants: vec![
                "no_agent_self_escalates_stratum".into(),
                "uncertainty_tracking_cannot_be_silently_discarded".into(),
            ],
        }
    }

    /// Kernel layer: assumes ASL invariants hold, guarantees
    /// capability-valid state transitions.
    pub fn kernel_layer() -> Self {
        Self {
            name: "Kernel Capability Contract".into(),
            layer: ContractLayer::Kernel,
            assumes: vec![
                "asl_invariants_preserved".into(),
                "agents_compiled_with_safety_proofs".into(),
            ],
            guarantees: vec![
                "all_state_transitions_are_capability_valid".into(),
                "provenance_log_is_append_only".into(),
                "trust_lattice_closure_computed_before_composition".into(),
            ],
            invariants: vec![
                "conservation_of_value".into(),
                "no_privilege_escalation".into(),
            ],
        }
    }

    /// VeriChain layer: assumes kernel provides valid provenance,
    /// guarantees tamper-evident on-chain audit trail.
    pub fn verichain_layer() -> Self {
        Self {
            name: "VeriChain Provenance Contract".into(),
            layer: ContractLayer::VeriChain,
            assumes: vec![
                "kernel_provides_valid_provenance_capsules".into(),
                "capability_tokens_are_unforgeable".into(),
            ],
            guarantees: vec![
                "audit_trail_is_tamper_evident".into(),
                "on_chain_anchoring_is_immutable".into(),
                "regulatory_evidence_is_cryptographically_verifiable".into(),
            ],
            invariants: vec![
                "merkle_root_consistency".into(),
                "scitt_anchoring_integrity".into(),
            ],
        }
    }
}
