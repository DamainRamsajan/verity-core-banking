use std::collections::HashMap;
use tokio::sync::RwLock;
use super::types::{RegulatoryFeed, Obligation};
use super::errors::RegTechError;

pub struct RegTechEngine {
    feeds: RwLock<HashMap<String, RegulatoryFeed>>,
    obligations: RwLock<Vec<Obligation>>,
}

impl RegTechEngine {
    pub fn new() -> Self {
        Self {
            feeds: RwLock::new(HashMap::new()),
            obligations: RwLock::new(Vec::new()),
        }
    }

    pub async fn register_feed(&self, feed: RegulatoryFeed) {
        self.feeds.write().await.insert(feed.source.clone(), feed);
    }

    pub async fn add_obligation(&self, obligation: Obligation) {
        self.obligations.write().await.push(obligation);
    }
}
