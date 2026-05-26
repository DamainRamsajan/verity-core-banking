use axum::{Router, routing::{get, post, put}, response::Json};

use verity_core_api::common::{HealthResponse, HealthComponents};

mod accounts;
mod payments;
mod agents;
mod compliance;
mod ledger;

pub fn build_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/accounts", post(accounts::create_account))
        .route("/api/v1/accounts/:id", get(accounts::get_account))
        .route("/api/v1/transfers", post(accounts::create_transfer))
        .route("/api/v1/payments", post(payments::create_payment))
        .route("/api/v1/payments", get(payments::list_payments))
        .route("/api/v1/agents", get(agents::list_agents))
        .route("/api/v1/agents/:id", get(agents::get_agent))
        .route("/api/v1/agents/:id/boundaries", put(agents::set_boundaries))
        .route("/api/v1/agents/:id/activity", get(agents::agent_activity))
        .route("/api/v1/compliance/reports", get(compliance::list_reports))
        .route("/api/v1/compliance/reports/zk-proof", post(compliance::generate_zk_proof))
        .route("/api/v1/ledger/proof/:tx_id", get(ledger::get_merkle_proof))
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".into(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        components: HealthComponents {
            ledger: "operational".into(),
            database: "connected".into(),
            agents: "running".into(),
            tee: std::env::var("TEE_MODE").unwrap_or("simulation".into()),
        },
    })
}