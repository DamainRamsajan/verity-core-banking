//! 1A1A smart account — capability-governed agent accounts.

use serde::{Deserialize, Serialize};

/// A capability-governed smart account for an AI agent (1A1A paradigm).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartAccount {
    pub account_id: String,
    pub spending_limit: rust_decimal::Decimal,
    pub spent_this_period: rust_decimal::Decimal,
    pub human_principal: Option<String>,
    pub frozen: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl SmartAccount {
    pub fn new(
        spending_limit: rust_decimal::Decimal,
        human_principal: Option<String>,
    ) -> Self {
        Self {
            account_id: format!("1A1A-{}", uuid::Uuid::new_v4()),
            spending_limit,
            spent_this_period: rust_decimal::Decimal::ZERO,
            human_principal,
            frozen: false,
            created_at: chrono::Utc::now(),
        }
    }
}
