use axum::{
    extract::{State, Path, Request},
    response::Response,
    body::Body,
    http::{StatusCode, Method},
};

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
        headers.insert(k.clone(), v.clone());
    }

    let client = reqwest::Client::new();
    let method = req.method().clone();
    let resp = match method {
        Method::GET => client.get(&full_url).headers(headers).send().await,
        Method::POST => {
            let body_bytes = axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            client
                .post(&full_url)
                .headers(headers)
                .body(body_bytes)
                .send()
                .await
        }
        Method::PUT => {
            let body_bytes = axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024)
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            client
                .put(&full_url)
                .headers(headers)
                .body(body_bytes)
                .send()
                .await
        }
        Method::DELETE => client.delete(&full_url).headers(headers).send().await,
        _ => return Err(StatusCode::METHOD_NOT_ALLOWED),
    };

    match resp {
        Ok(r) => {
            let status = r.status();
            let mut response = Response::builder().status(status);

            // Attach headers from the upstream response
            for (k, v) in r.headers() {
                if let Ok(v) = v.to_str() {
                    response = response.header(k.as_str(), v);
                }
            }

            let body_bytes = r.bytes().await.unwrap_or_default();
            Ok(response.body(Body::from(body_bytes)).unwrap())
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