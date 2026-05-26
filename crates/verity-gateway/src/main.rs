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
    #[arg(long, default_value = "/etc/verity/gateway.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .init();

    let cli = Cli::parse();
    let cfg = config::GatewayConfig::load(&cli.config)?;
    tracing::info!(?cfg.bind, core = %cfg.core_url, "Gateway starting");
    server::run(cfg).await
}