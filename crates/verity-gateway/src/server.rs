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
