use crate::models::log::{GetLogsResponse, ClearLogsResponse};
use crate::utils::config_loader::get_config;
use crate::utils::http_utils::send_request;

pub fn get_log_list(device_ip: String) -> Result<Vec<String>, String> {
    let http_port = get_config().server.http_port;
    let url = format!("http://{}:{}/log", device_ip, http_port);

    // 发送 GET 请求，获取响应并返回
    let response: GetLogsResponse = send_request(&url, reqwest::Method::GET, None)?;

    Ok(response.logs)
}

pub fn clear_log_list(device_ip: String) -> Result<(), String> {
    let http_port = get_config().server.http_port;
    let url = format!("http://{}:{}/log/clear", device_ip, http_port);

    // 发送 DELETE 请求，获取响应并返回
    let response: ClearLogsResponse = send_request(&url, reqwest::Method::DELETE, None)?;
    Ok(())
}


