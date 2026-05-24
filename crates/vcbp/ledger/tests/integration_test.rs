#[cfg(test)]
mod tests {
    use vcbp_ledger::*;

    #[tokio::test]
    async fn test_ledger_append_and_prove() {
        let config = merkle_ledger::LedgerConfig::default();
        let ledger = MerkleLedger::new(config);
        // ... test implementation
    }
}
