use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use vaos_core::types::{AgentId, AccountId};

/// A banking operation that requires capability tokens for authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BankingOperation {
    DebitAccount(DebitOp),
    CreditAccount(CreditOp),
    WireTransfer(WireTransferOp),
    LoanApproval(LoanApprovalOp),
    GlPosting(GlPostingOp),
    RegulatoryFiling(RegulatoryFilingOp),
}

impl BankingOperation {
    /// The amount involved in this operation (for dual‑control threshold checks).
    pub fn amount(&self) -> Decimal {
        match self {
            Self::DebitAccount(op) => op.amount,
            Self::CreditAccount(op) => op.amount,
            Self::WireTransfer(op) => op.amount,
            Self::LoanApproval(op) => op.amount,
            Self::GlPosting(op) => op.amount,
            Self::RegulatoryFiling(_) => Decimal::ZERO,
        }
    }

    /// Whether this operation requires dual‑control approval.
    pub fn requires_dual_control(&self, threshold: Decimal) -> bool {
        self.amount() >= threshold
    }

    /// The operation type name (e.g., "debit", "wire_transfer").
    pub fn operation_type(&self) -> &str {
        match self {
            Self::DebitAccount(_) => "debit",
            Self::CreditAccount(_) => "credit",
            Self::WireTransfer(_) => "wire_transfer",
            Self::LoanApproval(_) => "loan_approval",
            Self::GlPosting(_) => "gl_posting",
            Self::RegulatoryFiling(_) => "regulatory_filing",
        }
    }
}

/// Debit an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebitOp {
    pub id: Uuid,
    pub account_id: AccountId,
    pub amount: Decimal,
    pub currency: String,
    pub initiator: AgentId,
}

/// Credit an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditOp {
    pub id: Uuid,
    pub account_id: AccountId,
    pub amount: Decimal,
    pub currency: String,
    pub initiator: AgentId,
}

/// Execute a wire transfer (dual‑control for >$10K).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireTransferOp {
    pub id: Uuid,
    pub from_account: AccountId,
    pub to_account: AccountId,
    pub amount: Decimal,
    pub currency: String,
    pub initiator: AgentId,
}

/// Approve a loan (dual‑control required).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanApprovalOp {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub amount: Decimal,
    pub initiator: AgentId,
}

/// Post to the General Ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlPostingOp {
    pub id: Uuid,
    pub gl_account: String,
    pub amount: Decimal,
    pub initiator: AgentId,
}

/// File a regulatory report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFilingOp {
    pub id: Uuid,
    pub report_type: String,
    pub initiator: AgentId,
}
