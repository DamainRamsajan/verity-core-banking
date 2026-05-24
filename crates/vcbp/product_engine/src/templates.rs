use super::{BankingProduct, TemporalContract};
use uuid::Uuid;

/// Pre‑built checking account product template.
pub fn checking_account() -> BankingProduct {
    BankingProduct {
        id: Uuid::new_v4(),
        name: "Standard Checking".into(),
        asl_source: "product CheckingAccount { ... }".into(),
        bytecode: vec![],
        verified_invariants: vec![
            "no_negative_balance_without_overdraft".into(),
            "interest_rate_non_negative".into(),
            "fee_disclosure_complete".into(),
        ],
        compiler_version: "0.1.0".into(),
        compiled_at: chrono::Utc::now(),
        temporal_contracts: vec![
            TemporalContract::reg_dd_interest_rate(),
            TemporalContract::reg_e_error_resolution(),
        ],
        verified: true,
    }
}

/// Pre‑built savings account product template.
pub fn savings_account() -> BankingProduct {
    BankingProduct {
        id: Uuid::new_v4(),
        name: "High‑Yield Savings".into(),
        asl_source: "product SavingsAccount { ... }".into(),
        bytecode: vec![],
        verified_invariants: vec![
            "reg_d_withdrawal_limit_enforced".into(),
            "interest_calculation_daily_compounding".into(),
        ],
        compiler_version: "0.1.0".into(),
        compiled_at: chrono::Utc::now(),
        temporal_contracts: vec![
            TemporalContract::reg_dd_interest_rate(),
        ],
        verified: true,
    }
}

/// Pre‑built loan product template.
pub fn loan_product() -> BankingProduct {
    BankingProduct {
        id: Uuid::new_v4(),
        name: "Personal Loan".into(),
        asl_source: "product PersonalLoan { ... }".into(),
        bytecode: vec![],
        verified_invariants: vec![
            "apr_disclosure_accurate".into(),
            "no_prepayment_penalty_after_36_months".into(),
        ],
        compiler_version: "0.1.0".into(),
        compiled_at: chrono::Utc::now(),
        temporal_contracts: vec![],
        verified: true,
    }
}
