//! # Verity Core Banking — ASL Product Definition Engine
//!
//! Compiles banking products from ASL (Agent Seed Language) source code into
//! safe, seedvm‑executable bytecode. Every product is verified at compile time
//! against regulatory invariants — incorrect products **cannot compile**.
//!
//! ## Architecture
//! - **ASL Compiler**: full S0‑S3 grammar stratification with P1‑P8 safety
//!   invariants enforced at compile time
//! - **Temporal Contracts**: LTL + SMT enforcement via KindHML for Reg DD
//!   interest calculation correctness and Reg Z disclosure timing
//! - **Product Bytecode**: compiled products execute on seedvm with
//!   sandboxed WASM execution
//!
//! ## Regulatory Coverage
//! - **Reg DD** (Truth in Savings): interest rate ≥ 0, APY calculation accuracy
//! - **Reg Z** (Truth in Lending): APR disclosure timing, fee transparency
//! - **Reg E** (Electronic Fund Transfers): error resolution within 10 business days
//! - **ECOA / FCRA**: fair lending and credit reporting compliance
//!
//! ## Safety Guarantees
//! - If an ASL product compiles, it satisfies all declared regulatory invariants
//! - Temporal properties are verified via SMT solving before deployment
//! - Products are capability‑governed at runtime (P3 enforcement)
//!
//! Source: ARC42 v20.0 §3 VCBP ASL Product Definition Engine, ADR‑001

pub mod compiler;
pub mod product;
pub mod temporal;
pub mod templates;
pub mod errors;

pub use compiler::AslProductCompiler;
pub use product::BankingProduct;
pub use temporal::TemporalContract;
pub use templates::{CheckingAccount, SavingsAccount, LoanProduct};
pub use errors::ProductError;
