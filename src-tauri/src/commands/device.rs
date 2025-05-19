use crate::models::device::{DeviceConfig, DeviceInfo, DeviceMap};
use crate::services::device_service;

#[tauri::command]
pub fn list_devices(devices: tauri::State<DeviceMap>) -> Vec<DeviceInfo> {
    device_service::get_device_list(&devices)
}


#[tauri::command]
pub fn delete_device(sn: String, devices: tauri::State<DeviceMap>) -> Result<String, String> {
    device_service::remove_device(&sn, &devices)
}


#[tauri::command]
pub fn get_device_config(device_name: String, device_ip: String) -> Result<DeviceConfig, String> {
    device_service::fetch_device_config(&device_name, &device_ip)
}


#[tauri::command]
pub fn update_device_config(
    device_name: String,
    device_ip: String,
    config_data: DeviceConfig,
) -> Result<DeviceConfig, String> {
    device_service::update_device_config(&device_name, &device_ip, &config_data)
}

#[tauri::command]
pub fn refresh_device_config(device_ip: String) -> Result<String, String> {
    device_service::clear_device_config(&device_ip)
}
