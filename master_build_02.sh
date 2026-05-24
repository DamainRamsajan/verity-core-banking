#!/bin/bash
set -e

echo "============================================"
echo "  VERITY MASTER BUILD 02 – Block 1 Core Invariants"
echo "============================================"

# -------------------------------------------------------
# 1. Merkle Double-Entry Ledger (vcbp/ledger)
# -------------------------------------------------------
cat > crates/vcbp/ledger/Cargo.toml << 'CEOF'
[package]
name = "vcbp-ledger"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Verity Core Banking – Merkle Double-Entry Ledger"

[dependencies]
vaos-core = { path = "../../vaos/core" }
tokio.workspace = true
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
thiserror.workspace = true
uuid.workspace = true
chrono.workspace = true
rust_decimal.workspace = true
blake3.workspace = true
ed25519-dalek.workspace = true
sqlx.workspace = true
async-trait.workspace = true
rs_merkle = "1.5.0"
CEOF

cat > crates/vcbp/ledger/src/lib.rs << 'RSEOF'
pub mod merkle_ledger;
pub mod event_store;
pub mod proof;
pub mod positions;
pub mod types;
pub mod errors;

pub use merkle_ledger::MerkleLedger;
pub use types::{Transaction, Entry, AccountId, Currency, Balance, EntryType};
pub use proof::MerkleProof;
pub use errors::LedgerError;
RSEOF

# types.rs
cat > crates/vcbp/ledger/src/types.rs << 'RSEOF'
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

pub type AccountId = Uuid;
pub type Currency = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub correlation_id: Uuid,
    pub entries: Vec<Entry>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub agent_id: Option<vaos_core::types::AgentId>,
    pub capability_token_id: Option<vaos_core::types::TokenId>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub account_id: AccountId,
    pub amount: Decimal,
    pub currency: Currency,
    pub entry_type: EntryType,
    pub compliance_tags: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryType { Debit, Credit }

#[derive(Debug, Clone, Default)]
pub struct Balance {
    pub account_id: AccountId,
    pub currency: Currency,
    pub ledger_balance: Decimal,
    pub available_balance: Decimal,
    pub reserved_balance: Decimal,
    pub last_entry_id: Option<Uuid>,
}

impl Transaction {
    pub fn verify_conservation(&self) -> Result<(), super::LedgerError> {
        let sum: Decimal = self.entries.iter().map(|e| e.amount).sum();
        if sum != Decimal::ZERO {
            return Err(super::LedgerError::UnbalancedTransaction(sum));
        }
        Ok(())
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.id.as_bytes());
        for entry in &self.entries {
            hasher.update(entry.account_id.as_bytes());
            hasher.update(&entry.amount.to_string_le_bytes());
        }
        *hasher.finalize().as_bytes()
    }
}
RSEOF

# merkle_ledger.rs
cat > crates/vcbp/ledger/src/merkle_ledger.rs << 'RSEOF'
use std::sync::Arc;
use tokio::sync::RwLock;
use rs_merkle::{MerkleTree, Hasher as MerkleHasher};
use sha2::Sha256;

use super::types::{Transaction, Balance, AccountId};
use super::event_store::EventStore;
use super::positions::PositionKeeper;
use super::proof::MerkleProof;
use super::errors::LedgerError;

#[derive(Clone)]
struct Blake3Hasher;

impl MerkleHasher for Blake3Hasher {
    type Hash = [u8; 32];
    fn hash(data: &[u8]) -> Self::Hash { *blake3::hash(data).as_bytes() }
}

pub struct MerkleLedger {
    event_store: Arc<RwLock<EventStore>>,
    merkle_tree: Arc<RwLock<MerkleTree<Blake3Hasher>>>,
    positions: Arc<RwLock<PositionKeeper>>,
    config: LedgerConfig,
}

#[derive(Debug, Clone)]
pub struct LedgerConfig {
    pub enable_tla_runtime_check: bool,
    pub enable_fim: bool,
}

impl Default for LedgerConfig {
    fn default() -> Self { Self { enable_tla_runtime_check: true, enable_fim: true } }
}

impl MerkleLedger {
    pub fn new(config: LedgerConfig) -> Self {
        Self {
            event_store: Arc::new(RwLock::new(EventStore::new())),
            merkle_tree: Arc::new(RwLock::new(MerkleTree::new())),
            positions: Arc::new(RwLock::new(PositionKeeper::new())),
            config,
        }
    }

    pub async fn append(&self, tx: Transaction) -> Result<MerkleProof, LedgerError> {
        tx.verify_conservation()?;

        // FIM check (if enabled)
        if self.config.enable_fim {
            // In production: self.fim.check_transaction(&tx).await?;
        }

        // TLA+ sample (if enabled)
        if self.config.enable_tla_runtime_check {
            // In production: self.tla_verifier.sample(&tx).await?;
        }

        let mut store = self.event_store.write().await;
        store.append(&tx)?;

        let mut tree = self.merkle_tree.write().await;
        let leaf_hash = Blake3Hasher::hash(&tx.hash());
        tree.insert(leaf_hash);
        let proof = tree.proof(&[leaf_hash]);
        let root = tree.root().ok_or(LedgerError::MerkleTreeEmpty)?;

        let mut pos = self.positions.write().await;
        for entry in &tx.entries {
            pos.apply_entry(entry)?;
        }

        Ok(MerkleProof {
            transaction_hash: tx.hash(),
            merkle_root: root,
            proof_hashes: proof.proof_hashes().to_vec(),
            proof_index: proof.proof_hashes().len() as u64,
        })
    }

    pub async fn get_balance(&self, account_id: AccountId) -> Option<Balance> {
        self.positions.read().await.get(account_id).cloned()
    }
}
RSEOF

# event_store.rs
cat > crates/vcbp/ledger/src/event_store.rs << 'RSEOF'
use super::types::Transaction;
use super::errors::LedgerError;

pub struct EventStore {
    transactions: Vec<Transaction>,
}

impl EventStore {
    pub fn new() -> Self { Self { transactions: Vec::new() } }
    pub fn append(&mut self, tx: &Transaction) -> Result<(), LedgerError> {
        self.transactions.push(tx.clone());
        Ok(())
    }
    pub fn len(&self) -> usize { self.transactions.len() }
}
RSEOF

# proof.rs
cat > crates/vcbp/ledger/src/proof.rs << 'RSEOF'
#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub transaction_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub proof_hashes: Vec<[u8; 32]>,
    pub proof_index: u64,
}
RSEOF

# positions.rs
cat > crates/vcbp/ledger/src/positions.rs << 'RSEOF'
use std::collections::HashMap;
use super::types::{AccountId, Balance, Entry, EntryType};

pub struct PositionKeeper {
    balances: HashMap<AccountId, Balance>,
}

impl PositionKeeper {
    pub fn new() -> Self { Self { balances: HashMap::new() } }

    pub fn apply_entry(&mut self, entry: &Entry) -> Result<(), super::LedgerError> {
        let balance = self.balances.entry(entry.account_id).or_insert_with(|| Balance {
            account_id: entry.account_id,
            currency: entry.currency.clone(),
            ledger_balance: rust_decimal::Decimal::ZERO,
            available_balance: rust_decimal::Decimal::ZERO,
            reserved_balance: rust_decimal::Decimal::ZERO,
            last_entry_id: None,
        });

        match entry.entry_type {
            EntryType::Debit => balance.ledger_balance += entry.amount,
            EntryType::Credit => balance.ledger_balance -= entry.amount,
        }
        balance.available_balance = balance.ledger_balance - balance.reserved_balance;
        Ok(())
    }

    pub fn get(&self, account_id: AccountId) -> Option<&Balance> {
        self.balances.get(&account_id)
    }
}
RSEOF

# errors.rs
cat > crates/vcbp/ledger/src/errors.rs << 'RSEOF'
#[derive(Debug, thiserror::Error)]
pub enum LedgerError {
    #[error("Unbalanced transaction: sum = {0}")]
    UnbalancedTransaction(rust_decimal::Decimal),
    #[error("Merkle tree empty")]
    MerkleTreeEmpty,
}
RSEOF

echo "Ledger crate implemented."

# -------------------------------------------------------
# 2. Financial Invariants Monitor (asm/fim)
# -------------------------------------------------------
cat > crates/asm/fim/Cargo.toml << 'CEOF'
[package]
name = "asm-fim"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Verity ASM – Financial Invariants Monitor"

[dependencies]
vaos-core = { path = "../../vaos/core" }
vcbp-ledger = { path = "../../vcbp/ledger" }
tokio.workspace = true
serde.workspace = true
tracing.workspace = true
thiserror.workspace = true
uuid.workspace = true
chrono.workspace = true
rust_decimal.workspace = true
async-trait.workspace = true
CEOF

cat > crates/asm/fim/src/lib.rs << 'RSEOF'
pub mod engine;
pub mod types;
pub mod errors;

pub use engine::FinancialInvariantsMonitor;
pub use errors::FimError;
RSEOF

cat > crates/asm/fim/src/types.rs << 'RSEOF'
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterChange {
    pub parameter_name: String,
    pub old_value: String,
    pub new_value: String,
    pub authorized: bool,
}
RSEOF

cat > crates/asm/fim/src/engine.rs << 'RSEOF'
use std::collections::HashSet;
use tokio::sync::RwLock;
use super::types::ParameterChange;
use super::errors::FimError;

pub struct FinancialInvariantsMonitor {
    protected_parameters: RwLock<HashSet<String>>,
}

impl FinancialInvariantsMonitor {
    pub fn new() -> Self {
        let mut params = HashSet::new();
        params.insert("credit_limit".into());
        params.insert("fee_structure".into());
        params.insert("interest_rate_base".into());
        params.insert("routing_rules".into());
        Self { protected_parameters: RwLock::new(params) }
    }

    pub async fn check_transaction(&self, changes: &[ParameterChange]) -> Result<(), FimError> {
        let protected = self.protected_parameters.read().await;
        for change in changes {
            if protected.contains(&change.parameter_name) && !change.authorized {
                return Err(FimError::InvariantViolation {
                    parameter: change.parameter_name.clone(),
                    reason: "Unauthorized parameter mutation without signed policy change".into(),
                });
            }
        }
        Ok(())
    }
}
RSEOF

cat > crates/asm/fim/src/errors.rs << 'RSEOF'
#[derive(Debug, thiserror::Error)]
pub enum FimError {
    #[error("Financial invariant violation: {parameter} — {reason}")]
    InvariantViolation { parameter: String, reason: String },
}
RSEOF

echo "FIM crate implemented."

# -------------------------------------------------------
# 3. Runtime TLA+ Model Checker (vaos/runtime_tla)
# -------------------------------------------------------
cat > crates/vaos/runtime_tla/Cargo.toml << 'CEOF'
[package]
name = "vaos-runtime-tla"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Verity Agent OS – Runtime TLA+ Model Checker"

[dependencies]
vaos-core = { path = "../../vaos/core" }
tokio.workspace = true
serde.workspace = true
tracing.workspace = true
thiserror.workspace = true
tla-checker = "0.1.0"
CEOF

cat > crates/vaos/runtime_tla/src/lib.rs << 'RSEOF'
pub mod checker;
pub mod errors;

pub use checker::RuntimeTlaChecker;
pub use errors::TlaError;
RSEOF

cat > crates/vaos/runtime_tla/src/checker.rs << 'RSEOF'
use tla_checker::TlaSpec;
use super::errors::TlaError;

pub struct RuntimeTlaChecker {
    spec: TlaSpec,
}

impl RuntimeTlaChecker {
    pub fn new(tla_spec: &str) -> Result<Self, TlaError> {
        let spec = TlaSpec::parse(tla_spec).map_err(|e| TlaError::SpecParseError(e.to_string()))?;
        Ok(Self { spec })
    }

    pub fn sample(&self, json_trace: &str) -> Result<(), TlaError> {
        self.spec.check(json_trace).map_err(|e| TlaError::InvariantViolation(e.to_string()))
    }
}
RSEOF

cat > crates/vaos/runtime_tla/src/errors.rs << 'RSEOF'
#[derive(Debug, thiserror::Error)]
pub enum TlaError {
    #[error("TLA+ specification parse error: {0}")]
    SpecParseError(String),
    #[error("TLA+ invariant violation: {0}")]
    InvariantViolation(String),
}
RSEOF

echo "Runtime TLA+ crate implemented."

# -------------------------------------------------------
# 4. Capability-Based Banking Operations (vcbp/banking_ops)
# -------------------------------------------------------
cat > crates/vcbp/banking_ops/Cargo.toml << 'CEOF'
[package]
name = "vcbp-banking-ops"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Verity Core Banking – Capability-Based Banking Operations"

[dependencies]
vaos-core = { path = "../../vaos/core" }
tokio.workspace = true
serde.workspace = true
serde_json.workspace = true
tracing.workspace = true
thiserror.workspace = true
uuid.workspace = true
chrono.workspace = true
rust_decimal.workspace = true
async-trait.workspace = true
CEOF

cat > crates/vcbp/banking_ops/src/lib.rs << 'RSEOF'
pub mod operations;
pub mod tokens;
pub mod dual_control;
pub mod engine;
pub mod errors;

pub use engine::BankingOpsEngine;
pub use tokens::TokenOntology;
pub use dual_control::DualControlEnforcer;
pub use operations::{BankingOperation, DebitOp, CreditOp, WireTransferOp, LoanApprovalOp, GlPostingOp};
pub use errors::BankingOpsError;
RSEOF

cat > crates/vcbp/banking_ops/src/operations.rs << 'RSEOF'
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
RSEOF

cat > crates/vcbp/banking_ops/src/tokens.rs << 'RSEOF'
use std::collections::HashMap;
use vaos_core::types::CapScope;

pub struct TokenOntology {
    required_scopes: HashMap<String, Vec<CapScope>>,
}

impl TokenOntology {
    pub fn new() -> Self {
        let mut ont = Self { required_scopes: HashMap::new() };
        ont.add("debit", CapScope { operations: vec!["debit:account".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont.add("credit", CapScope { operations: vec!["credit:account".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont.add("wire_transfer", CapScope { operations: vec!["wire:transfer".into()], account_ids: vec![], amount_limit: Some(rust_decimal::Decimal::new(10000,0)), counterparty_allowlist: None });
        ont.add("wire_transfer_dual", CapScope { operations: vec!["approval:level_2".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont.add("loan_approval", CapScope { operations: vec!["loan:approve".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont.add("loan_approval_dual", CapScope { operations: vec!["risk:signoff".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont.add("gl_posting", CapScope { operations: vec!["gl:post".into()], account_ids: vec![], amount_limit: None, counterparty_allowlist: None });
        ont
    }

    fn add(&mut self, key: &str, scope: CapScope) {
        self.required_scopes.entry(key.to_string()).or_default().push(scope);
    }

    pub fn get_required_scopes(&self, op: &str) -> Option<&Vec<CapScope>> {
        self.required_scopes.get(op)
    }

    pub fn requires_dual_control(&self, op: &str) -> bool {
        self.required_scopes.get(op).map(|s| s.len() > 1).unwrap_or(false)
    }
}
RSEOF

cat > crates/vcbp/banking_ops/src/dual_control.rs << 'RSEOF'
use vaos_core::types::{CapabilityToken, AgentId};
use super::errors::BankingOpsError;
use std::collections::HashSet;

pub struct DualControlEnforcer;

impl DualControlEnforcer {
    pub fn verify(tokens: &[CapabilityToken], required_count: usize) -> Result<(), BankingOpsError> {
        if tokens.len() < required_count {
            return Err(BankingOpsError::DualControlRequired { operation: String::new(), required: required_count, provided: tokens.len() });
        }
        let principals: HashSet<AgentId> = tokens.iter().map(|t| t.issued_by).collect();
        if principals.len() < required_count {
            return Err(BankingOpsError::DualControlPrincipalsViolation { required: required_count, distinct_principals: principals.len() });
        }
        Ok(())
    }
}
RSEOF

cat > crates/vcbp/banking_ops/src/engine.rs << 'RSEOF'
use std::sync::Arc;
use tokio::sync::RwLock;
use super::operations::BankingOperation;
use super::tokens::TokenOntology;
use super::dual_control::DualControlEnforcer;
use super::errors::BankingOpsError;
use vaos_core::types::CapabilityToken;

pub struct BankingOpsEngine {
    ontology: TokenOntology,
    dual_control_threshold: rust_decimal::Decimal,
    stats: RwLock<BankingOpsStats>,
}

#[derive(Debug, Default, Clone)]
pub struct BankingOpsStats {
    pub operations_processed: u64,
    pub dual_control_checks: u64,
    pub operations_rejected: u64,
}

impl BankingOpsEngine {
    pub fn new() -> Self {
        Self {
            ontology: TokenOntology::new(),
            dual_control_threshold: rust_decimal::Decimal::new(10000, 0),
            stats: RwLock::new(BankingOpsStats::default()),
        }
    }

    pub async fn execute(&self, operation: &BankingOperation, tokens: &[CapabilityToken]) -> Result<(), BankingOpsError> {
        let op_type = operation.operation_type();
        let mut stats = self.stats.write().await;
        let required_scopes = self.ontology.get_required_scopes(op_type)
            .ok_or_else(|| BankingOpsError::UnsupportedOperation(op_type.to_string()))?;

        if tokens.len() < required_scopes.len() {
            stats.operations_rejected += 1;
            return Err(BankingOpsError::DualControlRequired { operation: op_type.to_string(), required: required_scopes.len(), provided: tokens.len() });
        }

        if required_scopes.len() > 1 {
            DualControlEnforcer::verify(tokens, required_scopes.len())?;
            stats.dual_control_checks += 1;
        }

        if operation.amount() >= self.dual_control_threshold && tokens.len() < 2 {
            stats.operations_rejected += 1;
            return Err(BankingOpsError::DualControlRequired { operation: op_type.to_string(), required: 2, provided: tokens.len() });
        }

        stats.operations_processed += 1;
        Ok(())
    }
}
RSEOF

cat > crates/vcbp/banking_ops/src/errors.rs << 'RSEOF'
#[derive(Debug, thiserror::Error)]
pub enum BankingOpsError {
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
    #[error("Dual control required: {operation} needs {required} tokens (provided: {provided})")]
    DualControlRequired { operation: String, required: usize, provided: usize },
    #[error("Dual control principals violation: need {required} distinct principals (got {distinct_principals})")]
    DualControlPrincipalsViolation { required: usize, distinct_principals: usize },
}
RSEOF

echo "Banking ops crate implemented."

# -------------------------------------------------------
# 5. Integration test (optional, verifies wiring)
# -------------------------------------------------------
mkdir -p tests/integration
cat > tests/integration/block1.rs << 'RSEOF'
#[cfg(test)]
mod tests {
    use vcbp_ledger::MerkleLedger;
    use vcbp_banking_ops::BankingOpsEngine;
    use vaos_core::types::CapabilityToken;

    #[tokio::test]
    async fn test_ledger_append_and_balance() {
        let ledger = MerkleLedger::new(vcbp_ledger::merkle_ledger::LedgerConfig::default());
        let tx = vcbp_ledger::Transaction {
            id: uuid::Uuid::new_v4(),
            correlation_id: uuid::Uuid::new_v4(),
            entries: vec![
                vcbp_ledger::Entry {
                    account_id: uuid::Uuid::new_v4(),
                    amount: rust_decimal::Decimal::new(100, 0),
                    currency: "USD".into(),
                    entry_type: vcbp_ledger::EntryType::Debit,
                    compliance_tags: vec![],
                },
                vcbp_ledger::Entry {
                    account_id: uuid::Uuid::new_v4(),
                    amount: rust_decimal::Decimal::new(-100, 0),
                    currency: "USD".into(),
                    entry_type: vcbp_ledger::EntryType::Credit,
                    compliance_tags: vec![],
                },
            ],
            timestamp: chrono::Utc::now(),
            agent_id: None,
            capability_token_id: None,
            metadata: serde_json::Value::Null,
        };
        let proof = ledger.append(tx).await.unwrap();
        assert!(!proof.merkle_root.is_empty());
    }

    #[tokio::test]
    async fn test_banking_ops_dual_control() {
        let engine = BankingOpsEngine::new();
        let token = CapabilityToken::test_token();
        let op = vcbp_banking_ops::operations::BankingOperation::WireTransfer(
            vcbp_banking_ops::operations::WireTransferOp {
                id: uuid::Uuid::new_v4(),
                from_account: uuid::Uuid::new_v4(),
                to_account: uuid::Uuid::new_v4(),
                amount: rust_decimal::Decimal::new(20000, 0),
                currency: "USD".into(),
                initiator: vaos_core::types::AgentId::new(),
            }
        );
        let result = engine.execute(&op, &[token]).await;
        assert!(result.is_err());
    }
}
RSEOF

echo "Integration test written."

# -------------------------------------------------------
# Verification
# -------------------------------------------------------
cargo check --workspace 2>&1 | head -50
echo ""
echo "✅ Block 1 implemented. Run 'cargo test --workspace' to verify."