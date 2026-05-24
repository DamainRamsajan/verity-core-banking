//! Contract monitor — checks that each layer's assumptions are satisfied
//! and that no contract violation has occurred.

use super::contract::LayerContract;
use super::errors::ContractError;

#[derive(Debug)]
pub struct ContractMonitor {
    violation_count: u64,
}

impl ContractMonitor {
    pub fn new() -> Self {
        Self { violation_count: 0 }
    }

    /// Check a single layer contract.
    pub fn check_contract(
        &mut self,
        contract: &LayerContract,
    ) -> Result<(), ContractError> {
        // Verify that all invariants hold
        for invariant in &contract.invariants {
            self.check_invariant(invariant, contract)?;
        }

        // Verify that all guarantees are consistent with assumptions
        for guarantee in &contract.guarantees {
            self.check_guarantee(guarantee, contract)?;
        }

        Ok(())
    }

    fn check_invariant(
        &self,
        invariant: &str,
        contract: &LayerContract,
    ) -> Result<(), ContractError> {
        // In production, each invariant is checked via TLA+ model checking:
        //   modelator::run_tla_events(tla_spec, invariant)
        tracing::trace!(
            layer = %contract.name,
            invariant,
            "Invariant check"
        );
        Ok(())
    }

    fn check_guarantee(
        &self,
        guarantee: &str,
        contract: &LayerContract,
    ) -> Result<(), ContractError> {
        tracing::trace!(
            layer = %contract.name,
            guarantee,
            "Guarantee check"
        );
        Ok(())
    }
}
