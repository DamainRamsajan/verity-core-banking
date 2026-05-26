use serde::{Deserialize, Serialize};

/// A safety contract that every evolution must satisfy.
///
/// Contracts are expressed in first‑order logic and correspond to
/// the P1‑P8 safety invariants from the ASL specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyContract {
    pub contract_id: String,
    pub description: String,
    pub formal_spec: String,
    pub asl_principle: String,
    pub is_hard_constraint: bool,
}

impl SafetyContract {
    /// Build the full set of P1‑P8 safety contracts.
    pub fn all_invariants() -> Vec<Self> {
        vec![
            Self {
                contract_id: "P1".into(),
                description: "Corrigibility – human oversight hooks enforced by VM".into(),
                formal_spec: "∀ agent · shutdown_access(agent) ∧ ¬weakenable(shutdown_hook)".into(),
                asl_principle: "P1".into(),
                is_hard_constraint: true,
            },
            Self {
                contract_id: "P2".into(),
                description: "First‑class uncertainty – Uncertain<T> cannot be silently discarded".into(),
                formal_spec: "∀ v: Uncertain<T> · ¬silently_discardable(v)".into(),
                asl_principle: "P2".into(),
                is_hard_constraint: true,
            },
            Self {
                contract_id: "P3".into(),
                description: "Unforgeable capability tokens – no ambient authority".into(),
                formal_spec: "∀ action · requires_capability_token(action)".into(),
                asl_principle: "P3".into(),
                is_hard_constraint: true,
            },
            Self {
                contract_id: "P4".into(),
                description: "zkVM binary‑hash identity – self‑declared identity not trusted".into(),
                formal_spec: "∀ agent · identity(agent) = hash(binary(agent))".into(),
                asl_principle: "P4".into(),
                is_hard_constraint: true,
            },
            Self {
                contract_id: "P5".into(),
                description: "Session‑typed communication – deadlock freedom at compile time".into(),
                formal_spec: "∀ session · deadlock_free(session)".into(),
                asl_principle: "P5".into(),
                is_hard_constraint: true,
            },
            Self {
                contract_id: "P6".into(),
                description: "Merkle‑proofed provenance logs – append‑only, tamper‑evident".into(),
                formal_spec: "∀ entry · merkle_verified(entry)".into(),
                asl_principle: "P6".into(),
                is_hard_constraint: true,
            },
            Self {
                contract_id: "P7".into(),
                description: "Evolutionary memory with adversarial gating – multi‑party approval".into(),
                formal_spec: "∀ amendment · adversarial_simulated(amendment) ∧ two_party_approved(amendment)".into(),
                asl_principle: "P7".into(),
                is_hard_constraint: true,
            },
            Self {
                contract_id: "P8".into(),
                description: "Trust lattice with conjunctive capability closures".into(),
                formal_spec: "∀ composition · hypergraph_closure_checked(composition)".into(),
                asl_principle: "P8".into(),
                is_hard_constraint: true,
            },
        ]
    }
}
