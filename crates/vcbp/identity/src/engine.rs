use std::collections::HashMap;
use tokio::sync::RwLock;
use super::types::{AgentIdentity, SmartAccount, SpendingLimit};
use super::errors::IdentityError;

pub struct IdentityEngine {
    identities: RwLock<HashMap<vaos_core::types::AgentId, AgentIdentity>>,
    accounts: RwLock<HashMap<uuid::Uuid, SmartAccount>>,
}

impl IdentityEngine {
    pub fn new() -> Self {
        Self {
            identities: RwLock::new(HashMap::new()),
            accounts: RwLock::new(HashMap::new()),
        }
    }

    pub async fn register_agent(
        &self,
        agent_id: vaos_core::types::AgentId,
        binary_hash: [u8; 32],
    ) -> Result<AgentIdentity, IdentityError> {
        let identity = AgentIdentity {
            agent_id,
            binary_hash,
            did: format!("did:key:{}", hex::encode(&binary_hash[..16])),
            kya_credential_id: None,
            created_at: chrono::Utc::now(),
        };
        self.identities.write().await.insert(agent_id, identity.clone());
        Ok(identity)
    }

    pub async fn create_smart_account(
        &self,
        agent_id: vaos_core::types::AgentId,
        limit: SpendingLimit,
        principal: Option<String>,
    ) -> Result<SmartAccount, IdentityError> {
        let account = SmartAccount {
            account_id: uuid::Uuid::new_v4(),
            spending_limit: limit,
            human_principal: principal,
            frozen: false,
        };
        self.accounts.write().await.insert(account.account_id, account.clone());
        Ok(account)
    }
}
