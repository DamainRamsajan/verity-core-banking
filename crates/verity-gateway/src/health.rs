use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::json;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "verity-gateway",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn ready_check() -> impl IntoResponse {
    // Always ready in this basic implementation
    (StatusCode::OK, "ready")
}

pub async fn metrics() -> String {
    "# HELP gateway_uptime_seconds Gateway uptime in seconds\n# TYPE gateway_uptime_seconds gauge\ngateway_uptime_seconds 1\n".into()
}
