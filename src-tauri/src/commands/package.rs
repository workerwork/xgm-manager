use crate::models::package::{PackageDetail, PackageInfo};
use crate::services::package_service;


#[tauri::command]
pub fn get_package_list(device_ip: String) -> Result<Vec<PackageInfo>, String> {
    // 调用 service 中的 list_packages 函数
    // package_service::list_packages_local()
    package_service::list_packages(device_ip)
}


#[tauri::command]
pub fn get_package_detail(pkg_name: String, device_ip: String) -> Result<PackageDetail, String> {
    package_service::fetch_package_detail(pkg_name, device_ip)
}


#[tauri::command]
pub fn upload_package(file_path: String, device_ip: String) -> Result<String, String> {
    package_service::upload_file(file_path, device_ip)
}