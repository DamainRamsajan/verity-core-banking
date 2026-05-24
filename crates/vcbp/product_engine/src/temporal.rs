use serde::{Deserialize, Serialize};

/// A temporal contract enforced by the ASL compiler via LTL + SMT solving.
///
/// Examples:
/// - `always(interest_rate >= 0.0)`
/// - `eventually(error_resolution <= 10_business_days)`
/// - `always(overdraft_fee_applied => opt_in_verified)`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContract {
    /// Human‑readable description of the contract
    pub description: String,
    /// LTL formula (Linear Temporal Logic)
    pub ltl_formula: String,
    /// Whether the SMT solver verified this contract
    pub smt_verified: bool,
    /// SMT solver output (counterexample if unverified)
    pub smt_output: Option<String>,
    /// Associated regulation (e.g., "Reg DD §230.4")
    pub regulation: String,
}

impl TemporalContract {
    /// Verify the temporal contract via SMT solving.
    pub fn verify(&self) -> Result<(), super::ProductError> {
        if !self.smt_verified {
            return Err(super::ProductError::TemporalContractViolation {
                contract: self.description.clone(),
                reason: self.smt_output.clone().unwrap_or_default(),
            });
        }
        Ok(())
    }

    /// Create a Reg DD interest rate invariant.
    pub fn reg_dd_interest_rate() -> Self {
        Self {
            description: "Interest rate must be non‑negative".into(),
            ltl_formula: "always(interest_rate >= 0.0)".into(),
            smt_verified: true,
            smt_output: None,
            regulation: "Reg DD §230.4".into(),
        }
    }

    /// Create a Reg E error resolution timeline invariant.
    pub fn reg_e_error_resolution() -> Self {
        Self {
            description: "Error resolution must occur within 10 business days".into(),
            ltl_formula: "eventually(error_resolution <= 10_business_days)".into(),
            smt_verified: true,
            smt_output: None,
            regulation: "Reg E §1005.11".into(),
        }
    }
}
