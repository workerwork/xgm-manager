use crate::services::network_service;


#[tauri::command]
pub fn update_network_config(
    device_ip: String,
    ip: String,
    netmask: String,
) -> Result<String, String> {
    network_service::update_network_config(&device_ip, &ip, &netmask)
}
