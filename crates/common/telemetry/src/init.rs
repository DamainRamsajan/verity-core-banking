use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_sdk::Resource;
use opentelemetry::KeyValue;

/// Telemetry configuration.
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

/// Initialize the OpenTelemetry pipeline.
pub fn init_telemetry(config: &TelemetryConfig) -> Result<(), super::errors::TelemetryError> {
    let resource = Resource::new(vec![
        KeyValue::new("service.name", config.service_name.clone()),
        KeyValue::new("service.version", config.service_version.clone()),
        KeyValue::new("deployment.environment", std::env::var("ENVIRONMENT").unwrap_or("development".into())),
    ]);

    // Initialize tracer provider
    let _tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(
            opentelemetry_sdk::trace::config()
                .with_resource(resource)
                .with_sampler(opentelemetry_sdk::trace::Sampler::TraceIdRatioBased(config.sample_rate))
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    // Initialize structured logging subscriber
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(&config.log_level)
        .with_target(true)
        .with_thread_ids(true)
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!(
        service = %config.service_name,
        version = %config.service_version,
        "Telemetry initialized"
    );

    Ok(())
}

use tracing_subscriber;
