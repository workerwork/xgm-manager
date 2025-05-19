use crate::services::log_service;

#[tauri::command]
pub fn get_log_list(device_ip: String) -> Result<Vec<String>, String> {
    log_service::get_log_list(device_ip)
}

#[tauri::command]
pub fn clear_log_list(device_ip: String) -> Result<(), String> {
    log_service::clear_log_list(device_ip)
}