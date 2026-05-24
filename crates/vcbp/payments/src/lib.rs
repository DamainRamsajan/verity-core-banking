//! # Verity Core Banking — Payment Rail Connectors
//!
//! Native integration with all major payment rails. Every rail is
//! capability‑gated and circuit‑breaker‑protected.
//!
//! ## Supported Rails
//! - **ISO 20022**: native MX message generation (structured address compliant
//!   for the November 2026 deadline)
//! - **FedNow**: direct FedNow Service integration with Network Intelligence API
//!   for pre‑transaction risk assessment (1,700+ institutions, $10M limit)
//! - **SWIFT Blockchain Bridge**: Hyperledger Besu EVM integration for tokenized
//!   deposit settlement (40+ banks, 24/7 cross‑border)
//! - **ACH, FedWire, CHIPS, RTP**: full US payment rail suite
//! - **Project Keystone**: bank‑owned digital money network (6 U.S. banks live)
//!
//! ## Architecture
//! - Strategy pattern per rail: common `PaymentRail` trait
//! - Circuit breakers on all external rails (CLOSED→OPEN→HALF_OPEN)
//! - Capability tokens required for all payment operations
//! - Smart routing: selects optimal rail by value, urgency, cost, counterparty
//!
//! Source: ARC42 v20.0 §3 VCBP Payment Rail Connectors, ADR‑015

pub mod rail;
pub mod engine;
pub mod router;
pub mod circuit;
pub mod errors;

pub mod rails;

pub use rail::PaymentRail;
pub use engine::PaymentEngine;
pub use router::SmartRouter;
pub use circuit::RailCircuitBreaker;
pub use errors::PaymentError;
