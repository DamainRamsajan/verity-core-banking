#[derive(Debug, thiserror::Error)]
pub enum RegTechError {
    #[error("Feed not found: {0}")]
    FeedNotFound(String),
}
