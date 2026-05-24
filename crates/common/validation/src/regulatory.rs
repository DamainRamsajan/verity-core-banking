use super::errors::ValidationError;

/// Regulatory constraint validator (Reg DD, Reg Z, Reg E).
pub struct RegulatoryValidator;

impl RegulatoryValidator {
    pub fn new() -> Self { Self }

    /// Validate that an interest rate satisfies Reg DD (Truth in Savings).
    /// Reg DD §230.4: interest rate must be non-negative.
    pub fn validate_interest_rate(&self, rate: rust_decimal::Decimal) -> Result<(), ValidationError> {
        if rate < rust_decimal::Decimal::ZERO {
            return Err(ValidationError::RegulatoryConstraintViolated {
                regulation: "Reg DD §230.4".into(),
                detail: format!("Interest rate {} is negative", rate),
            });
        }
        Ok(())
    }

    /// Validate that an APY calculation satisfies Reg DD accuracy requirements.
    pub fn validate_apy_calculation(
        &self,
        declared_apy: rust_decimal::Decimal,
        computed_apy: rust_decimal::Decimal,
        tolerance: rust_decimal::Decimal,
    ) -> Result<(), ValidationError> {
        let diff = (declared_apy - computed_apy).abs();
        if diff > tolerance {
            return Err(ValidationError::RegulatoryConstraintViolated {
                regulation: "Reg DD APY Accuracy".into(),
                detail: format!("Declared APY {} differs from computed {} by {}", declared_apy, computed_apy, diff),
            });
        }
        Ok(())
    }

    /// Validate that an overdraft fee is only applied with verified opt-in (Reg E).
    pub fn validate_overdraft_opt_in(&self, opt_in_verified: bool, fee_applied: bool) -> Result<(), ValidationError> {
        if fee_applied && !opt_in_verified {
            return Err(ValidationError::RegulatoryConstraintViolated {
                regulation: "Reg E Opt-In".into(),
                detail: "Overdraft fee applied without verified opt-in".into(),
            });
        }
        Ok(())
    }
}
