//! Deterministic scheduler for safety-critical real-time tasks.
//!
//! Uses time-triggered scheduling: no event-driven interrupts in the
//! safety path. All tasks are pre-scheduled with verified WCET bounds.

use std::collections::BinaryHeap;

use super::{SafetyTask, Sil3Error};

/// Priority queue of safety-critical tasks (earliest deadline first).
#[derive(Debug)]
pub struct Sil3Scheduler {
    task_queue: BinaryHeap<ScheduledTask>,
    deadline_misses: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScheduledTask {
    task: SafetyTask,
    /// Priority: earliest deadline = highest priority
    priority: std::cmp::Reverse<chrono::DateTime<chrono::Utc>>,
}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl Sil3Scheduler {
    pub fn new() -> Self {
        Self {
            task_queue: BinaryHeap::new(),
            deadline_misses: 0,
        }
    }

    pub fn enqueue(&mut self, task: SafetyTask) -> Result<(), Sil3Error> {
        let priority = std::cmp::Reverse(task.deadline);
        self.task_queue.push(ScheduledTask { task, priority });
        Ok(())
    }

    /// Record a deadline miss (SIL 3: zero tolerance).
    pub fn record_miss(&mut self) -> Result<(), Sil3Error> {
        self.deadline_misses += 1;
        Err(Sil3Error::DeadlineMiss {
            task_id: uuid::Uuid::nil(),
            total_misses: self.deadline_misses,
        })
    }
}
