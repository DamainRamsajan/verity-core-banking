use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub backend: SandboxBackend,
    pub max_runtime_ms: u64,
    pub max_memory_mb: u64,
    pub network_allowed: bool,
    pub filesystem_writable: bool,
    pub allowed_syscalls: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SandboxBackend { GVisor, Firecracker, Wasm, Process, Tdx, Sev, Auto }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxResult {
    pub execution_id: Uuid,
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub runtime_ms: u64,
    pub strength_score: u8,
    pub security_events: Vec<SecurityEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_type: String,
    pub severity: u8,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolDescriptor {
    pub tool_name: String,
    pub description: String,
    pub signature_hash: [u8; 32],
    pub registered_by: String,
    pub validated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationStatus { Valid, Mismatch, Unsigned, Revoked }
