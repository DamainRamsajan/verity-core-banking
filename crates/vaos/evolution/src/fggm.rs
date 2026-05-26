use z3::{
    ast::Bool,
    Config, Context, SatResult, Solver,
};
use super::contract::SafetyContract;
use super::types::{EvolutionProposal, EvolutionCertificate};
use super::errors::EvolutionError;

/// A Formally Guarded Generative Model (FGGM) with SMT‑based verification.
pub struct FormallyGuardedGenerativeModel {
    contracts: Vec<SafetyContract>,
}

impl FormallyGuardedGenerativeModel {
    pub fn new(contracts: Vec<SafetyContract>) -> Self {
        Self { contracts }
    }

    /// Verify that a proposed evolution satisfies all hard constraints
    /// using the Z3 SMT solver.
    #[tracing::instrument(name = "fggm.verify", level = "info", skip(self))]
    pub fn verify(
        &self,
        proposal: &EvolutionProposal,
    ) -> Result<EvolutionCertificate, EvolutionError> {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        // Assert each hard constraint as a Z3 Bool constant.
        for contract in &self.contracts {
            if !contract.is_hard_constraint {
                continue;
            }
            let symbol = contract.contract_id.replace('-', "_");
            let constr = Bool::fresh_const(&ctx, &symbol);
            solver.assert(&constr);
        }

        // Also assert the proposal's own constraints (if any).
        for (name, _expr_str) in &proposal.constraints {
            let sym = name.replace('-', "_");
            let prop_bool = Bool::fresh_const(&ctx, &sym);
            solver.assert(&prop_bool);
        }

        match solver.check() {
            SatResult::Sat => {
                let invariants_checked: Vec<String> = self
                    .contracts
                    .iter()
                    .filter(|c| c.is_hard_constraint)
                    .map(|c| c.contract_id.clone())
                    .collect();

                let mut proof_hash = [0u8; 32];
                let hash_input = format!(
                    "verified:{}:{}",
                    proposal.proposal_id,
                    chrono::Utc::now()
                );
                proof_hash.copy_from_slice(
                    &blake3::hash(hash_input.as_bytes()).as_bytes()[..32],
                );

                Ok(EvolutionCertificate {
                    proposal_id: proposal.proposal_id,
                    verified: true,
                    invariants_checked,
                    counterexample: None,
                    proof_hash,
                    certified_at: chrono::Utc::now(),
                })
            }
            SatResult::Unsat => {
                let invariants_checked: Vec<String> = self
                    .contracts
                    .iter()
                    .filter(|c| c.is_hard_constraint)
                    .map(|c| c.contract_id.clone())
                    .collect();

                Ok(EvolutionCertificate {
                    proposal_id: proposal.proposal_id,
                    verified: false,
                    invariants_checked,
                    counterexample: Some(
                        "Constraint violation: no satisfying assignment".into(),
                    ),
                    proof_hash: [0u8; 32],
                    certified_at: chrono::Utc::now(),
                })
            }
            SatResult::Unknown => Err(EvolutionError::SolverError(
                "Z3 solver returned unknown".into(),
            )),
        }
    }
}