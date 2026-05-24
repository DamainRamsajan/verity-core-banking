//! # Verity Cloudflare Workers — Edge API Gateway
//!
//! Rust-compiled-to-WASM Workers providing the customer-facing API layer.
//! Routes requests to the sovereign VCBP core while handling authentication,
//! rate limiting, and real-time notifications at the edge.
//!
//! ## Architecture
//! - Rust WASM via worker-rs 0.6: cold starts under 5ms, 300+ global locations
//! - D1 (SQLite) for edge-local state, KV for session cache
//! - OpenTelemetry tracing export to observability backends
//! - Routes: health, auth, dashboard API, real-time WebSocket upgrades
//!
//! Source: ARC42 v20.0 §5 Deployment View

pub mod router;
pub mod middleware;
pub mod routes;

use worker::*;

/// Main Worker entry point.
#[worker::event(fetch)]
pub async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<HttpResponse> {
    console_error_panic_hook::set_once();
    let router = router::Router::new();
    router.handle(req, env).await
}

use console_error_panic_hook;
