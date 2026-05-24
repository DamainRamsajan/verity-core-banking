#[cfg(test)]
mod tests {
    use vcbp_reporting::*;

    #[tokio::test]
    async fn test_generate_call_report() {
        let reporter = reporter::RegulatoryReporter::new();
        let report = reporter
            .generate_call_report(chrono::NaiveDate::from_ymd_opt(2026, 3, 31).unwrap())
            .await
            .unwrap();
        assert_eq!(report.period_end.to_string(), "2026-03-31");
    }
}
