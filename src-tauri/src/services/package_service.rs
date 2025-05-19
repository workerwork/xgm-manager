use crate::models::package::{PackageDetail, PackageInfo};
use reqwest::blocking::{multipart, Client};
use std::fs::File;
use crate::utils::config_loader::get_config;
use crate::utils::http_utils::send_request;


/// 获取安装的 Python 包
// pub fn _list_packages_local() -> Result<Vec<PackageInfo>, String> {
//     let output = Command::new("pip")
//         .args(["list", "--format=json"])
//         .output()
//         .map_err(|e| format!("执行 pip 命令失败: {}", e))?;

//     if !output.status.success() {
//         return Err(format!("pip 命令执行失败，状态码: {}", output.status));
//     }

//     let stdout = String::from_utf8(output.stdout).map_err(|e| format!("输出解析失败: {}", e))?;

//     // 使用 serde_json 直接解析为 Vec<PackageInfo>
//     let packages: Vec<PackageInfo> = serde_json::from_str(&stdout).map_err(|e| format!("JSON 解析失败: {}", e))?;

//     Ok(packages)
// }




/// 从远程获取安装的 Python 包列表
pub fn list_packages(device_ip: String) -> Result<Vec<PackageInfo>, String> {
    let http_port = get_config().server.http_port;
    let url = format!("http://{}:{}/package", device_ip, http_port);

    // 使用 send_request 发送 GET 请求并解析响应为 Vec<PackageInfo>
    send_request(&url, reqwest::Method::GET, None)
}

/// 从远程获取 Python 包详情
pub fn fetch_package_detail(pkg_name: String, device_ip: String) -> Result<PackageDetail, String> {
    let http_port = get_config().server.http_port;
    let url = format!("http://{}:{}/package/detail?name={}", device_ip, http_port, pkg_name);

    // 使用 send_request 发送 GET 请求并解析响应为 PackageDetail
    send_request(&url, reqwest::Method::GET, None)
}

/// 上传 Python 包到远程
pub fn upload_file(file_path: String, device_ip: String) -> Result<String, String> {
    let http_port = get_config().server.http_port;
    let base_url = format!("http://{}:{}", device_ip, http_port);
    let client = Client::new();
    let _file = File::open(&file_path).map_err(|e| format!("无法打开文件: {}", e))?;

    let form = multipart::Form::new()
        .file("file", &file_path)
        .map_err(|e| format!("构建 multipart 表单失败: {}", e))?;

    let url = format!("{}/package/update", base_url.trim_end_matches('/'));
    let response = client
        .put(&url)
        .multipart(form)
        .send()
        .map_err(|e| format!("发送请求失败: {}", e))?;

    if response.status().is_success() {
        Ok("上传成功".to_string())
    } else {
        Err(format!(
            "上传失败，状态码: {}, 内容: {:?}",
            response.status(),
            response.text().unwrap_or_default()
        ))
    }
}
