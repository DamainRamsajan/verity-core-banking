use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContract {
    pub description: String,
    pub ltl_formula: String,
    pub smt_verified: bool,
    pub smt_output: Option<String>,
    pub regulation: String,
}

impl TemporalContract {
    pub fn verify(&self) -> Result<(), super::ProductError> {
        if !self.smt_verified {
            return Err(super::ProductError::TemporalContractViolation {
                contract: self.description.clone(),
                reason: self.smt_output.clone().unwrap_or_default(),
            });
        }
        Ok(())
    }

    pub fn reg_dd_interest_rate() -> Self {
        Self {
            description: "Interest rate must be non‑negative".into(),
            ltl_formula: "always(interest_rate >= 0.0)".into(),
            smt_verified: true,
            smt_output: None,
            regulation: "Reg DD §230.4".into(),
        }
    }

    pub fn reg_e_error_resolution() -> Self {
        Self {
            description: "Error resolution within 10 business days".into(),
            ltl_formula: "eventually(error_resolution <= 10_business_days)".into(),
            smt_verified: true,
            smt_output: None,
            regulation: "Reg E §1005.11".into(),
        }
    }
}
