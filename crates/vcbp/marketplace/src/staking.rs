use std::collections::HashMap;
use std::sync::Mutex;
use vaos_core::types::AgentId;
use super::errors::MarketplaceError;

/// Staking pool — manages agent stakes and slashing.
pub struct StakingPool {
    stakes: Mutex<HashMap<AgentId, rust_decimal::Decimal>>,
    total_staked: Mutex<rust_decimal::Decimal>,
}

#[derive(Debug, Clone)]
pub struct SlashingCondition {
    pub reason: String,
    pub slash_percentage: f64,
}

impl StakingPool {
    pub fn new() -> Self {
        Self {
            stakes: Mutex::new(HashMap::new()),
            total_staked: Mutex::new(rust_decimal::Decimal::ZERO),
        }
    }

    /// Stake tokens for an agent.
    pub fn stake(
        &self,
        agent_id: AgentId,
        amount: rust_decimal::Decimal,
    ) -> Result<(), MarketplaceError> {
        let mut stakes = self.stakes.lock().unwrap();
        *stakes.entry(agent_id).or_default() += amount;
        *self.total_staked.lock().unwrap() += amount;
        Ok(())
    }

    /// Slash an agent's stake for misbehaviour.
    pub fn slash(
        &self,
        agent_id: AgentId,
        amount: rust_decimal::Decimal,
    ) -> Result<(), MarketplaceError> {
        let mut stakes = self.stakes.lock().unwrap();
        let stake = stakes.get_mut(&agent_id)
            .ok_or(MarketplaceError::AgentNotStaked(agent_id))?;
        if *stake < amount {
            return Err(MarketplaceError::InsufficientStake {
                required: amount,
                provided: *stake,
            });
        }
        *stake -= amount;
        *self.total_staked.lock().unwrap() -= amount;
        Ok(())
    }

    /// Get an agent's current stake.
    pub fn get_stake(&self, agent_id: AgentId) -> rust_decimal::Decimal {
        self.stakes.lock().unwrap().get(&agent_id).copied().unwrap_or_default()
    }
}
