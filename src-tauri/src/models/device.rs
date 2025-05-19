use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

// 使用 Arc 来共享设备配置信息
pub type DeviceMap = Arc<DashMap<String, DeviceInfo>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub path: String,
    #[serde(flatten)]
    pub device: HashMap<String, DeviceConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceConfig {
    pub name: String,
    pub addr: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DevicePayload {
    pub sn: Option<String>,
    pub name: Option<String>,
    pub mac: Option<String>,
    pub ip: Option<String>,
    pub os: Option<String>,
    pub broadcast_interval: Option<u32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DeviceInfo {
    pub sn: String,
    pub name: String,
    pub mac: String,
    pub ip: String,
    pub os: String,
    pub status: String,
    pub timestamp: String,
    pub broadcast_interval: u32,
}

impl DeviceInfo {
    pub fn from_payload(payload: DevicePayload, source_ip: String) -> Self {
        // let now = std::time::SystemTime::now()
        //     .duration_since(std::time::UNIX_EPOCH)
        //     .unwrap()
        //     .as_secs();
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        DeviceInfo {
            sn: payload.sn.unwrap_or_else(|| "Unknown SN".to_string()),
            name: payload.name.unwrap_or_else(|| "Unknown Device".to_string()),
            mac: payload.mac.unwrap_or_else(|| "Unknown MAC".to_string()),
            ip: payload.ip.unwrap_or_else(|| source_ip),
            os: payload.os.unwrap_or_else(|| "Unknown OS".to_string()),
            status: "Online".to_string(),
            timestamp,
            broadcast_interval: payload.broadcast_interval.unwrap_or_else(|| 30),
        }
    }
}
