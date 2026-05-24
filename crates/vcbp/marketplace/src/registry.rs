use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;

use super::types::{AgentListing, ListingStatus, Challenge, ChallengeOutcome};
use super::staking::StakingPool;
use super::errors::MarketplaceError;

/// Token Curated Registry for agent listings.
///
/// Agents stake tokens to be listed. Any listing can be challenged;
/// if upheld, the stake is slashed and the listing is removed.
pub struct TokenCuratedRegistry {
    listings: RwLock<HashMap<Uuid, AgentListing>>,
    staking: Arc<StakingPool>,
    config: RegistryConfig,
}

#[derive(Debug, Clone)]
pub struct RegistryConfig {
    pub min_stake: rust_decimal::Decimal,
    pub challenge_period_days: u32,
    pub challenge_deposit: rust_decimal::Decimal,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            min_stake: rust_decimal::Decimal::new(1_000, 0),
            challenge_period_days: 7,
            challenge_deposit: rust_decimal::Decimal::new(500, 0),
        }
    }
}

impl TokenCuratedRegistry {
    pub fn new(config: RegistryConfig) -> Self {
        Self { listings: RwLock::new(HashMap::new()), staking: Arc::new(StakingPool::new()), config }
    }

    /// Apply to list an agent in the marketplace.
    #[tracing::instrument(name = "marketplace.list", level = "info", skip(self))]
    pub async fn apply_listing(
        &self,
        listing: AgentListing,
    ) -> Result<AgentListing, MarketplaceError> {
        if listing.stake_amount < self.config.min_stake {
            return Err(MarketplaceError::InsufficientStake {
                required: self.config.min_stake,
                provided: listing.stake_amount,
            });
        }

        // Stake tokens
        self.staking.stake(listing.agent_id, listing.stake_amount)?;

        let mut listings = self.listings.write().await;
        listings.insert(listing.listing_id, listing.clone());

        tracing::info!(listing_id = %listing.listing_id, agent = %listing.name, "Agent listed");
        Ok(listing)
    }

    /// Challenge a listing.
    pub async fn challenge(
        &self,
        listing_id: Uuid,
        challenge: Challenge,
    ) -> Result<(), MarketplaceError> {
        let mut listings = self.listings.write().await;
        let listing = listings.get_mut(&listing_id)
            .ok_or(MarketplaceError::ListingNotFound(listing_id))?;

        listing.status = ListingStatus::Challenged;
        listing.challenges.push(challenge);
        Ok(())
    }

    /// Resolve a challenge.
    pub async fn resolve_challenge(
        &self,
        listing_id: Uuid,
        challenge_id: Uuid,
        outcome: ChallengeOutcome,
    ) -> Result<(), MarketplaceError> {
        let mut listings = self.listings.write().await;
        let listing = listings.get_mut(&listing_id)
            .ok_or(MarketplaceError::ListingNotFound(listing_id))?;

        if let Some(challenge) = listing.challenges.iter_mut().find(|c| c.challenge_id == challenge_id) {
            challenge.resolved = true;
            challenge.outcome = Some(outcome);
        }

        if outcome == ChallengeOutcome::Upheld {
            // Slash the stake
            self.staking.slash(listing.agent_id, listing.stake_amount)?;
            listing.status = ListingStatus::Slashed;
        } else {
            listing.status = ListingStatus::Active;
        }

        Ok(())
    }
}
