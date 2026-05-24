#[cfg(test)]
mod tests {
    use common_telemetry::*;

    #[test]
    fn test_metrics_recording() {
        let m = metrics::VerityMetrics::new();
        m.record_ledger_append();
        assert_eq!(m.ledger_appends.load(std::sync::atomic::Ordering::Relaxed), 1);
    }
}
