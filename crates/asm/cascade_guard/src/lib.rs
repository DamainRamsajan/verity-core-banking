//! # Verity ASM — CascadeGuard Inter-Agent Circuit Breaker
//!
//! CLOSED→OPEN→HALF_OPEN state machine on all inter-agent channels.
//! When error rate exceeds threshold, circuit trips and channel halts.
//! Data validity checks at every agent-to-agent handoff.
//!
//! Source: ARC42 v20.0 Addendum v17.0 §A-16

pub mod engine;
pub mod channels;
pub mod types;
pub mod errors;

pub use engine::CascadeGuard;
pub use types::{CircuitState, ChannelId, CircuitConfig};
pub use errors::CascadeError;
