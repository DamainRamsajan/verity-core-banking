#!/bin/bash
set -e

echo "============================================"
echo "  MASTER BUILD 10 – Frontend Gateway (verity-gateway)"
echo "============================================"

# -------------------------------------------------------
# 1. Create the verity-gateway crate
# -------------------------------------------------------
mkdir -p crates/verity-gateway/src

cat > crates/verity-gateway/Cargo.toml << 'CEOF'
[package]
name = "verity-gateway"
version.workspace = true
edition.workspace = true
license.workspace = true
description = "Verity – Frontend Gateway (dashboard, IAM, API proxy)"

[[bin]]
name = "verity-gateway"
path = "src/main.rs"

[dependencies]
verity-core-api = { path = "../verity-core-api" }
tokio.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
axum.workspace = true
tower-http.workspace = true
tower.workspace = true
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
anyhow = "1"
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }
rust-embed = "8"
toml = "0.8"

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
opt-level = "z"
CEOF

# -------------------------------------------------------
# 2. Gateway source files
# -------------------------------------------------------

cat > crates/verity-gateway/src/main.rs << 'RSEOF'
use clap::Parser;
use std::path::PathBuf;

mod server;
mod config;
mod auth;
mod proxy;
mod health;

#[derive(Parser)]
#[command(name = "verity-gateway", about = "Verity Frontend Gateway")]
struct Cli {
    /// Path to configuration file
    #[arg(long, default_value = "/etc/verity/gateway.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or("info".into())
        )
        .init();

    let cli = Cli::parse();
    let cfg = config::GatewayConfig::load(&cli.config)?;
    tracing::info!(?cfg.bind, core = %cfg.core_url, "Gateway starting");
    server::run(cfg).await
}
RSEOF

cat > crates/verity-gateway/src/config.rs << 'RSEOF'
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct GatewayConfig {
    pub bind: String,
    pub core_url: String,
    #[serde(default)]
    pub iam: Option<IamConfig>,
    #[serde(default)]
    pub dashboard_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IamConfig {
    pub iam_type: String,
    pub ldap_url: Option<String>,
    pub oidc_issuer: Option<String>,
    pub oidc_client_id: Option<String>,
}

impl GatewayConfig {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn default_bind() -> String { "0.0.0.0:443".into() }
    pub fn default_core_url() -> String { "http://127.0.0.1:8081".into() }
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            bind: Self::default_bind(),
            core_url: Self::default_core_url(),
            iam: None,
            dashboard_path: None,
        }
    }
}
RSEOF

cat > crates/verity-gateway/src/server.rs << 'RSEOF'
use std::net::SocketAddr;
use axum::{
    Router,
    routing::{get, any},
    response::Html,
};
use tower_http::{
    cors::{CorsLayer, Any},
    compression::CompressionLayer,
    trace::TraceLayer,
    services::ServeDir,
};
use crate::config::GatewayConfig;
use crate::health;
use crate::proxy;

pub async fn run(cfg: GatewayConfig) -> anyhow::Result<()> {
    let bind: SocketAddr = cfg.bind.parse()?;
    let core_url = cfg.core_url.clone();
    let dashboard_dir = cfg.dashboard_path.clone()
        .unwrap_or_else(|| "dashboard/dist".into());

    // Build the router
    let mut app = Router::new()
        .route("/health", get(health::health_check))
        .route("/ready", get(health::ready_check))
        .route("/metrics", get(health::metrics));

    // Dashboard: try embedded first, then fall back to local dir
    if dashboard_dir_exists(&dashboard_dir) {
        tracing::info!(%dashboard_dir, "Serving dashboard from filesystem");
        app = app.fallback_service(
            ServeDir::new(&dashboard_dir).not_found_service(
                axum::routing::get(|| async { Html::from(dashboard_fallback()) })
            )
        );
    } else {
        tracing::info!("Dashboard not found – serving API gateway only");
        app = app.fallback(proxy::proxy_to_core(core_url.clone()));
    }

    // Proxy all /api/* requests to Core
    let proxy_router = Router::new()
        .route("/api/*path", any(proxy::proxy_handler))
        .with_state(core_url);

    app = app.merge(proxy_router);

    // Layers
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    app = app.layer(cors)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    tracing::info!("Gateway listening on {}", bind);
    let listener = tokio::net::TcpListener::bind(bind).await?;

    // Graceful shutdown
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        let _ = tx.send(());
    });

    axum::serve(listener, app)
        .with_graceful_shutdown(async { rx.await.ok(); })
        .await?;

    Ok(())
}

fn dashboard_dir_exists(path: &str) -> bool {
    let p = std::path::Path::new(path);
    p.exists() && p.is_dir()
}

fn dashboard_fallback() -> String {
    r#"<!DOCTYPE html>
<html><head><title>Verity Gateway</title></head>
<body style="font-family:sans-serif;text-align:center;padding:40px;">
<h1>Verity Core Banking Platform</h1>
<p>The dashboard will appear here after building the frontend.</p>
<p>Run: <code>cd dashboard && npm run build</code></p>
</body></html>"#.to_string()
}
RSEOF

cat > crates/verity-gateway/src/proxy.rs << 'RSEOF'
use axum::{
    extract::{State, Path, Request},
    response::{Response, IntoResponse},
    body::Body,
    http::{StatusCode, Method},
};
use std::collections::HashMap;

/// Proxy all /api/* requests to the Core binary.
pub async fn proxy_handler(
    State(core_url): State<String>,
    Path(path): Path<String>,
    req: Request,
) -> Result<Response, StatusCode> {
    let full_url = format!("{}/api/{}", core_url.trim_end_matches('/'), path);

    // Copy headers
    let mut headers = reqwest::header::HeaderMap::new();
    for (k, v) in req.headers() {
        if let Ok(k) = reqwest::header::HeaderName::from_bytes(k.as_str().as_bytes()) {
            if let Ok(v) = v.to_str() {
                headers.insert(k, reqwest::header::HeaderValue::from_str(v).unwrap());
            }
        }
    }

    let client = reqwest::Client::new();
    let resp = match *req.method() {
        Method::GET => client.get(&full_url).headers(headers).send().await,
        Method::POST => {
            let body_bytes = axum::body::to_bytes(req.into_body(), 10*1024*1024).await
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            client.post(&full_url).headers(headers).body(body_bytes).send().await
        }
        Method::PUT => {
            let body_bytes = axum::body::to_bytes(req.into_body(), 10*1024*1024).await
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            client.put(&full_url).headers(headers).body(body_bytes).send().await
        }
        Method::DELETE => client.delete(&full_url).headers(headers).send().await,
        _ => return Err(StatusCode::METHOD_NOT_ALLOWED),
    };

    match resp {
        Ok(r) => {
            let mut builder = Response::builder().status(r.status());
            for (k, v) in r.headers() {
                if let Some(k) = k {
                    builder = builder.header(k.as_str(), v.to_str().unwrap_or(""));
                }
            }
            let body_bytes = r.bytes().await.unwrap_or_default();
            Ok(builder.body(Body::from(body_bytes)).unwrap())
        }
        Err(_) => Err(StatusCode::BAD_GATEWAY),
    }
}

/// Fallback service for non‑API routes (used when dashboard not present).
pub fn proxy_to_core(core_url: String) -> axum::routing::MethodRouter {
    axum::routing::any(move || {
        let url = core_url.clone();
        async move {
            format!("Verity Gateway – Core at {}. Dashboard not yet built.", url)
        }
    })
}
RSEOF

cat > crates/verity-gateway/src/auth.rs << 'RSEOF'
//! Enterprise IAM Authentication Bridge (LDAPS / OIDC)
//!
//! Placeholder for production IAM integration.
//! Source: ARC42 v22 ADR‑024

/// Verify operator credentials and return a capability token request.
pub async fn authenticate(
    _username: &str,
    _password: &str,
    _iam_config: &crate::config::IamConfig,
) -> anyhow::Result<String> {
    // In production:
    //   LDAPS: bind to directory, verify group membership
    //   OIDC:  validate id_token, extract claims, map groups to roles
    // For now, return a placeholder token
    Ok("capability-token-placeholder".into())
}
RSEOF

cat > crates/verity-gateway/src/health.rs << 'RSEOF'
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
RSEOF

echo "  ✓ verity-gateway crate created"

# -------------------------------------------------------
# 3. Add verity-gateway to workspace members
# -------------------------------------------------------
if ! grep -q '"crates/verity-gateway"' Cargo.toml; then
    sed -i '/^members = \[/a \    "crates/verity-gateway",' Cargo.toml
fi

echo "  ✓ Workspace Cargo.toml updated"

# -------------------------------------------------------
# 4. Create default Gateway configuration
# -------------------------------------------------------
mkdir -p config
cat > config/gateway.toml << 'CEOF'
bind = "0.0.0.0:443"
core_url = "http://127.0.0.1:8081"

[iam]
iam_type = "ldaps"
ldap_url = "ldaps://ldap.internal:636"
CEOF

echo "  ✓ Default Gateway configuration created"

# -------------------------------------------------------
# 5. Verify compilation
# -------------------------------------------------------
echo ""
echo "============================================"
echo "  Verifying compilation"
echo "============================================"
cargo check -p verity-gateway 2>&1
echo ""
echo " MASTER BUILD 10 COMPLETE"
echo "   - verity-gateway: Frontend Gateway binary"
echo "   - Serves dashboard from filesystem (when built)"
echo "   - Proxies /api/* to Core on :8081"
echo "   - Health/readiness/metrics endpoints"
echo "   - IAM authentication bridge (placeholder ready)"
echo "   Next: cargo test --workspace"
echo "   Then: master_build_11.sh (Operational CLIs)"