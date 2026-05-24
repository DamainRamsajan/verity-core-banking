use std::sync::atomic::{AtomicU64, Ordering};

/// Key performance metrics for Verity operations.
#[derive(Debug, Default)]
pub struct VerityMetrics {
    pub ledger_appends: AtomicU64,
    pub capability_validations: AtomicU64,
    pub compliance_checks: AtomicU64,
    pub fraud_scores: AtomicU64,
    pub payment_sends: AtomicU64,
    pub fl_rounds: AtomicU64,
}

impl VerityMetrics {
    pub fn new() -> Self { Self::default() }

    pub fn record_ledger_append(&self) { self.ledger_appends.fetch_add(1, Ordering::Relaxed); }
    pub fn record_capability_validation(&self) { self.capability_validations.fetch_add(1, Ordering::Relaxed); }
    pub fn record_compliance_check(&self) { self.compliance_checks.fetch_add(1, Ordering::Relaxed); }
    pub fn record_fraud_score(&self) { self.fraud_scores.fetch_add(1, Ordering::Relaxed); }
    pub fn record_payment_send(&self) { self.payment_sends.fetch_add(1, Ordering::Relaxed); }
    pub fn record_fl_round(&self) { self.fl_rounds.fetch_add(1, Ordering::Relaxed); }
}
