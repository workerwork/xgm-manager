use crate::models::network::NetworkConfig;
use crate::utils::config_loader::get_config;
use reqwest::{blocking::Client, Method};
use std::time::Duration;

pub fn update_network_config(device_ip: &str, ip: &str, netmask: &str) -> Result<String, String> {
    let http_port = get_config().server.http_port;
    let url = format!("http://{}:{}/network/update", device_ip, http_port);

    // 创建一个 NetworkConfig 结构体
    let config = NetworkConfig {
        ip: ip.to_string(),
        netmask: netmask.to_string(),
    };

    // 构建请求体
    let body = serde_json::json!({
        "ip": config.ip,
        "netmask": config.netmask,
    });

    let client = Client::builder()
        .timeout(Duration::from_millis(100)) // 超短超时
        .build()
        .unwrap();

    let _ = client
        .request(Method::POST, &url)
        .json(&body)
        .send(); // 不等待结果
    Ok("success".to_string())
}
