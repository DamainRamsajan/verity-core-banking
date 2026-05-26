use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct GatewayConfig {
    pub bind: String,
    pub core_url: String,
    #[serde(default)]
    pub iam: Option<IamConfig>,
    #[serde(default)]
    pub dashboard_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
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
