use crate::models::auth::LoginResponse;

/// 登录逻辑处理
pub fn login(username: &str, password: &str) -> Result<LoginResponse, String> {
    if username == "admin" && password == "123456" {
        Ok(LoginResponse {
            token: "fake-token".to_string(),
        })
    } else {
        Err("用户名或密码错误".to_string())
    }
}
