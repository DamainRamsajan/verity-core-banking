use super::{BankingProduct, TemporalContract};

pub fn checking_account() -> BankingProduct {
    BankingProduct {
        id: uuid::Uuid::new_v4(),
        name: "Standard Checking".into(),
        asl_source: "product CheckingAccount { ... }".into(),
        bytecode: vec![],
        verified_invariants: vec!["no_negative_balance_without_overdraft".into()],
        compiler_version: "0.1.0".into(),
        compiled_at: chrono::Utc::now(),
        temporal_contracts: vec![TemporalContract::reg_e_error_resolution()],
        verified: true,
    }
}

pub fn savings_account() -> BankingProduct {
    BankingProduct {
        id: uuid::Uuid::new_v4(),
        name: "High‑Yield Savings".into(),
        asl_source: "product SavingsAccount { ... }".into(),
        bytecode: vec![],
        verified_invariants: vec!["reg_d_withdrawal_limit_enforced".into()],
        compiler_version: "0.1.0".into(),
        compiled_at: chrono::Utc::now(),
        temporal_contracts: vec![TemporalContract::reg_dd_interest_rate()],
        verified: true,
    }
}

pub fn loan_product() -> BankingProduct {
    BankingProduct {
        id: uuid::Uuid::new_v4(),
        name: "Personal Loan".into(),
        asl_source: "product PersonalLoan { ... }".into(),
        bytecode: vec![],
        verified_invariants: vec!["apr_disclosure_accurate".into()],
        compiler_version: "0.1.0".into(),
        compiled_at: chrono::Utc::now(),
        temporal_contracts: vec![],
        verified: true,
    }
}
