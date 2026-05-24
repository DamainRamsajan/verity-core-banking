use serde::{Deserialize, Serialize};

/// Telemetry context carried across async boundaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryContext {
    pub trace_id: String,
    pub span_id: String,
    pub service_name: String,
    pub correlation_id: uuid::Uuid,
}
