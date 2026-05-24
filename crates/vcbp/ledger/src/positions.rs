use std::collections::HashMap;
use super::types::{AccountId, Balance, Entry, EntryType};

/// Real‑time account position keeper.
pub struct PositionKeeper {
    balances: HashMap<AccountId, Balance>,
}

impl PositionKeeper {
    pub fn new() -> Self {
        Self { balances: HashMap::new() }
    }

    pub fn apply_entry(&mut self, entry: &Entry) -> Result<(), super::LedgerError> {
        let balance = self.balances.entry(entry.account_id).or_insert_with(|| Balance {
            account_id: entry.account_id,
            currency: entry.currency.clone(),
            ledger_balance: rust_decimal::Decimal::ZERO,
            available_balance: rust_decimal::Decimal::ZERO,
            reserved_balance: rust_decimal::Decimal::ZERO,
            last_entry_id: None,
        });

        match entry.entry_type {
            EntryType::Debit => balance.ledger_balance += entry.amount,
            EntryType::Credit => balance.ledger_balance -= entry.amount,
        }
        balance.available_balance = balance.ledger_balance - balance.reserved_balance;
        Ok(())
    }

    pub fn get(&self, account_id: AccountId) -> Option<&Balance> {
        self.balances.get(&account_id)
    }
}
