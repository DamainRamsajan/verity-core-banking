use tracing::Span;

/// Extension trait for enriching spans with Verity-specific attributes.
pub trait SpanExt {
    fn with_agent_id(self, agent_id: uuid::Uuid) -> Self;
    fn with_token_id(self, token_id: uuid::Uuid) -> Self;
    fn with_transaction_id(self, tx_id: uuid::Uuid) -> Self;
    fn with_compliance_domain(self, domain: &str) -> Self;
    fn with_theorem_id(self, theorem: &str) -> Self;
}

impl SpanExt for Span {
    fn with_agent_id(self, agent_id: uuid::Uuid) -> Self {
        self.record("agent.id", agent_id.to_string());
        self
    }
    fn with_token_id(self, token_id: uuid::Uuid) -> Self {
        self.record("capability.token_id", token_id.to_string());
        self
    }
    fn with_transaction_id(self, tx_id: uuid::Uuid) -> Self {
        self.record("transaction.id", tx_id.to_string());
        self
    }
    fn with_compliance_domain(self, domain: &str) -> Self {
        self.record("compliance.domain", domain.to_string());
        self
    }
    fn with_theorem_id(self, theorem: &str) -> Self {
        self.record("theorem.id", theorem.to_string());
        self
    }
}
