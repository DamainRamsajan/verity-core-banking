use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFeed {
    pub source: String,
    pub url: String,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obligation {
    pub id: uuid::Uuid,
    pub description: String,
    pub domain: String,
    pub regulation: String,
}
