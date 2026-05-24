use worker::*;

/// Edge API router.
pub struct Router;

impl Router {
    pub fn new() -> Self { Self }

    pub async fn handle(&self, req: HttpRequest, env: Env) -> Result<HttpResponse> {
        let url = req.url()?;
        let path = url.path();

        match path {
            "/health" => self.health(),
            "/api/v1/auth/login" => self.handle_auth(req, env).await,
            "/api/v1/dashboard/summary" => self.handle_dashboard(req, env).await,
            "/api/v1/agent/activity" => self.handle_agent_activity(req, env).await,
            "/ws/realtime" => self.handle_ws_upgrade(req, env).await,
            _ => Response::error("Not Found", 404),
        }
    }

    fn health(&self) -> Result<HttpResponse> {
        Response::ok(serde_json::json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "service": "verity-workers"
        }).to_string())
    }

    async fn handle_auth(&self, _req: HttpRequest, _env: Env) -> Result<HttpResponse> {
        // Delegate to Supabase Auth Edge Function
        Response::ok(r#"{"message":"Auth endpoint"}"#)
    }

    async fn handle_dashboard(&self, _req: HttpRequest, _env: Env) -> Result<HttpResponse> {
        Response::ok(r#"{"message":"Dashboard API"}"#)
    }

    async fn handle_agent_activity(&self, _req: HttpRequest, _env: Env) -> Result<HttpResponse> {
        Response::ok(r#"{"message":"Agent activity"}"#)
    }

    async fn handle_ws_upgrade(&self, _req: HttpRequest, _env: Env) -> Result<HttpResponse> {
        Response::error("WebSocket upgrade", 426)
    }
}

use serde_json;
