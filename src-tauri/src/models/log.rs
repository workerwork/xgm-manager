use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetLogsResponse {
    pub logs: Vec<String>,  // 获取的日志
}

#[derive(Deserialize, Debug)]
pub struct ClearLogsResponse {
    pub message: String, // 清除日志的反馈信息
}
