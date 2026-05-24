//! # Verity Core Banking — BIAN v14.0 Domain Engine
//!
//! Implements all 328 BIAN Service Domains as bounded contexts with
//! session‑typed inter‑domain communication. Each domain is a Rust
//! struct implementing the `ServiceDomain` trait, ensuring strict
//! isolation (no direct cross‑domain DB access) and typed messaging.
//!
//! ## Architecture
//! - **328 bounded contexts** mapped to BIAN Service Landscape v14.0
//! - **Session‑typed channels**: McDermott‑Yoshida semantics (ESOP 2026)
//!   guarantee deadlock‑freedom at compile time
//! - **Domain registry**: dynamic discovery and routing
//! - **BIAN‑ServiceNow CSDM unified metamodel**: bidirectional traceability
//!   from strategy to APIs
//!
//! Source: ARC42 v20.0 §3 VCBP BIAN 14.0 Domain Engine, ADR‑014

pub mod domain;
pub mod engine;
pub mod registry;
pub mod channels;
pub mod errors;

// Example domain implementations
pub mod domains;

pub use domain::ServiceDomain;
pub use engine::BianDomainEngine;
pub use registry::DomainRegistry;
pub use channels::SessionTypedChannel;
pub use errors::DomainError;
