use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

/// Unique account identifier
pub type AccountId = Uuid;
/// Currency code (ISO 4217)
pub type Currency = String;

/// A double‑entry ledger transaction
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

/// A single debit or credit entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub account_id: AccountId,
    pub amount: Decimal,  // positive = debit, negative = credit
    pub currency: Currency,
    pub entry_type: EntryType,
    pub compliance_tags: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType {
    Debit,
    Credit,
}

/// Current balance of an account (materialised view)
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
    /// Verify conservation of value: Σ entries = 0
    pub fn verify_conservation(&self) -> Result<(), super::LedgerError> {
        let sum: Decimal = self.entries.iter().map(|e| e.amount).sum();
        if sum != Decimal::ZERO {
            return Err(super::LedgerError::UnbalancedTransaction(sum));
        }
        Ok(())
    }

    /// Compute the transaction hash for Merkle tree insertion
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.id.as_bytes());
        for entry in &self.entries {
            hasher.update(entry.account_id.as_bytes());
            hasher.update(&entry.amount.to_string_le_bytes());
        }
        *hasher.finalize().as_bytes()
    }
}
