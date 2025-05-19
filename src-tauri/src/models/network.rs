
#[derive(Debug, serde::Serialize)]
pub struct NetworkConfig {
    pub ip: String,
    pub netmask: String,
}