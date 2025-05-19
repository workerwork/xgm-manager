use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub jupyter: JupyterConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub http_port: u16,
    pub udp_port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JupyterConfig {
    pub workspace_dir: String,
}