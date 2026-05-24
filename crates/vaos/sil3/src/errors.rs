//! Error types for SIL3 safety kernel.

#[derive(Debug, thiserror::Error)]
pub enum Sil3Error {
    #[error("WCET not verified for task {0:?}")]
    WcetNotVerified(uuid::Uuid),

    #[error("Deadline miss: task {task_id:?} (total misses: {total_misses})")]
    DeadlineMiss { task_id: uuid::Uuid, total_misses: u32 },

    #[error("Safety-critical failure: dynamic allocation in critical path")]
    DynamicAllocationInCriticalPath,

    #[error("SIL level insufficient: required {required}, actual {actual}")]
    SilLevelInsufficient { required: u8, actual: u8 },
}
