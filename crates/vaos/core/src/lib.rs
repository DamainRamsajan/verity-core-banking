pub mod microkernel;
pub mod traits;
pub mod errors;
pub mod types;
pub mod provenance;

pub use types::{
    CapabilityToken, TokenId, CapScope, AgentId, AgentAction, SessionId,
    ProvenanceCapsule, DelegationChain, TrustLevel, CapabilityMask, ClosureResult,
};
pub use traits::{CapabilityValidator, SessionManager, TrustLatticeEvaluator, ContainmentVerifier};
pub use errors::VaosError;
pub use provenance::TraceCaps;

#[derive(Debug, Clone)]
pub struct KernelConfig {
    pub max_delegation_depth: u8,
    pub token_expiry_seconds: u64,
    pub require_dual_control_threshold: rust_decimal::Decimal,
    pub enable_runtime_tla: bool,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            max_delegation_depth: 3,
            token_expiry_seconds: 3600,
            require_dual_control_threshold: rust_decimal::Decimal::new(10000, 0),
            enable_runtime_tla: true,
        }
    }
}
