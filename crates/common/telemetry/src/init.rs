#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub service_name: String,
    pub service_version: String,
    pub otlp_endpoint: String,
    pub log_level: String,
    pub sample_rate: f64,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            service_name: "verity-core-banking".into(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            otlp_endpoint: "http://localhost:4317".into(),
            log_level: "info".into(),
            sample_rate: 0.10,
        }
    }
}

/// Initialize the telemetry pipeline.
/// Full OpenTelemetry integration is deferred to a feature-gated module.
pub fn init_telemetry(config: &TelemetryConfig) -> Result<(), super::errors::TelemetryError> {
    let subscriber = tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| super::errors::TelemetryError::InitFailed(e.to_string()))?;

    tracing::info!(
        service = %config.service_name,
        version = %config.service_version,
        "Telemetry initialized"
    );

    Ok(())
}