use crate::models::device::{DeviceConfig, DeviceInfo, DeviceMap};
use crate::utils::config_loader::get_config;
use crate::utils::http_utils::send_request;
use std::collections::HashMap;



/// 获取所有设备列表
pub fn get_device_list(devices: &DeviceMap) -> Vec<DeviceInfo> {
    devices.iter().map(|entry| entry.value().clone()).collect()
}

/// 删除设备
pub fn remove_device(sn: &str, devices: &DeviceMap) -> Result<String, String> {
    if devices.remove(sn).is_some() {
        Ok(format!("设备 {} 已删除", sn))
    } else {
        Err(format!("设备 {} 未找到", sn))
    }
}

/// 获取远程配置
pub fn fetch_device_config(device_name: &str, device_ip: &str) -> Result<DeviceConfig, String> {
    let http_port = get_config().server.http_port;
    let url = format!("http://{}:{}/config", device_ip, http_port);
    
    let config_map: HashMap<String, serde_json::Value> = send_request(&url, reqwest::Method::GET, None)?;
    
    config_map.get(device_name)
        .ok_or_else(|| format!("未找到设备 {} 的配置", device_name))
        .and_then(|device_data| {
            serde_json::from_value(device_data.clone()).map_err(|e| format!("解析设备配置失败: {}", e))
        })
}

/// 更新设备配置
pub fn update_device_config(
    device_name: &str,
    device_ip: &str,
    config_data: &DeviceConfig,
) -> Result<DeviceConfig, String> {
    let http_port = get_config().server.http_port;
    let url = format!("http://{}:{}/config", device_ip, http_port);
    
    let mut config_map: HashMap<String, serde_json::Value> = send_request(&url, reqwest::Method::GET, None)?;

    // 如果设备配置不存在，先删除配置
    if !config_map.contains_key(device_name) {
        send_request::<serde_json::Value>(&url, reqwest::Method::DELETE, Some(&serde_json::json!({})))?;
        config_map.clear();
    }

    // 更新配置
    config_map.insert(
        device_name.to_string(),
        serde_json::to_value(config_data).unwrap(),
    );

    // 提交更新
    send_request::<serde_json::Value>(&url, reqwest::Method::PUT, Some(&serde_json::json!(config_map)))?;

    Ok(config_data.clone())
}

/// 清空设备配置
pub fn clear_device_config(device_ip: &str) -> Result<String, String> {
    let http_port = get_config().server.http_port;
    let url = format!("http://{}:{}/config", device_ip, http_port);
    
    send_request::<serde_json::Value>(&url, reqwest::Method::DELETE, None)?;
    Ok(format!("配置已清除: {}", device_ip))
}
