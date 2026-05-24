//! agent route handler.

use worker::*;

pub async fn handle(_req: HttpRequest, _env: Env) -> Result<HttpResponse> {
    Response::ok(r#"{"status":"ok"}"#)
}
