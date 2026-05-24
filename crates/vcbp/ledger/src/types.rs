use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

pub type AccountId = Uuid;
pub type Currency = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub correlation_id: Uuid,
    pub entries: Vec<Entry>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub agent_id: Option<vaos_core::types::AgentId>,
    pub capability_token_id: Option<vaos_core::types::TokenId>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub account_id: AccountId,
    pub amount: Decimal,
    pub currency: Currency,
    pub entry_type: EntryType,
    pub compliance_tags: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType { Debit, Credit }

#[derive(Debug, Clone, Default)]
pub struct Balance {
    pub account_id: AccountId,
    pub currency: Currency,
    pub ledger_balance: Decimal,
    pub available_balance: Decimal,
    pub reserved_balance: Decimal,
    pub last_entry_id: Option<Uuid>,
}

impl Transaction {
    pub fn verify_conservation(&self) -> Result<(), super::LedgerError> {
        let sum: Decimal = self.entries.iter().map(|e| e.amount).sum();
        if sum != Decimal::ZERO {
            return Err(super::LedgerError::UnbalancedTransaction(sum));
        }
        Ok(())
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.id.as_bytes());
        for entry in &self.entries {
            hasher.update(entry.account_id.as_bytes());
            hasher.update(&entry.amount.to_string().as_bytes());
        }
        *hasher.finalize().as_bytes()
    }
}
