//! # Verity Core API – Shared Types
//!
//! Request and response DTOs shared between the Gateway and Core.
//! Source: ARC42 v22

pub mod accounts;
pub mod payments;
pub mod agents;
pub mod compliance;
pub mod ledger;
pub mod common;

pub use common::*;
