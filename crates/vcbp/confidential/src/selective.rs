use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Selective disclosure – enables authorised regulators to decrypt
/// specific transactions or balances without exposing all data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectiveDisclosure {
    pub disclosure_id: Uuid,
    pub target_type: DisclosureTarget,
    pub target_id: Uuid,
    pub authorised_regulator: String,
    pub proof: Vec<u8>,
    pub valid_until: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisclosureTarget {
    Transaction,
    Balance,
    AccountHistory,
    ComplianceReport,
}
