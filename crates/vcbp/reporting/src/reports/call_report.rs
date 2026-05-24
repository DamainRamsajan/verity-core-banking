use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallReport {
    pub institution_name: String,
    pub period_end: chrono::NaiveDate,
    pub total_assets: rust_decimal::Decimal,
    pub total_liabilities: rust_decimal::Decimal,
    pub tier1_capital: rust_decimal::Decimal,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}
