use super::types::Transaction;
use super::errors::LedgerError;

/// Append‑only event store for transaction entries.
pub struct EventStore {
    transactions: Vec<Transaction>,
}

impl EventStore {
    pub fn new() -> Self {
        Self { transactions: Vec::new() }
    }

    pub fn append(&mut self, tx: &Transaction) -> Result<(), LedgerError> {
        self.transactions.push(tx.clone());
        Ok(())
    }

    pub fn get(&self, tx_id: &uuid::Uuid) -> Option<&Transaction> {
        self.transactions.iter().find(|t| &t.id == tx_id)
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }
}
