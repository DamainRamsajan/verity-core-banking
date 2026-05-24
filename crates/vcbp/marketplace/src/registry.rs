use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use super::types::AgentListing;
use super::errors::MarketplaceError;

pub struct TokenCuratedRegistry {
    listings: RwLock<HashMap<Uuid, AgentListing>>,
    config: RegistryConfig,
}

#[derive(Debug, Clone)]
pub struct RegistryConfig {
    pub min_stake: rust_decimal::Decimal,
}

impl Default for RegistryConfig {
    fn default() -> Self { Self { min_stake: rust_decimal::Decimal::new(1_000, 0) } }
}

impl TokenCuratedRegistry {
    pub fn new(config: RegistryConfig) -> Self {
        Self { listings: RwLock::new(HashMap::new()), config }
    }

    pub async fn apply_listing(&self, listing: AgentListing) -> Result<AgentListing, MarketplaceError> {
        if listing.stake_amount < self.config.min_stake {
            return Err(MarketplaceError::InsufficientStake { required: self.config.min_stake, provided: listing.stake_amount });
        }
        let mut listings = self.listings.write().await;
        listings.insert(listing.listing_id, listing.clone());
        Ok(listing)
    }
}
