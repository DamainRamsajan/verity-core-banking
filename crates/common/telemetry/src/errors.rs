#[derive(Debug, thiserror::Error)]
pub enum TelemetryError {
    #[error("OpenTelemetry initialization failed: {0}")]
    InitFailed(String),
    #[error("OTLP export failed: {0}")]
    ExportFailed(String),
}
