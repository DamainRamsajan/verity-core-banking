//! # Verity Common — Shared Observability Utilities
//!
//! Provides unified telemetry infrastructure across all Verity crates:
//! - OpenTelemetry tracing with GenAI Semantic Conventions
//! - Structured logging with automatic span correlation
//! - PII-safe telemetry via telemetry-safe-core patterns
//! - Metrics export via OTLP
//!
//! Source: ARC42 v20.0 §6 Cross-Cutting Concepts (Observability)

pub mod init;
pub mod spans;
pub mod metrics;
pub mod types;
pub mod errors;

pub use init::TelemetryConfig;
pub use spans::SpanExt;
pub use types::TelemetryContext;
