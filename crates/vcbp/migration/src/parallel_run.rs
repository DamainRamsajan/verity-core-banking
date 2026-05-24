use rayon::prelude::*;
use super::errors::MigrationError;

/// Parallel‑run simulator — compares legacy and Verity outputs.
///
/// Runs both systems simultaneously for ≥90 days, comparing every
/// transaction output, balance computation, and regulatory report.
/// Uses rayon for data‑parallel comparison.
pub struct ParallelRunSimulator {
    min_days: u32,
    days_completed: u32,
    mismatches: Vec<Mismatch>,
}

#[derive(Debug, Clone)]
pub struct Mismatch {
    pub transaction_id: uuid::Uuid,
    pub legacy_value: String,
    pub verity_value: String,
    pub field: String,
    pub severity: MismatchSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MismatchSeverity {
    Critical,
    High,
    Medium,
    Low,
    Cosmetic,
}

impl ParallelRunSimulator {
    pub fn new(min_days: u32) -> Self {
        Self { min_days, days_completed: 0, mismatches: Vec::new() }
    }

    /// Compare legacy and Verity outputs for a batch of transactions.
    pub fn compare_batch(
        &mut self,
        legacy_outputs: &[(uuid::Uuid, String, String)],
        verity_outputs: &[(uuid::Uuid, String, String)],
    ) -> Result<Vec<Mismatch>, MigrationError> {
        let mismatches: Vec<Mismatch> = legacy_outputs
            .par_iter()
            .zip(verity_outputs.par_iter())
            .filter_map(|((id, field_l, val_l), (_, field_v, val_v))| {
                if val_l != val_v {
                    Some(Mismatch {
                        transaction_id: *id,
                        legacy_value: val_l.clone(),
                        verity_value: val_v.clone(),
                        field: format!("{}/{}", field_l, field_v),
                        severity: MismatchSeverity::Critical,
                    })
                } else {
                    None
                }
            })
            .collect();

        self.mismatches.extend(mismatches.clone());
        self.days_completed += 1;
        Ok(mismatches)
    }

    /// Whether the minimum validation period has been reached with zero critical mismatches.
    pub fn is_cutover_ready(&self) -> bool {
        self.days_completed >= self.min_days
            && !self.mismatches.iter().any(|m| matches!(m.severity, MismatchSeverity::Critical))
    }
}
