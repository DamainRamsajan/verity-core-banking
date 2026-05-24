use tokio::sync::RwLock;
use super::types::ActivityEvent;

/// Real‑time activity feed for the dashboard.
pub struct ActivityFeed {
    events: RwLock<Vec<ActivityEvent>>,
}

impl ActivityFeed {
    pub fn new() -> Self {
        Self { events: RwLock::new(Vec::new()) }
    }

    pub async fn push(&self, event: ActivityEvent) {
        self.events.write().await.push(event);
    }

    pub async fn recent(&self, limit: usize) -> Vec<ActivityEvent> {
        let events = self.events.read().await;
        events.iter().rev().take(limit).cloned().collect()
    }
}
