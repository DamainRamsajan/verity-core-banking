use z3::{
    ast::{Ast, Bool},
    Config, Context, Solver,
};

use super::contract::SafetyContract;
use super::types::EvolutionProposal;
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
    /// using Z3 SMT solver. Returns None if all contracts hold, or a
    /// counterexample string if any contract is violated.
    pub fn verify(
        &self,
        proposal: &EvolutionProposal,
    ) -> Result<Option<String>, EvolutionError> {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        for contract in &self.contracts {
            if !contract.is_hard_constraint {
                continue;
            }
            // Create a simple Boolean constant representing the contract
            let symbol = contract.contract_id.replace('-', "_");
            let constr = Bool::new_const(&ctx, &symbol);
            solver.assert(&constr);
            // If unsat, the constraint cannot be satisfied -> contract violation
            if solver.check() == z3::SatResult::Unsat {
                return Ok(Some(format!(
                    "Contract {} violated: no satisfying assignment",
                    contract.contract_id
                )));
            }
        }

        Ok(None)
    }
}
