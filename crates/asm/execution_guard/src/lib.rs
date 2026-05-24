//! # Verity ASM — ExecutionGuard Tool Execution Sandbox
//!
//! Mandatory sandbox for all agent-generated code and validated MCP tool
//! invocation. Implements gVisor-backed isolation with multi-turn trajectory
//! analysis for Boiling the Frog incremental attack detection.
//!
//! ## Sandbox Backends (via kavach)
//! - gVisor (runsc) — user-space kernel, used at Tencent for millions of sandboxes
//! - Firecracker microVM — hardware-level isolation
//! - WASM (wasmtime) — lightweight, near-native performance
//! - Process — basic isolation for trusted workloads
//! - TDX/SEV — TEE-enforced
//!
//! ## MCP Tool Descriptor Validation
//! All MCP tool descriptors are validated against a signed registry.
//! Tool descriptions are treated as untrusted metadata — any mismatch
//! blocks execution.
//!
//! Source: ARC42 v20.0 Addendum v17.0 §A-12

pub mod engine;
pub mod backends;
pub mod mcp_validator;
pub mod trajectory;
pub mod types;
pub mod errors;

pub use engine::ExecutionGuard;
pub use types::{SandboxConfig, SandboxResult, McpToolDescriptor, ValidationStatus};
pub use errors::GuardError;
