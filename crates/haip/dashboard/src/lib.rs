//! # Verity HAIP — Delegative Governance Dashboard Backend
//!
//! Provides the API for the human principal to set explicit boundaries for
//! delegated agents: spending limits, approval thresholds, time windows,
//! counterparty restrictions, and jurisdiction constraints. All agent
//! activity is surfaced with progressive disclosure.
//!
//! ## Features
//! - Configure per‑agent delegation policies
//! - Real‑time activity feed with risk scores
//! - One‑click override for any agent action
//! - Session‑scoped access tokens (Keycard pattern, OAuth 2.0 Token Exchange)
//! - Apple Principle enforced: agent never deviates without notifying user
//!
//! Source: ARC42 v20.0 Addendum v16.0 §A-3

pub mod engine;
pub mod policy;
pub mod activity;
pub mod session;
pub mod types;
pub mod errors;

pub use engine::DashboardEngine;
pub use policy::DelegationPolicy;
pub use activity::ActivityFeed;
pub use session::SessionBridge;
pub use types::{AgentBoundary, ActivityEvent, OverrideAction};
pub use errors::DashboardError;
