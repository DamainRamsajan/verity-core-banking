use worker::*;

/// JWT-based authentication middleware for Workers.
pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> Self { Self }

    /// Validate a JWT bearer token from the Authorization header.
    pub fn validate(&self, req: &HttpRequest) -> Result<Option<String>> {
        let auth_header = req.headers().get("Authorization")?;
        if let Some(header) = auth_header {
            if header.starts_with("Bearer ") {
                let token = &header[7..];
                // In production: verify JWT signature via Supabase Auth
                return Ok(Some(token.to_string()));
            }
        }
        Ok(None)
    }
}
