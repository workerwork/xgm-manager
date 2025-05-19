use reqwest::{blocking::Client, Method};
use serde_json::Value;
use std::time::Duration;

pub fn send_request<T>(url: &str, method: Method, body: Option<&Value>) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    // let client = Client::new();
    let client = Client::builder()
        .timeout(Duration::from_secs(3)) // 举例：设置超时时间
        .build()
        .unwrap();
    let mut request = client.request(method, url);

    if let Some(b) = body {
        request = request.json(b);
    }

    let response = request.send().map_err(|e| format!("请求失败: {}", e))?;

    if response.status().is_success() {
        response.json().map_err(|e| format!("解析响应失败: {}", e))
    } else {
        Err(format!("请求失败，状态码: {}", response.status()))
    }
}
