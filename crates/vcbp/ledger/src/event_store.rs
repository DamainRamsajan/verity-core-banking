use super::types::Transaction;
use super::errors::LedgerError;

pub struct EventStore {
    transactions: Vec<Transaction>,
}

impl EventStore {
    pub fn new() -> Self { Self { transactions: Vec::new() } }
    pub fn append(&mut self, tx: &Transaction) -> Result<(), LedgerError> {
        self.transactions.push(tx.clone());
        Ok(())
    }
    pub fn len(&self) -> usize { self.transactions.len() }
}
