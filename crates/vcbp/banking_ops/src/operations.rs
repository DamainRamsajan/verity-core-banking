use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use vaos_core::types::AgentId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BankingOperation {
    DebitAccount(DebitOp),
    CreditAccount(CreditOp),
    WireTransfer(WireTransferOp),
    LoanApproval(LoanApprovalOp),
    GlPosting(GlPostingOp),
}

impl BankingOperation {
    pub fn amount(&self) -> Decimal {
        match self {
            Self::DebitAccount(op) => op.amount,
            Self::CreditAccount(op) => op.amount,
            Self::WireTransfer(op) => op.amount,
            Self::LoanApproval(op) => op.amount,
            Self::GlPosting(op) => op.amount,
        }
    }
    pub fn operation_type(&self) -> &str {
        match self {
            Self::DebitAccount(_) => "debit",
            Self::CreditAccount(_) => "credit",
            Self::WireTransfer(_) => "wire_transfer",
            Self::LoanApproval(_) => "loan_approval",
            Self::GlPosting(_) => "gl_posting",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebitOp { pub id: Uuid, pub account_id: Uuid, pub amount: Decimal, pub currency: String, pub initiator: AgentId }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditOp { pub id: Uuid, pub account_id: Uuid, pub amount: Decimal, pub currency: String, pub initiator: AgentId }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireTransferOp {
    pub id: Uuid,
    pub from_account: Uuid,
    pub to_account: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub initiator: AgentId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanApprovalOp { pub id: Uuid, pub loan_id: Uuid, pub amount: Decimal, pub initiator: AgentId }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlPostingOp { pub id: Uuid, pub gl_account: String, pub amount: Decimal, pub initiator: AgentId }
